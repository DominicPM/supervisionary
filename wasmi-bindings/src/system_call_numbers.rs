//! # System call numbers
//!
//! Defines the number (and name) of each Supervisionary system call.
//!
//! # Authors
//!
//! [Dominic Mulligan], Systems Research Group, [Arm Research] Cambridge.
//! [Nick Spinale], Systems Research Group, [Arm Research] Cambridge.
//!
//! # Copyright
//!
//! Copyright (c) Arm Limited, 2021.  All rights reserved (r).  Please see the
//! `LICENSE.markdown` file in the *Supervisionary* root directory for licensing
//! information.
//!
//! [Dominic Mulligan]: https://dominicpm.github.io
//! [Nick Spinale]: https://nickspinale.com
//! [Arm Research]: http://www.arm.com/research

/// The name of the `TypeFormer.Resolve` ABI call.
pub(crate) const ABI_TYPE_FORMER_RESOLVE_NAME: &str = "__type_former_resolve";
/// The name of the `TypeFormer.IsRegistered` ABI call.
pub(crate) const ABI_TYPE_FORMER_IS_REGISTERED_NAME: &str =
    "__type_former_is_registered";
/// The name of the `TypeFormer.Register` ABI call.
pub(crate) const ABI_TYPE_FORMER_REGISTER_NAME: &str = "__type_former_register";

/// The host-call number of the `TypeFormer.Resolve` ABI call.
pub(crate) const ABI_TYPE_FORMER_RESOLVE_INDEX: usize = 0;
/// The host-call number of the `TypeFormer.IsRegistered` ABI call.
pub(crate) const ABI_TYPE_FORMER_IS_REGISTERED_INDEX: usize = 1;
/// The host-call number of the `TypeFormer.Register` ABI call.
pub(crate) const ABI_TYPE_FORMER_REGISTER_INDEX: usize = 2;

/// The name of the `Type.IsRegistered` ABI call.
pub(crate) const ABI_TYPE_IS_REGISTERED_NAME: &str = "__type_is_registered";
/// The name of the `Type.Register.Variable` ABI call.
pub(crate) const ABI_TYPE_REGISTER_VARIABLE_NAME: &str =
    "__type_register_variable";
/// The name of the `Type.Register.Combination` ABI call.
pub(crate) const ABI_TYPE_REGISTER_COMBINATION_NAME: &str =
    "__type_register_combination";
/// The name of the `Type.Register.Function` ABI call.
pub(crate) const ABI_TYPE_REGISTER_FUNCTION_NAME: &str =
    "__type_register_function";

/// The name of the `Type.Split.Variable` ABI call.
pub(crate) const ABI_TYPE_SPLIT_VARIABLE_NAME: &str = "__type_split_variable";
/// The name of the `Type.Split.Combination` ABI call.
pub(crate) const ABI_TYPE_SPLIT_COMBINATION_NAME: &str =
    "__type_split_combination";
/// The name of the `Type.Split.Function` ABI call.
pub(crate) const ABI_TYPE_SPLIT_FUNCTION_NAME: &str = "__type_split_function";

/// The name of the `Type.Test.Variable` ABI call.
pub(crate) const ABI_TYPE_TEST_VARIABLE_NAME: &str = "__type_test_variable";
/// The name of the `Type.Test.Combination` ABI call.
pub(crate) const ABI_TYPE_TEST_COMBINATION_NAME: &str =
    "__type_test_combination";
/// The name of the `Type.Test.Function` ABI call.
pub(crate) const ABI_TYPE_TEST_FUNCTION_NAME: &str = "__type_test_function";

/// The name of the `Type.Size` ABI call.
pub(crate) const ABI_TYPE_SIZE_NAME: &str = "__type_size";
/// The name of the `Type.Variables` ABI call.
pub(crate) const ABI_TYPE_VARIABLES_NAME: &str = "__type_variables";
/// The name of the `Type.Substitute` ABI call.
pub(crate) const ABI_TYPE_SUBSTITUTE_NAME: &str = "__type_substitute";

