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
    mod_euclidean::register(runner);
    mod_op::register(runner);
    mod_power_of_2::register(runner);
    multi_crt::register(runner);
}

mod balanced_mod;
mod balanced_mod_power_of_2;
mod crt;
mod divisible_by;
mod divisible_by_power_of_2;
mod eq_mod;
mod eq_mod_power_of_2;
mod mod_euclidean;
mod mod_op;
mod mod_power_of_2;
mod multi_crt;
