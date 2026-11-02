// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    acosh::register(runner);
    acoth::register(runner);
    acsch::register(runner);
    asech::register(runner);
    asinh::register(runner);
    atanh::register(runner);
    cosh::register(runner);
    coth::register(runner);
    csch::register(runner);
    sech::register(runner);
    sinh::register(runner);
    sinh_cosh::register(runner);
    tanh::register(runner);
}

mod acosh;
mod acoth;
mod acsch;
mod asech;
mod asinh;
mod atanh;
mod cosh;
mod coth;
mod csch;
mod sech;
mod sinh;
mod sinh_cosh;
mod tanh;