/// The host-call number of the `Type.IsRegistered` ABI call.
pub(crate) const ABI_TYPE_IS_REGISTERED_INDEX: usize = 3;
/// The host-call number of the `Type.Register.Variable` ABI call.
pub(crate) const ABI_TYPE_REGISTER_VARIABLE_INDEX: usize = 4;
/// The host-call number of the `Type.Register.Combination` ABI call.
pub(crate) const ABI_TYPE_REGISTER_COMBINATION_INDEX: usize = 5;
/// The host-call number of the `Type.Register.Function` ABI call.
pub(crate) const ABI_TYPE_REGISTER_FUNCTION_INDEX: usize = 6;

/// The host-call number of the `Type.Split.Variable` ABI call.
pub(crate) const ABI_TYPE_SPLIT_VARIABLE_INDEX: usize = 7;
/// The host-call number of the `Type.Split.Combination` ABI call.
pub(crate) const ABI_TYPE_SPLIT_COMBINATION_INDEX: usize = 8;
/// The host-call number of the `Type.Split.Function` ABI call.
pub(crate) const ABI_TYPE_SPLIT_FUNCTION_INDEX: usize = 9;

/// The host-call number of the `Type.Test.Variable` ABI call.
pub(crate) const ABI_TYPE_TEST_VARIABLE_INDEX: usize = 10;
/// The host-call number of the `Type.Test.Combination` ABI call.
pub(crate) const ABI_TYPE_TEST_COMBINATION_INDEX: usize = 11;
/// The host-call number of the `Type.Test.Function` ABI call.
pub(crate) const ABI_TYPE_TEST_FUNCTION_INDEX: usize = 12;

/// The host-call number of the `Type.Size` ABI call.
pub(crate) const ABI_TYPE_SIZE_INDEX: usize = 13;
/// The host-call number of the `Type.Variables` ABI call.
pub(crate) const ABI_TYPE_VARIABLES_INDEX: usize = 14;
/// The host-call number of the `Type.Substitute` ABI call.
pub(crate) const ABI_TYPE_SUBSTITUTE_INDEX: usize = 15;

/// The name of the `Constant.Resolve` ABI call.
pub(crate) const ABI_CONSTANT_RESOLVE_NAME: &str = "__constant_resolve";
/// The name of the `Constant.IsRegistered` ABI call.
pub(crate) const ABI_CONSTANT_IS_REGISTERED_NAME: &str =
    "__constant_is_registered";
/// The name of the `Constant.Register` ABI call.
pub(crate) const ABI_CONSTANT_REGISTER_NAME: &str = "__constant_register";

/// The host-call number of the `Constant.Resolve` ABI call.
pub(crate) const ABI_CONSTANT_RESOLVE_INDEX: usize = 16;
/// The host-call number of the `Constant.IsRegistered` ABI call.
pub(crate) const ABI_CONSTANT_IS_REGISTERED_INDEX: usize = 17;
/// The host-call number of the `Constant.Register` ABI call.
pub(crate) const ABI_CONSTANT_REGISTER_INDEX: usize = 18;

/// The name of the `Term.IsRegistered` ABI call.
pub(crate) const ABI_TERM_IS_REGISTERED_NAME: &str = "__term_is_registered";

/// The name of the `Term.Register.Variable` ABI call.
pub(crate) const ABI_TERM_REGISTER_VARIABLE_NAME: &str =
    "__term_register_variable";
/// The name of the `Term.Register.Constant` ABI call.
pub(crate) const ABI_TERM_REGISTER_CONSTANT_NAME: &str =
    "__term_register_constant";
/// The name of the `Term.Register.Application` ABI call.
pub(crate) const ABI_TERM_REGISTER_APPLICATION_NAME: &str =
    "__term_register_application";
/// The name of the `Term.Register.Lambda` ABI call.
pub(crate) const ABI_TERM_REGISTER_LAMBDA_NAME: &str = "__term_register_lambda";
/// The name of the `Term.Register.Negation` ABI call.
pub(crate) const ABI_TERM_REGISTER_NEGATION_NAME: &str =
    "__term_register_negation";
/// The name of the `Term.Register.Conjunction` ABI call.
pub(crate) const ABI_TERM_REGISTER_CONJUNCTION_NAME: &str =
    "__term_register_conjunction";
/// The name of the `Term.Register.Disjunction` ABI call.
pub(crate) const ABI_TERM_REGISTER_DISJUNCTION_NAME: &str =
    "__term_register_disjunction";
/// The name of the `Term.Register.Implication` ABI call.
pub(crate) const ABI_TERM_REGISTER_IMPLICATION_NAME: &str =
    "__term_register_implication";
