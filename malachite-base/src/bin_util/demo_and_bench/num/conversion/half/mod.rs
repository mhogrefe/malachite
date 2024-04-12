// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    join_halves::register(runner);
    lower_half::register(runner);
    split_in_half::register(runner);
    upper_half::register(runner);
}

mod join_halves;
mod lower_half;
mod split_in_half;
mod upper_half;
