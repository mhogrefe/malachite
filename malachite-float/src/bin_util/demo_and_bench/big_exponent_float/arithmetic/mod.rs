// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    abs::register(runner);
    add::register(runner);
    div::register(runner);
    mul::register(runner);
    neg::register(runner);
    reciprocal::register(runner);
    shl::register(runner);
    shr::register(runner);
    sign::register(runner);
    sqrt::register(runner);
    square::register(runner);
    sub::register(runner);
}

mod abs;
mod add;
mod div;
mod mul;
mod neg;
mod reciprocal;
mod shl;
mod shr;
mod sign;
mod sqrt;
mod square;
mod sub;
