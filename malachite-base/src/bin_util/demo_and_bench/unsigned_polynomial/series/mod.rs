// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    mod_add_truncated::register(runner);
    mod_compose_truncated::register(runner);
    mod_compositional_inverse_truncated::register(runner);
    mod_div_truncated::register(runner);
    mod_inverse_truncated::register(runner);
    mod_mul_truncated::register(runner);
    mod_pow_truncated::register(runner);
    mod_power_of_2_add_truncated::register(runner);
    mod_power_of_2_compose_truncated::register(runner);
    mod_power_of_2_compositional_inverse_truncated::register(runner);
    mod_power_of_2_div_truncated::register(runner);
    mod_power_of_2_inverse_truncated::register(runner);
    mod_power_of_2_mul_truncated::register(runner);
    mod_power_of_2_pow_truncated::register(runner);
    mod_power_of_2_square_truncated::register(runner);
    mod_power_of_2_sub_truncated::register(runner);
    mod_square_truncated::register(runner);
    mod_sub_truncated::register(runner);
}

mod mod_add_truncated;
mod mod_compose_truncated;
mod mod_compositional_inverse_truncated;
mod mod_div_truncated;
mod mod_inverse_truncated;
mod mod_mul_truncated;
mod mod_pow_truncated;
mod mod_power_of_2_add_truncated;
mod mod_power_of_2_compose_truncated;
mod mod_power_of_2_compositional_inverse_truncated;
mod mod_power_of_2_div_truncated;
mod mod_power_of_2_inverse_truncated;
mod mod_power_of_2_mul_truncated;
mod mod_power_of_2_pow_truncated;
mod mod_power_of_2_square_truncated;
mod mod_power_of_2_sub_truncated;
mod mod_square_truncated;
mod mod_sub_truncated;
