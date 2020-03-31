use crate::coherence::{CoherenceError, CoherenceSolver};
use crate::ext::*;
use crate::{goal_builder::GoalBuilder, Solution};
use chalk_ir::cast::*;
use chalk_ir::fold::shift::Shift;
use chalk_ir::interner::Interner;
use chalk_ir::*;
use chalk_rust_ir::*;
use itertools::Itertools;

impl<I: Interner> CoherenceSolver<'_, I> {
    pub(super) fn visit_specializations_of_trait(
        &self,
        mut record_specialization: impl FnMut(ImplId<I>, ImplId<I>),
    ) -> Result<(), CoherenceError<I>> {
        // Ignore impls for marker traits as they are allowed to overlap.
        let trait_datum = self.db.trait_datum(self.trait_id);
        if trait_datum.flags.marker {
            return Ok(());
        }

        // Iterate over every pair of impls for the same trait.
        let impls = self.db.local_impls_to_coherence_check(self.trait_id);
        for (l_id, r_id) in impls.into_iter().tuple_combinations() {
            let lhs = &self.db.impl_datum(l_id);
            let rhs = &self.db.impl_datum(r_id);

            // Two negative impls never overlap.
            if !lhs.is_positive() && !rhs.is_positive() {
                continue;
            }

            // Check if the impls overlap, then if they do, check if one specializes
            // the other. Note that specialization can only run one way - if both
            // specialization checks return *either* true or false, that's an error.
            if !self.disjoint(lhs, rhs) {
                match (self.specializes(l_id, r_id), self.specializes(r_id, l_id)) {
                    (true, false) => record_specialization(l_id, r_id),
                    (false, true) => record_specialization(r_id, l_id),
                    (_, _) => {
                        Err(CoherenceError::OverlappingImpls(self.trait_id))?;
                    }
                }
            }
        }

        Ok(())
    }

    // Test if the set of types that these two impls apply to overlap. If the test succeeds, these
    // two impls are disjoint.
    //
    // We combine the binders of the two impls & treat them as existential quantifiers. Then we
    // attempt to unify the input types to the trait provided by each impl, as well as prove that
    // the where clauses from both impls all hold. At the end, we apply the `compatible` modality
    // and negate the query. Negating the query means that we are asking chalk to prove that no
    // such overlapping impl exists. By applying `compatible { G }`, chalk attempts to prove that
    // "there exists a compatible world where G is provable." When we negate compatible, it turns
    // into the statement "for all compatible worlds, G is not provable." This is exactly what we
    // want since we want to ensure that there is no overlap in *all* compatible worlds, not just
    // that there is no overlap in *some* compatible world.
    //
    // Examples:
    //
    //  Impls:
    //      impl<T> Foo for T { }   // rhs
    //      impl Foo for i32 { }    // lhs
    //  Generates:
    //      not { compatible { exists<T> { exists<> { T = i32 } } } }
    //
    //  Impls:
    //      impl<T1, U> Foo<T1> for Vec<U> { }  // rhs
    //      impl<T2> Foo<T2> for Vec<i32> { }   // lhs
    //  Generates:
    //      not { compatible { exists<T1, U> { exists<T2> { Vec<U> = Vec<i32>, T1 = T2 } } } }
    //
    //  Impls:
    //      impl<T> Foo for Vec<T> where T: Bar { }
    //      impl<U> Foo for Vec<U> where U: Baz { }
    //  Generates:
    //      not { compatible { exists<T> { exists<U> { Vec<T> = Vec<U>, T: Bar, U: Baz } } } }
    //
    fn disjoint(&self, lhs: &ImplDatum<I>, rhs: &ImplDatum<I>) -> bool {
        debug_heading!("overlaps(lhs={:#?}, rhs={:#?})", lhs, rhs);

        let interner = self.db.interner();

        // Upshift the rhs variables in params to account for the joined binders
        let lhs_params = params(interner, lhs).iter().cloned();
        let rhs_params = params(interner, rhs)
            .iter()
            .map(|param| param.shifted_in(interner));

        // Create an equality goal for every input type the trait, attempting
        // to unify the inputs to both impls with one another
        let params_goals = lhs_params
            .zip(rhs_params)
            .map(|(a, b)| GoalData::EqGoal(EqGoal { a, b }).intern(interner));

        // Upshift the rhs variables in where clauses
        let lhs_where_clauses = lhs.binders.value.where_clauses.iter().cloned();
        let rhs_where_clauses = rhs
            .binders
            .value
            .where_clauses
            .iter()
            .map(|wc| wc.shifted_in(interner));

        // Create a goal for each clause in both where clauses
        let wc_goals = lhs_where_clauses
            .chain(rhs_where_clauses)
            .map(|wc| wc.cast(interner));

        // Join all the goals we've created together with And, then quantify them
        // over the joined binders. This is our query.
        let goal = Box::new(Goal::all(interner, params_goals.chain(wc_goals)))
            .quantify(
                interner,
                QuantifierKind::Exists,
                lhs.binders.binders.clone(),
            )
            .quantify(
                interner,
                QuantifierKind::Exists,
                rhs.binders.binders.clone(),
            )
            .compatible(interner)
            .negate(interner);

        let canonical_goal = &goal.into_closed_goal(interner);
        let solution = self
            .solver_choice
            .into_solver()
            .solve(self.db, canonical_goal);
        let result = match solution {
            // Goal was proven with a unique solution, so no impl was found that causes these two
            // to overlap
            Some(Solution::Unique(_)) => true,
            // Goal was ambiguous, so there *may* be overlap
            Some(Solution::Ambig(_)) |
            // Goal cannot be proven, so there is some impl that causes overlap
            None => false,
        };
        debug!("overlaps: result = {:?}", result);
        result
    }

    // Creates a goal which, if provable, means "more special" impl specializes the "less special" one.
    //
    // # General rule
    //
    // Given the more special impl:
    //
    // ```ignore
    // impl<P0..Pn> SomeTrait<T1..Tm> for T0 where WC_more
    // ```
    //
    // and less special impl
    //
    // ```ignore
    // impl<Q0..Qo> SomeTrait<U1..Um> for U0 where WC_less
    // ```
    //
    // create the goal:
    //
    // ```ignore
    // forall<P0..Pn> {
    //   if (WC_more) {}
    //     exists<Q0..Qo> {
    //       T0 = U0, ..., Tm = Um,
    //       WC_less
    //     }
    //   }
    // }
    // ```
    //
    // # Example
    //
    // Given:
    //
    // * more: `impl<T: Clone> Foo for Vec<T>`
    // * less: `impl<U: Clone> Foo for U`
    //
    // Resulting goal:
    //
    // ```ignore
    // forall<T> {
    //  if (T: Clone) {
    //    exists<U> {
    //      Vec<T> = U, U: Clone
    //    }
    //  }
    // }
    // ```
    fn specializes(&self, less_special_id: ImplId<I>, more_special_id: ImplId<I>) -> bool {
        let more_special = &self.db.impl_datum(more_special_id);
        let less_special = &self.db.impl_datum(less_special_id);
        debug_heading!(
            "specializes(less_special={:#?}, more_special={:#?})",
            less_special,
            more_special
        );

        let interner = self.db.interner();

        let gb = &mut GoalBuilder::new(self.db);

        // forall<P0..Pn> { ... }
        let goal = gb.forall(
            &more_special.binders,
            less_special_id,
            |gb, _, more_special_impl, less_special_id| {
                // if (WC_more) { ... }
                gb.implies(more_special_impl.where_clauses.iter().cloned(), |gb| {
                    let less_special = &gb.db().impl_datum(less_special_id);

                    // exists<Q0..Qn> { ... }
                    gb.exists(
                        &less_special.binders,
                        &more_special_impl.trait_ref,
                        |gb, _, less_special_impl, more_special_trait_ref| {
                            let interner = gb.interner();

                            // T0 = U0, ..., Tm = Um
                            let params_goals = more_special_trait_ref
                                .substitution
                                .parameters(interner)
                                .iter()
                                .cloned()
                                .zip(
                                    less_special_impl
                                        .trait_ref
                                        .substitution
                                        .parameters(interner)
                                        .iter()
                                        .cloned(),
                                )
                                .map(|(a, b)| GoalData::EqGoal(EqGoal { a, b }).intern(interner));

                            // <less_special_wc_goals> = where clauses from the less special impl
                            let less_special_wc_goals = less_special_impl
                                .where_clauses
                                .iter()
                                .cloned()
                                .casted(interner);

                            // <equality_goals> && WC_less
                            gb.all(params_goals.chain(less_special_wc_goals))
                        },
                    )
                })
            },
        );

        let canonical_goal = &goal.into_closed_goal(interner);
        let result = match self
            .solver_choice
            .into_solver()
            .solve(self.db, canonical_goal)
        {
            Some(sol) => sol.is_unique(),
            None => false,
        };

        debug!("specializes: result = {:?}", result);

        result
    }
}

fn params<'a, I: Interner>(interner: &I, impl_datum: &'a ImplDatum<I>) -> &'a [Parameter<I>] {
    impl_datum
        .binders
        .value
        .trait_ref
        .substitution
        .parameters(interner)
}
