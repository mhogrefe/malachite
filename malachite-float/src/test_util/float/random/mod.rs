// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::itertools::Itertools;
use crate::{ComparableFloat, ComparableFloatRef, Float};
use malachite_base::num::conversion::traits::RoundingFrom;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::stats::common_values_map::common_values_map;
use malachite_base::test_util::stats::median;
use malachite_base::test_util::stats::moments::{MomentStats, moment_stats};

pub fn random_floats_helper_helper<I: Clone + Iterator<Item = Float>>(
    xs: I,
    expected_values: &[&str],
    expected_values_hex: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_common_values_hex: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_median_hex: (&str, Option<&str>),
    expected_sample_moment_stats: MomentStats,
) {
    let comparable_xs = xs.clone().map(ComparableFloat);
    let raw_actual_values = xs.clone().take(20).collect_vec();
    let actual_values = raw_actual_values
        .iter()
        .map(Float::to_string)
        .take(20)
        .collect_vec();
    let actual_values_hex = raw_actual_values
        .iter()
        .map(|x| format!("{:#x}", ComparableFloatRef(x)))
        .take(20)
        .collect_vec();
    let actual_values = actual_values.iter().map(String::as_str).collect_vec();
    let actual_values_hex = actual_values_hex.iter().map(String::as_str).collect_vec();
    for x in xs.clone().take(1000000) {
        assert!(x.is_valid());
    }
    let raw_common_values = common_values_map(1000000, 10, comparable_xs.clone())
        .into_iter()
        .collect_vec();
    let actual_common_values = raw_common_values
        .iter()
        .map(|(x, freq)| (x.0.to_string(), *freq))
        .collect_vec();
    let actual_common_values = actual_common_values
        .iter()
        .map(|(x, freq)| (x.as_str(), *freq))
        .collect_vec();
    let actual_common_values_hex = raw_common_values
        .iter()
        .map(|(x, freq)| (format!("{x:#x}"), *freq))
        .collect_vec();
    let actual_common_values_hex = actual_common_values_hex
        .iter()
        .map(|(x, freq)| (x.as_str(), *freq))
        .collect_vec();
    let (median_lo, median_hi) = median(comparable_xs.take(1000000));
    let (median_lo_hex, median_hi_hex) = (
        format!("{median_lo:#x}"),
        median_hi.as_ref().map(|x| format!("{x:#x}")),
    );
    let (median_lo, median_hi) = (
        median_lo.0.to_string(),
        median_hi.map(|x| Float::to_string(&x.0)),
    );
    let actual_sample_median = (median_lo.as_str(), median_hi.as_deref());
    let actual_sample_median_hex = (median_lo_hex.as_str(), median_hi_hex.as_deref());
    let actual_sample_moment_stats =
        moment_stats(xs.take(1000000).map(|x| f64::rounding_from(&x, Nearest).0));
    assert_eq!(
        (
            actual_values.as_slice(),
            actual_values_hex.as_slice(),
            actual_common_values.as_slice(),
            actual_common_values_hex.as_slice(),
            actual_sample_median,
            actual_sample_median_hex,
            actual_sample_moment_stats
        ),
        (
            expected_values,
            expected_values_hex,
            expected_common_values,
            expected_common_values_hex,
            expected_sample_median,
            expected_sample_median_hex,
            expected_sample_moment_stats
        )
    );
}

// TODO remove once we have fast to_string
pub fn random_floats_helper_helper_no_common_values<I: Clone + Iterator<Item = Float>>(
    xs: I,
    expected_values: &[&str],
    expected_values_hex: &[&str],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_median_hex: (&str, Option<&str>),
    expected_sample_moment_stats: MomentStats,
) {
    let comparable_xs = xs.clone().map(ComparableFloat);
    let raw_actual_values = xs.clone().take(20).collect_vec();
    let actual_values = raw_actual_values
        .iter()
        .map(Float::to_string)
        .take(20)
        .collect_vec();
    let actual_values_hex = raw_actual_values
        .iter()
        .map(|x| format!("{:#x}", ComparableFloatRef(x)))
        .take(20)
        .collect_vec();
    let actual_values = actual_values.iter().map(String::as_str).collect_vec();
    let actual_values_hex = actual_values_hex.iter().map(String::as_str).collect_vec();
    for x in xs.clone().take(1000000) {
        assert!(x.is_valid());
    }
    let (median_lo, median_hi) = median(comparable_xs.take(1000000));
    let (median_lo_hex, median_hi_hex) = (
        format!("{median_lo:#x}"),
        median_hi.as_ref().map(|x| format!("{x:#x}")),
    );
    let (median_lo, median_hi) = (
        median_lo.0.to_string(),
        median_hi.map(|x| Float::to_string(&x.0)),
    );
    let actual_sample_median = (median_lo.as_str(), median_hi.as_deref());
    let actual_sample_median_hex = (median_lo_hex.as_str(), median_hi_hex.as_deref());
    let actual_sample_moment_stats =
        moment_stats(xs.take(1000000).map(|x| f64::rounding_from(&x, Nearest).0));
    assert_eq!(
        (
            actual_values.as_slice(),
            actual_values_hex.as_slice(),
            actual_sample_median,
            actual_sample_median_hex,
            actual_sample_moment_stats
        ),
        (
            expected_values,
            expected_values_hex,
            expected_sample_median,
            expected_sample_median_hex,
            expected_sample_moment_stats
        )
    );
}

/// Feeds a Malachite u64 stream to MPFR through rug's custom-RandGen interface, low half of each
/// word first. Partial-word requests take the low bits of the next word, matching Malachite's
/// conventions (rug's default takes the high bits), so MPFR driven by this generator consumes bits
/// exactly as Malachite's samplers do, and their outputs can be compared sequence-for-sequence.
pub struct MalachiteRandGen<J: Iterator<Item = u64> + Send + Sync> {
    pub xs: J,
    pub hi: Option<u32>,
}

impl<J: Iterator<Item = u64> + Send + Sync> MalachiteRandGen<J> {
    pub const fn new(xs: J) -> Self {
        Self { xs, hi: None }
    }
}

impl<J: Iterator<Item = u64> + Send + Sync> rug::rand::RandGen for MalachiteRandGen<J> {
    fn r#gen(&mut self) -> u32 {
        if let Some(h) = self.hi.take() {
            h
        } else {
            let x = self.xs.next().unwrap();
            self.hi = Some((x >> 32) as u32);
            x as u32
        }
    }

    fn gen_bits(&mut self, bits: u32) -> u32 {
        match bits {
            0 => 0,
            1..=31 => self.r#gen() & !(!0u32 << bits),
            _ => self.r#gen(),
        }
    }
}
