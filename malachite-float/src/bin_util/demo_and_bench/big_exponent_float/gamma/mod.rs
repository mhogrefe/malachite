// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    gamma::register(runner);
    ln_abs_gamma::register(runner);
    ln_gamma::register(runner);
    reciprocal_gamma::register(runner);
}

#[allow(clippy::module_inception)]
mod gamma;
mod ln_abs_gamma;
mod ln_gamma;
mod reciprocal_gamma;
