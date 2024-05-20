// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::extra_variadic::random_union3s;
use crate::extra_variadic::Union3;
use core::hash::Hash;
use itertools::Itertools;
use malachite_base::bools::random::random_bools;
use malachite_base::chars::random::random_char_inclusive_range;
use malachite_base::num::random::geometric::geometric_random_unsigneds;
use malachite_base::num::random::random_unsigned_inclusive_range;
use malachite_base::orderings::random::random_orderings;
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::strings::random::random_strings_using_chars;
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;
use malachite_base::unions::random::random_union2s;
use malachite_base::unions::Union2;
use std::cmp::Ordering::*;
use std::fmt::Debug;

fn random_union2s_helper<
    X: Clone + Debug + Eq + Hash + Ord,
    I: Clone + Iterator<Item = X>,
    Y: Clone + Debug + Eq + Hash + Ord,
    J: Clone + Iterator<Item = Y>,
>(
    xs_gen: &dyn Fn(Seed) -> I,
    ys_gen: &dyn Fn(Seed) -> J,
    expected_values: &[Union2<X, Y>],
    expected_common_values: &[(Union2<X, Y>, usize)],
    expected_median: (Union2<X, Y>, Option<Union2<X, Y>>),
) {
    let us = random_union2s(EXAMPLE_SEED, xs_gen, ys_gen);
    let values = us.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, us.clone());
    let median = median(us.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_union2s() {
    random_union2s_helper(
        &|seed| random_char_inclusive_range(seed, 'a', 'z'),
        &|seed| random_unsigned_inclusive_range::<u32>(seed, 1, 10),
        &[
            Union2::A('v'),
            Union2::B(3),
            Union2::A('c'),
            Union2::A('q'),
            Union2::A('i'),
            Union2::A('e'),
            Union2::A('p'),
            Union2::A('g'),
            Union2::A('s'),
            Union2::B(7),
            Union2::A('n'),
            Union2::A('t'),
            Union2::B(9),
            Union2::A('m'),
            Union2::A('z'),
            Union2::B(7),
            Union2::B(9),
            Union2::A('o'),
            Union2::A('m'),
            Union2::B(3),
        ],
        &[
            (Union2::B(5), 50535),
            (Union2::B(4), 50190),
            (Union2::B(10), 50183),
            (Union2::B(3), 50068),
            (Union2::B(9), 50064),
            (Union2::B(6), 50002),
            (Union2::B(2), 49882),
            (Union2::B(1), 49807),
            (Union2::B(7), 49533),
            (Union2::B(8), 49495),
        ],
        (Union2::A('z'), None),
    );
    random_union2s_helper(
        &random_bools,
        &|seed| geometric_random_unsigneds::<u32>(seed, 4, 1),
        &[
            Union2::A(true),
            Union2::B(6),
            Union2::A(false),
            Union2::A(true),
            Union2::A(false),
            Union2::A(true),
            Union2::A(false),
            Union2::A(true),
            Union2::A(false),
            Union2::B(0),
            Union2::A(true),
            Union2::A(true),
            Union2::B(2),
            Union2::A(false),
            Union2::A(true),
            Union2::B(2),
            Union2::B(2),
            Union2::A(false),
            Union2::A(false),
            Union2::B(8),
        ],
        &[
            (Union2::A(false), 250462),
            (Union2::A(true), 249779),
            (Union2::B(0), 100190),
            (Union2::B(1), 79510),
            (Union2::B(2), 63929),
            (Union2::B(3), 51260),
            (Union2::B(4), 41099),
            (Union2::B(5), 32494),
            (Union2::B(6), 26037),
            (Union2::B(7), 21139),
        ],
        (Union2::A(true), None),
    );
}

#[allow(clippy::type_complexity)]
fn random_union3s_helper<
    X: Clone + Debug + Eq + Hash + Ord,
    I: Clone + Iterator<Item = X>,
    Y: Clone + Debug + Eq + Hash + Ord,
    J: Clone + Iterator<Item = Y>,
    Z: Clone + Debug + Eq + Hash + Ord,
    K: Clone + Iterator<Item = Z>,
>(
    xs_gen: &dyn Fn(Seed) -> I,
    ys_gen: &dyn Fn(Seed) -> J,
    zs_gen: &dyn Fn(Seed) -> K,
    expected_values: &[Union3<X, Y, Z>],
    expected_common_values: &[(Union3<X, Y, Z>, usize)],
    expected_median: (Union3<X, Y, Z>, Option<Union3<X, Y, Z>>),
) {
    let us = random_union3s(EXAMPLE_SEED, xs_gen, ys_gen, zs_gen);
    let values = us.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, us.clone());
    let median = median(us.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_union3s() {
    random_union3s_helper(
        &|seed| random_char_inclusive_range(seed, 'a', 'z'),
        &|seed| random_unsigned_inclusive_range::<u32>(seed, 1, 10),
        &random_orderings,
        &[
            Union3::C(Equal),
            Union3::A('v'),
            Union3::A('c'),
            Union3::A('q'),
            Union3::C(Greater),
            Union3::A('i'),
            Union3::B(3),
            Union3::C(Greater),
            Union3::B(7),
            Union3::C(Greater),
            Union3::C(Greater),
            Union3::C(Greater),
            Union3::B(9),
            Union3::C(Equal),
            Union3::A('e'),
            Union3::A('p'),
            Union3::A('g'),
            Union3::A('s'),
            Union3::C(Equal),
            Union3::A('n'),
        ],
        &[
            (Union3::C(Less), 111378),
            (Union3::C(Greater), 111191),
            (Union3::C(Equal), 110903),
            (Union3::B(5), 33724),
            (Union3::B(10), 33503),
            (Union3::B(4), 33375),
            (Union3::B(9), 33348),
            (Union3::B(2), 33347),
            (Union3::B(6), 33288),
            (Union3::B(7), 33283),
        ],
        (Union3::B(5), None),
    );
    random_union3s_helper(
        &random_bools,
        &|seed| geometric_random_unsigneds::<u32>(seed, 4, 1),
        &|seed| {
            random_strings_using_chars(
                seed,
                &|seed_2| random_char_inclusive_range(seed_2, 'a', 'z'),
                4,
                1,
            )
        },
        &[
            Union3::C("qvfm".to_string()),
            Union3::A(true),
            Union3::A(false),
            Union3::A(true),
            Union3::C("kt".to_string()),
            Union3::A(false),
            Union3::B(6),
            Union3::C("auqoox".to_string()),
            Union3::B(0),
            Union3::C("ak".to_string()),
            Union3::C("".to_string()),
            Union3::C("dz".to_string()),
            Union3::B(2),
            Union3::C("ebaq".to_string()),
            Union3::A(true),
            Union3::A(false),
            Union3::A(true),
            Union3::A(false),
            Union3::C("gvqmloscuftfzjrn".to_string()),
            Union3::A(true),
        ],
        &[
            (Union3::A(false), 166833),
            (Union3::A(true), 166495),
            (Union3::B(0), 66610),
            (Union3::C("".to_string()), 66483),
            (Union3::B(1), 53004),
            (Union3::B(2), 42754),
            (Union3::B(3), 34312),
            (Union3::B(4), 27360),
            (Union3::B(5), 21599),
            (Union3::B(6), 17332),
        ],
        (Union3::B(3), None),
    );
}
