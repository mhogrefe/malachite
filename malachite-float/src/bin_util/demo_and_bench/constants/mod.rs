// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    cbrt_2::register(runner);
    e::register(runner);
    gauss_constant::register(runner);
    gelfond_schneider_constant::register(runner);
    gelfonds_constant::register(runner);
    lemniscate_constant::register(runner);
    ln_10::register(runner);
    ln_2::register(runner);
    log_10_2::register(runner);
    log_10_e::register(runner);
    log_2_10::register(runner);
    log_2_e::register(runner);
    one_over_pi::register(runner);
    one_over_sqrt_pi::register(runner);
    one_over_sqrt_tau::register(runner);
    phi::register(runner);
    pi::register(runner);
    pi_over_2::register(runner);
    pi_over_3::register(runner);
    pi_over_4::register(runner);
    pi_over_6::register(runner);
    pi_over_8::register(runner);
    prime_constant::register(runner);
    prouhet_thue_morse_constant::register(runner);
    ramanujans_constant::register(runner);
    sqrt_2::register(runner);
    sqrt_2_over_2::register(runner);
    sqrt_3::register(runner);
    sqrt_3_over_3::register(runner);
    sqrt_5::register(runner);
    sqrt_5_over_5::register(runner);
    sqrt_pi::register(runner);
    tau::register(runner);
    two_over_pi::register(runner);
    two_over_sqrt_pi::register(runner);
}

mod cbrt_2;
mod e;
mod gauss_constant;
mod gelfond_schneider_constant;
mod gelfonds_constant;
mod lemniscate_constant;
mod ln_10;
mod ln_2;
mod log_10_2;
mod log_10_e;
mod log_2_10;
mod log_2_e;
mod one_over_pi;
mod one_over_sqrt_pi;
mod one_over_sqrt_tau;
mod phi;
mod pi;
mod pi_over_2;
mod pi_over_3;
mod pi_over_4;
mod pi_over_6;
mod pi_over_8;
mod prime_constant;
mod prouhet_thue_morse_constant;
mod ramanujans_constant;
mod sqrt_2;
mod sqrt_2_over_2;
mod sqrt_3;
mod sqrt_3_over_3;
mod sqrt_5;
mod sqrt_5_over_5;
mod sqrt_pi;
mod tau;
mod two_over_pi;
mod two_over_sqrt_pi;
