// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    add::register(runner);
    add_truncated::register(runner);
    canonicalize_unit::register(runner);
    content::register(runner);
    div_power_of_x::register(runner);
    evaluate::register(runner);
    height::register(runner);
    is_unit::register(runner);
    l2_norm_squared::register(runner);
    make_monic::register(runner);
    mul_power_of_x::register(runner);
    neg::register(runner);
    sub::register(runner);
    sub_truncated::register(runner);
}

mod add;
mod add_truncated;
mod canonicalize_unit;
mod content;
mod div_power_of_x;
mod evaluate;
mod height;
mod is_unit;
mod l2_norm_squared;
mod make_monic;
mod mul_power_of_x;
mod neg;
mod sub;
mod sub_truncated;
