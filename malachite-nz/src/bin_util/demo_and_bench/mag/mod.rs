// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    add_mul::register(runner);
    ceiling::register(runner);
    cmp::register(runner);
    div::register(runner);
    from_natural::register(runner);
    from_u64::register(runner);
    mul::register(runner);
    sqrt::register(runner);
}

mod add_mul;
mod ceiling;
mod cmp;
mod div;
mod from_natural;
mod from_u64;
mod mul;
mod sqrt;
