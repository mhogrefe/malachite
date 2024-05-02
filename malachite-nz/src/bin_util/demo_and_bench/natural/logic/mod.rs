// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    and::register(runner);
    assign_bit::register(runner);
    assign_bits::register(runner);
    bits::register(runner);
    clear_bit::register(runner);
    count_ones::register(runner);
    flip_bit::register(runner);
    from_bits::register(runner);
    get_bit::register(runner);
    get_bits::register(runner);
    hamming_distance::register(runner);
    index_of_next_false_bit::register(runner);
    index_of_next_true_bit::register(runner);
    low_mask::register(runner);
    not::register(runner);
    or::register(runner);
    set_bit::register(runner);
    significant_bits::register(runner);
    to_bits::register(runner);
    trailing_zeros::register(runner);
    xor::register(runner);
}

mod and;
mod assign_bit;
mod assign_bits;
mod bits;
mod clear_bit;
mod count_ones;
mod flip_bit;
mod from_bits;
mod get_bit;
mod get_bits;
mod hamming_distance;
mod index_of_next_false_bit;
mod index_of_next_true_bit;
mod low_mask;
mod not;
mod or;
mod set_bit;
mod significant_bits;
mod to_bits;
mod trailing_zeros;
mod xor;
