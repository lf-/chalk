initSidebarItems({"enum":[["AliasTy","An alias, which is a trait indirection such as a projection or opaque type."],["ClausePriority","Specifies how important an implication is."],["ConstValue","A constant value, not necessarily concrete."],["Constraint","A constraint on lifetimes."],["DomainGoal","A “domain goal” is a goal that is directly about Rust, rather than a pure logical statement. As much as possible, the Chalk solver should avoid decomposing this enum, and instead treat its values opaquely."],["FallibleOrFloundered","A combination of `Fallible` and `Floundered`."],["FloatTy","Different kinds of float types."],["FromEnv","Checks whether a type or trait ref can be derived from the contents of the environment."],["GenericArgData","Generic arguments data."],["GoalData","A general goal; this is the full range of questions you can pose to Chalk."],["IntTy","Different signed int types."],["LifetimeData","Lifetime data, including what kind of lifetime it is and what it points to."],["Mutability","Whether a type is mutable or not."],["QuantifierKind","Kinds of quantifiers in the logic, such as `forall` and `exists`."],["Safety","Whether a function is safe or not."],["Scalar","Types of scalar values."],["TyKind","Type data, which holds the actual type information."],["TyVariableKind","Represents some extra knowledge we may have about the type variable."],["UintTy","Different unsigned int types."],["VariableKind","The “kind” of variable. Type, lifetime or constant."],["Variance","Variance"],["Void","Uninhabited (empty) type, used in combination with `PhantomData`."],["WellFormed","Checks whether a type or trait ref is well-formed."],["WhereClause","Where clauses that can be written by a Rust programmer."]],"macro":[["try_break","Unwraps a `ControlFlow` or propagates its `Break` value. This replaces the `Try` implementation that would be used with `std::ops::ControlFlow`."]],"mod":[["cast","Upcasts, to avoid writing out wrapper types."],["could_match","Fast matching check for zippable values."],["debug","Debug impls for types."],["fold","Traits for transforming bits of IR."],["interner","Encapsulates the concrete representation of core types such as types and goals."],["visit","Traits for visiting bits of IR."],["zip","Traits for “zipping” types, walking through two structures and checking that they match."]],"struct":[["AdtId","The id for an Abstract Data Type (i.e. structs, unions and enums)."],["AliasEq","Proves equality between an alias and a type."],["AnswerSubst","The resulting substitution after solving a goal."],["AssocTypeId","The id for the associated type member of a trait. The details of the type can be found by invoking the `associated_ty_data` method."],["Binders","Indicates that the `value` is universally quantified over `N` parameters of the given kinds, where `N == self.binders.len()`. A variable with depth `i < N` refers to the value at `self.binders[i]`. Variables with depth `>= N` are free."],["BindersIntoIterator","`IntoIterator` for binders."],["BoundVar","Identifies a particular bound variable within a binder. Variables are identified by the combination of a [`DebruijnIndex`], which identifies the binder, and an index within that binder."],["Canonical","Wraps a “canonicalized item”. Items are canonicalized as follows:"],["CanonicalVarKinds","List of interned elements."],["ClauseId","Id for a specific clause."],["ClosureId","Id for Rust closures."],["ConcreteConst","Concrete constant, whose value is known (as opposed to inferred constants and placeholders)."],["Const","Constants."],["ConstData","Constant data, containing the constant’s type and value."],["ConstrainedSubst","Combines a substitution (`subst`) with a set of region constraints (`constraints`). This represents the result of a query; the substitution stores the values for the query’s unknown variables, and the constraints represents any region constraints that must additionally be solved."],["Constraints","List of interned elements."],["DebruijnIndex","References the binder at the given depth. The index is a de Bruijn index, so it counts back through the in-scope binders, with 0 being the innermost binder. This is used in impls and the like. For example, if we had a rule like `for<T> { (T: Clone) :- (T: Copy) }`, then `T` would be represented as a `BoundVar(0)` (as the `for` is the innermost binder)."],["DynTy","A “DynTy” represents a trait object (`dyn Trait`). Trait objects are conceptually very related to an “existential type” of the form `exists<T> { T: Trait }` (another example of such type is `impl Trait`). `DynTy` represents the bounds on that type."],["Environment","The set of assumptions we’ve made so far, and the current number of universal (forall) quantifiers we’re within."],["EqGoal","Equality goal: tries to prove that two values are equal."],["Floundered","Indicates that the complete set of program clauses for this goal cannot be enumerated."],["FnDefId","Function definition id."],["FnPointer","for<’a…’z> X – all binders are instantiated at once, and we use deBruijn indices within `self.ty`"],["FnSig","A function signature."],["FnSubst","A wrapper for the substs on a Fn."],["ForeignDefId","Id for foreign types."],["GeneratorId","Id for Rust generators."],["GenericArg","A generic argument, see `GenericArgData` for more information."],["Goal","A general goal; this is the full range of questions you can pose to Chalk."],["Goals","List of interned elements."],["ImplId","The id for an impl."],["InEnvironment","A goal with an environment to solve it in."],["InferenceVar","A type, lifetime or constant whose value is being inferred."],["Lifetime","A Rust lifetime."],["LifetimeOutlives","Lifetime outlives, which for `'a: 'b`` checks that the lifetime `’a`is a superset of the value of`’b`."],["NoSolution","Indicates that the attempted operation has “no solution” – i.e., cannot be performed."],["Normalize","Proves that the given type alias normalizes to the given type. A projection `T::Foo` normalizes to the type `U` if we can match it to an impl and that impl has a `type Foo = V` where `U = V`."],["OpaqueTy","An opaque type `opaque type T<..>: Trait = HiddenTy`."],["OpaqueTyId","Id for an opaque type."],["PlaceholderIndex","Index of an universally quantified parameter in the environment. Two indexes are required, the one of the universe itself and the relative index inside the universe."],["ProgramClause","A program clause is a logic expression used to describe a part of the program."],["ProgramClauseData","Contains the data for a program clause."],["ProgramClauseImplication","Represents one clause of the form `consequence :- conditions` where `conditions = cond_1 && cond_2 && ...` is the conjunction of the individual conditions."],["ProgramClauses","List of interned elements."],["ProjectionTy","A projection `<P0 as TraitName<P1..Pn>>::AssocItem<Pn+1..Pm>`."],["QuantifiedWhereClauses","List of interned elements."],["SubstFolder",""],["Substitution","List of interned elements."],["SubtypeGoal","Subtype goal: tries to prove that `a` is a subtype of `b`"],["TraitId","The id of a trait definition; could be used to load the trait datum by invoking the `trait_datum` method."],["TraitRef","A trait reference describes the relationship between a type and a trait. This can be used in two forms:"],["Ty","A Rust type. The actual type data is stored in `TyKind`."],["TyData","Contains the data for a Ty"],["TypeFlags","Contains flags indicating various properties of a Ty"],["TypeOutlives","Type outlives, which for `T: 'a` checks that the type `T` lives at least as long as the lifetime `'a`"],["UCanonical","A “universe canonical” value. This is a wrapper around a `Canonical`, indicating that the universes within have been “renumbered” to start from 0 and collapse unimportant distinctions."],["UniverseIndex","An universe index is how a universally quantified parameter is represented when it’s binder is moved into the environment. An example chain of transformations would be: `forall<T> { Goal(T) }` (syntactical representation) `forall { Goal(?0) }` (used a DeBruijn index) `Goal(!U1)` (the quantifier was moved to the environment and replaced with a universe index) See https://rustc-dev-guide.rust-lang.org/borrow_check/region_inference.html#placeholders-and-universes for more."],["UniverseMap","Maps the universes found in the `u_canonicalize` result (the “canonical” universes) to the universes found in the original value (and vice versa). When used as a folder – i.e., from outside this module – converts from “canonical” universes to the original (but see the `UMapToCanonical` folder)."],["VariableKinds","List of interned elements."],["Variances","List of interned elements."],["WithKind","A value with an associated variable kind."]],"trait":[["AsParameters","Convert a value to a list of parameters."],["Substitute","An extension trait to anything that can be represented as list of `GenericArg`s that signifies that it can applied as a substituion to a value"],["ToGenericArg","Utility for converting a list of all the binders into scope into references to those binders. Simply pair the binders with the indices, and invoke `to_generic_arg()` on the `(binder, index)` pair. The result will be a reference to a bound variable of appropriate kind at the corresponding index."],["UnificationDatabase","Logic to decide the Variance for a given subst"]],"type":[["CanonicalVarKind","A variable kind with universe index."],["Fallible","Many of our internal operations (e.g., unification) are an attempt to perform some operation which may not complete."],["QuantifiedWhereClause","A where clause that can contain `forall<>` or `exists<>` quantifiers."]]});