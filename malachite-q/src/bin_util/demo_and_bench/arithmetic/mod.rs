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
    abs_diff::register(runner);
    add::register(runner);
    approximate::register(runner);
    ceiling::register(runner);
    denominators_in_closed_interval::register(runner);
    div::register(runner);
    floor::register(runner);
    is_power_of_2::register(runner);
    log_base::register(runner);
    log_base_2::register(runner);
    log_base_power_of_2::register(runner);
    mul::register(runner);
    neg::register(runner);
    next_power_of_2::register(runner);
    pow::register(runner);
    power_of_2::register(runner);
    reciprocal::register(runner);
    root::register(runner);
    round_to_multiple::register(runner);
    round_to_multiple_of_power_of_2::register(runner);
    shl::register(runner);
    shr::register(runner);
    sign::register(runner);
    simplest_rational_in_interval::register(runner);
    sqrt::register(runner);
    square::register(runner);
    sub::register(runner);
}

mod abs;
mod abs_diff;
mod add;
mod approximate;
mod ceiling;
mod denominators_in_closed_interval;
mod div;
mod floor;
mod is_power_of_2;
mod log_base;
mod log_base_2;
mod log_base_power_of_2;
mod mul;
mod neg;
mod next_power_of_2;
mod pow;
mod power_of_2;
mod reciprocal;
mod root;
mod round_to_multiple;
mod round_to_multiple_of_power_of_2;
mod shl;
mod shr;
mod sign;
mod simplest_rational_in_interval;
mod sqrt;
mod square;
mod sub;
