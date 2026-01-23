// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    ln_2::register(runner);
    log_2_e::register(runner);
    phi::register(runner);
    pi::register(runner);
    prime_constant::register(runner);
    prouhet_thue_morse_constant::register(runner);
    sqrt_2::register(runner);
    sqrt_2_over_2::register(runner);
    sqrt_3::register(runner);
    sqrt_3_over_3::register(runner);
}

mod ln_2;
mod log_2_e;
mod phi;
mod pi;
mod prime_constant;
mod prouhet_thue_morse_constant;
mod sqrt_2;
mod sqrt_2_over_2;
mod sqrt_3;
mod sqrt_3_over_3;
