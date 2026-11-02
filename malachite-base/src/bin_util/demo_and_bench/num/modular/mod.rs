// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    balanced_mod::register(runner);
    balanced_mod_power_of_2::register(runner);
    crt::register(runner);
    divisible_by::register(runner);
    divisible_by_power_of_2::register(runner);
    eq_mod::register(runner);
    eq_mod_power_of_2::register(runner);
    mod_abs_squared::register(runner);
    mod_add::register(runner);
    mod_add_mul::register(runner);
    mod_add_mul_shl::register(runner);
    mod_div::register(runner);
    mod_div_list::register(runner);
    mod_euclidean::register(runner);
    mod_inverse::register(runner);
    mod_is_reduced::register(runner);
    mod_mul::register(runner);
    mod_neg::register(runner);
    mod_op::register(runner);
    mod_pow::register(runner);
    mod_power_of_2::register(runner);
    mod_power_of_2_abs_squared::register(runner);
    mod_power_of_2_add::register(runner);
    mod_power_of_2_add_mul::register(runner);
    mod_power_of_2_add_mul_shl::register(runner);
    mod_power_of_2_div::register(runner);
    mod_power_of_2_inverse::register(runner);
    mod_power_of_2_is_reduced::register(runner);
    mod_power_of_2_mul::register(runner);
    mod_power_of_2_neg::register(runner);
    mod_power_of_2_pow::register(runner);
    mod_power_of_2_product::register(runner);
    mod_power_of_2_shl::register(runner);
    mod_power_of_2_shr::register(runner);
    mod_power_of_2_sqrt::register(runner);
    mod_power_of_2_square::register(runner);
    mod_power_of_2_sub::register(runner);
    mod_power_of_2_sub_mul::register(runner);
    mod_power_of_2_sub_mul_shl::register(runner);
    mod_power_of_2_sum::register(runner);
    mod_product::register(runner);
    mod_shl::register(runner);
    mod_shr::register(runner);
    mod_sqrt::register(runner);
    mod_square::register(runner);
    mod_sub::register(runner);
    mod_sub_mul::register(runner);
    mod_sub_mul_shl::register(runner);
    mod_sum::register(runner);
}

mod balanced_mod;
mod balanced_mod_power_of_2;
mod crt;
mod divisible_by;
mod divisible_by_power_of_2;
mod eq_mod;
mod eq_mod_power_of_2;
mod mod_abs_squared;
mod mod_add;
mod mod_add_mul;
mod mod_add_mul_shl;
mod mod_div;
mod mod_div_list;
mod mod_euclidean;
mod mod_inverse;
mod mod_is_reduced;
mod mod_mul;
mod mod_neg;
mod mod_op;
mod mod_pow;
mod mod_power_of_2;
mod mod_power_of_2_abs_squared;
mod mod_power_of_2_add;
mod mod_power_of_2_add_mul;
mod mod_power_of_2_add_mul_shl;
mod mod_power_of_2_div;
mod mod_power_of_2_inverse;
mod mod_power_of_2_is_reduced;
mod mod_power_of_2_mul;
mod mod_power_of_2_neg;
mod mod_power_of_2_pow;
mod mod_power_of_2_product;
mod mod_power_of_2_shl;
mod mod_power_of_2_shr;
mod mod_power_of_2_sqrt;
mod mod_power_of_2_square;
mod mod_power_of_2_sub;
mod mod_power_of_2_sub_mul;
mod mod_power_of_2_sub_mul_shl;
mod mod_power_of_2_sum;
mod mod_product;
mod mod_shl;
mod mod_shr;
mod mod_sqrt;
mod mod_square;
mod mod_sub;
mod mod_sub_mul;
mod mod_sub_mul_shl;
mod mod_sum;
