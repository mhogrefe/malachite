// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    eq_abs::register(runner);
    cmp_abs_and_partial_cmp_abs::register(runner);
    ord_abs_comparators::register(runner);
}

mod cmp_abs_and_partial_cmp_abs;
mod eq_abs;
mod ord_abs_comparators;
