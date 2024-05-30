// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::nevers::nevers;
use malachite_base::num::exhaustive::exhaustive_unsigneds;
use malachite_base::test_util::vecs::exhaustive::exhaustive_vecs_helper_helper;
use malachite_base::vecs::exhaustive::shortlex_vecs_from_length_iterator;
use std::fmt::Debug;
use std::iter::empty;

fn shortlex_vecs_from_element_iterator_helper<I: Iterator<Item = u64>, J: Clone + Iterator>(
    lengths: I,
    xs: J,
    out: &[&[J::Item]],
) where
    J::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_helper_helper(shortlex_vecs_from_length_iterator(lengths, xs), out);
}

#[test]
fn test_shortlex_vecs_from_element_iterator() {
    shortlex_vecs_from_element_iterator_helper(empty(), exhaustive_bools(), &[]);
    shortlex_vecs_from_element_iterator_helper(
        [2, 1, 2].iter().copied(),
        exhaustive_bools(),
        &[
            &[false, false],
            &[false, true],
            &[true, false],
            &[true, true],
            &[false],
            &[true],
            &[false, false],
            &[false, true],
            &[true, false],
            &[true, true],
        ],
    );
    shortlex_vecs_from_element_iterator_helper(
        exhaustive_unsigneds::<u64>().map(|u| u << 1),
        exhaustive_bools(),
        &[
            &[],
            &[false, false],
            &[false, true],
            &[true, false],
            &[true, true],
            &[false, false, false, false],
            &[false, false, false, true],
            &[false, false, true, false],
            &[false, false, true, true],
            &[false, true, false, false],
            &[false, true, false, true],
            &[false, true, true, false],
            &[false, true, true, true],
            &[true, false, false, false],
            &[true, false, false, true],
            &[true, false, true, false],
            &[true, false, true, true],
            &[true, true, false, false],
            &[true, true, false, true],
            &[true, true, true, false],
        ],
    );
    shortlex_vecs_from_element_iterator_helper(
        [2, 1, 0, 2].iter().copied(),
        exhaustive_bools(),
        &[
            &[false, false],
            &[false, true],
            &[true, false],
            &[true, true],
            &[false],
            &[true],
            &[],
            &[false, false],
            &[false, true],
            &[true, false],
            &[true, true],
        ],
    );
    // Stops after first empty ys
    shortlex_vecs_from_element_iterator_helper([0, 0, 1, 0].iter().copied(), nevers(), &[&[], &[]]);
}
