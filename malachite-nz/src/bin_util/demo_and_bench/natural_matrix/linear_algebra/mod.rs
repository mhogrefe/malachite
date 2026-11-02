// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    characteristic_polynomial::register(runner);
    determinant::register(runner);
    determinant_sign::register(runner);
    elementary_divisors::register(runner);
    mod_characteristic_polynomial::register(runner);
    mod_determinant::register(runner);
    mod_howell_form::register(runner);
    mod_inverse::register(runner);
    mod_left_nullspace::register(runner);
    mod_lu_decomposition::register(runner);
    mod_nullspace::register(runner);
    mod_permanent::register(runner);
    mod_power_of_2_characteristic_polynomial::register(runner);
    mod_power_of_2_determinant::register(runner);
    mod_power_of_2_howell_form::register(runner);
    mod_power_of_2_inverse::register(runner);
    mod_power_of_2_permanent::register(runner);
    mod_power_of_2_similarity_transform::register(runner);
    mod_power_of_2_smith_normal_form::register(runner);
    mod_power_of_2_solve::register(runner);
    mod_power_of_2_strong_echelon_form::register(runner);
    mod_power_of_2_trace::register(runner);
    mod_rank::register(runner);
    mod_reduced_row_echelon_form::register(runner);
    mod_similarity_transform::register(runner);
    mod_smith_normal_form::register(runner);
    mod_solve::register(runner);
    mod_solve_lower_triangular::register(runner);
    mod_solve_upper_triangular::register(runner);
    mod_strong_echelon_form::register(runner);
    mod_trace::register(runner);
    permanent::register(runner);
    rank::register(runner);
    smith_normal_form::register(runner);
    trace::register(runner);
}

mod characteristic_polynomial;
mod determinant;
mod determinant_sign;
mod elementary_divisors;
mod mod_characteristic_polynomial;
mod mod_determinant;
mod mod_howell_form;
mod mod_inverse;
mod mod_left_nullspace;
mod mod_lu_decomposition;
mod mod_nullspace;
mod mod_permanent;
mod mod_power_of_2_characteristic_polynomial;
mod mod_power_of_2_determinant;
mod mod_power_of_2_howell_form;
mod mod_power_of_2_inverse;
mod mod_power_of_2_permanent;
mod mod_power_of_2_similarity_transform;
mod mod_power_of_2_smith_normal_form;
mod mod_power_of_2_solve;
mod mod_power_of_2_strong_echelon_form;
mod mod_power_of_2_trace;
mod mod_rank;
mod mod_reduced_row_echelon_form;
mod mod_similarity_transform;
mod mod_smith_normal_form;
mod mod_solve;
mod mod_solve_lower_triangular;
mod mod_solve_upper_triangular;
mod mod_strong_echelon_form;
mod mod_trace;
mod permanent;
mod rank;
mod smith_normal_form;
mod trace;
