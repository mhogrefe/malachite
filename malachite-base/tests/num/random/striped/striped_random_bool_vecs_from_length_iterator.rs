// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::random::striped::get_striped_bool_vec::bool_slice_to_string;
use itertools::Itertools;
use malachite_base::num::random::geometric::geometric_random_unsigneds;
use malachite_base::num::random::striped::striped_random_bool_vecs_from_length_iterator;
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;
use malachite_base::vecs::random_values_from_vec;
use std::iter::repeat;

fn striped_random_bool_vecs_from_length_iterator_helper<I: Clone + Iterator<Item = u64>>(
    lengths_gen: &dyn Fn(Seed) -> I,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_median: (&str, Option<&str>),
) {
    let xs = striped_random_bool_vecs_from_length_iterator(
        EXAMPLE_SEED,
        lengths_gen,
        mean_stripe_numerator,
        mean_stripe_denominator,
    );
    let values = xs
        .clone()
        .take(20)
        .map(|bs| bool_slice_to_string(&bs))
        .collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone())
        .into_iter()
        .map(|(bs, freq)| (bool_slice_to_string(&bs), freq))
        .collect_vec();
    let (median_lo, median_hi) = median(xs.take(1000000));
    let median_lo = bool_slice_to_string(&median_lo);
    let median_hi = median_hi.map(|bs| bool_slice_to_string(&bs));
    assert_eq!(
        (
            values.iter().map(String::as_str).collect_vec().as_slice(),
            common_values
                .iter()
                .map(|(s, f)| (s.as_str(), *f))
                .collect_vec()
                .as_slice(),
            (median_lo.as_str(), median_hi.as_deref())
        ),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_striped_random_bool_vecs_from_length_iterator() {
    striped_random_bool_vecs_from_length_iterator_helper(
        &|seed| random_values_from_vec(seed, vec![0, 2, 4]),
        2,
        1,
        &[
            "00", "0110", "00", "0110", "0001", "11", "", "01", "", "1110", "0110", "11", "1000",
            "01", "0100", "0001", "1010", "", "0000", "",
        ],
        &[
            ("", 333820),
            ("00", 83428),
            ("10", 83346),
            ("11", 83184),
            ("01", 83167),
            ("0010", 20955),
            ("0001", 20940),
            ("1110", 20921),
            ("1011", 20912),
            ("0011", 20870),
        ],
        ("0011", None),
    );
    striped_random_bool_vecs_from_length_iterator_helper(
        &|seed| random_values_from_vec(seed, vec![0, 2, 4]),
        10,
        1,
        &[
            "00", "0000", "00", "0000", "0000", "11", "", "00", "", "1111", "0001", "11", "1100",
            "00", "0000", "0000", "1110", "", "0000", "",
        ],
        &[
            ("", 333820),
            ("11", 149822),
            ("00", 149788),
            ("0000", 121534),
            ("1111", 121330),
            ("01", 16807),
            ("10", 16708),
            ("0011", 13644),
            ("1110", 13490),
            ("0001", 13454),
        ],
        ("0000", None),
    );
    striped_random_bool_vecs_from_length_iterator_helper(
        &|seed| random_values_from_vec(seed, vec![0, 2, 4]),
        11,
        10,
        &[
            "01", "0100", "01", "0101", "0101", "10", "", "01", "", "1001", "0101", "10", "1101",
            "01", "0101", "0110", "1010", "", "0010", "",
        ],
        &[
            ("", 333820),
            ("01", 151459),
            ("10", 151370),
            ("0101", 125515),
            ("1010", 124901),
            ("11", 15160),
            ("00", 15136),
            ("0010", 12625),
            ("1011", 12598),
            ("1001", 12494),
        ],
        ("01", None),
    );
    striped_random_bool_vecs_from_length_iterator_helper(
        &|seed| geometric_random_unsigneds::<u64>(seed, 2, 1).map(|x| x << 1),
        2,
        1,
        &[
            "001100110001",
            "00",
            "0111011001110100",
            "00",
            "0010100000101111001100110100",
            "",
            "1110000000",
            "01101110",
            "10",
            "",
            "001110000111",
            "1111",
            "",
            "",
            "11",
            "010100",
            "",
            "01",
            "",
            "00",
        ],
        &[
            ("", 333981),
            ("11", 56301),
            ("01", 55922),
            ("00", 55472),
            ("10", 55087),
            ("0101", 9537),
            ("0100", 9341),
            ("0010", 9326),
            ("1011", 9314),
            ("0001", 9297),
        ],
        ("00111111100101", Some("0011111110010100")),
    );
    striped_random_bool_vecs_from_length_iterator_helper(
        &|seed| geometric_random_unsigneds::<u64>(seed, 2, 1).map(|x| x << 1),
        10,
        1,
        &[
            "000000000000",
            "00",
            "0000000111000000",
            "00",
            "0000011111111000000111111111",
            "",
            "1110000000",
            "00000000",
            "11",
            "",
            "000000000000",
            "1111",
            "",
            "",
            "11",
            "011111",
            "",
            "00",
            "",
            "00",
        ],
        &[
            ("", 333981),
            ("00", 100383),
            ("11", 100353),
            ("1111", 53920),
            ("0000", 53883),
            ("000000", 29226),
            ("111111", 29014),
            ("00000000", 15928),
            ("11111111", 15670),
            ("10", 11035),
        ],
        ("000000", None),
    );
    striped_random_bool_vecs_from_length_iterator_helper(
        &|seed| geometric_random_unsigneds::<u64>(seed, 2, 1).map(|x| x << 1),
        11,
        10,
        &[
            "010110101010",
            "01",
            "0100101011010101",
            "01",
            "0010100101001010101010101010",
            "",
            "1010101101",
            "01011010",
            "10",
            "",
            "010101010101",
            "1100",
            "",
            "",
            "10",
            "010110",
            "",
            "01",
            "",
            "01",
        ],
        &[
            ("", 333981),
            ("10", 101326),
            ("01", 101218),
            ("0101", 55895),
            ("1010", 55433),
            ("101010", 30661),
            ("010101", 30567),
            ("01010101", 17148),
            ("10101010", 16775),
            ("00", 10176),
        ],
        ("0101", None),
    );
}

#[test]
#[should_panic]
fn striped_random_bool_vecs_from_length_iterator_fail_1() {
    striped_random_bool_vecs_from_length_iterator(EXAMPLE_SEED, &|_| repeat(1), 1, 0);
}

#[test]
#[should_panic]
fn striped_random_bool_vecs_from_length_iterator_fail_2() {
    striped_random_bool_vecs_from_length_iterator(EXAMPLE_SEED, &|_| repeat(1), 2, 3);
}