/// The name of the `Term.Register.Equality` ABI call.
pub(crate) const ABI_TERM_REGISTER_EQUALITY_NAME: &str =
    "__term_register_equality";
/// The name of the `Term.Register.Forall` ABI call.
pub(crate) const ABI_TERM_REGISTER_FORALL_NAME: &str = "__term_register_forall";
/// The name of the `Term.Register.Exists` ABI call.
pub(crate) const ABI_TERM_REGISTER_EXISTS_NAME: &str = "__term_register_exists";

/// The name of the `Term.Split.Variable` ABI call.
pub(crate) const ABI_TERM_SPLIT_VARIABLE_NAME: &str = "__term_split_variable";
/// The name of the `Term.Split.Constant` ABI call.
pub(crate) const ABI_TERM_SPLIT_CONSTANT_NAME: &str = "__term_split_constant";
/// The name of the `Term.Split.Application` ABI call.
pub(crate) const ABI_TERM_SPLIT_APPLICATION_NAME: &str =
    "__term_split_application";
/// The name of the `Term.Split.Lambda` ABI call.
pub(crate) const ABI_TERM_SPLIT_LAMBDA_NAME: &str = "__term_split_lambda";
/// The name of the `Term.Split.Negation` ABI call.
pub(crate) const ABI_TERM_SPLIT_NEGATION_NAME: &str = "__term_split_negation";
/// The name of the `Term.Split.Conjunction` ABI call.
pub(crate) const ABI_TERM_SPLIT_CONJUNCTION_NAME: &str =
    "__term_split_conjunction";
/// The name of the `Term.Split.Disjunction` ABI call.
pub(crate) const ABI_TERM_SPLIT_DISJUNCTION_NAME: &str =
    "__term_split_disjunction";
/// The name of the `Term.Split.Implication` ABI call.
pub(crate) const ABI_TERM_SPLIT_IMPLICATION_NAME: &str =
    "__term_split_implication";
/// The name of the `Term.Split.Equality` ABI call.
pub(crate) const ABI_TERM_SPLIT_EQUALITY_NAME: &str = "__term_split_equality";
/// The name of the `Term.Split.Forall` ABI call.
pub(crate) const ABI_TERM_SPLIT_FORALL_NAME: &str = "__term_split_forall";
/// The name of the `Term.Split.Exists` ABI call.
pub(crate) const ABI_TERM_SPLIT_EXISTS_NAME: &str = "__term_split_exists";

/// The name of the `Term.Test.Variable` ABI call.
pub(crate) const ABI_TERM_TEST_VARIABLE_NAME: &str = "__term_test_variable";
/// The name of the `Term.Test.Constant` ABI call.
pub(crate) const ABI_TERM_TEST_CONSTANT_NAME: &str = "__term_test_constant";
/// The name of the `Term.Test.Application` ABI call.
pub(crate) const ABI_TERM_TEST_APPLICATION_NAME: &str =
    "__term_test_application";
/// The name of the `Term.Test.Lambda` ABI call.
pub(crate) const ABI_TERM_TEST_LAMBDA_NAME: &str = "__term_test_lambda";
/// The name of the `Term.Test.Negation` ABI call.
pub(crate) const ABI_TERM_TEST_NEGATION_NAME: &str = "__term_test_negation";
/// The name of the `Term.Test.Conjunction` ABI call.
pub(crate) const ABI_TERM_TEST_CONJUNCTION_NAME: &str =
    "__term_test_conjunction";
/// The name of the `Term.Test.Disjunction` ABI call.
pub(crate) const ABI_TERM_TEST_DISJUNCTION_NAME: &str =
    "__term_test_disjunction";
/// The name of the `Term.Test.Implication` ABI call.
pub(crate) const ABI_TERM_TEST_IMPLICATION_NAME: &str =
    "__term_test_implication";
/// The name of the `Term.Test.Equality` ABI call.
pub(crate) const ABI_TERM_TEST_EQUALITY_NAME: &str = "__term_test_equality";
/// The name of the `Term.Test.Forall` ABI call.
pub(crate) const ABI_TERM_TEST_FORALL_NAME: &str = "__term_test_forall";
/// The name of the `Term.Test.Exists` ABI call.
pub(crate) const ABI_TERM_TEST_EXISTS_NAME: &str = "__term_test_exists";

