// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    column::register(runner);
    column_iter::register(runner);
    columns::register(runner);
    dimensions::register(runner);
    identity::register(runner);
    index::register(runner);
    is_diagonal::register(runner);
    is_identity::register(runner);
    is_lower_triangular::register(runner);
    is_skew_symmetric::register(runner);
    is_square::register(runner);
    is_symmetric::register(runner);
    is_upper_triangular::register(runner);
    is_zero::register(runner);
    row::register(runner);
    row_iter::register(runner);
    rows::register(runner);
    set_column::register(runner);
    set_row::register(runner);
    set_zero::register(runner);
    standard_basis_index::register(runner);
    standard_basis_matrix::register(runner);
    transpose::register(runner);
    window::register(runner);
    zero::register(runner);
}

mod column;
mod column_iter;
mod columns;
mod dimensions;
mod identity;
mod index;
mod is_diagonal;
mod is_identity;
mod is_lower_triangular;
mod is_skew_symmetric;
mod is_square;
mod is_symmetric;
mod is_upper_triangular;
mod is_zero;
mod row;
mod row_iter;
mod rows;
mod set_column;
mod set_row;
mod set_zero;
mod standard_basis_index;
mod standard_basis_matrix;
mod transpose;
mod window;
mod zero;
