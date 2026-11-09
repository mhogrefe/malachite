// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    compound::register(runner);
    exp::register(runner);
    exp_x_minus_1::register(runner);
    ln::register(runner);
    ln_1_plus_x::register(runner);
    log_base::register(runner);
    log_base_10::register(runner);
    log_base_10_1_plus_x::register(runner);
    log_base_1_plus_x::register(runner);
    log_base_2::register(runner);
    log_base_2_1_plus_x::register(runner);
    log_base_float_base::register(runner);
    log_base_float_base_1_plus_x::register(runner);
    log_base_power_of_2::register(runner);
    log_base_power_of_2_1_plus_x::register(runner);
    log_base_rational_base::register(runner);
    log_base_rational_base_1_plus_x::register(runner);
    log_base_rational_float_base::register(runner);
    log_base_rational_rational_base::register(runner);
    power_of_10::register(runner);
    power_of_10_x_minus_1::register(runner);
    power_of_2_of_float::register(runner);
    power_of_2_x_minus_1::register(runner);
}

mod compound;
mod exp;
mod exp_x_minus_1;
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
mod power_of_10;
mod power_of_10_x_minus_1;
mod power_of_2_of_float;
mod power_of_2_x_minus_1;
