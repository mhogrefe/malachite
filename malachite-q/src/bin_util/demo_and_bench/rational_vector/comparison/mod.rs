// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    partial_eq_integer_vector::register(runner);
    partial_eq_natural_vector::register(runner);
    partial_eq_unsigned_vector::register(runner);
    shortlex_cmp::register(runner);
}

mod partial_eq_integer_vector;
mod partial_eq_natural_vector;
mod partial_eq_unsigned_vector;
mod shortlex_cmp;
