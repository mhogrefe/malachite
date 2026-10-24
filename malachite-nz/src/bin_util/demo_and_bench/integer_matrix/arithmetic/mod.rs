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
    div_exact::register(runner);
    fraction_free_lu_decomposition::register(runner);
    inverse::register(runner);
    is_in_reduced_row_echelon_form_with_rank::register(runner);
    kronecker_product::register(runner);
    mod_op::register(runner);
    mod_power_of_2::register(runner);
    mul::register(runner);
    mul_vector::register(runner);
    multi_crt::register(runner);
    multi_mod::register(runner);
    pow::register(runner);
    rank::register(runner);
    reduced_row_echelon_form::register(runner);
    neg::register(runner);
    nullspace::register(runner);
    scalar_mul::register(runner);
    shl::register(runner);
    solve::register(runner);
    solve_bounds::register(runner);
    solve_dixon::register(runner);
    solve_multi_mod::register(runner);
    square::register(runner);
    sub::register(runner);
    submatrix_determinant_bound::register(runner);
    trace::register(runner);
    vector_mul::register(runner);
}

mod add;
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
mod div_exact;
mod fraction_free_lu_decomposition;
mod inverse;
mod is_in_reduced_row_echelon_form_with_rank;
mod kronecker_product;
mod mod_op;
mod mod_power_of_2;
mod mul;
mod mul_vector;
mod multi_crt;
mod multi_mod;
mod neg;
mod nullspace;
mod pow;
mod rank;
mod reduced_row_echelon_form;
mod scalar_mul;
mod shl;
mod solve;
mod solve_bounds;
mod solve_dixon;
mod solve_multi_mod;
mod square;
mod sub;
mod submatrix_determinant_bound;
mod trace;
mod vector_mul;
