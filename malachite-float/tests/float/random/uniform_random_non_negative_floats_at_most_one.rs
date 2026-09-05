// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::LowMask;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::stats::moments::MomentStats;
use malachite_float::float::random::{
    uniform_random_non_negative_floats_at_most_one,
    uniform_random_non_negative_floats_at_most_one_from_u64s,
};
use malachite_float::test_util::common::rug_round_try_from_rounding_mode;
use malachite_float::test_util::float::random::{
    MalachiteRandGen, random_floats_helper_helper, random_floats_helper_helper_no_common_values,
};
use malachite_float::{ComparableFloat, Float};

#[test]
fn test_uniform_random_non_negative_floats_at_most_one_vs_rug() {
    for rm in [Floor, Ceiling, Down, Up, Nearest] {
        let rug_rm = rug_round_try_from_rounding_mode(rm).unwrap();
        for prec in [1u64, 2, 10, 31, 32, 33, 63, 64, 65, 100] {
            let mut ours = uniform_random_non_negative_floats_at_most_one(EXAMPLE_SEED, prec, rm);
            let mut bit_source = MalachiteRandGen::new(random_primitive_ints(EXAMPLE_SEED));
            let mut state = rug::rand::RandState::new_custom(&mut bit_source);
            for i in 0..50u32 {
                let x = ours.next().unwrap();
                let (theirs, o) = rug::Float::with_val_round(
                    u32::exact_from(prec),
                    rug::Float::random_cont(&mut state),
                    rug_rm,
                );
                assert_ne!(o, core::cmp::Ordering::Equal);
                assert_eq!(
                    ComparableFloat(Float::from(&theirs)),
                    ComparableFloat(x),
                    "rm {rm} prec {prec} output {i}"
                );
            }
        }
    }
}

// Packs a u32 stream into u64s, low half first, matching how U32BitSource unpacks them.
struct PackU32s<J: Iterator<Item = u32>> {
    xs: J,
}

impl<J: Iterator<Item = u32>> Iterator for PackU32s<J> {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let lo = u64::from(self.xs.next()?);
        let hi = u64::from(self.xs.next().unwrap_or(0));
        Some(lo | (hi << 32))
    }
}

// A u64 stream whose u32 halves are `zero_blocks` zeros, then `tail`, then a little padding. Each
// zero u32 makes the exponent loop of the sampler (and of mpfr_urandom) draw again, so
// `zero_blocks` all-zero 8-bit blocks walk the exponent down by 8 each.
fn rigged(zero_blocks: u64, tail: &[u32]) -> impl Iterator<Item = u64> + Send + Sync {
    PackU32s {
        xs: core::iter::repeat_n(0u32, usize::exact_from(zero_blocks))
            .chain(tail.to_vec())
            .chain(core::iter::repeat_n(0u32, 4)),
    }
}

// Runs one rigged case through both Malachite and MPFR (fed the identical stream) and returns the
// agreed-upon output.
fn run_rigged_case(zero_blocks: u64, tail: &[u32], prec: u64, rm: RoundingMode) -> Float {
    let ours = uniform_random_non_negative_floats_at_most_one_from_u64s(
        rigged(zero_blocks, tail),
        prec,
        rm,
    )
    .next()
    .unwrap();
    let mut bit_source = MalachiteRandGen::new(rigged(zero_blocks, tail));
    let mut state = rug::rand::RandState::new_custom(&mut bit_source);
    let (theirs, o) = rug::Float::with_val_round(
        u32::exact_from(prec),
        rug::Float::random_cont(&mut state),
        rug_round_try_from_rounding_mode(rm).unwrap(),
    );
    assert_ne!(o, core::cmp::Ordering::Equal);
    assert_eq!(
        ComparableFloat(Float::from(&theirs)),
        ComparableFloat(ours.clone()),
        "zero_blocks {zero_blocks} prec {prec} rm {rm}"
    );
    ours
}

fn min_positive(prec: u64) -> Float {
    Float::from_float_prec(
        Float::ONE >> u64::exact_from(1 - i64::from(Float::MIN_EXPONENT)),
        prec,
    )
    .0
}

