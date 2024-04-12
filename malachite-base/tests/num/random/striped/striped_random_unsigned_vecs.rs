// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::random::striped::striped_random_unsigned_vecs;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::strings::ToBinaryString;
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;

fn striped_random_unsigned_vecs_helper<T: PrimitiveUnsigned>(
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&[&str]],
    expected_common_values: &[(&[&str], usize)],
    expected_median: (&[&str], Option<&[&str]>),
) {
    let xss = striped_random_unsigned_vecs::<T>(
        EXAMPLE_SEED,
        mean_stripe_numerator,
        mean_stripe_denominator,
        mean_length_numerator,
        mean_length_denominator,
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
fn test_striped_random_unsigned_vecs() {
    striped_random_unsigned_vecs_helper::<u8>(
        2,
        1,
        4,
        1,
        &[
            &[],
            &[
                "11001100", "11000", "11001", "10111101", "10111110", "110000", "11010011", "11",
                "1110110", "11100011", "1", "1100101", "10111110", "11010111",
            ],
            &["101110", "1111000", "10110010", "11101110"],
            &["10110110", "11000010", "11111010", "1100110"],
            &["1000"],
            &[],
            &["10100000", "100101", "1000010", "1100110", "11000111"],
            &["111", "11100001"],
            &["1010110", "10101110", "10111000", "10111101"],
            &[],
            &["101011", "10", "1110101", "1110001", "11101111", "10001001"],
            &[],
            &[],
            &[
                "10000100", "11110101", "11011100", "10011111", "10001000", "11001111", "1111000",
                "11010111", "1101001", "111110", "1100100",
            ],
            &[
                "10101", "11011001", "10100000", "10100001", "1101100", "1101111", "10100011",
                "11110101",
            ],
            &[],
            &["10111111", "100111", "1111110"],
            &[],
            &["11000", "11110010", "11111", "1110011", "11110011"],
            &[
                "10001110", "10011", "1100101", "111100", "10110111", "1101110", "100001",
                "10000000", "10101100",
            ],
        ],
        &[
            (&[], 199913),
            (&["11010010"], 689),
            (&["11"], 688),
            (&["11110111"], 681),
            (&["10000110"], 673),
            (&["110010"], 672),
            (&["11101111"], 671),
            (&["1000100"], 670),
            (&["1111101"], 670),
            (&["11110010"], 670),
        ],
        (&["1100000"], None),
    );
    striped_random_unsigned_vecs_helper::<u8>(
        10,
        1,
        4,
        1,
        &[
            &[],
            &[
                "0", "0", "111000", "0", "11111110", "10000001", "11111111", "11", "0", "0", "0",
                "11111111", "11111111", "11111",
            ],
            &["11110000", "11111111", "11111101", "1111111"],
            &["0", "0", "11100000", "1"],
            &["11111110"],
            &[],
            &["0", "0", "10011000", "11111111", "111"],
            &["11111111", "11111111"],
            &["0", "0", "0", "0"],
            &[],
            &["11111111", "11111110", "11111111", "11111111", "11111", "0"],
            &[],
            &[],
            &[
                "0", "0", "0", "0", "11111100", "111011", "0", "1111000", "111", "1101000",
                "11011111",
            ],
            &[
                "11111111", "11111111", "11111111", "11111", "11000000", "11", "11001000",
                "11111111",
            ],
            &[],
            &["1", "11100000", "11111111"],
            &[],
            &["1000", "0", "0", "11111111", "1111"],
            &["0", "10000000", "111", "10000000", "111111", "0", "0", "11111000", "11111111"],
        ],
        &[
            (&[], 199913),
            (&["0"], 38129),
            (&["11111111"], 38051),
            (&["0", "0"], 13204),
            (&["11111111", "11111111"], 13153),
            (&["0", "0", "0"], 4662),
            (&["11111111", "11111111", "11111111"], 4549),
            (&["1"], 4369),
            (&["11111100"], 4338),
            (&["11111"], 4311),
        ],
        (&["11100", "11111110", "111111", "0"], None),
    );
    striped_random_unsigned_vecs_helper::<u8>(
        11,
        10,
        4,
        1,
        &[
            &[],
            &[
                "1011010", "11010101", "1001010", "10110101", "11010110", "10101010", "10101010",
                "1101010", "10100101", "10101010", "10011010", "1010010", "1010101", "1010101",
            ],
            &["10101010", "1010110", "101011", "1010101"],
            &["1010100", "1010101", "1010101", "10101010"],
            &["1101010"],
            &[],
            &["1001010", "1010101", "1010101", "1010101", "1001001"],
            &["10101011", "10101010"],
            &["10101010", "10101101", "10101010", "1011010"],
            &[],
            &["10101011", "10101010", "10101010", "11010", "11010", "1010111"],
            &[],
            &[],
            &[
                "10101010", "1001011", "11010101", "1010010", "1010101", "10101010", "101010",
                "1010101", "10101001", "1101010", "1010101",
            ],
            &[
                "1010101", "1010101", "1010101", "10110101", "10100100", "10110100", "10101010",
                "10101010",
            ],
            &[],
            &["1010101", "10100101", "10101010"],
            &[],
            &["10101010", "1010100", "1101010", "10100101", "1001010"],
            &[
                "10101100", "10101010", "10101010", "10010101", "10101010", "10101101", "10101010",
                "1001010", "1010101",
            ],
        ],
        &[
            (&[], 199913),
            (&["1010101"], 41088),
            (&["10101010"], 40900),
            (&["1010101", "1010101"], 15274),
            (&["10101010", "10101010"], 15212),
            (&["10101010", "10101010", "10101010"], 5901),
            (&["1010101", "1010101", "1010101"], 5641),
            (&["10101001"], 4206),
            (&["10100101"], 4201),
            (&["10101101"], 4181),
        ],
        (&["1010101", "10110101"], None),
    );
}

#[test]
#[should_panic]
fn striped_random_unsigned_vecs_fail_1() {
    striped_random_unsigned_vecs::<u8>(EXAMPLE_SEED, 1, 0, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_unsigned_vecs_fail_2() {
    striped_random_unsigned_vecs::<u8>(EXAMPLE_SEED, 2, 3, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_unsigned_vecs_fail_3() {
    striped_random_unsigned_vecs::<u8>(EXAMPLE_SEED, 4, 1, 0, 1);
}

#[test]
#[should_panic]
fn striped_random_unsigned_vecs_fail_4() {
    striped_random_unsigned_vecs::<u8>(EXAMPLE_SEED, 4, 1, 1, 0);
}

#[test]
#[should_panic]
fn striped_random_unsigned_vecs_fail_5() {
    striped_random_unsigned_vecs::<u8>(EXAMPLE_SEED, 4, 1, u64::MAX, u64::MAX - 1);
}
