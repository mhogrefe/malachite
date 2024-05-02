// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    bit_access::register(runner);
    bit_block_access::register(runner);
    bit_convertible::register(runner);
    bit_iterable::register(runner);
    bit_scan::register(runner);
    get_highest_bit::register(runner);
    hamming_distance::register(runner);
    low_mask::register(runner);
    not_assign::register(runner);
    significant_bits::register(runner);
}

mod bit_access;
mod bit_block_access;
mod bit_convertible;
mod bit_iterable;
mod bit_scan;
mod get_highest_bit;
mod hamming_distance;
mod low_mask;
mod not_assign;
mod significant_bits;
