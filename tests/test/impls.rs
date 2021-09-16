//! Tests related to the basic conversion of impls into logical predicates
//! and other core logic functions.

use super::*;

#[test]
fn prove_clone() {
    test! {
        program {
            struct Foo { }
            struct Bar { }
            struct Vec<T> { }
            trait Clone { }
            impl<T> Clone for Vec<T> where T: Clone { }
            impl Clone for Foo { }
        }

        goal {
            Vec<Foo>: Clone
        } yields {
            "Unique; substitution [], lifetime constraints []"
        }

        goal {
            Foo: Clone
        } yields {
            "Unique; substitution [], lifetime constraints []"
        }

        goal {
            Bar: Clone
        } yields {
            "No possible solution"
        }

        goal {
            Vec<Bar>: Clone
        } yields {
            "No possible solution"
        }
    }
}

/// Test that given `?0: Map<?1>` where *either* `?0` or `?1` is
/// known, we can infer the other (but if neither is known, we get an
/// ambiguous result).
///
/// In rustc today, if `?0` is not known we will not attempt to match
/// impls.
#[test]
fn prove_infer() {
    test! {
        program {
            struct Foo { }
            struct Bar { }
            trait Map<T> { }
            impl Map<Bar> for Foo { }
            impl Map<Foo> for Bar { }
        }

        goal {
            exists<A, B> { A: Map<B> }
        } yields {
            "Ambiguous; no inference guidance"
        }

        goal {
            exists<A> { A: Map<Bar> }
        } yields {
            "Unique; substitution [?0 := Foo], lifetime constraints []"
        }

        goal {
            exists<A> { Foo: Map<A> }
        } yields {
            "Unique; substitution [?0 := Bar], lifetime constraints []"
        }
    }
}

/// Test the interaction of `forall` goals and impls. For example,
/// test that we can prove things like
///
/// ```notrust
/// forall<T> { Vec<T>: Marker }
/// ```
///
/// given a suitably generic impl.
#[test]
fn prove_forall() {
    test! {
        program {
            struct Foo { }
            struct Vec<T> { }

            trait Marker { }
            impl<T> Marker for Vec<T> { }

            trait Clone { }
            impl Clone for Foo { }

            impl<T> Clone for Vec<T> where T: Clone { }
        }

        goal {
            forall<T> { T: Marker }
        } yields {
            "No possible solution"
        }

        goal {
            forall<T> { not { T: Marker } }
        } yields {
            "No"
        }

        goal {
            not { forall<T> { T: Marker } }
        } yields {
            "Unique"
        }

        // If we assume `T: Marker`, then obviously `T: Marker`.
        goal {
            forall<T> { if (T: Marker) { T: Marker } }
        } yields {
            "Unique; substitution [], lifetime constraints []"
        }

        // We don't have to know anything about `T` to know that
        // `Vec<T>: Marker`.
        goal {
            forall<T> { Vec<T>: Marker }
        } yields {
            "Unique; substitution [], lifetime constraints []"
        }

        // Here, we don't know that `T: Clone`, so we can't prove that
        // `Vec<T>: Clone`.
        goal {
            forall<T> { Vec<T>: Clone }
        } yields {
            "No possible solution"
        }

        // Here, we do know that `T: Clone`, so we can.
        goal {
            forall<T> {
                if (T: Clone) {
                    Vec<T>: Clone
                }
            }
        } yields {
            "Unique; substitution [], lifetime constraints []"
        }
    }
}

#[test]
fn higher_ranked() {
    test! {
        program {
            struct BestType { }
            struct SomeType<T> { }
            trait Foo<T> { }
            impl<U> Foo<BestType> for SomeType<U> { }
        }

        goal {
            exists<V> {
                forall<U> {
                    SomeType<U>: Foo<V>
                }
            }
        } yields {
            "Unique; substitution [?0 := BestType], lifetime constraints []"
        }
    }
}

#[test]
fn ordering() {
    test! {
        program {
            trait Foo<T> { }
            impl<U> Foo<U> for U { }
        }

        goal {
            exists<V> {
                forall<U> {
                    U: Foo<V>
                }
            }
        } yields {
            "No possible solution"
        }
    }
}

/// Demonstrates that, given the expected value of the associated
/// type, we can use that to narrow down the relevant impls.
#[test]
fn normalize_rev_infer() {
    test! {
        program {
            trait Identity { type Item; }
            struct A { }
            struct B { }
            impl Identity for A { type Item = A; }
            impl Identity for B { type Item = B; }
        }

        goal {
            exists<T> {
                T: Identity<Item = A>
            }
        } yields {
            "Unique; substitution [?0 := A]"
        }
    }
}

