// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::nevers::nevers;
use malachite_base::num::exhaustive::exhaustive_unsigneds;
use malachite_base::test_util::vecs::exhaustive::exhaustive_vecs_helper_helper;
use malachite_base::tuples::exhaustive::exhaustive_units;
use malachite_base::vecs::exhaustive::exhaustive_vecs;
use std::fmt::Debug;

fn exhaustive_vecs_helper<I: Clone + Iterator>(xs: I, out: &[&[I::Item]])
where
    I::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_helper_helper(exhaustive_vecs(xs), out);
}

#[test]
fn test_exhaustive_vecs() {
    exhaustive_vecs_helper(nevers(), &[&[]]);
    exhaustive_vecs_helper(
        exhaustive_units(),
        &[
            &[],
            &[()],
            &[(), ()],
            &[(), (), (), ()],
            &[(), (), ()],
            &[(), (), (), (), ()],
            &[(), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()],
        ],
    );
    exhaustive_vecs_helper(
        exhaustive_bools(),
        &[
            &[],
            &[false],
            &[true],
            &[false, false, false],
            &[false, false],
            &[false, false, true],
            &[false, true],
            &[false, false, false, false, false],
            &[true, false],
            &[false, true, false],
            &[true, true],
            &[false, false, false, false],
            &[false, true, true],
            &[false, false, false, true],
            &[true, false, false],
            &[false, false, false, false, false, false, false],
            &[true, false, true],
            &[false, false, true, false],
            &[true, true, false],
            &[false, false, false, false, true],
        ],
    );
    exhaustive_vecs_helper(
        'a'..='c',
        &[
            &[],
            &['a'],
            &['b'],
            &['a', 'a', 'a'],
            &['c'],
            &['a', 'a'],
            &['a', 'b'],
            &['a', 'a', 'a', 'a', 'a'],
            &['b', 'a'],
            &['a', 'a', 'b'],
            &['b', 'b'],
            &['a', 'a', 'a', 'a'],
            &['a', 'c'],
            &['a', 'b', 'a'],
            &['b', 'c'],
            &['a', 'a', 'a', 'a', 'a', 'a'],
            &['c', 'a'],
            &['a', 'b', 'b'],
            &['c', 'b'],
            &['a', 'a', 'a', 'b'],
        ],
    );
    exhaustive_vecs_helper(
        exhaustive_ascii_chars(),
        &[
            &[],
            &['a'],
            &['b'],
            &['a', 'a', 'a'],
            &['c'],
            &['a', 'a'],
            &['d'],
            &['a', 'a', 'a', 'a'],
            &['e'],
            &['a', 'b'],
            &['f'],
            &['a', 'a', 'b'],
            &['g'],
            &['b', 'a'],
            &['h'],
            &['a', 'a', 'a', 'a', 'a'],
            &['i'],
            &['b', 'b'],
            &['j'],
            &['a', 'b', 'a'],
        ],
    );
    exhaustive_vecs_helper(
        exhaustive_unsigneds::<u32>(),
        &[
            &[],
            &[0],
            &[1],
            &[0, 0, 0],
            &[2],
            &[0, 0],
            &[3],
            &[0, 0, 0, 0],
            &[4],
            &[0, 1],
            &[5],
            &[0, 0, 1],
            &[6],
            &[1, 0],
            &[7],
            &[0, 0, 0, 0, 0],
            &[8],
            &[1, 1],
            &[9],
            &[0, 1, 0],
        ],
    );
}
