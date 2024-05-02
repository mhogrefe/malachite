// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::random::striped::striped_random_fixed_length_unsigned_vecs;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::strings::ToBinaryString;
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;

fn striped_random_fixed_length_unsigned_vecs_helper<T: PrimitiveUnsigned>(
    len: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    expected_values: &[&[&str]],
    expected_common_values: &[(&[&str], usize)],
    expected_median: (&[&str], Option<&[&str]>),
) {
    let xss = striped_random_fixed_length_unsigned_vecs::<T>(
        EXAMPLE_SEED,
        len,
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
fn test_striped_random_fixed_length_unsigned_vecs() {
    striped_random_fixed_length_unsigned_vecs_helper::<u8>(
        0,
        10,
        1,
        &[&[][..]; 20],
        &[(&[], 1000000)],
        (&[], None),
    );
    striped_random_fixed_length_unsigned_vecs_helper::<u8>(
        1,
        10,
        1,
        &[
            &["0"],
            &["0"],
            &["11100000"],
            &["11111110"],
            &["11100000"],
            &["111111"],
            &["11100000"],
            &["11111111"],
            &["11111100"],
            &["11111111"],
            &["11111111"],
            &["0"],
            &["11110000"],
            &["0"],
            &["11111111"],
            &["11110000"],
            &["111"],
            &["0"],
            &["100"],
            &["11111111"],
        ],
        &[
            (&["0"], 238932),
            (&["11111111"], 238695),
            (&["11100000"], 26922),
            (&["1111111"], 26791),
            (&["11000000"], 26775),
            (&["11"], 26754),
            (&["111"], 26726),
            (&["11110000"], 26662),
            (&["11111"], 26607),
            (&["1111"], 26594),
        ],
        (&["1111111"], None),
    );
    striped_random_fixed_length_unsigned_vecs_helper::<u8>(
        2,
        10,
        1,
        &[
            &["0", "0"],
            &["1110000", "0"],
            &["11111000", "111"],
            &["11111100", "11111"],
            &["0", "0"],
            &["11111111", "11111"],
            &["0", "0"],
            &["1111", "11111100"],
            &["0", "1"],
            &["1111111", "0"],
            &["11111111", "11111"],
            &["11111100", "1"],
            &["0", "0"],
            &["110000", "11111111"],
            &["11111", "0"],
            &["0", "0"],
            &["11111111", "11111111"],
            &["0", "0"],
            &["1000", "0"],
            &["11111111", "11111111"],
        ],
        &[
            (&["0", "0"], 103184),
            (&["11111111", "11111111"], 102963),
            (&["0", "11111111"], 11776),
            (&["11111000", "11111111"], 11585),
            (&["0", "11111110"], 11579),
            (&["11111110", "11111111"], 11542),
            (&["11000000", "11111111"], 11499),
            (&["1111", "0"], 11487),
            (&["11111111", "1111111"], 11483),
            (&["11111111", "1"], 11459),
        ],
        (&["1111111", "11111111"], None),
    );
    striped_random_fixed_length_unsigned_vecs_helper::<u8>(
        5,
        10,
        1,
        &[
            &["0", "0", "111000", "0", "11111110"],
            &["11111100", "0", "11111000", "11111111", "11111111"],
            &["0", "11111100", "11111111", "1111111", "11100000"],
            &["0", "1000", "0", "11111110", "11111111"],
            &["10000000", "111", "11111100", "11111111", "11111111"],
            &["11001111", "0", "11110000", "11111111", "11111111"],
            &["0", "0", "0", "0", "10000000"],
            &["1", "0", "0", "11100000", "11111111"],
            &["0", "0", "0", "0", "11111100"],
            &["1110111", "0", "11110000", "1110", "11010000"],
            &["1111101", "11111111", "11111111", "11111111", "111111"],
            &["0", "1111", "100000", "11111111", "11"],
            &["10000000", "11111111", "11101111", "11111111", "11111111"],
            &["11111100", "111111", "0", "0", "1111"],
            &["11111111", "1", "11111111", "11111111", "11111"],
            &["0", "11100000", "11111111", "1", "1100"],
            &["11111111", "1110111", "0", "11111100", "1"],
            &["11111110", "111", "0", "0", "0"],
            &["11110000", "11", "10000000", "11111111", "11000011"],
            &["1", "10000000", "11111111", "11111111", "111"],
        ],
        &[
            (&["0", "0", "0", "0", "0"], 8118),
            (
                &["11111111", "11111111", "11111111", "11111111", "11111111"],
                8057,
            ),
            (&["0", "11000000", "11111111", "11111111", "11111111"], 972),
            (&["11111111", "11111111", "11111111", "111", "0"], 961),
            (&["11111111", "11111111", "11111111", "11111", "0"], 955),
            (
                &["11111100", "11111111", "11111111", "11111111", "11111111"],
                948,
            ),
            (&["0", "0", "0", "0", "11111110"], 947),
            (&["0", "0", "10000000", "11111111", "11111111"], 946),
            (&["11111111", "1111", "0", "0", "0"], 944),
            (&["0", "0", "0", "11111111", "11111111"], 944),
        ],
        (
            &["1111111", "11111111", "11111111", "10000001", "11111100"],
            Some(&["1111111", "11111111", "11111111", "10000001", "11111111"]),
        ),
    );
}

#[test]
#[should_panic]
fn striped_random_fixed_length_unsigned_vecs_fail_1() {
    striped_random_fixed_length_unsigned_vecs::<u8>(EXAMPLE_SEED, 5, 1, 0);
}

#[test]
#[should_panic]
fn striped_random_fixed_length_unsigned_vecs_fail_2() {
    striped_random_fixed_length_unsigned_vecs::<u8>(EXAMPLE_SEED, 5, 2, 3);
}
