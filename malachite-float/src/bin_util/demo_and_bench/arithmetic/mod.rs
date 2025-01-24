// Copyright © 2025 Mikhail Hogrefe
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
    is_power_of_2::register(runner);
    mul::register(runner);
    neg::register(runner);
    power_of_2::register(runner);
    reciprocal::register(runner);
    shl::register(runner);
    shl_round::register(runner);
    shr::register(runner);
    shr_round::register(runner);
    sign::register(runner);
    square::register(runner);
    sub::register(runner);
}

mod abs;
mod add;
mod div;
mod is_power_of_2;
mod mul;
mod neg;
mod power_of_2;
mod reciprocal;
mod shl;
mod shl_round;
mod shr;
mod shr_round;
mod sign;
mod square;
mod sub;
