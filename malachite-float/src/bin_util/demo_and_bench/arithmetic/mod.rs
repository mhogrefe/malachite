// Copyright © 2024 Mikhail Hogrefe
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
    is_power_of_2::register(runner);
    mul::register(runner);
    neg::register(runner);
    power_of_2::register(runner);
    shl::register(runner);
    shr::register(runner);
    sign::register(runner);
    square::register(runner);
    sub::register(runner);
}

mod abs;
mod add;
mod is_power_of_2;
mod mul;
mod neg;
mod power_of_2;
mod shl;
mod shr;
mod sign;
mod square;
mod sub;
