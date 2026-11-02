// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    mod_derivative::register(runner);
    mod_integral::register(runner);
    mod_nth_derivative::register(runner);
    mod_power_of_2_derivative::register(runner);
    mod_power_of_2_integral::register(runner);
    mod_power_of_2_nth_derivative::register(runner);
}

mod mod_derivative;
mod mod_integral;
mod mod_nth_derivative;
mod mod_power_of_2_derivative;
mod mod_power_of_2_integral;
mod mod_power_of_2_nth_derivative;
