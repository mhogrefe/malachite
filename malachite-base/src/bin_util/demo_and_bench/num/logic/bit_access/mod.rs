// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    assign_bit::register(runner);
    clear_bit::register(runner);
    flip_bit::register(runner);
    get_bit::register(runner);
    set_bit::register(runner);
}

mod assign_bit;
mod clear_bit;
mod flip_bit;
mod get_bit;
mod set_bit;