// A fast rigged case: a modest exponent walk, checked against MPFR in ordinary builds.
#[test]
fn test_uniform_random_non_negative_floats_at_most_one_rigged() {
    // exponent -16 - 3 = -19; significand bits 0b101010101; rounding bit 1
    for rm in [Floor, Ceiling, Down, Up, Nearest] {
        let x = run_rigged_case(2, &[0x10, 0x155, 1], 10, rm);
        assert!(x > 0u32);
        assert_eq!(x.get_prec(), Some(10));
    }
}

// The underflow arm of the sampler, unreachable by sampling: it requires the exponent walk to take
// on the order of 2^27 consecutive all-zero blocks. A rigged stream reaches it directly, and MPFR,
// fed the identical stream, genuinely performs the same 2^27-block walk, so these cases are
// oracle-checked too. Release-scale: each case draws over 10^8 blocks.
#[test]
fn test_uniform_random_non_negative_floats_at_most_one_underflow_extreme() {
    // The exponent walk subtracts 8 per all-zero block and cnt for the final block, where the final
    // block with value 1 << (7 - cnt) has cnt leading zeros within its 8 bits. So zero_blocks =
    // (-target - cnt) / 8 reaches exactly `target`, when the division is exact.
    let k_for = |target: i64, cnt: u64| {
        let total = u64::exact_from(-target) - cnt;
        assert_eq!(total & 7, 0);
        total >> 3
    };
    let block_for = |cnt: u64| 1u32 << (7 - cnt);
    let min = i64::from(Float::MIN_EXPONENT);
    let all_ones_sig = u32::low_mask(9);
    let zero = Float::ZERO;

    // -MIN_EXPONENT is congruent to 7 mod 8, so exponent MIN_EXPONENT needs cnt 7, and the
    // exponents below it need cnt 0, 1, and so on.

    // - exponent exactly MIN_EXPONENT: in range, no underflow
    let (k, b) = (k_for(min, 7), block_for(7));
    let x = run_rigged_case(k, &[b, 0, 0], 10, Floor);
    assert_eq!(ComparableFloat(x), ComparableFloat(min_positive(10)));

    // - exponent MIN_EXPONENT - 1: Floor and Down give zero
    let (k, b) = (k_for(min - 1, 0), block_for(0));
    let x = run_rigged_case(k, &[b, 0, 0], 10, Floor);
    assert_eq!(ComparableFloat(x), ComparableFloat(zero.clone()));
    let x = run_rigged_case(k, &[b, 0, 0], 10, Down);
    assert_eq!(ComparableFloat(x), ComparableFloat(zero.clone()));

    // - exponent MIN_EXPONENT - 1: Ceiling and Up give the minimum positive Float
    let x = run_rigged_case(k, &[b, 0, 0], 10, Ceiling);
    assert_eq!(ComparableFloat(x), ComparableFloat(min_positive(10)));
    let x = run_rigged_case(k, &[b, 0, 0], 10, Up);
    assert_eq!(ComparableFloat(x), ComparableFloat(min_positive(10)));

    // - exponent MIN_EXPONENT - 1, Nearest, significand a power of 2 (the value is exactly half the
    //   minimum positive Float), rounding bit 0: the exact value is above the half, so the result
    //   is the minimum positive Float
    let x = run_rigged_case(k, &[b, 0, 0], 10, Nearest);
    assert_eq!(ComparableFloat(x), ComparableFloat(min_positive(10)));
    // - same with rounding bit 1: rounding up leaves a non-power-of-2 significand, still the
    //   minimum positive Float
    let x = run_rigged_case(k, &[b, 0, 1], 10, Nearest);
    assert_eq!(ComparableFloat(x), ComparableFloat(min_positive(10)));

    // - exponent MIN_EXPONENT - 2 with an all-ones significand and rounding up: the carry lifts the
    //   exponent to MIN_EXPONENT - 1 and leaves a power-of-2 significand with the exact value below
    //   it, so Nearest rounds to zero
    let (k, b) = (k_for(min - 2, 1), block_for(1));
    let x = run_rigged_case(k, &[b, all_ones_sig, 1], 10, Nearest);
    assert_eq!(ComparableFloat(x), ComparableFloat(zero.clone()));
    // - the same carry under Ceiling gives the minimum positive Float
    let x = run_rigged_case(k, &[b, all_ones_sig, 0], 10, Ceiling);
    assert_eq!(ComparableFloat(x), ComparableFloat(min_positive(10)));

    // - deep below the range: Nearest gives zero, Ceiling the minimum positive Float
    let (k, b) = (k_for(min - 1, 0) + 2, block_for(0));
    let x = run_rigged_case(k, &[b, 0, 1], 10, Nearest);
    assert_eq!(ComparableFloat(x), ComparableFloat(zero.clone()));
    let x = run_rigged_case(k, &[b, 0, 0], 10, Ceiling);
    assert_eq!(ComparableFloat(x), ComparableFloat(min_positive(10)));

    // - precision 1 (no significand bits are drawn): exponent MIN_EXPONENT - 1, Nearest, rounding
    //   bit 0: the value is exactly half the minimum positive Float, and the result is the minimum
    //   positive Float at precision 1
    let (k, b) = (k_for(min - 1, 0), block_for(0));
    let x = run_rigged_case(k, &[b, 0], 1, Nearest);
    assert_eq!(ComparableFloat(x), ComparableFloat(min_positive(1)));
}

