// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    adjugate::register(runner);
    can_solve_multi_mod::register(runner);
    certify_lu_mod_p::register(runner);
    characteristic_polynomial::register(runner);
    cofactor::register(runner);
    determinant::register(runner);
    determinant_bound::register(runner);
    determinant_divisor::register(runner);
    determinant_sign::register(runner);
    elementary_divisors::register(runner);
    fraction_free_lu_decomposition::register(runner);
    hermite_normal_form::register(runner);
    hermite_normal_form_and_transform::register(runner);
    inverse::register(runner);
    is_in_hermite_normal_form::register(runner);
    is_in_reduced_row_echelon_form_with_rank::register(runner);
    is_in_smith_normal_form::register(runner);
    minor::register(runner);
    nullspace::register(runner);
    permanent::register(runner);
    rank::register(runner);
    reduced_row_echelon_form::register(runner);
    similarity_transform::register(runner);
    smith_normal_form::register(runner);
    smith_normal_form_and_transforms::register(runner);
    solve::register(runner);
    solve_bounds::register(runner);
    solve_dixon::register(runner);
    solve_multi_mod::register(runner);
    submatrix_determinant_bound::register(runner);
    trace::register(runner);
}

mod adjugate;
mod can_solve_multi_mod;
mod certify_lu_mod_p;
mod characteristic_polynomial;
mod cofactor;
mod determinant;
mod determinant_bound;
mod determinant_divisor;
mod determinant_sign;
mod elementary_divisors;
mod fraction_free_lu_decomposition;
mod hermite_normal_form;
mod hermite_normal_form_and_transform;
mod inverse;
mod is_in_hermite_normal_form;
mod is_in_reduced_row_echelon_form_with_rank;
mod is_in_smith_normal_form;
mod minor;
mod nullspace;
mod permanent;
mod rank;
mod reduced_row_echelon_form;
mod similarity_transform;
mod smith_normal_form;
mod smith_normal_form_and_transforms;
mod solve;
mod solve_bounds;
mod solve_dixon;
mod solve_multi_mod;
mod submatrix_determinant_bound;
mod trace;