/// Demonstrates that, given the expected value of the associated
/// type, we can use that to narrow down the relevant impls.
#[test]
fn normalize_rev_infer_gat() {
    test! {
        program {
            trait Combine { type Item<T>; }
            struct A { }
            struct B { }
            struct Either<T, U> { }
            impl Combine for A { type Item<U> = Either<A, U>; }
            impl Combine for B { type Item<U> = Either<B, U>; }
        }

        goal {
            exists<T, U> {
                T: Combine<Item<U> = Either<A, B>>
            }
        } yields {
            // T is ?1 and U is ?0, so this is surprising, but correct! (See #126.)
            "Unique; substitution [?0 := B, ?1 := A]"
        }
    }
}

#[test]
fn generic_trait() {
    test! {
        program {
            struct Int { }
            struct Uint { }

            trait Eq<T> { }

            impl Eq<Int> for Int { }
            impl Eq<Uint> for Uint { }
        }

        goal {
            Int: Eq<Int>
        } yields {
            "Unique; substitution [], lifetime constraints []"
        }

        goal {
            Uint: Eq<Uint>
        } yields {
            "Unique; substitution [], lifetime constraints []"
        }

        goal {
            Int: Eq<Uint>
        } yields {
            "No possible solution"
        }
    }
}

#[test]
// Test that we properly detect failure even if there are applicable impls at
// the top level, if we can't find anything to fill in those impls with
fn deep_failure() {
    test! {
        program {
            struct Foo<T> {}
            trait Bar {}
            trait Baz {}

            impl<T> Bar for Foo<T> where T: Baz {}
        }

        goal {
            exists<T> { T: Baz }
        } yields {
            "No possible solution"
        }

        goal {
            exists<T> { Foo<T>: Bar }
        } yields {
            "No possible solution"
        }
    }
}

#[test]
// Test that we infer a unique solution even if it requires multiple levels of
// search to do so
fn deep_success() {
    test! {
        program {
            struct Foo<T> {}
            struct ImplsBaz {}
            trait Bar {}
            trait Baz {}

            impl Baz for ImplsBaz {}
            impl<T> Bar for Foo<T> where T: Baz {}
        }

        goal {
            exists<T> { Foo<T>: Bar }
        } yields {
            "Unique; substitution [?0 := ImplsBaz]"
        }
    }
}

#[test]
fn definite_guidance() {
    test! {
        program {
            trait Display {}
            trait Debug {}
            struct Foo<T> {}
            struct Bar {}
            struct Baz {}

            impl Display for Bar {}
            impl Display for Baz {}

            impl<T> Debug for Foo<T> where T: Display {}
        }

        goal {
            exists<T> {
                T: Debug
            }
        } yields {
            "Ambiguous; definite substitution for<?U0> { [?0 := Foo<^0.0>] }"
        }
    }
}

#[test]
fn suggested_subst() {
    test! {
        program {
            trait SomeTrait<A> {}
            struct Foo {}
            struct Bar {}
            struct Baz {}
            struct Qux {}
            impl SomeTrait<Baz> for Foo {}
            impl SomeTrait<Qux> for Bar {}
            impl SomeTrait<Baz> for Bar {}
        }

        goal {
            exists<T> {
                Foo: SomeTrait<T>
            }
        } yields {
            "Unique; substitution [?0 := Baz]"
        }

        goal {
            exists<T> {
                if (Baz: SomeTrait<Qux>) {
                    Baz: SomeTrait<T>
                }
            }
        } yields {
            "Unique; substitution [?0 := Qux]"
        }

        goal {
            exists<T> {
                if (Baz: SomeTrait<Qux>) {
                    Foo: SomeTrait<T>
                }
            }
        } yields {
            "Unique; substitution [?0 := Baz]"
        }

        goal {
            exists<T> {
                if (Foo: SomeTrait<Baz>) {
                    Foo: SomeTrait<T>
                }
            }
        } yields {
            "Unique; substitution [?0 := Baz]"
        }

        goal {
            exists<T> {
                if (Foo: SomeTrait<Qux>) {
                    Foo: SomeTrait<T>
                }
            }
        } yields {
            // FIXME: we need to rework the "favor environment" heuristic.
            // Should be: "Ambiguous; suggested substitution [?0 := bool]"
            "Ambiguous; no inference guidance"
        }

        goal {
            exists<T> {
                if (Foo: SomeTrait<bool>) {
                    if (Foo: SomeTrait<Baz>) {
                        Foo: SomeTrait<T>
                    }
                }
            }
        } yields {
            "Ambiguous; no inference guidance"
        }

        goal {
            exists<T> {
                Bar: SomeTrait<T>
            }
        } yields {
            "Ambiguous; no inference guidance"
        }

        goal {
            exists<T> {
                if (Bar: SomeTrait<Qux>) {
                    Bar: SomeTrait<T>
                }
            }
        } yields {
            // FIXME: same as above, should be: "Ambiguous; suggested substitution [?0 := bool]"
            "Ambiguous; no inference guidance"
        }

        goal {
            exists<T> {
                if (Bar: SomeTrait<Qux>) {
                    if (Bar: SomeTrait<Baz>) {
                        Bar: SomeTrait<T>
                    }
                }
            }
        } yields {
            "Ambiguous; no inference guidance"
        }
    }
}

