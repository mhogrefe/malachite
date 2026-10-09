// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    dimension::register(runner);
    extend::register(runner);
    height_index::register(runner);
    index::register(runner);
    is_zero::register(runner);
    max_bits::register(runner);
    pivot::register(runner);
    push::register(runner);
    select_coordinates::register(runner);
    set_dimension::register(runner);
    set_zero::register(runner);
    sort::register(runner);
    standard_basis_index::register(runner);
    standard_basis_vector::register(runner);
    sum_max_bits::register(runner);
    zero::register(runner);
}

mod dimension;
mod extend;
mod height_index;
mod index;
mod is_zero;
mod max_bits;
mod pivot;
mod push;
mod select_coordinates;
mod set_dimension;
mod set_zero;
mod sort;
mod standard_basis_index;
mod standard_basis_vector;
mod sum_max_bits;
mod zero;
