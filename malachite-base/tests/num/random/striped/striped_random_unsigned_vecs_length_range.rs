// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::random::striped::striped_random_unsigned_vecs_length_range;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::strings::ToBinaryString;
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;

fn striped_random_unsigned_vecs_length_range_helper<T: PrimitiveUnsigned>(
    a: u64,
    b: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    expected_values: &[&[&str]],
    expected_common_values: &[(&[&str], usize)],
    expected_median: (&[&str], Option<&[&str]>),
) {
    let xss = striped_random_unsigned_vecs_length_range::<T>(
        EXAMPLE_SEED,
        a,
        b,
        mean_stripe_numerator,
        mean_stripe_denominator,
    );
    let values = xss
        .clone()
        .take(20)
        .map(|xs| {
            xs.into_iter()
                .map(|x: T| x.to_binary_string())
                .collect_vec()
        })
        .collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xss.clone())
        .into_iter()
        .map(|(xs, freq)| {
            (
                xs.into_iter()
                    .map(|x: T| x.to_binary_string())
                    .collect_vec(),
                freq,
            )
        })
        .collect_vec();
    let (median_lo, median_hi) = median(xss.take(1000000));
    let median_lo = median_lo
        .into_iter()
        .map(|x: T| x.to_binary_string())
        .collect_vec();
    let median_hi = median_hi.map(|xs| {
        xs.into_iter()
            .map(|x: T| x.to_binary_string())
            .collect_vec()
    });

    let values = values
        .iter()
        .map(|xs| xs.iter().map(String::as_str).collect_vec())
        .collect_vec();
    let common_values = common_values
        .iter()
        .map(|(xs, freq)| (xs.iter().map(String::as_str).collect_vec(), *freq))
        .collect_vec();
    let median_lo = median_lo.iter().map(String::as_str).collect_vec();
    let median_hi = median_hi
        .as_ref()
        .map(|xs| xs.iter().map(String::as_str).collect_vec());
    assert_eq!(
        (
            values.iter().map(Vec::as_slice).collect_vec().as_slice(),
            common_values
                .iter()
                .map(|(xs, f)| (xs.as_slice(), *f))
                .collect_vec()
                .as_slice(),
            (median_lo.as_slice(), median_hi.as_deref())
        ),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_striped_random_unsigned_vecs_length_range() {
    striped_random_unsigned_vecs_length_range_helper::<u8>(
        3,
        6,
        2,
        1,
        &[
            &["11001100", "11000", "11001", "10111101"],
            &["10000010", "10011110", "1011001", "11111000", "10011"],
            &["1110010", "11111000", "1101011", "110"],
            &["1000010", "10111001", "11100000", "11001001", "10111010"],
            &["11011000", "1010", "11101011", "10011011", "10001"],
            &["10111111", "10110100", "1111011", "110011"],
            &["11100010", "11110000", "111101"],
            &["1010011", "10100011", "10001110", "10000100"],
            &["1010110", "100", "11101010"],
            &["11000101", "10111101", "100111", "11110110", "10100"],
            &["1110011", "1111111", "100010", "111110", "11100011"],
            &["1000100", "10110001", "1100", "11011110"],
            &["1010110", "110111", "11111001", "11110010", "10011010"],
            &["11110110", "110110", "1011010", "11111111"],
            &["1111011", "11100010", "11000111", "10010000", "11111111"],
            &["11001110", "11001000", "1110000", "10011100", "101000"],
            &["111001", "10001100", "10100", "11101001", "11111101"],
            &["10010000", "10010101", "11000100"],
            &["10110100", "1110110", "111001", "10101100", "1000110"],
            &["1101", "1011010", "11100010"],
        ],
        &[
            (&["1001110", "1011", "11000"], 3),
            (&["1011010", "10011", "100010"], 3),
            (&["1100010", "101000", "1000010"], 3),
            (&["1110111", "101000", "100101"], 3),
            (&["10110", "1100010", "10011111"], 3),
            (&["11001", "1011", "10110011"], 3),
            (&["11001", "1110010", "1000101"], 3),
            (&["101001", "101000", "1110000"], 3),
            (&["1100101", "1111001", "11100"], 3),
            (&["10010000", "1110110", "1011111"], 3),
        ],
        (
            &["1111111", "11110100", "1010100", "1100"],
            Some(&["1111111", "11110100", "1100001"]),
        ),
    );
    striped_random_unsigned_vecs_length_range_helper::<u8>(
        3,
        6,
        10,
        1,
        &[
            &["0", "0", "111000", "0"],
            &["11111100", "11", "11111111", "111", "0"],
            &["0", "0", "11111100", "11111111"],
            &["0", "111111", "0", "1000", "0"],
            &["11111100", "11111111", "1111111", "11111000", "11"],
            &["11111111", "11111111", "11001111", "0"],
            &["11100000", "11111111", "11111111"],
            &["11111111", "11111111", "11111111", "11111111"],
            &["0", "10", "0"],
            &["11111111", "1111111", "0", "0", "0"],
            &["11111111", "11111111", "1111", "10000", "11111111"],
            &["11000000", "111011", "1000000", "11111011"],
            &["10", "0", "0", "0", "11111111"],
            &["111100", "10000000", "11111100", "1111"],
            &["11111111", "1", "1000000", "0", "0"],
            &["11110000", "11111111", "0", "0", "111100"],
            &["11111111", "111", "11111100", "11111111", "1111111"],
            &["0", "10000000", "11111111"],
            &["11110000", "10011111", "11111111", "11011111", "1"],
            &["11111", "11110000", "111"],
        ],
        &[
            (&["0", "0", "0"], 14915),
            (&["11111111", "11111111", "11111111"], 14783),
            (&["0", "0", "0", "0"], 6456),
            (&["11111111", "11111111", "11111111", "11111111"], 6260),
            (&["0", "0", "0", "0", "0"], 2738),
            (
                &["11111111", "11111111", "11111111", "11111111", "11111111"],
                2722,
            ),
            (&["11111111", "111111", "0"], 1721),
            (&["11111111", "11111111", "1111"], 1717),
            (&["0", "0", "10000000"], 1708),
            (&["11110000", "11111111", "11111111"], 1707),
        ],
        (&["10000000", "0", "0"], None),
    );
    striped_random_unsigned_vecs_length_range_helper::<u8>(
        3,
        6,
        11,
        10,
        &[
            &["1011010", "11010101", "1001010", "10110101"],
            &["1010010", "10101010", "10101010", "101010", "10110101"],
            &["10101010", "1101010", "1001010", "1010101"],
            &["10101010", "10101010", "1011010", "10101101", "1010100"],
            &["1010010", "1010101", "1010101", "10101001", "101010"],
            &["10010101", "10101010", "10101010", "10101010"],
            &["11011010", "1010110", "1010101"],
            &["1010101", "1011011", "1010101", "10110101"],
            &["1010110", "1010101", "1010101"],
            &["10010101", "10010111", "10100011", "10101010", "1101000"],
            &["1010101", "1001011", "1010101", "10101001", "10101010"],
            &["1010110", "10110101", "10101010", "1010100"],
            &["10101010", "10101010", "10101010", "10101010", "100101"],
            &["1001010", "10101011", "10101010", "10101010"],
            &["1010101", "10101010", "10101010", "1011010", "10101101"],
            &["1010110", "10101010", "1100100", "1010101", "1010101"],
            &["10100101", "1010110", "100101", "1010101", "1010101"],
            &["1010110", "1010101", "10101101"],
            &["10101010", "10101010", "1010100", "1010101", "11010101"],
            &["1010101", "10101", "1010101"],
        ],
        &[
            (&["10101010", "10101010", "10101010"], 18583),
            (&["1010101", "1010101", "1010101"], 18554),
            (&["1010101", "1010101", "1010101", "1010101"], 8801),
            (&["10101010", "10101010", "10101010", "10101010"], 8734),
            (
                &["1010101", "1010101", "1010101", "1010101", "1010101"],
                4002,
            ),
            (
                &["10101010", "10101010", "10101010", "10101010", "10101010"],
                3983,
            ),
            (&["1101010", "1010101", "1010101"], 1971),
            (&["10110101", "10101010", "10101010"], 1953),
            (&["1010101", "10101101", "10101010"], 1936),
            (&["1010101", "10110101", "10101010"], 1934),
        ],
        (
            &["10000101", "101010", "10101010"],
            Some(&["10000101", "1001010", "1001001", "100101"]),
        ),
    );
}

#[test]
#[should_panic]
fn striped_random_unsigned_vecs_length_range_fail_1() {
    striped_random_unsigned_vecs_length_range::<u8>(EXAMPLE_SEED, 3, 10, 1, 0);
}

#[test]
#[should_panic]
fn striped_random_unsigned_vecs_length_range_fail_2() {
    striped_random_unsigned_vecs_length_range::<u8>(EXAMPLE_SEED, 3, 10, 2, 3);
}

#[test]
#[should_panic]
fn striped_random_unsigned_vecs_length_range_fail_3() {
    striped_random_unsigned_vecs_length_range::<u8>(EXAMPLE_SEED, 1, 1, 4, 1);
}