#[test]
fn where_clause_trumps() {
    test! {
        program {
            struct Foo { }

            trait Marker { }
            impl Marker for Foo { }
        }

        goal {
            forall<T> {
                if (T: Marker) {
                    T: Marker
                }
            }
        } yields {
            "Unique"
        }
    }
}

#[test]
fn inapplicable_assumption_does_not_shadow() {
    test! {
        program {
            struct A { }
            struct B { }

            trait Foo<T> { }

            impl<T> Foo<A> for T { }
        }

        goal {
            forall<T> {
                exists<U> {
                    if (A: Foo<T>) {
                        T: Foo<U>
                    }
                }
            }
        } yields {
            "Unique"
        }
    }
}

#[test]
fn partial_overlap_2() {
    test! {
        program {
            trait Marker<T> {}
            trait Foo {}
            trait Bar {}

            struct TypeA {}
            struct TypeB {}

            impl<T> Marker<TypeA> for T where T: Foo {}
            impl<T> Marker<TypeB> for T where T: Bar {}
        }

        goal {
            forall<T> {
                if (T: Foo; T: Bar) {
                    exists<A> { T: Marker<A> }
                }
            }
        } yields {
            "Ambiguous"
        }

        goal {
            forall<T> {
                if (T: Foo; T: Bar) {
                    T: Marker<TypeB>
                }
            }
        } yields {
            "Unique"
        }

        goal {
            forall<T> {
                if (T: Foo; T: Bar) {
                    T: Marker<TypeA>
                }
            }
        } yields {
            "Unique"
        }
    }
}

#[test]
fn partial_overlap_3() {
    test! {
        program {
            #[marker] trait Marker {}
            trait Foo {}
            trait Bar {}

            impl<T> Marker for T where T: Foo {}
            impl<T> Marker for T where T: Bar {}

            struct Struct {}
            impl Foo for Struct {}
            impl Bar for Struct {}
        }

        goal {
            forall<T> {
                if (T: Foo; T: Bar) { T: Marker }
            }
        } yields {
            "Unique"
        }

        goal {
            Struct: Marker
        } yields {
            "Unique"
        }
    }
}

#[test]
fn clauses_in_if_goals() {
    test! {
        program {
            trait Foo { }
            struct Vec<T> { }
            struct A { }
        }

        goal {
            if (forall<T> { T: Foo }) {
                forall<T> { T: Foo }
            }
        } yields {
            "Unique"
        }

        goal {
            forall<T> {
                if (Vec<T>: Foo :- T: Foo) {
                    if (T: Foo) {
                        Vec<T>: Foo
                    }
                }
            }
        } yields {
            "Unique"
        }

        goal {
            if (forall<T> { Vec<T>: Foo :- T: Foo }) {
                if (A: Foo) {
                    Vec<A>: Foo
                }
            }
        } yields {
            "Unique"
        }

        goal {
            if (forall<T> { Vec<T>: Foo :- T: Foo }) {
                Vec<A>: Foo
            }
        } yields {
            "No possible solution"
        }
    }
}

#[test]
fn unify_types_in_ambiguous_impl() {
    test! {
        program {
            #[non_enumerable]
            trait Constraint {}
            trait Trait<T> {}
            struct A<T> {}
            impl<T> Trait<T> for A<T> where T: Constraint {}
        }

        goal {
            exists<T,U> { A<T>: Trait<U> }
        } yields {
            "Ambiguous; definite substitution for<?U0> { [?0 := ^0.0, ?1 := ^0.0] }"
        }
    }
}

#[test]
fn unify_types_in_impl() {
    test! {
        program {
            #[non_enumerable]
            trait Constraint {}
            trait Trait<T> {}
            struct A<T> {}
            impl<T> Trait<T> for A<T> {}
        }

        goal {
            exists<T,U> { A<T>: Trait<U> }
        } yields {
            "Unique; for<?U0> { substitution [?0 := ^0.0, ?1 := ^0.0], lifetime constraints [] }"
        }
    }
}

