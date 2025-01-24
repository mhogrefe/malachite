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
use malachite_base::test_util::vecs::exhaustive::exhaustive_vecs_helper_helper;
use malachite_base::tuples::exhaustive::exhaustive_units;
use malachite_base::vecs::exhaustive::shortlex_vecs_min_length;
use std::fmt::Debug;

fn shortlex_vecs_min_length_helper<I: Clone + Iterator>(min_length: u64, xs: I, out: &[&[I::Item]])
where
    I::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_helper_helper(shortlex_vecs_min_length(min_length, xs), out);
}

#[test]
fn test_shortlex_vecs_min_length() {
    shortlex_vecs_min_length_helper(0, nevers(), &[&[]]);
    shortlex_vecs_min_length_helper(4, nevers(), &[]);
    shortlex_vecs_min_length_helper(
        0,
        exhaustive_units(),
        &[
            &[],
            &[()],
            &[(), ()],
            &[(), (), ()],
            &[(), (), (), ()],
            &[(), (), (), (), ()],
            &[(), (), (), (), (), ()],
            &[(), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[(); 16],
            &[(); 17],
            &[(); 18],
            &[(); 19],
        ],
    );
    shortlex_vecs_min_length_helper(
        5,
        exhaustive_units(),
        &[
            &[(), (), (), (), ()],
            &[(), (), (), (), (), ()],
            &[(), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[(); 16],
            &[(); 17],
            &[(); 18],
            &[(); 19],
            &[(); 20],
            &[(); 21],
            &[(); 22],
            &[(); 23],
            &[(); 24],
        ],
    );
    shortlex_vecs_min_length_helper(
        0,
        exhaustive_bools(),
        &[
            &[],
            &[false],
            &[true],
            &[false, false],
            &[false, true],
            &[true, false],
            &[true, true],
            &[false, false, false],
            &[false, false, true],
            &[false, true, false],
            &[false, true, true],
            &[true, false, false],
            &[true, false, true],
            &[true, true, false],
            &[true, true, true],
            &[false, false, false, false],
            &[false, false, false, true],
            &[false, false, true, false],
            &[false, false, true, true],
            &[false, true, false, false],
        ],
    );
    shortlex_vecs_min_length_helper(
        3,
        exhaustive_bools(),
        &[
            &[false, false, false],
            &[false, false, true],
            &[false, true, false],
            &[false, true, true],
            &[true, false, false],
            &[true, false, true],
            &[true, true, false],
            &[true, true, true],
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
        ],
    );
    shortlex_vecs_min_length_helper(
        0,
        'a'..='c',
        &[
            &[],
            &['a'],
            &['b'],
            &['c'],
            &['a', 'a'],
            &['a', 'b'],
            &['a', 'c'],
            &['b', 'a'],
            &['b', 'b'],
            &['b', 'c'],
            &['c', 'a'],
            &['c', 'b'],
            &['c', 'c'],
            &['a', 'a', 'a'],
            &['a', 'a', 'b'],
            &['a', 'a', 'c'],
            &['a', 'b', 'a'],
            &['a', 'b', 'b'],
            &['a', 'b', 'c'],
            &['a', 'c', 'a'],
        ],
    );
    shortlex_vecs_min_length_helper(
        3,
        'a'..='c',
        &[
            &['a', 'a', 'a'],
            &['a', 'a', 'b'],
            &['a', 'a', 'c'],
            &['a', 'b', 'a'],
            &['a', 'b', 'b'],
            &['a', 'b', 'c'],
            &['a', 'c', 'a'],
            &['a', 'c', 'b'],
            &['a', 'c', 'c'],
            &['b', 'a', 'a'],
            &['b', 'a', 'b'],
            &['b', 'a', 'c'],
            &['b', 'b', 'a'],
            &['b', 'b', 'b'],
            &['b', 'b', 'c'],
            &['b', 'c', 'a'],
            &['b', 'c', 'b'],
            &['b', 'c', 'c'],
            &['c', 'a', 'a'],
            &['c', 'a', 'b'],
        ],
    );
    shortlex_vecs_min_length_helper(
        0,
        exhaustive_ascii_chars(),
        &[
            &[],
            &['a'],
            &['b'],
            &['c'],
            &['d'],
            &['e'],
            &['f'],
            &['g'],
            &['h'],
            &['i'],
            &['j'],
            &['k'],
            &['l'],
            &['m'],
            &['n'],
            &['o'],
            &['p'],
            &['q'],
            &['r'],
            &['s'],
        ],
    );
    shortlex_vecs_min_length_helper(
        3,
        exhaustive_ascii_chars(),
        &[
            &['a', 'a', 'a'],
            &['a', 'a', 'b'],
            &['a', 'a', 'c'],
            &['a', 'a', 'd'],
            &['a', 'a', 'e'],
            &['a', 'a', 'f'],
            &['a', 'a', 'g'],
            &['a', 'a', 'h'],
            &['a', 'a', 'i'],
            &['a', 'a', 'j'],
            &['a', 'a', 'k'],
            &['a', 'a', 'l'],
            &['a', 'a', 'm'],
            &['a', 'a', 'n'],
            &['a', 'a', 'o'],
            &['a', 'a', 'p'],
            &['a', 'a', 'q'],
            &['a', 'a', 'r'],
            &['a', 'a', 's'],
            &['a', 'a', 't'],
        ],
    );
}
