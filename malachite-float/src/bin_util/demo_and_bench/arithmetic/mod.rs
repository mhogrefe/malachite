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
    agm::register(runner);
    cbrt::register(runner);
    div::register(runner);
    exp::register(runner);
    exp_x_minus_1::register(runner);
    is_power_of_2::register(runner);
    ln::register(runner);
    ln_1_plus_x::register(runner);
    log_base::register(runner);
    log_base_10::register(runner);
    log_base_10_1_plus_x::register(runner);
    log_base_1_plus_x::register(runner);
    log_base_2::register(runner);
    log_base_2_1_plus_x::register(runner);
    log_base_power_of_2::register(runner);
    log_base_power_of_2_1_plus_x::register(runner);
    log_base_float_base::register(runner);
    log_base_float_base_1_plus_x::register(runner);
    log_base_rational_base::register(runner);
    log_base_rational_base_1_plus_x::register(runner);
    log_base_rational_float_base::register(runner);
    log_base_rational_rational_base::register(runner);
    mul::register(runner);
    neg::register(runner);
    pow::register(runner);
    power_of_2::register(runner);
    power_of_10::register(runner);
    power_of_10_x_minus_1::register(runner);
    power_of_2_of_float::register(runner);
    power_of_2_x_minus_1::register(runner);
    reciprocal::register(runner);
    reciprocal_sqrt::register(runner);
    root::register(runner);
    shl::register(runner);
    shl_round::register(runner);
    shr::register(runner);
    shr_round::register(runner);
    sign::register(runner);
    sqrt::register(runner);
    square::register(runner);
    sub::register(runner);
}

mod abs;
mod add;
mod agm;
mod cbrt;
mod div;
mod exp;
mod exp_x_minus_1;
mod is_power_of_2;
mod ln;
mod ln_1_plus_x;
mod log_base;
mod log_base_10;
mod log_base_10_1_plus_x;
mod log_base_1_plus_x;
mod log_base_2;
mod log_base_2_1_plus_x;
mod log_base_float_base;
mod log_base_float_base_1_plus_x;
mod log_base_power_of_2;
mod log_base_power_of_2_1_plus_x;
mod log_base_rational_base;
mod log_base_rational_base_1_plus_x;
mod log_base_rational_float_base;
mod log_base_rational_rational_base;
mod mul;
mod neg;
mod pow;
mod power_of_10;
mod power_of_10_x_minus_1;
mod power_of_2;
mod power_of_2_of_float;
mod power_of_2_x_minus_1;
mod reciprocal;
mod reciprocal_sqrt;
mod root;
mod shl;
mod shl_round;
mod shr;
mod shr_round;
mod sign;
mod sqrt;
mod square;
mod sub;