#[test]
fn impl_function_basic() {
    test! {
        program {
            trait Trait {
                fn a();
            }

            struct A {}
            struct B {}

            impl@Impl Trait for A {
                fn a();
            }

            impl@NotImpl Trait for B {
                fn a();
            }
        }

        goal {
            exists <F> {
                NormalizeFn(<A as Trait>::a -> F)
            }
        } yields {
            "Unique; substitution [?0 := {impl @Impl}::a], lifetime constraints []"
        }
    }
}

#[test]
fn impl_function_basic_generics() {
    test! {
        program {
            trait Trait<T> {
                fn a();
            }

            struct A<T> {}
            struct B<T> {}
            struct C {}

            impl@Impl<T> Trait<T> for A<T> {
                fn a();
            }
            impl@NotImpl<T> Trait<T> for B<T> {
                fn a();
            }
        }

        goal {
            exists <F> {
                NormalizeFn(<A<C> as Trait<C>>::a -> F)
            }
        } yields {
            "Unique; substitution [?0 := {impl @Impl}::a<C>], lifetime constraints []"
        }
    }
}

#[ignore = "broken; TODO bug in chalk-integration"]
#[test]
fn impl_function_basic_fn_generics() {
    test! {
        program {
            trait Trait<T> {
                fn a<V>(v: V);
            }

            struct A<T> {}
            struct B<T> {}
            struct C {}

            impl@Impl<T> Trait<T> for A<T> {
                fn a<Vii>(v: Vii);
            }
            impl@NotImpl<T> Trait<T> for B<T> {
                fn a<Vee>(v: Vee);
            }
        }

        goal {
            exists <F> {
                NormalizeFn(<A<C> as Trait<C>>::a<C> -> F)
            }
        } yields {
            "Unique; substitution [?0 := {impl @Impl}::a<C>], lifetime constraints []"
        }
    }
}

#[test]
fn impl_function_tautology() {
    test! {
        program {
            trait Trait<T> {
                fn a();
            }

            struct A<T> {}
            struct B<T> {}
            struct C {}

            impl@Impl<T> Trait<T> for A<T> {
                fn a();
            }
            impl@NotImpl<T> Trait<T> for B<T> {
                fn a();
            }
        }

        goal {
            NormalizeFn(<A<C> as Trait<C>>::a -> @Impl::a<C>)
        } yields {
            "Unique; substitution [], lifetime constraints []"
        }
    }
}

#[test]
fn impl_function_generic_arg() {
    test! {
        program {
            trait Trait<T> {
                fn a(v: T);
            }

            struct A<T> {}
            struct B<T> {}
            struct C {}

            impl@Impl<T> Trait<T> for A<T> {
                fn a(v: T);
            }
            impl@NotImpl<T> Trait<T> for B<T> {
                fn a(v: T);
            }
        }

        goal {
            exists <F> {
                NormalizeFn(<A<C> as Trait<C>>::a -> F)
            }
        } yields {
            "Unique; substitution [?0 := {impl @Impl}::a<C>], lifetime constraints []"
        }
    }
}

#[test]
fn impl_function_misses() {
    test! {
        program {
            trait Trait {
                fn a(v: u32);
            }

            struct A {}
            struct C {}

            impl@Impl Trait for A {
                fn a(v: u32);
            }
        }

        goal {
            exists <F> {
                NormalizeFn(<C as Trait>::a -> F)
            }
        } yields {
            "No possible solution"
        }
    }
}

#[test]
fn impl_function_where_unsat() {
    test! {
        program {
            trait Trait {
                fn a(v: u32);
            }

            trait WhereC {
            }

            struct A<T> {}
            struct C {}


            impl@Impl<T> Trait for A<T> where T: WhereC {
                fn a(v: u32);
            }
        }

        goal {
            exists <F> {
                NormalizeFn(<A<C> as Trait>::a -> F)
            }
        } yields {
            "No possible solution"
        }
    }
}

#[test]
fn impl_function_fn_where_unsat() {
    test! {
        program {
            trait Trait<T> {
                fn a(v: T) where T: WhereC;
            }

            trait WhereC {
            }

            struct A<T> {}
            struct C {}


            impl@Impl<T> Trait<T> for A<T> {
                fn a(v: T) where T: WhereC;
            }
        }

        goal {
            exists <F> {
                NormalizeFn(<A<C> as Trait<C>>::a -> F)
            }
        } yields {
            "No possible solution"
        }
    }
}
