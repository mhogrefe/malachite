// Copyright © 2025 Mikhail Hogrefe
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
use malachite_base::vecs::exhaustive::exhaustive_vecs_from_length_iterator;
use std::fmt::Debug;
use std::iter::empty;

fn exhaustive_vecs_from_element_iterator_helper<I: Iterator<Item = u64>, J: Clone + Iterator>(
    lengths: I,
    xs: J,
    out: &[&[J::Item]],
) where
    J::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_helper_helper(exhaustive_vecs_from_length_iterator(lengths, xs), out);
}

#[test]
fn test_exhaustive_vecs_from_element_iterator() {
    exhaustive_vecs_from_element_iterator_helper(empty(), exhaustive_bools(), &[]);
    exhaustive_vecs_from_element_iterator_helper(
        [2, 1, 2].iter().copied(),
        exhaustive_bools(),
        &[
            &[false, false],
            &[false],
            &[false, true],
            &[false, false],
            &[true, false],
            &[true],
            &[true, true],
            &[false, true],
            &[true, false],
            &[true, true],
        ],
    );
    exhaustive_vecs_from_element_iterator_helper(
        exhaustive_unsigneds::<u64>().map(|u| u << 1),
        exhaustive_bools(),
        &[
            &[],
            &[false, false],
            &[false, true],
            &[false, false, false, false, false, false],
            &[true, false],
            &[false, false, false, false],
            &[true, true],
            &[false, false, false, false, false, false, false, false],
            &[false, false, false, true],
            &[false, false, false, false, false, true],
            &[false, false, true, false],
            &[false, false, false, false, false, false, false, true],
            &[false, false, true, true],
            &[false, false, false, false, true, false],
            &[false, true, false, false],
            &[false, false, false, false, false, false, false, false, false, false, false, false],
            &[false, true, false, true],
            &[false, false, false, false, true, true],
            &[false, true, true, false],
            &[false, false, false, false, false, false, true, false],
        ],
    );
    exhaustive_vecs_from_element_iterator_helper(
        [2, 1, 0, 2].iter().copied(),
        exhaustive_bools(),
        &[
            &[false, false],
            &[false],
            &[false, true],
            &[],
            &[true, false],
            &[true],
            &[true, true],
            &[false, false],
            &[false, true],
            &[true, false],
            &[true, true],
        ],
    );
    exhaustive_vecs_from_element_iterator_helper(empty(), exhaustive_unsigneds::<u32>(), &[]);
    exhaustive_vecs_from_element_iterator_helper(
        [2, 1, 2].iter().copied(),
        exhaustive_unsigneds::<u32>(),
        &[
            &[0, 0],
            &[0],
            &[0, 1],
            &[0, 0],
            &[1, 0],
            &[1],
            &[1, 1],
            &[0, 2],
            &[0, 3],
            &[2],
            &[1, 2],
            &[0, 1],
            &[1, 3],
            &[3],
            &[2, 0],
            &[4],
            &[2, 1],
            &[5],
            &[3, 0],
            &[1, 0],
        ],
    );
    exhaustive_vecs_from_element_iterator_helper(
        exhaustive_unsigneds::<u64>().map(|u| u << 1),
        exhaustive_unsigneds::<u32>(),
        &[
            &[],
            &[0, 0],
            &[0, 1],
            &[0, 0, 0, 0, 0, 0],
            &[1, 0],
            &[0, 0, 0, 0],
            &[1, 1],
            &[0, 0, 0, 0, 0, 0, 0, 0],
            &[0, 2],
            &[0, 0, 0, 1],
            &[0, 3],
            &[0, 0, 0, 0, 0, 1],
            &[1, 2],
            &[0, 0, 1, 0],
            &[1, 3],
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            &[2, 0],
            &[0, 0, 1, 1],
            &[2, 1],
            &[0, 0, 0, 0, 1, 0],
        ],
    );
    exhaustive_vecs_from_element_iterator_helper(
        [2, 1, 0, 2].iter().copied(),
        exhaustive_unsigneds::<u32>(),
        &[
            &[0, 0],
            &[0],
            &[0, 1],
            &[],
            &[1, 0],
            &[1],
            &[1, 1],
            &[0, 0],
            &[0, 2],
            &[2],
            &[0, 3],
            &[0, 1],
            &[1, 2],
            &[3],
            &[1, 3],
            &[4],
            &[2, 0],
            &[5],
            &[2, 1],
            &[1, 0],
        ],
    );
    // Stops after first empty ys
    exhaustive_vecs_from_element_iterator_helper(
        [0, 0, 1, 0].iter().copied(),
        nevers(),
        &[&[], &[]],
    );
}
