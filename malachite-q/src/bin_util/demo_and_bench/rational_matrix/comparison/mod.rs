// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    partial_eq_integer_matrix::register(runner);
    partial_eq_natural_matrix::register(runner);
    partial_eq_unsigned_matrix::register(runner);
}

mod partial_eq_integer_matrix;
mod partial_eq_natural_matrix;
mod partial_eq_unsigned_matrix;