/// The name of the `Term.FreeVariables` ABI call.
pub(crate) const ABI_TERM_FREE_VARIABLES_NAME: &str = "__term_free_variables";
/// The name of the `Term.Substitution` ABI call.
pub(crate) const ABI_TERM_SUBSTITUTE_NAME: &str = "__term_substitute";

/// The name of the `Term.Type.Variables` ABI call.
pub(crate) const ABI_TERM_TYPE_VARIABLES_NAME: &str = "__term_type_variables";
/// The name of the `Term.Type.Substitution` ABI call.
pub(crate) const ABI_TERM_TYPE_SUBSTITUTE_NAME: &str = "__term_type_substitute";
/// The name of the `Term.Type.Infer` ABI call.
pub(crate) const ABI_TERM_TYPE_INFER_NAME: &str = "__term_type_infer";
/// The name of the `Term.Type.IsProposition` ABI call.
pub(crate) const ABI_TERM_TYPE_IS_PROPOSITION_NAME: &str =
    "__term_type_is_proposition";

/// The host-call number of the `Term.IsRegistered` ABI call.
pub(crate) const ABI_TERM_IS_REGISTERED_INDEX: usize = 19;

/// The host-call number of the `Term.Register.Variable` ABI call.
pub(crate) const ABI_TERM_REGISTER_VARIABLE_INDEX: usize = 20;
/// The host-call number of the `Term.Register.Constant` ABI call.
pub(crate) const ABI_TERM_REGISTER_CONSTANT_INDEX: usize = 21;
/// The host-call number of the `Term.Register.Application` ABI call.
pub(crate) const ABI_TERM_REGISTER_APPLICATION_INDEX: usize = 22;
/// The host-call number of the `Term.Register.Lambda` ABI call.
pub(crate) const ABI_TERM_REGISTER_LAMBDA_INDEX: usize = 23;
/// The host-call number of the `Term.Register.Negation` ABI call.
pub(crate) const ABI_TERM_REGISTER_NEGATION_INDEX: usize = 24;
/// The host-call number of the `Term.Register.Conjunction` ABI call.
pub(crate) const ABI_TERM_REGISTER_CONJUNCTION_INDEX: usize = 25;
/// The host-call number of the `Term.Register.Disjunction` ABI call.
pub(crate) const ABI_TERM_REGISTER_DISJUNCTION_INDEX: usize = 26;
/// The host-call number of the `Term.Register.Implication` ABI call.
pub(crate) const ABI_TERM_REGISTER_IMPLICATION_INDEX: usize = 27;
/// The host-call number of the `Term.Register.Equality` ABI call.
pub(crate) const ABI_TERM_REGISTER_EQUALITY_INDEX: usize = 28;
/// The host-call number of the `Term.Register.Forall` ABI call.
pub(crate) const ABI_TERM_REGISTER_FORALL_INDEX: usize = 29;
/// The host-call number of the `Term.Register.Exists` ABI call.
pub(crate) const ABI_TERM_REGISTER_EXISTS_INDEX: usize = 30;

/// The host-call number of the `Term.Split.Variable` ABI call.
pub(crate) const ABI_TERM_SPLIT_VARIABLE_INDEX: usize = 31;
/// The host-call number of the `Term.Split.Constant` ABI call.
pub(crate) const ABI_TERM_SPLIT_CONSTANT_INDEX: usize = 32;
/// The host-call number of the `Term.Split.Application` ABI call.
pub(crate) const ABI_TERM_SPLIT_APPLICATION_INDEX: usize = 33;
/// The host-call number of the `Term.Split.Lambda` ABI call.
pub(crate) const ABI_TERM_SPLIT_LAMBDA_INDEX: usize = 34;
/// The host-call number of the `Term.Split.Negation` ABI call.
pub(crate) const ABI_TERM_SPLIT_NEGATION_INDEX: usize = 35;
/// The host-call number of the `Term.Split.Conjunction` ABI call.
pub(crate) const ABI_TERM_SPLIT_CONJUNCTION_INDEX: usize = 36;
/// The host-call number of the `Term.Split.Disjunction` ABI call.
pub(crate) const ABI_TERM_SPLIT_DISJUNCTION_INDEX: usize = 37;
/// The host-call number of the `Term.Split.Implication` ABI call.
pub(crate) const ABI_TERM_SPLIT_IMPLICATION_INDEX: usize = 38;
/// The host-call number of the `Term.Split.Equality` ABI call.
pub(crate) const ABI_TERM_SPLIT_EQUALITY_INDEX: usize = 39;
/// The host-call number of the `Term.Split.Forall` ABI call.
pub(crate) const ABI_TERM_SPLIT_FORALL_INDEX: usize = 40;
/// The host-call number of the `Term.Split.Exists` ABI call.
pub(crate) const ABI_TERM_SPLIT_EXISTS_INDEX: usize = 41;

