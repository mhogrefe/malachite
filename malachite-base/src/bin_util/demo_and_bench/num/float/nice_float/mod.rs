// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    cmp::register(runner);
    eq::register(runner);
    from_str::register(runner);
    hash::register(runner);
    to_string::register(runner);
}

mod cmp;
mod eq;
mod from_str;
mod hash;
mod to_string;
