// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    mod_add::register(runner);
    mod_compose::register(runner);
    mod_div_exact::register(runner);
    mod_div_rem::register(runner);
    mod_div_rem_precomputed::register(runner);
    mod_div_root::register(runner);
    mod_divisible_by::register(runner);
    mod_is_reduced::register(runner);
    mod_make_monic::register(runner);
    mod_mul::register(runner);
    mod_neg::register(runner);
    mod_op::register(runner);
    mod_pow::register(runner);
    mod_power_of_2::register(runner);
    mod_power_of_2_add::register(runner);
    mod_power_of_2_compose::register(runner);
    mod_power_of_2_div_exact::register(runner);
    mod_power_of_2_div_rem::register(runner);
    mod_power_of_2_div_rem_precomputed::register(runner);
    mod_power_of_2_div_root::register(runner);
    mod_power_of_2_divisible_by::register(runner);
    mod_power_of_2_mul::register(runner);
    mod_power_of_2_neg::register(runner);
    mod_power_of_2_pow::register(runner);
    mod_power_of_2_product::register(runner);
    mod_power_of_2_quotient::register(runner);
    mod_power_of_2_quotient_precomputed::register(runner);
    mod_power_of_2_rem::register(runner);
    mod_power_of_2_rem_precomputed::register(runner);
    mod_power_of_2_remove_power::register(runner);
    mod_power_of_2_shl::register(runner);
    mod_power_of_2_square::register(runner);
    mod_power_of_2_sub::register(runner);
    mod_power_of_2_sum::register(runner);
    mod_power_of_2_taylor_shift::register(runner);
    mod_product::register(runner);
    mod_quotient::register(runner);
    mod_quotient_precomputed::register(runner);
    mod_rem::register(runner);
    mod_rem_precomputed::register(runner);
    mod_remove_power::register(runner);
    mod_shl::register(runner);
    mod_square::register(runner);
    mod_sub::register(runner);
    mod_sum::register(runner);
    mod_taylor_shift::register(runner);
}

mod mod_add;
mod mod_compose;
mod mod_div_exact;
mod mod_div_rem;
mod mod_div_rem_precomputed;
mod mod_div_root;
mod mod_divisible_by;
mod mod_is_reduced;
mod mod_make_monic;
mod mod_mul;
mod mod_neg;
mod mod_op;
mod mod_pow;
mod mod_power_of_2;
mod mod_power_of_2_add;
mod mod_power_of_2_compose;
mod mod_power_of_2_div_exact;
mod mod_power_of_2_div_rem;
mod mod_power_of_2_div_rem_precomputed;
mod mod_power_of_2_div_root;
mod mod_power_of_2_divisible_by;
mod mod_power_of_2_mul;
mod mod_power_of_2_neg;
mod mod_power_of_2_pow;
mod mod_power_of_2_product;
mod mod_power_of_2_quotient;
mod mod_power_of_2_quotient_precomputed;
mod mod_power_of_2_rem;
mod mod_power_of_2_rem_precomputed;
mod mod_power_of_2_remove_power;
mod mod_power_of_2_shl;
mod mod_power_of_2_square;
mod mod_power_of_2_sub;
mod mod_power_of_2_sum;
mod mod_power_of_2_taylor_shift;
mod mod_product;
mod mod_quotient;
mod mod_quotient_precomputed;
mod mod_rem;
mod mod_rem_precomputed;
mod mod_remove_power;
mod mod_shl;
mod mod_square;
mod mod_sub;
mod mod_sum;
mod mod_taylor_shift;
