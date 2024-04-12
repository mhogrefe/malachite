// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::rational_sequences::random::random_rational_sequences;
use malachite_base::rational_sequences::RationalSequence;
use malachite_base::test_util::stats::common_values_map::common_values_map;
use malachite_base::test_util::stats::median;

#[test]
fn test_random_rational_sequences() {
    let xs = random_rational_sequences(EXAMPLE_SEED, &random_primitive_ints::<u8>, 4, 1);
    let values = xs
        .clone()
        .map(|x| RationalSequence::to_string(&x))
        .take(20)
        .collect_vec();
    let values = values.iter().map(String::as_str).collect_vec();
    let common_values = common_values_map(1000000, 10, xs.clone())
        .into_iter()
        .map(|(x, freq)| (x.to_string(), freq))
        .collect_vec();
    let common_values = common_values
        .iter()
        .map(|(x, freq)| (x.as_str(), *freq))
        .collect_vec();
    let (median_lo, median_hi) = median(xs.take(1000000));
    let (median_lo, median_hi) = (
        median_lo.to_string(),
        median_hi.map(|x| RationalSequence::to_string(&x)),
    );
    let actual_median = (median_lo.as_str(), median_hi.as_deref());
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), actual_median),
        (
            &[
                "[[85, 11, 136, 200, 235, 134, 203, 223, 38, 235, 217, 177, 162, 32]]",
                "[166, 234, 30, 218, [90, 106, 9, 216]]",
                "[204]",
                "[151, 213, 97, 253, 78, [91, 39]]",
                "[191, 175, 170, 232]",
                "[233, 2, 35, 22, 217, 198]",
                "[[114, 17, 32, 173, 114, 65, 121, 222, 173, 25, 144]]",
                "[148, 79, 115, 52, 73, 69, 137, 91]",
                "[153, 178, 112]",
                "[34, 95, 106, 167, 197, [130, 168, 122, 207, 172, 177, 86, 150, 221]]",
                "[218, [101]]",
                "[115, 74, 9, 123, 109, 52, 201]",
                "[159, 247, 250, 48, 133, 235, 196, 40, [97]]",
                "[104, 68, 190, [216, 7, 216, 157, 43, 43, 112]]",
                "[]",
                "[217, 24, 11, 103, 211, [84, 135]]",
                "[[55, 29, 206, 89, 65, 191, 51, 9, 79]]",
                "[[148, 34]]",
                "[22, 22, 62, 3, 114, 118, 20, 47, 194, 50, 32, [120, 176, 166, 23]]",
                "[204, 248, 177, 238, 237, 222, 154, 113, [225, 65]]"
            ][..],
            &[
                ("[]", 39885),
                ("[[243]]", 157),
                ("[68]", 154),
                ("[1]", 153),
                ("[120]", 153),
                ("[71]", 152),
                ("[[40]]", 152),
                ("[[158]]", 151),
                ("[[169]]", 151),
                ("[[183]]", 151)
            ][..],
            (
                "[122, 194, 41, 122, [232]]",
                Some("[[122, 194, 89, 228, 124, 219]]")
            )
        )
    );
}