fn uniform_random_non_negative_floats_at_most_one_helper(
    prec: u64,
    rm: RoundingMode,
    expected_values: &[&str],
    expected_values_hex: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_common_values_hex: &[(&str, usize)],
    expected_median: (&str, Option<&str>),
    expected_median_hex: (&str, Option<&str>),
    expected_moment_stats: MomentStats,
) {
    random_floats_helper_helper(
        uniform_random_non_negative_floats_at_most_one(EXAMPLE_SEED, prec, rm),
        expected_values,
        expected_values_hex,
        expected_common_values,
        expected_common_values_hex,
        expected_median,
        expected_median_hex,
        expected_moment_stats,
    );
}

fn uniform_random_non_negative_floats_at_most_one_helper_no_common_values(
    prec: u64,
    rm: RoundingMode,
    expected_values: &[&str],
    expected_values_hex: &[&str],
    expected_median: (&str, Option<&str>),
    expected_median_hex: (&str, Option<&str>),
    expected_moment_stats: MomentStats,
) {
    random_floats_helper_helper_no_common_values(
        uniform_random_non_negative_floats_at_most_one(EXAMPLE_SEED, prec, rm),
        expected_values,
        expected_values_hex,
        expected_median,
        expected_median_hex,
        expected_moment_stats,
    );
}

