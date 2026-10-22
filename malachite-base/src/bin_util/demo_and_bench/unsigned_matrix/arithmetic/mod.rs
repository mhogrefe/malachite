// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    canonical_primitive_part::register(runner);
    content::register(runner);
    content_chained::register(runner);
    div_exact::register(runner);
    mod_add::register(runner);
    mod_determinant::register(runner);
    mod_howell_form::register(runner);
    mod_inverse::register(runner);
    mod_is_reduced::register(runner);
    mod_kronecker_product::register(runner);
    mod_left_nullspace::register(runner);
    mod_lu_decomposition::register(runner);
    mod_mul::register(runner);
    mod_mul_matrix::register(runner);
    mod_mul_vector::register(runner);
    mod_neg::register(runner);
    mod_nullspace::register(runner);
    mod_op::register(runner);
    mod_pow::register(runner);
    mod_power_of_2::register(runner);
    mod_power_of_2_add::register(runner);
    mod_power_of_2_is_reduced::register(runner);
    mod_power_of_2_kronecker_product::register(runner);
    mod_power_of_2_mul::register(runner);
    mod_power_of_2_mul_matrix::register(runner);
    mod_power_of_2_mul_vector::register(runner);
    mod_power_of_2_neg::register(runner);
    mod_power_of_2_pow::register(runner);
    mod_power_of_2_shl::register(runner);
    mod_power_of_2_square::register(runner);
    mod_power_of_2_sub::register(runner);
    mod_power_of_2_trace::register(runner);
    mod_rank::register(runner);
    mod_reduced_row_echelon_form::register(runner);
    mod_shl::register(runner);
    mod_solve::register(runner);
    mod_solve_lower_triangular::register(runner);
    mod_solve_upper_triangular::register(runner);
    mod_square::register(runner);
    mod_strong_echelon_form::register(runner);
    mod_sub::register(runner);
    mod_trace::register(runner);
    vector_mod_mul::register(runner);
    vector_mod_power_of_2_mul::register(runner);
}

mod canonical_primitive_part;
mod content;
mod content_chained;
mod div_exact;
mod mod_add;
mod mod_determinant;
mod mod_howell_form;
mod mod_inverse;
mod mod_is_reduced;
mod mod_kronecker_product;
mod mod_left_nullspace;
mod mod_lu_decomposition;
mod mod_mul;
mod mod_mul_matrix;
mod mod_mul_vector;
mod mod_neg;
mod mod_nullspace;
mod mod_op;
mod mod_pow;
mod mod_power_of_2;
mod mod_power_of_2_add;
mod mod_power_of_2_is_reduced;
mod mod_power_of_2_kronecker_product;
mod mod_power_of_2_mul;
mod mod_power_of_2_mul_matrix;
mod mod_power_of_2_mul_vector;
mod mod_power_of_2_neg;
mod mod_power_of_2_pow;
mod mod_power_of_2_shl;
mod mod_power_of_2_square;
mod mod_power_of_2_sub;
mod mod_power_of_2_trace;
mod mod_rank;
mod mod_reduced_row_echelon_form;
mod mod_shl;
mod mod_solve;
mod mod_solve_lower_triangular;
mod mod_solve_upper_triangular;
mod mod_square;
mod mod_strong_echelon_form;
mod mod_sub;
mod mod_trace;
mod vector_mod_mul;
mod vector_mod_power_of_2_mul;
