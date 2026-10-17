// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    column_iter::register(runner);
    columns::register(runner);
    dimensions::register(runner);
    index::register(runner);
    is_square::register(runner);
    row_iter::register(runner);
    rows::register(runner);
    transpose::register(runner);
}

mod column_iter;
mod columns;
mod dimensions;
mod index;
mod is_square;
mod row_iter;
mod rows;
mod transpose;
