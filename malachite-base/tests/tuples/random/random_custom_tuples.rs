// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::extra_variadic::{random_triples_from_single, random_triples_xxy, random_triples_xyx};
use core::hash::Hash;
use itertools::Itertools;
use malachite_base::bools::random::random_bools;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;
use malachite_base::tuples::random::random_pairs_from_single;
use std::fmt::Debug;

#[allow(clippy::type_complexity)]
fn random_triples_xxy_helper<
    X: Clone + Debug + Eq + Hash + Ord,
    I: Clone + Iterator<Item = X>,
    Y: Clone + Debug + Eq + Hash + Ord,
    J: Clone + Iterator<Item = Y>,
>(
    xs_gen: &dyn Fn(Seed) -> I,
    ys_gen: &dyn Fn(Seed) -> J,
    expected_values: &[(X, X, Y)],
    expected_common_values: &[((X, X, Y), usize)],
    expected_median: ((X, X, Y), Option<(X, X, Y)>),
) {
    let xs = random_triples_xxy(EXAMPLE_SEED, xs_gen, ys_gen);
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_triples_xxy() {
    random_triples_xxy_helper(
        &random_primitive_ints::<u8>,
        &random_bools,
        &[
            (85, 11, false),
            (136, 200, true),
            (235, 134, false),
            (203, 223, false),
            (38, 235, false),
            (217, 177, true),
            (162, 32, true),
            (166, 234, false),
            (30, 218, false),
            (90, 106, false),
            (9, 216, false),
            (204, 151, true),
            (213, 97, false),
            (253, 78, true),
            (91, 39, false),
            (191, 175, true),
            (170, 232, false),
            (233, 2, true),
            (35, 22, true),
            (217, 198, false),
        ],
        &[
            ((87, 70, false), 23),
            ((36, 187, false), 23),
            ((228, 249, false), 22),
            ((130, 73, true), 20),
            ((67, 187, true), 20),
            ((89, 216, true), 20),
            ((132, 195, true), 20),
            ((145, 23, false), 20),
            ((24, 126, false), 20),
            ((146, 106, false), 20),
        ],
        ((127, 197, true), None),
    );
    random_triples_xxy_helper(
        &|seed| random_pairs_from_single(random_primitive_ints::<u8>(seed)),
        &|seed| random_triples_from_single(random_primitive_ints::<i8>(seed)),
        &[
            ((85, 11), (136, 200), (98, -88, -58)),
            ((235, 134), (203, 223), (40, 20, -4)),
            ((38, 235), (217, 177), (47, 87, -124)),
            ((162, 32), (166, 234), (72, 77, 63)),
            ((30, 218), (90, 106), (91, 108, 127)),
            ((9, 216), (204, 151), (53, -115, 84)),
            ((213, 97), (253, 78), (18, 10, 112)),
            ((91, 39), (191, 175), (-102, 104, 53)),
            ((170, 232), (233, 2), (75, -18, -107)),
            ((35, 22), (217, 198), (-66, 51, -109)),
            ((114, 17), (32, 173), (100, 114, -116)),
            ((114, 65), (121, 222), (2, 63, -67)),
            ((173, 25), (144, 148), (-34, 67, 119)),
            ((79, 115), (52, 73), (0, -33, 5)),
            ((69, 137), (91, 153), (-20, -24, 50)),
            ((178, 112), (34, 95), (44, -15, 21)),
            ((106, 167), (197, 130), (22, 94, 27)),
            ((168, 122), (207, 172), (-128, -36, 25)),
            ((177, 86), (150, 221), (-5, -13, 50)),
            ((218, 101), (115, 74), (-119, -21, 46)),
        ],
        &[
            (((8, 24), (5, 3), (0, 54, 59)), 1),
            (((8, 72), (11, 57), (6, 5, 9)), 1),
            (((80, 9), (9, 5), (84, 9, 10)), 1),
            (((86, 2), (49, 4), (2, 0, 27)), 1),
            (((0, 2), (92, 5), (-49, 31, 7)), 1),
            (((1, 15), (12, 5), (51, 5, 47)), 1),
            (((1, 25), (3, 66), (70, 65, 7)), 1),
            (((1, 72), (2, 1), (8, 49, -10)), 1),
            (((1, 82), (6, 26), (86, 3, 70)), 1),
            (((1, 85), (14, 92), (3, 5, 53)), 1),
        ],
        (
            ((128, 20), (243, 155), (-90, 7, -77)),
            Some(((128, 21), (19, 63), (52, 113, -21))),
        ),
    );
}

#[allow(clippy::type_complexity)]
fn random_triples_xyx_helper<
    X: Clone + Debug + Eq + Hash + Ord,
    I: Clone + Iterator<Item = X>,
    Y: Clone + Debug + Eq + Hash + Ord,
    J: Clone + Iterator<Item = Y>,
>(
    xs_gen: &dyn Fn(Seed) -> I,
    ys_gen: &dyn Fn(Seed) -> J,
    expected_values: &[(X, Y, X)],
    expected_common_values: &[((X, Y, X), usize)],
    expected_median: ((X, Y, X), Option<(X, Y, X)>),
) {
    let xs = random_triples_xyx(EXAMPLE_SEED, xs_gen, ys_gen);
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_triples_xyx() {
    random_triples_xyx_helper(
        &random_primitive_ints::<u8>,
        &random_bools,
        &[
            (85, false, 11),
            (136, true, 200),
            (235, false, 134),
            (203, false, 223),
            (38, false, 235),
            (217, true, 177),
            (162, true, 32),
            (166, false, 234),
            (30, false, 218),
            (90, false, 106),
            (9, false, 216),
            (204, true, 151),
            (213, false, 97),
            (253, true, 78),
            (91, false, 39),
            (191, true, 175),
            (170, false, 232),
            (233, true, 2),
            (35, true, 22),
            (217, false, 198),
        ],
        &[
            ((87, false, 70), 23),
            ((36, false, 187), 23),
            ((228, false, 249), 22),
            ((130, true, 73), 20),
            ((67, true, 187), 20),
            ((89, true, 216), 20),
            ((132, true, 195), 20),
            ((145, false, 23), 20),
            ((24, false, 126), 20),
            ((146, false, 106), 20),
        ],
        ((127, true, 141), None),
    );
    random_triples_xyx_helper(
        &|seed| random_pairs_from_single(random_primitive_ints::<u8>(seed)),
        &|seed| random_triples_from_single(random_primitive_ints::<i8>(seed)),
        &[
            ((85, 11), (98, -88, -58), (136, 200)),
            ((235, 134), (40, 20, -4), (203, 223)),
            ((38, 235), (47, 87, -124), (217, 177)),
            ((162, 32), (72, 77, 63), (166, 234)),
            ((30, 218), (91, 108, 127), (90, 106)),
            ((9, 216), (53, -115, 84), (204, 151)),
            ((213, 97), (18, 10, 112), (253, 78)),
            ((91, 39), (-102, 104, 53), (191, 175)),
            ((170, 232), (75, -18, -107), (233, 2)),
            ((35, 22), (-66, 51, -109), (217, 198)),
            ((114, 17), (100, 114, -116), (32, 173)),
            ((114, 65), (2, 63, -67), (121, 222)),
            ((173, 25), (-34, 67, 119), (144, 148)),
            ((79, 115), (0, -33, 5), (52, 73)),
            ((69, 137), (-20, -24, 50), (91, 153)),
            ((178, 112), (44, -15, 21), (34, 95)),
            ((106, 167), (22, 94, 27), (197, 130)),
            ((168, 122), (-128, -36, 25), (207, 172)),
            ((177, 86), (-5, -13, 50), (150, 221)),
            ((218, 101), (-119, -21, 46), (115, 74)),
        ],
        &[
            (((8, 24), (0, 54, 59), (5, 3)), 1),
            (((8, 72), (6, 5, 9), (11, 57)), 1),
            (((80, 9), (84, 9, 10), (9, 5)), 1),
            (((86, 2), (2, 0, 27), (49, 4)), 1),
            (((0, 2), (-49, 31, 7), (92, 5)), 1),
            (((1, 15), (51, 5, 47), (12, 5)), 1),
            (((1, 25), (70, 65, 7), (3, 66)), 1),
            (((1, 72), (8, 49, -10), (2, 1)), 1),
            (((1, 82), (86, 3, 70), (6, 26)), 1),
            (((1, 85), (3, 5, 53), (14, 92)), 1),
        ],
        (
            ((128, 20), (118, 50, 18), (55, 110)),
            Some(((128, 21), (-107, 66, -82), (216, 142))),
        ),
    );
}