#[test]
fn test_uniform_random_non_negative_floats_at_most_one() {
    // precision 1, rounding mode Floor
    let values = &[
        "0.25", "0.25", "0.25", "0.25", "0.016", "0.50", "0.25", "0.25", "0.50", "0.50", "0.25",
        "0.50", "0.50", "0.25", "0.062", "0.50", "0.50", "0.50", "0.25", "0.50",
    ];
    let values_hex = &[
        "0x0.4#1", "0x0.4#1", "0x0.4#1", "0x0.4#1", "0x0.04#1", "0x0.8#1", "0x0.4#1", "0x0.4#1",
        "0x0.8#1", "0x0.8#1", "0x0.4#1", "0x0.8#1", "0x0.8#1", "0x0.4#1", "0x0.1#1", "0x0.8#1",
        "0x0.8#1", "0x0.8#1", "0x0.4#1", "0x0.8#1",
    ];
    let common_values = &[
        ("0.50", 500312),
        ("0.25", 249854),
        ("0.12", 124809),
        ("0.062", 62189),
        ("0.031", 31393),
        ("0.016", 15733),
        ("0.0078", 7780),
        ("0.0039", 3970),
        ("0.0020", 1977),
        ("0.00098", 1013),
    ];
    let common_values_hex = &[
        ("0x0.8#1", 500312),
        ("0x0.4#1", 249854),
        ("0x0.2#1", 124809),
        ("0x0.1#1", 62189),
        ("0x0.08#1", 31393),
        ("0x0.04#1", 15733),
        ("0x0.02#1", 7780),
        ("0x0.01#1", 3970),
        ("0x0.008#1", 1977),
        ("0x0.004#1", 1013),
    ];
    let sample_median = ("0.50", None);
    let sample_median_hex = ("0x0.8#1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.3334157534462278),
        standard_deviation: NiceFloat(0.17820199459888503),
        skewness: NiceFloat(-0.37557167879891246),
        excess_kurtosis: NiceFloat(-1.4406955262393404),
    };
    uniform_random_non_negative_floats_at_most_one_helper(
        1,
        Floor,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // precision 1, rounding mode Ceiling
    let values = &[
        "0.50", "0.50", "0.50", "0.50", "0.031", "1.0", "0.50", "0.50", "1.0", "1.0", "0.50",
        "1.0", "1.0", "0.50", "0.12", "1.0", "1.0", "1.0", "0.50", "1.0",
    ];
    let values_hex = &[
        "0x0.8#1", "0x0.8#1", "0x0.8#1", "0x0.8#1", "0x0.08#1", "0x1.0#1", "0x0.8#1", "0x0.8#1",
        "0x1.0#1", "0x1.0#1", "0x0.8#1", "0x1.0#1", "0x1.0#1", "0x0.8#1", "0x0.2#1", "0x1.0#1",
        "0x1.0#1", "0x1.0#1", "0x0.8#1", "0x1.0#1",
    ];
    let common_values = &[
        ("1.0", 500312),
        ("0.50", 249854),
        ("0.25", 124809),
        ("0.12", 62189),
        ("0.062", 31393),
        ("0.031", 15733),
        ("0.016", 7780),
        ("0.0078", 3970),
        ("0.0039", 1977),
        ("0.0020", 1013),
    ];
    let common_values_hex = &[
        ("0x1.0#1", 500312),
        ("0x0.8#1", 249854),
        ("0x0.4#1", 124809),
        ("0x0.2#1", 62189),
        ("0x0.1#1", 31393),
        ("0x0.08#1", 15733),
        ("0x0.04#1", 7780),
        ("0x0.02#1", 3970),
        ("0x0.01#1", 1977),
        ("0x0.008#1", 1013),
    ];
    let sample_median = ("1.0", None);
    let sample_median_hex = ("0x1.0#1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.6668315068924556),
        standard_deviation: NiceFloat(0.35640398919777005),
        skewness: NiceFloat(-0.37557167879891246),
        excess_kurtosis: NiceFloat(-1.4406955262393404),
    };
    uniform_random_non_negative_floats_at_most_one_helper(
        1,
        Ceiling,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // precision 1, rounding mode Nearest
    let values = &[
        "0.25", "0.25", "0.50", "0.50", "0.016", "0.50", "0.50", "0.25", "1.0", "0.50", "0.50",
        "1.0", "0.50", "0.25", "0.062", "1.0", "0.50", "0.50", "0.25", "1.0",
    ];
    let values_hex = &[
        "0x0.4#1", "0x0.4#1", "0x0.8#1", "0x0.8#1", "0x0.04#1", "0x0.8#1", "0x0.8#1", "0x0.4#1",
        "0x1.0#1", "0x0.8#1", "0x0.8#1", "0x1.0#1", "0x0.8#1", "0x0.4#1", "0x0.1#1", "0x1.0#1",
        "0x0.8#1", "0x0.8#1", "0x0.4#1", "0x1.0#1",
    ];
    let common_values = &[
        ("0.50", 375544),
        ("1.0", 250056),
        ("0.25", 186731),
        ("0.12", 93804),
        ("0.062", 46845),
        ("0.031", 23391),
        ("0.016", 11852),
        ("0.0078", 5814),
        ("0.0039", 2955),
        ("0.0020", 1528),
    ];
    let common_values_hex = &[
        ("0x0.8#1", 375544),
        ("0x1.0#1", 250056),
        ("0x0.4#1", 186731),
        ("0x0.2#1", 93804),
        ("0x0.1#1", 46845),
        ("0x0.08#1", 23391),
        ("0x0.04#1", 11852),
        ("0x0.02#1", 5814),
        ("0x0.01#1", 2955),
        ("0x0.008#1", 1528),
    ];
    let sample_median = ("0.50", None);
    let sample_median_hex = ("0x0.8#1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.5001411449715016),
        standard_deviation: NiceFloat(0.3273323211433457),
        skewness: NiceFloat(0.4060541624304694),
        excess_kurtosis: NiceFloat(-1.048051654504901),
    };
    uniform_random_non_negative_floats_at_most_one_helper(
        1,
        Nearest,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // precision 2, rounding mode Nearest
    let values = &[
        "0.38", "1.0", "0.50", "0.38", "0.38", "0.75", "0.50", "1.0", "0.75", "0.50", "1.0",
        "0.047", "0.38", "0.50", "0.50", "0.50", "0.12", "0.094", "0.19", "0.38",
    ];
    let values_hex = &[
        "0x0.6#2", "0x1.0#2", "0x0.8#2", "0x0.6#2", "0x0.6#2", "0x0.c#2", "0x0.8#2", "0x1.0#2",
        "0x0.c#2", "0x0.8#2", "0x1.0#2", "0x0.0c#2", "0x0.6#2", "0x0.8#2", "0x0.8#2", "0x0.8#2",
        "0x0.2#2", "0x0.18#2", "0x0.3#2", "0x0.6#2",
    ];
    let common_values = &[
        ("0.75", 250025),
        ("0.50", 186991),
        ("0.38", 125037),
        ("1.0", 124872),
        ("0.25", 94239),
        ("0.19", 62412),
        ("0.12", 46956),
        ("0.094", 31200),
        ("0.062", 23543),
        ("0.047", 15576),
    ];
    let common_values_hex = &[
        ("0x0.c#2", 250025),
        ("0x0.8#2", 186991),
        ("0x0.6#2", 125037),
        ("0x1.0#2", 124872),
        ("0x0.4#2", 94239),
        ("0x0.3#2", 62412),
        ("0x0.2#2", 46956),
        ("0x0.18#2", 31200),
        ("0x0.10#2", 23543),
        ("0x0.0c#2", 15576),
    ];
    let sample_median = ("0.50", None);
    let sample_median_hex = ("0x0.8#2", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.49976695299338053),
        standard_deviation: NiceFloat(0.2988501687186379),
        skewness: NiceFloat(0.13481159763004025),
        excess_kurtosis: NiceFloat(-1.0966791186591278),
    };
    uniform_random_non_negative_floats_at_most_one_helper(
        2,
        Nearest,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // precision 10, rounding mode Floor
    let values = &[
        "0.36133", "0.59082", "0.44873", "0.48877", "0.26904", "0.73633", "0.69531", "0.65137",
        "0.84961", "0.52148", "0.85449", "0.039124", "0.30078", "0.38916", "0.94336", "0.48486",
        "0.21631", "0.078857", "0.12842", "0.36182",
    ];
    let values_hex = &[
        "0x0.5c8#10",
        "0x0.974#10",
        "0x0.72e#10",
        "0x0.7d2#10",
        "0x0.44e#10",
        "0x0.bc8#10",
        "0x0.b20#10",
        "0x0.a6c#10",
        "0x0.d98#10",
        "0x0.858#10",
        "0x0.dac#10",
        "0x0.0a04#10",
        "0x0.4d0#10",
        "0x0.63a#10",
        "0x0.f18#10",
        "0x0.7c2#10",
        "0x0.376#10",
        "0x0.1430#10",
        "0x0.20e#10",
        "0x0.5ca#10",
    ];
    let common_values = &[
        ("0.54492", 1067),
        ("0.54590", 1065),
        ("0.69824", 1064),
        ("0.66602", 1055),
        ("0.65723", 1052),
        ("0.79590", 1051),
        ("0.77539", 1046),
        ("0.63867", 1044),
        ("0.75195", 1040),
        ("0.53027", 1039),
    ];
    let common_values_hex = &[
        ("0x0.8b8#10", 1067),
        ("0x0.8bc#10", 1065),
        ("0x0.b2c#10", 1064),
        ("0x0.aa8#10", 1055),
        ("0x0.a84#10", 1052),
        ("0x0.cbc#10", 1051),
        ("0x0.c68#10", 1046),
        ("0x0.a38#10", 1044),
        ("0x0.c08#10", 1040),
        ("0x0.87c#10", 1039),
    ];
    let sample_median = ("0.49951", None);
    let sample_median_hex = ("0x0.7fe#10", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.49947757533091786),
        standard_deviation: NiceFloat(0.2884608297230272),
        skewness: NiceFloat(0.00024750287406313765),
        excess_kurtosis: NiceFloat(-1.1989770461669647),
    };
    uniform_random_non_negative_floats_at_most_one_helper(
        10,
        Floor,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // precision 10, rounding mode Nearest
    let values = &[
        "0.36182", "0.59180", "0.44922", "0.48877", "0.26904", "0.73730", "0.69531", "0.65234",
        "0.85059", "0.52148", "0.85547", "0.039124", "0.30127", "0.38965", "0.94336", "0.48535",
        "0.21631", "0.078979", "0.12866", "0.36182",
    ];
    let values_hex = &[
        "0x0.5ca#10",
        "0x0.978#10",
        "0x0.730#10",
        "0x0.7d2#10",
        "0x0.44e#10",
        "0x0.bcc#10",
        "0x0.b20#10",
        "0x0.a70#10",
        "0x0.d9c#10",
        "0x0.858#10",
        "0x0.db0#10",
        "0x0.0a04#10",
        "0x0.4d2#10",
        "0x0.63c#10",
        "0x0.f18#10",
        "0x0.7c4#10",
        "0x0.376#10",
        "0x0.1438#10",
        "0x0.20f#10",
        "0x0.5ca#10",
    ];
    let common_values = &[
        ("0.82520", 1065),
        ("0.69824", 1063),
        ("0.83398", 1058),
        ("0.64453", 1050),
        ("0.98535", 1050),
        ("0.77637", 1048),
        ("0.95605", 1048),
        ("0.98926", 1048),
        ("0.54688", 1043),
        ("0.54590", 1041),
    ];
    let common_values_hex = &[
        ("0x0.d34#10", 1065),
        ("0x0.b2c#10", 1063),
        ("0x0.d58#10", 1058),
        ("0x0.a50#10", 1050),
        ("0x0.fc4#10", 1050),
        ("0x0.c6c#10", 1048),
        ("0x0.f4c#10", 1048),
        ("0x0.fd4#10", 1048),
        ("0x0.8c0#10", 1043),
        ("0x0.8bc#10", 1041),
    ];
    let sample_median = ("0.49951", None);
    let sample_median_hex = ("0x0.7fe#10", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.4998027772154481),
        standard_deviation: NiceFloat(0.2886222944463385),
        skewness: NiceFloat(-0.00012945721291895187),
        excess_kurtosis: NiceFloat(-1.199228552710833),
    };
    uniform_random_non_negative_floats_at_most_one_helper(
        10,
        Nearest,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // precision 64, rounding mode Nearest
    let values = &[
        "0.412949572274401119983",
        "0.390865876081920733274",
        "0.0301735026987570150939",
        "0.492951554330205752810",
        "0.530278558782121627513",
        "0.341141480456087607925",
        "0.720063052404371703430",
        "0.105843818462323999397",
        "0.971349388726597110389",
        "0.421053186915903768299",
        "0.0859018012811982129031",
        "0.816121094294791993436",
        "0.249490339711245559060",
        "0.660501373758508483422",
        "0.348148134232373885320",
        "0.552444908019478117132",
        "0.0229665196379937614155",
        "0.446329364744981323837",
        "0.0627203636173458319364",
        "0.433760077671795959514",
    ];
    let values_hex = &[
        "0x0.69b7102bd0d469720#64",
        "0x0.640fc93ae4eccadf0#64",
        "0x0.07b9735f4c038bfa40#64",
        "0x0.7e3212b45c5057138#64",
        "0x0.87c055ebbd3de9ef#64",
        "0x0.57550c4dde2ff7eb0#64",
        "0x0.b8560d5d227c6566#64",
        "0x0.1b18949ac7898c9e2#64",
        "0x0.f8aa5a819200dd08#64",
        "0x0.6bca2443ae2d80348#64",
        "0x0.15fda9132b94aaae2#64",
        "0x0.d0ed4fe19265aa69#64",
        "0x0.3fde9951ba4c7f5dc#64",
        "0x0.a9169e374180782d#64",
        "0x0.59203c72b103c73d0#64",
        "0x0.8d6d078cc90f195f#64",
        "0x0.05e12242bf851e1f20#64",
        "0x0.7242a428d2fb815a0#64",
        "0x0.100e7116879c4b40a#64",
        "0x0.6f0ae683e9291af78#64",
    ];
    let sample_median = ("0.499702544768665295308", Some("0.499702750351059438638"));
    let sample_median_hex = ("0x0.7fec8187826799e88#64", Some("0x0.7fec84fa7aa3346f0#64"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.4999150061812989),
        standard_deviation: NiceFloat(0.2885977400207046),
        skewness: NiceFloat(0.0011301433810804963),
        excess_kurtosis: NiceFloat(-1.1994209037536399),
    };
    uniform_random_non_negative_floats_at_most_one_helper_no_common_values(
        64,
        Nearest,
        values,
        values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // precision 100, rounding mode Ceiling
    let values = &[
        "0.43534523758226641180625885492205",
        "0.29692911968278158913418164826977",
        "0.32097410204722766581588374906040",
        "0.73348993001382978313804429687688",
        "0.91929067601770021505078143241531",
        "0.59814535465636542697098889263550",
        "0.42404763301122952815663162281706",
        "0.59824148718069315784706428630351",
        "0.21989275804479970741829668344958",
        "0.20910824693627099485593043801684",
        "0.98113793782325876102578299639764",
        "0.30150938538201232992490668579333",
        "0.064915036837195226160363320842490",
        "0.030186932080687425720688416209603",
        "0.82297102859028944468523290092277",
        "0.94457626463209124114110773331245",
        "0.68867032415454599473163456817325",
        "0.82104781872690386473651116718595",
        "0.76804693419916669152294207574676",
        "0.50602447200585600742480673350068",
    ];
    let values_hex = &[
        "0x0.6f72c915e29b7102bd0d469728#100",
        "0x0.4c038bfa44828930390d754cc0#100",
        "0x0.522b5bd727e3212b45c5057140#100",
        "0x0.bbc5fefd5acf70054b2b95ac9#100",
        "0x0.eb56a23d038560d5d227c6567#100",
        "0x0.99200dd080df882eb2d24436c#100",
        "0x0.6c8e62bbbabca2443ae2d80348#100",
        "0x0.99265aa68d2dd10b88c7461c7#100",
        "0x0.384ae44c6dfde9951ba4c7f5dc#100",
        "0x0.35881e39ea48fb6d9cedfd483c#100",
        "0x0.fb2bdb1bd0d6d078cc90f195f#100",
        "0x0.4d2fb815a71e200b7dfea0ce98#100",
        "0x0.109e45983c00e7116879c4b40c#100",
        "0x0.07ba54ae0d99a5ca9e6135a7290#100",
        "0x0.d2ae3ab559cc55a65bf4c1c0c#100",
        "0x0.f1cfc0052c3452231eb0e71aa#100",
        "0x0.b04cb2c7f830c085b77e503e9#100",
        "0x0.d2303099e25a2caf0f5ae6800#100",
        "0x0.c49eb9502db12f8fb6f797d67#100",
        "0x0.818ad1de3da651981a304cbd5#100",
    ];
    let sample_median = (
        "0.49922569932085363772528535675291",
        Some("0.49922585148629366359822004952317"),
    );
    let sample_median_hex = (
        "0x0.7fcd4163e7e23b88b613bc9808#100",
        Some("0x0.7fcd43f1738deb856395526df0#100"),
    );
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.49945259605987297),
        standard_deviation: NiceFloat(0.28887018499595507),
        skewness: NiceFloat(0.0019757110416099063),
        excess_kurtosis: NiceFloat(-1.2011893865256904),
    };
    uniform_random_non_negative_floats_at_most_one_helper_no_common_values(
        100,
        Ceiling,
        values,
        values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );
}

#[test]
fn uniform_random_non_negative_floats_at_most_one_properties() {
    for rm in [Floor, Ceiling, Down, Up, Nearest] {
        for prec in [1, 3, 10, 63, 64, 65, 100] {
            for x in
                uniform_random_non_negative_floats_at_most_one(EXAMPLE_SEED, prec, rm).take(2000)
            {
                assert!(x.is_valid());
                // - the value is in (0, 1]; zero can only arise from the underflow branch, which is
                //   unreachable by sampling
                assert!(x > 0u32);
                assert!(x <= 1u32);
                // - every output has precision `prec`
                assert_eq!(x.get_prec(), Some(prec));
                // - under the downward modes the value is strictly less than one
                if rm == Floor || rm == Down {
                    assert!(x < 1u32);
                }
            }
        }
    }
}

#[test]
#[should_panic]
fn uniform_random_non_negative_floats_at_most_one_fail_1() {
    uniform_random_non_negative_floats_at_most_one(EXAMPLE_SEED, 0, Nearest);
}

#[test]
#[should_panic]
fn uniform_random_non_negative_floats_at_most_one_fail_2() {
    uniform_random_non_negative_floats_at_most_one(EXAMPLE_SEED, 10, Exact);
}
