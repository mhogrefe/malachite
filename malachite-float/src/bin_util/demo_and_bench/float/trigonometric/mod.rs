// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    acos::register(runner);
    acot::register(runner);
    acsc::register(runner);
    asec::register(runner);
    asin::register(runner);
    atan::register(runner);
    atan2::register(runner);
    cos::register(runner);
    cot::register(runner);
    csc::register(runner);
    sec::register(runner);
    sin::register(runner);
    sin_cos::register(runner);
    tan::register(runner);
}

mod acos;
mod acot;
mod acsc;
mod asec;
mod asin;
mod atan;
mod atan2;
mod cos;
mod cot;
mod csc;
mod sec;
mod sin;
mod sin_cos;
mod tan;
