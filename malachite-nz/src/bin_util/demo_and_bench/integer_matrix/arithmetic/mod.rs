// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    add::register(runner);
    add_mul::register(runner);
    balanced_mod::register(runner);
    balanced_mod_power_of_2::register(runner);
    can_solve_multi_mod::register(runner);
    canonical_primitive_part::register(runner);
    certify_lu_mod_p::register(runner);
    content::register(runner);
    content_chained::register(runner);
    crt::register(runner);
    determinant::register(runner);
    determinant_bound::register(runner);
    determinant_divisor::register(runner);
    determinant_sign::register(runner);
    div_exact::register(runner);
    elementary_divisors::register(runner);
    entrywise_shl_round::register(runner);
    entrywise_shr_round::register(runner);
    fraction_free_lu_decomposition::register(runner);
    hermite_normal_form::register(runner);
    hermite_normal_form_and_transform::register(runner);
    inverse::register(runner);
    is_in_hermite_normal_form::register(runner);
    is_in_reduced_row_echelon_form_with_rank::register(runner);
    is_in_smith_normal_form::register(runner);
    kronecker_product::register(runner);
    mod_op::register(runner);
    mod_power_of_2::register(runner);
    mul::register(runner);
    mul_vector::register(runner);
    multi_crt::register(runner);
    multi_mod::register(runner);
    neg::register(runner);
    nullspace::register(runner);
    permanent::register(runner);
    pow::register(runner);
    rank::register(runner);
    reduced_row_echelon_form::register(runner);
    scalar_mul::register(runner);
    shl::register(runner);
    shr::register(runner);
    smith_normal_form::register(runner);
    smith_normal_form_and_transforms::register(runner);
    solve::register(runner);
    solve_bounds::register(runner);
    solve_dixon::register(runner);
    solve_multi_mod::register(runner);
    square::register(runner);
    sub::register(runner);
    sub_mul::register(runner);
    submatrix_determinant_bound::register(runner);
    trace::register(runner);
    vector_mul::register(runner);
}

mod add;
mod add_mul;
mod balanced_mod;
mod balanced_mod_power_of_2;
mod can_solve_multi_mod;
mod canonical_primitive_part;
mod certify_lu_mod_p;
mod content;
mod content_chained;
mod crt;
mod determinant;
mod determinant_bound;
mod determinant_divisor;
mod determinant_sign;
mod div_exact;
mod elementary_divisors;
mod entrywise_shl_round;
mod entrywise_shr_round;
mod fraction_free_lu_decomposition;
mod hermite_normal_form;
mod hermite_normal_form_and_transform;
mod inverse;
mod is_in_hermite_normal_form;
mod is_in_reduced_row_echelon_form_with_rank;
mod is_in_smith_normal_form;
mod kronecker_product;
mod mod_op;
mod mod_power_of_2;
mod mul;
mod mul_vector;
mod multi_crt;
mod multi_mod;
mod neg;
mod nullspace;
mod permanent;
mod pow;
mod rank;
mod reduced_row_echelon_form;
mod scalar_mul;
mod shl;
mod shr;
mod smith_normal_form;
mod smith_normal_form_and_transforms;
mod solve;
mod solve_bounds;
mod solve_dixon;
mod solve_multi_mod;
mod square;
mod sub;
mod sub_mul;
mod submatrix_determinant_bound;
mod trace;
mod vector_mul;
