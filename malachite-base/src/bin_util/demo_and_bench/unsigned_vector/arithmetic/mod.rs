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
    height::register(runner);
    mod_add::register(runner);
    mod_is_reduced::register(runner);
    mod_mul::register(runner);
    mod_neg::register(runner);
    mod_op::register(runner);
    mod_power_of_2::register(runner);
    mod_power_of_2_add::register(runner);
    mod_power_of_2_is_reduced::register(runner);
    mod_power_of_2_mul::register(runner);
    mod_power_of_2_neg::register(runner);
    mod_power_of_2_shl::register(runner);
    mod_power_of_2_sub::register(runner);
    mod_shl::register(runner);
    mod_sub::register(runner);
}

mod canonical_primitive_part;
mod content;
mod height;
mod mod_add;
mod mod_is_reduced;
mod mod_mul;
mod mod_neg;
mod mod_op;
mod mod_power_of_2;
mod mod_power_of_2_add;
mod mod_power_of_2_is_reduced;
mod mod_power_of_2_mul;
mod mod_power_of_2_neg;
mod mod_power_of_2_shl;
mod mod_power_of_2_sub;
mod mod_shl;
mod mod_sub;