/// The host-call number of the `Term.Test.Variable` ABI call.
pub(crate) const ABI_TERM_TEST_VARIABLE_INDEX: usize = 42;
/// The host-call number of the `Term.Test.Constant` ABI call.
pub(crate) const ABI_TERM_TEST_CONSTANT_INDEX: usize = 43;
/// The host-call number of the `Term.Test.Application` ABI call.
pub(crate) const ABI_TERM_TEST_APPLICATION_INDEX: usize = 44;
/// The host-call number of the `Term.Test.Lambda` ABI call.
pub(crate) const ABI_TERM_TEST_LAMBDA_INDEX: usize = 45;
/// The host-call number of the `Term.Test.Negation` ABI call.
pub(crate) const ABI_TERM_TEST_NEGATION_INDEX: usize = 46;
/// The host-call number of the `Term.Test.Conjunction` ABI call.
pub(crate) const ABI_TERM_TEST_CONJUNCTION_INDEX: usize = 47;
/// The host-call number of the `Term.Test.Disjunction` ABI call.
pub(crate) const ABI_TERM_TEST_DISJUNCTION_INDEX: usize = 48;
/// The host-call number of the `Term.Test.Implication` ABI call.
pub(crate) const ABI_TERM_TEST_IMPLICATION_INDEX: usize = 49;
/// The host-call number of the `Term.Test.Equality` ABI call.
pub(crate) const ABI_TERM_TEST_EQUALITY_INDEX: usize = 50;
/// The host-call number of the `Term.Test.Forall` ABI call.
pub(crate) const ABI_TERM_TEST_FORALL_INDEX: usize = 51;
/// The host-call number of the `Term.Test.Exists` ABI call.
pub(crate) const ABI_TERM_TEST_EXISTS_INDEX: usize = 52;

/// The host-call number of the `Term.FreeVariables` ABI call.
pub(crate) const ABI_TERM_FREE_VARIABLES_INDEX: usize = 53;
/// The host-call number of the `Term.Substitute` ABI call.
pub(crate) const ABI_TERM_SUBSTITUTE_INDEX: usize = 54;

/// The host-call number of the `Term.Type.Variables` ABI call.
pub(crate) const ABI_TERM_TYPE_VARIABLES_INDEX: usize = 55;
/// The host-call number of the `Term.Type.Substitute` ABI call.
pub(crate) const ABI_TERM_TYPE_SUBSTITUTE_INDEX: usize = 56;
/// The host-call number of the `Term.Type.Infer` ABI call.
pub(crate) const ABI_TERM_TYPE_INFER_INDEX: usize = 57;
/// The host-call number of the `Term.Type.IsProposition` ABI call.
pub(crate) const ABI_TERM_TYPE_IS_PROPOSITION_INDEX: usize = 58;

/// The name of the `Theorem.IsRegistered` ABI call.
pub(crate) const ABI_THEOREM_IS_REGISTERED_NAME: &str =
    "__theorem_is_registered";

/// The name of the `Theorem.Register.Assumption` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_ASSUMPTION_NAME: &str =
    "__theorem_register_assumption";
/// The name of the `Theorem.Register.Weaken` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_WEAKEN_NAME: &str =
    "__theorem_register_weaken";

/// The name of the `Theorem.Register.Reflexivity` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_REFLEXIVITY_NAME: &str =
    "__theorem_register_reflexivity";
/// The name of the `Theorem.Register.Symmetry` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_SYMMETRY_NAME: &str =
    "__theorem_register_symmetry";
/// The name of the `Theorem.Register.Transitivity` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_TRANSITIVITY_NAME: &str =
    "__theorem_register_transitivity";
/// The name of the `Theorem.Register.Beta` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_BETA_NAME: &str =
    "__theorem_register_beta";
