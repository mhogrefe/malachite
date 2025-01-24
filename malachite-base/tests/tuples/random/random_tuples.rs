// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::extra_variadic::{random_triples, random_triples_from_single};
use core::hash::Hash;
use itertools::Itertools;
use malachite_base::bools::random::random_bools;
use malachite_base::chars::random::random_ascii_chars;
use malachite_base::num::random::geometric::geometric_random_signeds;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;
use malachite_base::tuples::random::{random_pairs, random_pairs_from_single};
use malachite_base::tuples::singletons;
use std::fmt::Debug;

fn random_pairs_helper<
    X: Clone + Debug + Eq + Hash + Ord,
    I: Clone + Iterator<Item = X>,
    Y: Clone + Debug + Eq + Hash + Ord,
    J: Clone + Iterator<Item = Y>,
>(
    xs_gen: &dyn Fn(Seed) -> I,
    ys_gen: &dyn Fn(Seed) -> J,
    expected_values: &[(X, Y)],
    expected_common_values: &[((X, Y), usize)],
    expected_median: ((X, Y), Option<(X, Y)>),
) {
    let xs = random_pairs(EXAMPLE_SEED, xs_gen, ys_gen);
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_pairs() {
    random_pairs_helper(
        &random_primitive_ints::<u8>,
        &random_bools,
        &[
            (85, false),
            (11, true),
            (136, false),
            (200, false),
            (235, false),
            (134, true),
            (203, true),
            (223, false),
            (38, false),
            (235, false),
            (217, false),
            (177, true),
            (162, false),
            (32, true),
            (166, false),
            (234, true),
            (30, false),
            (218, true),
            (90, true),
            (106, false),
        ],
        &[
            ((81, true), 2077),
            ((58, false), 2074),
            ((220, false), 2064),
            ((14, false), 2053),
            ((194, true), 2050),
            ((66, false), 2050),
            ((71, true), 2049),
            ((208, false), 2043),
            ((7, true), 2041),
            ((64, true), 2038),
        ],
        ((127, true), None),
    );
    random_pairs_helper(
        &|seed| random_pairs_from_single(random_primitive_ints::<u8>(seed)),
        &|seed| random_triples_from_single(random_primitive_ints::<i8>(seed)),
        &[
            ((85, 11), (98, -88, -58)),
            ((136, 200), (40, 20, -4)),
            ((235, 134), (47, 87, -124)),
            ((203, 223), (72, 77, 63)),
            ((38, 235), (91, 108, 127)),
            ((217, 177), (53, -115, 84)),
            ((162, 32), (18, 10, 112)),
            ((166, 234), (-102, 104, 53)),
            ((30, 218), (75, -18, -107)),
            ((90, 106), (-66, 51, -109)),
            ((9, 216), (100, 114, -116)),
            ((204, 151), (2, 63, -67)),
            ((213, 97), (-34, 67, 119)),
            ((253, 78), (0, -33, 5)),
            ((91, 39), (-20, -24, 50)),
            ((191, 175), (44, -15, 21)),
            ((170, 232), (22, 94, 27)),
            ((233, 2), (-128, -36, 25)),
            ((35, 22), (-5, -13, 50)),
            ((217, 198), (-119, -21, 46)),
        ],
        &[
            (((0, 5), (6, 7, 42)), 1),
            (((8, 8), (18, 5, 6)), 1),
            (((9, 1), (5, 3, 23)), 1),
            (((0, 0), (97, 7, 73)), 1),
            (((0, 2), (12, 20, 6)), 1),
            (((0, 99), (20, 8, 6)), 1),
            (((1, 81), (3, 21, 3)), 1),
            (((1, 83), (-6, 8, 8)), 1),
            (((1, 9), (-37, 9, 7)), 1),
            (((1, 9), (4, 95, 15)), 1),
        ],
        (
            ((127, 197), (-18, 55, -20)),
            Some(((127, 197), (-8, -68, 49))),
        ),
    );
}

#[allow(clippy::type_complexity)]
fn random_triples_helper<
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
    expected_values: &[(X, Y, Z)],
    expected_common_values: &[((X, Y, Z), usize)],
    expected_median: ((X, Y, Z), Option<(X, Y, Z)>),
) {
    let xs = random_triples(EXAMPLE_SEED, xs_gen, ys_gen, zs_gen);
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_triples() {
    random_triples_helper(
        &random_primitive_ints::<u8>,
        &random_ascii_chars,
        &|seed| geometric_random_signeds::<i8>(seed, 10, 1),
        &[
            (85, 'b', 0),
            (11, 'P', -6),
            (136, '\u{1a}', -6),
            (200, 'F', 8),
            (235, 'B', 0),
            (134, '\u{2}', -3),
            (203, '\u{7f}', -17),
            (223, '\u{17}', 2),
            (38, 'W', 0),
            (235, '\u{8}', 2),
            (217, '\"', 9),
            (177, 'j', -6),
            (162, 't', 20),
            (32, 'g', -20),
            (166, '\u{16}', 43),
            (234, '6', 9),
            (30, '\u{7f}', -7),
            (218, 'j', -16),
            (90, '4', -29),
            (106, '$', -2),
        ],
        &[
            ((54, '*', -1), 9),
            ((252, '\u{c}', 0), 9),
            ((253, '\u{6}', 0), 9),
            ((4, '~', 0), 8),
            ((51, '1', 1), 8),
            ((131, '!', 1), 8),
            ((138, 'i', 1), 8),
            ((185, 'q', 1), 8),
            ((58, '?', -2), 8),
            ((225, 'k', -4), 8),
        ],
        ((127, 'x', -2), None),
    );
    random_triples_helper(
        &|seed| singletons(random_bools(seed)),
        &|seed| random_pairs_from_single(random_primitive_ints::<u8>(seed)),
        &|seed| random_triples_from_single(random_ascii_chars(seed)),
        &[
            ((true,), (98, 168), ('(', '\u{15}', 'h')),
            ((false,), (198, 40), ('\u{7f}', '%', '\u{7f}')),
            ((true,), (20, 252), ('\u{13}', '\u{2}', '+')),
            ((false,), (47, 87), ('\u{1b}', 'v', '\r')),
            ((true,), (132, 72), ('\u{1b}', '\u{15}', 'I')),
            ((false,), (77, 63), ('$', '\u{1a}', '}')),
            ((true,), (91, 108), ('(', '\u{e}', '1')),
            ((false,), (127, 53), ('$', '/', 'O')),
            ((true,), (141, 84), ('\u{1f}', 'Z', '>')),
            ((true,), (18, 10), ('}', '\u{13}', '\\')),
            ((false,), (112, 154), ('\u{1a}', '\u{14}', 't')),
            ((true,), (104, 53), (' ', '`', '\u{2}')),
            ((false,), (75, 238), ('\u{17}', 'a', '8')),
            ((false,), (149, 190), ('H', ']', '*')),
            ((false,), (51, 147), ('i', '2', '}')),
            ((false,), (100, 114), ('\u{3}', '\u{f}', '\u{7f}')),
            ((false,), (140, 2), ('\u{f}', 'Y', 'D')),
            ((false,), (63, 189), ('m', '\\', '8')),
            ((false,), (222, 67), ('M', '\u{7}', '8')),
            ((true,), (119, 0), ('\u{13}', '.', '\"')),
        ],
        &[
            (((true,), (57, 9), ('R', '}', 'Q')), 2),
            (((true,), (233, 229), ('t', '\u{b}', 'Q')), 2),
            (((false,), (236, 203), ('b', '\u{e}', '\u{e}')), 2),
            (((true,), (0, 0), ('{', '{', '4')), 1),
            (((true,), (0, 2), ('-', 'S', '{')), 1),
            (((true,), (0, 2), ('N', 'E', '-')), 1),
            (((true,), (0, 2), ('O', '3', 'S')), 1),
            (((true,), (0, 3), ('"', '/', 'P')), 1),
            (((true,), (0, 3), (';', 'M', 'W')), 1),
            (((true,), (0, 3), ('i', ']', 'P')), 1),
        ],
        (
            ((false,), (255, 175), ('g', '\u{1e}', '4')),
            Some(((false,), (255, 176), ('\u{10}', 's', '\''))),
        ),
    );
}
