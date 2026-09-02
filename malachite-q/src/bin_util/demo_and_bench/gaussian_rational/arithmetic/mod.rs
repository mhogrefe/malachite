// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    abs_squared::register(runner);
    add::register(runner);
    conjugate::register(runner);
    div_i::register(runner);
    is_power_of_2::register(runner);
    mul::register(runner);
    mul_i::register(runner);
    neg::register(runner);
    power_of_2::register(runner);
    shl::register(runner);
    shr::register(runner);
    square::register(runner);
    sub::register(runner);
}

mod abs_squared;
mod add;
mod conjugate;
mod div_i;
mod is_power_of_2;
mod mul;
mod mul_i;
mod neg;
mod power_of_2;
mod shl;
mod shr;
mod square;
mod sub;