/// The name of the `Theorem.Register.Eta` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_ETA_NAME: &str = "__theorem_register_eta";
/// The name of the `Theorem.Register.Application` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_APPLICATION_NAME: &str =
    "__theorem_register_application";
/// The name of the `Theorem.Register.Lambda` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_LAMBDA_NAME: &str =
    "__theorem_register_lambda";

/// The name of the `Theorem.Register.Substitute` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_SUBSTITUTE_NAME: &str =
    "__theorem_register_substitute";
/// The name of the `Theorem.Register.TypeSubstitute` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_TYPE_SUBSTITUTE_NAME: &str =
    "__theorem_register_type_substitute";

/// The name of the `Theorem.Register.TruthIntroduction` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_TRUTH_INTRODUCTION_NAME: &str =
    "__theorem_register_truth_introduction";
/// The name of the `Theorem.Register.FalsityElimination` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_FALSITY_ELIMINATION_NAME: &str =
    "__theorem_register_falsity_elimination";

/// The name of the `Theorem.Register.ConjunctionIntroduction` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_CONJUNCTION_INTRODUCTION_NAME: &str =
    "__theorem_register_conjunction_introduction";
/// The name of the `Theorem.Register.ConjunctionLeftElimination` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_CONJUNCTION_LEFT_ELIMINATION_NAME: &str =
    "__theorem_register_conjunction_left_elimination";
/// The name of the `Theorem.Register.ConjunctionRightElimination` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_CONJUNCTION_RIGHT_ELIMINATION_NAME: &str =
    "__theorem_register_conjunction_right_elimination";

/// The name of the `Theorem.Register.DisjunctionElimination` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_DISJUNCTION_ELIMINATION_NAME: &str =
    "__theorem_register_disjunction_elimination";
/// The name of the `Theorem.Register.DisjunctionLeftIntroduction` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_DISJUNCTION_LEFT_INTRODUCTION_NAME: &str =
    "__theorem_register_disjunction_left_introduction";
/// The name of the `Theorem.Register.DisjunctionRightIntroduction` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_DISJUNCTION_RIGHT_INTRODUCTION_NAME:
    &str = "__theorem_register_disjunction_right_introduction";

/// The name of the `Theorem.Register.ImplicationIntroduction` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_IMPLICATION_INTRODUCTION_NAME: &str =
    "__theorem_register_implication_introduction";
/// The name of the `Theorem.Register.ImplicationElimination` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_IMPLICATION_ELIMINATION_NAME: &str =
    "__theorem_register_implication_elimination";

/// The name of the `Theorem.Register.IffIntroduction` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_IFF_INTRODUCTION_NAME: &str =
    "__theorem_register_iff_elimination";
/// The name of the `Theorem.Register.IffLeftElimination` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_IFF_LEFT_ELIMINATION_NAME: &str =
    "__theorem_register_iff_left_elimination";

/// The name of the `Theorem.Register.NegationIntroduction` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_NEGATION_INTRODUCTION_NAME: &str =
    "__theorem_register_negation_introduction";
/// The name of the `Theorem.Register.NegationElimination` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_NEGATION_ELIMINATION_NAME: &str =
    "__theorem_register_negation_elimination";

/// The name of the `Theorem.Register.ForallIntroduction` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_FORALL_INTRODUCTION_NAME: &str =
    "__theorem_register_forall_introduction";
/// The name of the `Theorem.Register.ForallElimination` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_FORALL_ELIMINATION_NAME: &str =
    "__theorem_register_forall_elimination";
/// The name of the `Theorem.Register.ExistsIntroduction` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_EXISTS_INTRODUCTION_NAME: &str =
    "__theorem_register_exists_introduction";
/// The name of the `Theorem.Register.ExistsElimination` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_EXISTS_ELIMINATION_NAME: &str =
    "__theorem_register_exists_elimination";

/// The name of the `Theorem.Split.Hypotheses` ABI call.
pub(crate) const ABI_THEOREM_SPLIT_HYPOTHESES_NAME: &str =
    "__theorem_split_hypotheses";
/// The name of the `Theorem.Split.Conclusion` ABI call.
pub(crate) const ABI_THEOREM_SPLIT_CONCLUSION_NAME: &str =
    "__theorem_split_conclusion";

