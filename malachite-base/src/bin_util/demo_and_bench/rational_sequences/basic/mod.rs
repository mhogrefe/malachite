// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    component_len::register(runner);
    is_empty::register(runner);
    is_finite::register(runner);
    iter::register(runner);
    len::register(runner);
}

mod component_len;
mod is_empty;
mod is_finite;
mod iter;
mod len;