/// The index of the `Theorem.IsRegistered` ABI call.
pub(crate) const ABI_THEOREM_IS_REGISTERED_INDEX: usize = 59;

/// The index of the `Theorem.Register.Assumption` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_ASSUMPTION_INDEX: usize = 60;
/// The index of the `Theorem.Register.Weaken` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_WEAKEN_INDEX: usize = 61;

/// The index of the `Theorem.Register.Reflexivity` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_REFLEXIVITY_INDEX: usize = 62;
/// The index of the `Theorem.Register.Symmetry` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_SYMMETRY_INDEX: usize = 63;
/// The index of the `Theorem.Register.Transitivity` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_TRANSITIVITY_INDEX: usize = 64;
/// The index of the `Theorem.Register.Beta` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_BETA_INDEX: usize = 65;
/// The index of the `Theorem.Register.Eta` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_ETA_INDEX: usize = 66;
/// The index of the `Theorem.Register.Application` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_APPLICATION_INDEX: usize = 67;
/// The index of the `Theorem.Register.Lambda` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_LAMBDA_INDEX: usize = 68;

/// The index of the `Theorem.Register.Substitute` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_SUBSTITUTE_INDEX: usize = 69;
/// The index of the `Theorem.Register.TypeSubstitute` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_TYPE_SUBSTITUTE_INDEX: usize = 70;

/// The index of the `Theorem.Register.TruthIntroduction` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_TRUTH_INTRODUCTION_INDEX: usize = 71;
/// The index of the `Theorem.Register.FalsityElimination` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_FALSITY_ELIMINATION_INDEX: usize = 72;

/// The index of the `Theorem.Register.ConjunctionIntroduction` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_CONJUNCTION_INTRODUCTION_INDEX: usize =
    73;
/// The index of the `Theorem.Register.ConjunctionLeftElimination` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_CONJUNCTION_LEFT_ELIMINATION_INDEX:
    usize = 74;
/// The index of the `Theorem.Register.ConjunctionRightElimination` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_CONJUNCTION_RIGHT_ELIMINATION_INDEX:
    usize = 75;

/// The index of the `Theorem.Register.DisjunctionElimination` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_DISJUNCTION_ELIMINATION_INDEX: usize = 76;
/// The index of the `Theorem.Register.DisjunctionLeftIntroduction` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_DISJUNCTION_LEFT_INTRODUCTION_INDEX:
    usize = 77;
/// The index of the `Theorem.Register.DisjunctionRightIntroduction` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_DISJUNCTION_RIGHT_INTRODUCTION_INDEX:
    usize = 78;

/// The index of the `Theorem.Register.ImplicationIntroduction` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_IMPLICATION_INTRODUCTION_INDEX: usize =
    79;
/// The index of the `Theorem.Register.ImplicationElimination` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_IMPLICATION_ELIMINATION_INDEX: usize = 80;

/// The index of the `Theorem.Register.IffIntroduction` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_IFF_INTRODUCTION_INDEX: usize = 81;
/// The index of the `Theorem.Register.IffLeftElimination` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_IFF_LEFT_ELIMINATION_INDEX: usize = 82;

/// The index of the `Theorem.Register.NegationIntroduction` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_NEGATION_INTRODUCTION_INDEX: usize = 83;
/// The index of the `Theorem.Register.NegationElimination` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_NEGATION_ELIMINATION_INDEX: usize = 84;

/// The index of the `Theorem.Register.ForallIntroduction` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_FORALL_INTRODUCTION_INDEX: usize = 85;
/// The index of the `Theorem.Register.ForallElimination` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_FORALL_ELIMINATION_INDEX: usize = 86;
/// The index of the `Theorem.Register.ExistsIntroduction` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_EXISTS_INTRODUCTION_INDEX: usize = 87;
/// The index of the `Theorem.Register.ExistsElimination` ABI call.
pub(crate) const ABI_THEOREM_REGISTER_EXISTS_ELIMINATION_INDEX: usize = 88;

/// The index of the `Theorem.Split.Hypotheses` ABI call.
pub(crate) const ABI_THEOREM_SPLIT_HYPOTHESES_INDEX: usize = 89;
/// The index of the `Theorem.Split.Conclusion` ABI call.
pub(crate) const ABI_THEOREM_SPLIT_CONCLUSION_INDEX: usize = 90;
