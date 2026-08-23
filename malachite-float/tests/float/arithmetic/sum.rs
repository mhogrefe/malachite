// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::iter::Sum;
use malachite_base::apply_fn_to_primitive_floats;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{Infinity, NegativeInfinity, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_float::float::arithmetic::sum::primitive_float_sum;
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::float::arithmetic::sum::{
    naive_sum, naive_sum_prec, naive_sum_prec_round, naive_sum_round, rug_sum, rug_sum_prec,
    rug_sum_prec_round, rug_sum_round,
};
use malachite_float::test_util::generators::{
    float_vec_gen, float_vec_gen_var_1, float_vec_rounding_mode_pair_gen_var_1,
    float_vec_rounding_mode_pair_gen_var_2, float_vec_unsigned_pair_gen_var_1,
    float_vec_unsigned_rounding_mode_triple_gen_var_1,
    float_vec_unsigned_rounding_mode_triple_gen_var_2, primitive_float_vec_gen_var_1,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn parse_hex_strings(xs_hex: &[&str]) -> Vec<Float> {
    xs_hex.iter().copied().map(parse_hex_string).collect()
}

#[test]
fn test_sum() {
    let test = |xs_hex: &[&str], out: &str, out_hex: &str| {
        let xs = parse_hex_strings(xs_hex);

        let sum = Float::sum(xs.iter().cloned());
        assert!(sum.is_valid());
        assert_eq!(sum.to_string(), out);
        assert_eq!(to_hex_string(&sum), out_hex);

        let sum_alt = Float::sum(xs.iter());
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));

        if xs.iter().all(in_gate) {
            let sum_alt = naive_sum(&xs);
            assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        }

        let rug_xs: Vec<rug::Float> = xs.iter().map(rug::Float::exact_from).collect();
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_sum(&rug_xs))),
            ComparableFloatRef(&sum),
        );
    };
    test(&[], "0.0", "0x0.0");
    test(&["NaN"], "NaN", "NaN");
    test(&["Infinity", "-Infinity"], "NaN", "NaN");
    test(&["Infinity", "NaN"], "NaN", "NaN");
    test(&["Infinity", "0x1.0#1", "Infinity"], "Infinity", "Infinity");
    test(&["-0x0.0", "-0x0.0", "-0x0.0"], "-0.0", "-0x0.0");
    test(&["-0x0.0", "0x0.0", "-0x0.0"], "0.0", "0x0.0");
    test(&["0x1.0#1", "0x2.0#1", "0x3.0#2"], "6.0", "0x6.0#2");
    test(&["0x1.0#1", "-0x1.0#1", "0x1.4#3"], "1.2", "0x1.4#3");
    test(
        &[
            "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1",
            "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1",
            "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1",
        ],
        "16.0",
        "0x1.0E+1#1",
    );
    test(
        &["0x0.55555555558#40", "0x0.249248#20", "0x4.0E-8#1"],
        "0.47619040900236",
        "0x0.79e79d59558#40",
    );
}

#[test]
fn test_sum_prec() {
    let test = |xs_hex: &[&str], prec: u64, out: &str, out_hex: &str, o_out: Ordering| {
        let xs = parse_hex_strings(xs_hex);

        let (sum, o) = Float::sum_prec(&xs, prec);
        assert!(sum.is_valid());
        assert_eq!(sum.to_string(), out);
        assert_eq!(to_hex_string(&sum), out_hex);
        assert_eq!(o, o_out);

        if xs.iter().all(in_gate) {
            let (sum_alt, o_alt) = naive_sum_prec(&xs, prec);
            assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
            assert_eq!(o_alt, o);
        }

        let rug_xs: Vec<rug::Float> = xs.iter().map(rug::Float::exact_from).collect();
        let (rug_sum, rug_o) = rug_sum_prec(&rug_xs, prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_sum)),
            ComparableFloatRef(&sum)
        );
        assert_eq!(rug_o, o);
    };
    test(&[], 10, "0.0", "0x0.0", Equal);
    test(&["NaN", "0x2.0#1"], 10, "NaN", "NaN", Equal);
    test(
        &["0x1.0#1", "0x2.0#1", "0x3.0#2"],
        10,
        "6.0000",
        "0x6.00#10",
        Equal,
    );
    test(
        &["0x1.0#1", "0x2.0#1", "0x3.0#2"],
        2,
        "6.0",
        "0x6.0#2",
        Equal,
    );
    test(
        &["0x1.0#1", "0x0.00001#1", "0x1.0E-10#1"],
        30,
        "1.0000009537",
        "0x1.00001000#30",
        Less,
    );
    test(
        &["0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1"],
        2,
        "8.0",
        "0x8.0#2",
        Greater,
    );
}

#[test]
fn sum_prec_fail() {
    assert_panic!(Float::sum_prec(&[Float::from(3), Float::from(4)], 0));
    assert_panic!(Float::sum_prec_round(&[], 0, Floor));
}

#[test]
fn test_sum_round() {
    let test = |xs_hex: &[&str], rm: RoundingMode, out: &str, out_hex: &str, o_out: Ordering| {
        let xs = parse_hex_strings(xs_hex);

        let (sum, o) = Float::sum_round(&xs, rm);
        assert!(sum.is_valid());
        assert_eq!(sum.to_string(), out);
        assert_eq!(to_hex_string(&sum), out_hex);
        assert_eq!(o, o_out);

        if xs.iter().all(in_gate) {
            let (sum_alt, o_alt) = naive_sum_round(&xs, rm);
            assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
            assert_eq!(o_alt, o);
        }

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let rug_xs: Vec<rug::Float> = xs.iter().map(rug::Float::exact_from).collect();
            let (rug_sum, rug_o) = rug_sum_round(&rug_xs, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_sum)),
                ComparableFloatRef(&sum)
            );
            assert_eq!(rug_o, o);
        }
    };
    test(
        &["Infinity", "Infinity"],
        Nearest,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        &["0x1.0#1", "0x2.0#1", "0x3.0#2"],
        Exact,
        "6.0",
        "0x6.0#2",
        Equal,
    );
    test(
        &["0x1.000#10", "0x0.00001#1", "0x1.0E-10#1"],
        Floor,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        &["0x1.000#10", "0x0.00001#1", "0x1.0E-10#1"],
        Ceiling,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
    test(
        &["0x1.000#10", "0x0.00001#1", "0x1.0E-10#1"],
        Down,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        &["0x1.000#10", "0x0.00001#1", "0x1.0E-10#1"],
        Up,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
    // - the singular rules: NaN, opposing infinities, matching infinities
    test(
        &["0x1.000#10", "0x0.00001#1", "0x1.0E-10#1"],
        Nearest,
        "1.0000",
        "0x1.000#10",
        Less,
    );
}

#[test]
fn sum_round_fail() {
    assert_panic!(Float::sum_round(
        &[Float::from(1), Float::from(1) >> 5u32],
        Exact
    ));
}

#[test]
fn test_sum_prec_round() {
    let test = |xs_hex: &[&str],
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let xs = parse_hex_strings(xs_hex);

        let (sum, o) = Float::sum_prec_round(&xs, prec, rm);
        assert!(sum.is_valid());
        assert_eq!(sum.to_string(), out);
        assert_eq!(to_hex_string(&sum), out_hex);
        assert_eq!(o, o_out);

        if xs.iter().all(in_gate) {
            let (sum_alt, o_alt) = naive_sum_prec_round(&xs, prec, rm);
            assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
            assert_eq!(o_alt, o);
        }

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let rug_xs: Vec<rug::Float> = xs.iter().map(rug::Float::exact_from).collect();
            let (rug_sum, rug_o) = rug_sum_prec_round(&rug_xs, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_sum)),
                ComparableFloatRef(&sum)
            );
            assert_eq!(rug_o, o);
        }
    };
    test(&["NaN", "Infinity"], 1, Floor, "NaN", "NaN", Equal);
    test(&["Infinity", "-Infinity"], 2, Nearest, "NaN", "NaN", Equal);
    // - all-zero inputs: same-sign zeros keep their sign; mixed signs give +0 except under Floor
    test(
        &["-Infinity", "0x3.0#2", "-Infinity"],
        5,
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        &["-0x0.0", "0x0.0", "-0x0.0"],
        3,
        Floor,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    // - complete cancellation of nonzero values: +0 except under Floor
    test(&["-0x0.0", "-0x0.0"], 3, Floor, "-0.0", "-0x0.0", Equal);
    test(
        &["0x7.0#3", "-0x7.0#3", "0x3.0#2", "-0x3.0#2"],
        8,
        Floor,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        &["-0x0.0", "0x0.0", "-0x0.0"],
        3,
        Ceiling,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(&["-0x0.0", "-0x0.0"], 3, Ceiling, "-0.0", "-0x0.0", Equal);
    test(
        &["0x7.0#3", "-0x7.0#3", "0x3.0#2", "-0x3.0#2"],
        8,
        Ceiling,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        &["-0x0.0", "0x0.0", "-0x0.0"],
        3,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(&["-0x0.0", "-0x0.0"], 3, Nearest, "-0.0", "-0x0.0", Equal);
    // - one or two regular values among zeros: the set/add delegation paths
    test(
        &["0x7.0#3", "-0x7.0#3", "0x3.0#2", "-0x3.0#2"],
        8,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        &["0x0.0", "0x5.0#3", "-0x0.0"],
        2,
        Floor,
        "4.0",
        "0x4.0#2",
        Less,
    );
    // - exact sums, including through the Exact rounding mode and at large exponents
    test(
        &["0x5.0#3", "0x0.0", "0x3.0#2"],
        2,
        Ceiling,
        "8.0",
        "0x8.0#2",
        Equal,
    );
    test(
        &["0x1.0#1", "0x2.0#1", "0x3.0#2"],
        10,
        Floor,
        "6.0000",
        "0x6.00#10",
        Equal,
    );
    test(
        &["0x3.0E+25#2", "0x4.0E+25#1", "0x5.0E+25#3"],
        4,
        Floor,
        "1.52e31",
        "0xc.0E+25#4",
        Equal,
    );
    test(
        &["0x1.0#1", "0x2.0#1", "0x3.0#2"],
        10,
        Nearest,
        "6.0000",
        "0x6.00#10",
        Equal,
    );
    test(
        &["0x3.0E+25#2", "0x4.0E+25#1", "0x5.0E+25#3"],
        4,
        Nearest,
        "1.52e31",
        "0xc.0E+25#4",
        Equal,
    );
    test(
        &["0x1.0#1", "0x2.0#1", "0x3.0#2"],
        10,
        Exact,
        "6.0000",
        "0x6.00#10",
        Equal,
    );
    test(
        &["0x3.0E+25#2", "0x4.0E+25#1", "0x5.0E+25#3"],
        4,
        Exact,
        "1.52e31",
        "0xc.0E+25#4",
        Equal,
    );
    // - deep cancellation with a distant tail: the accumulator window slides until it finds the
    //   surviving bit
    test(
        &[
            "0x10000000000000000000000000.0000000000000000000000000#200",
            "-0x10000000000000000000000000.0000000000000000000000000#200",
            "0x1.0E-100#1",
        ],
        10,
        Floor,
        "3.8726e-121",
        "0x1.000E-100#10",
        Equal,
    );
    test(
        &[
            "0x10000000000000000000000000.0000000000000000000000000#200",
            "-0x10000000000000000000000000.0000000000000000000000000#200",
            "0x1.0E-100#1",
        ],
        10,
        Ceiling,
        "3.8726e-121",
        "0x1.000E-100#10",
        Equal,
    );
    test(
        &[
            "0x10000000000000000000000000.0000000000000000000000000#200",
            "-0x10000000000000000000000000.0000000000000000000000000#200",
            "0x1.0E-100#1",
        ],
        10,
        Down,
        "3.8726e-121",
        "0x1.000E-100#10",
        Equal,
    );
    test(
        &[
            "0x10000000000000000000000000.0000000000000000000000000#200",
            "-0x10000000000000000000000000.0000000000000000000000000#200",
            "0x1.0E-100#1",
        ],
        10,
        Up,
        "3.8726e-121",
        "0x1.000E-100#10",
        Equal,
    );
    // - the table-maker's dilemma: the leading sum sits exactly on a rounding boundary, and dust
    //   thousands of bits below decides the direction
    test(
        &[
            "0x10000000000000000000000000.0000000000000000000000000#200",
            "-0x10000000000000000000000000.0000000000000000000000000#200",
            "0x1.0E-100#1",
        ],
        10,
        Nearest,
        "3.8726e-121",
        "0x1.000E-100#10",
        Equal,
    );
    test(
        &["0x1.0#1", "0x1.0E-16#1", "0x1.0E-500#1"],
        64,
        Floor,
        "1.00000000000000000000",
        "0x1.0000000000000000#64",
        Less,
    );
    test(
        &["0x1.0#1", "0x1.0E-16#1", "-0x1.0E-500#1"],
        64,
        Floor,
        "1.00000000000000000000",
        "0x1.0000000000000000#64",
        Less,
    );
    test(
        &["0x1.0#1", "0x1.0E-16#1", "0x1.0E-500#1"],
        64,
        Ceiling,
        "1.00000000000000000011",
        "0x1.0000000000000002#64",
        Greater,
    );
    test(
        &["0x1.0#1", "0x1.0E-16#1", "-0x1.0E-500#1"],
        64,
        Ceiling,
        "1.00000000000000000011",
        "0x1.0000000000000002#64",
        Greater,
    );
    test(
        &["0x1.0#1", "0x1.0E-16#1", "0x1.0E-500#1"],
        64,
        Nearest,
        "1.00000000000000000011",
        "0x1.0000000000000002#64",
        Greater,
    );
    test(
        &["0x1.0#1", "0x1.0E-16#1", "-0x1.0E-500#1"],
        64,
        Nearest,
        "1.00000000000000000000",
        "0x1.0000000000000000#64",
        Less,
    );
    // - long carry chains, and an alternating chain cancelling to a signed zero
    test(
        &[
            "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1",
            "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1",
            "0x1.0#1", "0x1.0#1", "0x1.0#1", "0x1.0#1",
        ],
        3,
        Nearest,
        "20.0",
        "0x14.0#3",
        Equal,
    );
    // - overflow saturation at the top of the exponent range
    test(
        &[
            "0x1.0#1", "-0x1.0#1", "0x1.0#1", "-0x1.0#1", "0x1.0#1", "-0x1.0#1", "0x1.0#1",
            "-0x1.0#1", "0x1.0#1", "-0x1.0#1", "0x1.0#1", "-0x1.0#1", "0x1.0#1", "-0x1.0#1",
            "0x1.0#1", "-0x1.0#1", "0x1.0#1", "-0x1.0#1", "0x1.0#1", "-0x1.0#1",
        ],
        3,
        Floor,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        &["0x4.0E+268435455#1", "0x4.0E+268435455#1", "0x4.0E+268435455#1"],
        1,
        Floor,
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Less,
    );
    test(
        &["0x4.0E+268435455#1", "0x4.0E+268435455#1", "0x4.0E+268435455#1"],
        1,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        &["0x4.0E+268435455#1", "0x4.0E+268435455#1", "0x4.0E+268435455#1"],
        1,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    // - underflow: cancellation leaves a value below the minimum exponent
    test(
        &["0x4.0E+268435455#1", "0x4.0E+268435455#1", "0x4.0E+268435455#1"],
        1,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        &["0x1.8E-268435456#2", "-0x1.0E-268435456#1", "0x0.0"],
        5,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        &["0x1.8E-268435456#2", "-0x1.0E-268435456#1", "0x0.0"],
        5,
        Ceiling,
        "2.38e-323228497",
        "0x1.0E-268435456#5",
        Greater,
    );
    test(
        &["0x1.8E-268435456#2", "-0x1.0E-268435456#1", "0x0.0"],
        5,
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        &["0x1.8E-268435456#2", "-0x1.0E-268435456#1", "0x0.0"],
        5,
        Up,
        "2.38e-323228497",
        "0x1.0E-268435456#5",
        Greater,
    );
    test(
        &["0x1.8E-268435456#2", "-0x1.0E-268435456#1", "0x0.0"],
        5,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );
    // Step-4 branch-coverage rows.
    // - a NaN encountered in the singular scan (three or more inputs, so no delegation to add)
    test(
        &["NaN", "0x1.0#1", "0x2.0#1"],
        5,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    // - opposing infinities encountered in the singular scan, in both orders
    test(
        &["-Infinity", "0x1.0#1", "Infinity"],
        5,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        &["Infinity", "0x1.0#1", "-Infinity"],
        5,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    // - exact pair cancellation with a lone halfway-remainder term: virtual halfway elimination
    //   (tmd == 2, secondary accumulator zero) with lbit == 0, so sst = -1 and the halfway case
    //   rounds down
    test(
        &["0xe938.0#13", "-0xe938.0#13", "0x0.00005#3"],
        2,
        Nearest,
        "3.8e-6",
        "0x0.00004#2",
        Less,
    );
    // - deep cancellation of exactly Limb::WIDTH bits: the copy to the destination is limb-aligned
    //   (sh == 0) with u > minexp
    test(
        &[
            "0x1.44baa146cfe262ef432c5d72e0174a4b1aa6304374eb41E+56#185",
            "-0x1.44baa146cfe262ef432c5d72e0174a4b1aa6304374eb4E+56#179",
            "-0x4.07331E-9#23",
            "-0x1.26f5bf074E-18#35",
            "0x2.0E-618#1",
            "-0x2.0E-618#1",
        ],
        59,
        Ceiling,
        "1099511627776.000000",
        "0x10000000000.00000#59",
        Greater,
    );
    // The next rows all use the corr == 2 pattern: X - 2^s with s exactly at the accumulator
    // window's bottom supplies a rounding bit of 1 followed by ones down to the window bottom, and
    // three dust terms just below the window (together exceeding 2^minexp) push the true value past
    // the rounding boundary, so the TMD resolves with sst = 1 and, in a ceiling-like rounding mode,
    // corr = 2.
    // - positive corr == 2 at prec 65: sd == Limb::WIDTH - 1, so the correction limb overflows to 0
    //   and the carry is added one limb up
    test(
        &["0x3.0#2", "-0x4.0E-31#1", "0x2.0E-31#1", "0x2.0E-31#1", "0x2.0E-31#1"],
        65,
        Up,
        "3.00000000000000000011",
        "0x3.0000000000000002#65",
        Greater,
    );
    // - negative corr == 2 at prec 65: the truncated window is 0111...1, so com(x) - 1 runs the
    //   all-ones borrow chain and the result drops a binade
    test(
        &["-0x1.0E+16#1", "-0x2.0E-15#1", "0x1.0E-15#1", "0x1.0E-15#1", "0x1.0E-15#1"],
        65,
        Ceiling,
        "-18446744073709551615.5",
        "-0xffffffffffffffff.8#65",
        Greater,
    );
    // - same shape under Down, which is also ceiling-like for negative results
    test(
        &["-0x1.0E+16#1", "-0x2.0E-15#1", "0x1.0E-15#1", "0x1.0E-15#1", "0x1.0E-15#1"],
        65,
        Down,
        "-18446744073709551615.5",
        "-0xffffffffffffffff.8#65",
        Greater,
    );
    // - negative corr == 2 with a non-power-of-2 leading term: the all-ones borrow chain runs
    //   without the binade drop
    test(
        &["-0x10000000000000002.0#65", "-0x2.0E-15#1", "0x1.0E-15#1", "0x1.0E-15#1", "0x1.0E-15#1"],
        65,
        Ceiling,
        "-18446744073709551617.0",
        "-0x10000000000000001.0#65",
        Greater,
    );
    // - deep cancellation forces an accumulator shift-and-reiterate, and after the window slides, a
    //   high input is skipped whole (vd >= 0 with skip >= vs); the ignored dust pair cancels
    //   exactly, so the TMD's secondary accumulator is zero and sst = 0
    test(
        &[
            "0x1.2a6dd59a82b5cE+20#51",
            "-0x12a6dd59a82b5c0000000.00000000000000000000000000000#194",
            "-0x0.0f9f7827b9b32#48",
            "0x1.0E-604#1",
            "-0x1.0E-604#1",
        ],
        122,
        Ceiling,
        "-0.061027059267598549041622391087003052235",
        "-0x0.0f9f7827b9b320000000000000000000#122",
        Equal,
    );
    // - corr == -1 (floor-like with sst == -1): subtracting an ulp drops a binade; the target ulp
    //   is at or below the window bottom, so the whole accumulator is copied (u <= minexp with a
    //   shifted copy)
    test(
        &[
            "0x2.7cc8a22ad7df8487596fcbfac84edd186569c24f9ef50E+53#180",
            "-0x2.7cc8a22ad7df8487596fcbfac84edd186569c24f9ef4E+53#176",
            "-0x1.e037b595828E-42#44",
            "-0xc.0E-26#2",
            "0x1.0E-192#1",
        ],
        129,
        Floor,
        "68719476735.99999999999999999999999999990",
        "0xfffffffff.fffffffffffffffffffffff8#129",
        Less,
    );
    // - negative inexact result with the plain complement path, and a TMD detection chunk that
    //   starts exactly at a word boundary (td == 0)
    test(
        &["-0x513c2a2f36bfbf672b775735.0#97", "-0x2.0E-15#1", "-0x2.0E-553#1", "0x2.0E-553#1"],
        125,
        Ceiling,
        "-25141020555040162555069159221.0000000000",
        "-0x513c2a2f36bfbf672b775735.00000000#125",
        Greater,
    );
    // - virtual halfway elimination (tmd == 2, secondary accumulator zero) with lbit == 0, so sst =
    //   -1; the detection chunk has td == 1 (borrowing a full extra limb)
    test(
        &[
            "-0x2021483fdff4.2f539da5ce#86",
            "0x2021483fdff4.2f539da5ce000000000000000#146",
            "0x4.7ad9cd0E-35#29",
            "-0xc.da8da3c2cE-42#38",
            "-0x8.0E-709#1",
            "0x8.0E-709#1",
        ],
        64,
        Nearest,
        "3.21416045134855126255e-42",
        "0x4.7ad9cc325725c3d0E-35#64",
        Less,
    );
    // - negative result whose low-limb correction carries through all-zero upper limbs, bumping the
    //   exponent (all_zero branch), with an aligned u <= minexp copy
    test(
        &["-0xd03df0ed8d2430.0#53", "0xd03df0ed8d2430.000000000000000000#125", "-0x0.08#1"],
        64,
        Nearest,
        "-0.0312500000000000000000",
        "-0x0.08000000000000000#64",
        Equal,
    );
    // - virtual halfway elimination with lbit == 1, so sst = 1 and the halfway case rounds up
    test(
        &[
            "-0x2.9eE+5#9",
            "0x29e000.0000000000000000000000000000000000000000000#191",
            "0x9.f5cf48501d0898E-10#59",
            "-0x1.960E-7#10",
        ],
        65,
        Nearest,
        "-5.89901883892751064304e-9",
        "-0x1.9560a30b7afe2f76E-7#65",
        Greater,
    );
    // - u <= minexp copy where the accumulator contributes no limbs at all (en == 0), at precision
    //   1 with corr == -1
    test(
        &[
            "-0xe.064cf804dd67bc3d0E+19#70",
            "0xe064cf804dd67bc3d000.000000#102",
            "-0x1.b50e26E-30#24",
            "-0x4.0E-11#1",
        ],
        1,
        Up,
        "-4.5e-13",
        "-0x8.0E-11#1",
        Less,
    );
    // - TMD detection scan that ends on a partial-limb comparison (d < Limb::WIDTH)
    test(
        &[
            "-0x2aa91b4ea.e4a9#50",
            "0x2aa91b4ea.e4a900000000000000000#116",
            "0x1.5a619E-21#21",
            "0xd.bc87f1afb2e80E-53#53",
            "0x1.0E-604#1",
        ],
        56,
        Nearest,
        "6.99511067829219666e-26",
        "0x1.5a619000000000E-21#56",
        Less,
    );
    // - positive TMD result with corr == 1
    test(
        &[
            "0x1.4c9e7b31eef6a63885d57f725df5e982de8a4268E+42#158",
            "-0x14c9e7b31eef6a63885d57f725df5e982de8a426800.000#178",
            "0x7.01a650E-66#24",
            "0x0.1f80#11",
        ],
        64,
        Ceiling,
        "0.123046875000000000007",
        "0x0.1f800000000000002#64",
        Greater,
    );
    // - negative TMD result with corr == 1 after a shift-and-reiterate
    test(
        &[
            "-0x750b76.0#22",
            "0x750b76.00000000000000000000000#115",
            "0x1.4ab38d504fE-44#41",
            "-0xf.ba09525E-8#32",
        ],
        40,
        Down,
        "-3.6616587860729e-9",
        "-0xf.ba09524ffE-8#40",
        Greater,
    );
    // - negative halfway case (tmd == 2) resolved by a nonzero secondary accumulator
    test(
        &["-0x0.02608#11", "0x0.02608000#24", "-0x5.0E-55#6", "-0x8.0E-727#1"],
        2,
        Nearest,
        "-3.6e-66",
        "-0x6.0E-55#2",
        Less,
    );
    // - exact accumulator (no ignored bits) whose sticky-bit determination scans across limbs
    test(
        &[
            "-0xa.d4cE+11#14",
            "0xad4c00000000.0000000000#85",
            "-0x1.173d282ccafbeE+19#52",
            "0x0.39f20#15",
        ],
        58,
        Floor,
        "-8.241677468881068333e22",
        "-0x1.173d282ccafbe00E+19#58",
        Less,
    );
    // - precision-1 halfway case (tmd == 2) with a nonzero secondary accumulator
    test(
        &[
            "0x2.bc2dcf5d333f5897a4E+22#74",
            "-0x2bc2dcf5d333f5897a40000.000000000000#136",
            "0x62b05a05e.0#34",
            "-0x7.1418f9b742E-28#42",
        ],
        1,
        Nearest,
        "3.4e10",
        "0x8.0E+8#1",
        Greater,
    );
    // - negative corr == 2 whose truncated window has a low bit of 0: the borrow stops in the low
    //   limb and the upper limbs are simply complemented
    test(
        &["-0x10000000000000001.0#65", "-0x2.0E-15#1", "0x1.0E-15#1", "0x1.0E-15#1", "0x1.0E-15#1"],
        65,
        Ceiling,
        "-18446744073709551616.0",
        "-0x10000000000000000.0#65",
        Greater,
    );
}

#[test]
fn sum_prec_round_fail() {
    assert_panic!(Float::sum_prec_round(&[Float::from(3)], 0, Floor));
    // 1 + 1/32 needs 6 bits
    assert_panic!(Float::sum_prec_round(
        &[Float::from(1), Float::from(1) >> 5u32, Float::from(1) >> 5u32],
        4,
        Exact
    ));
    // sum of three 1s is 3, which needs 2 bits
    assert_panic!(Float::sum_prec_round(
        &[Float::from(1), Float::from(1), Float::from(1)],
        1,
        Exact
    ));
}

// The exact sum as a Rational decides everything: the correctly-rounded value and ternary are
// unique, so this is a complete oracle. A cancelled zero is +0 except under Floor; the all-zeros
// input case is excluded by the caller.
fn rational_oracle(xs: &[Float], prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let exact: Rational = xs.iter().map(Rational::exact_from).sum();
    if exact == 0u32 {
        (
            if rm == Floor {
                -Float::ZERO
            } else {
                Float::ZERO
            },
            Equal,
        )
    } else {
        Float::from_rational_prec_round(exact, prec, rm)
    }
}

const EXPONENT_GATE: i64 = 1 << 16;

fn in_gate(x: &Float) -> bool {
    x.get_exponent()
        .is_none_or(|e| i64::from(e).abs() < EXPONENT_GATE)
}

#[allow(clippy::needless_pass_by_value)]
fn sum_prec_round_properties_helper(xs: Vec<Float>, prec: u64, rm: RoundingMode) {
    let (sum, o) = Float::sum_prec_round(&xs, prec, rm);
    assert!(sum.is_valid());

    // reversal invariance
    let reversed: Vec<Float> = xs.iter().rev().cloned().collect();
    let (sum_alt, o_alt) = Float::sum_prec_round(&reversed, prec, rm);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    let any_nan = xs.iter().any(Float::is_nan);
    let pos_inf = xs.iter().any(|x| x.is_infinite() && *x > 0u32);
    let neg_inf = xs.iter().any(|x| x.is_infinite() && *x < 0u32);
    if any_nan || (pos_inf && neg_inf) {
        assert!(sum.is_nan());
        assert_eq!(o, Equal);
        return;
    } else if pos_inf {
        assert_eq!(sum, Float::INFINITY);
        assert_eq!(o, Equal);
        return;
    } else if neg_inf {
        assert_eq!(sum, Float::NEGATIVE_INFINITY);
        assert_eq!(o, Equal);
        return;
    }

    // the naive exact-accumulation oracle, which extends precisions so that every partial sum is
    // exact and rounds only once
    if xs.iter().all(in_gate) {
        let (sum_alt, o_alt) = naive_sum_prec_round(&xs, prec, rm);
        assert_eq!(ComparableFloat(sum_alt), ComparableFloat(sum.clone()));
        assert_eq!(o_alt, o);
    }

    // appending a positive zero changes nothing, unless every input is a zero
    if !xs.is_empty() && xs.iter().any(|x| *x != 0u32) {
        let mut with_zero = xs.clone();
        with_zero.push(Float::ZERO);
        let (sum_alt, o_alt) = Float::sum_prec_round(&with_zero, prec, rm);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
    }

    // consistency with the two-input delegation
    let regulars: Vec<&Float> = xs.iter().filter(|x| **x != 0u32).collect();
    if regulars.len() == 1 && xs.iter().all(|x| !x.is_nan()) {
        let (sum_alt, o_alt) = Float::from_float_prec_round_ref(regulars[0], prec, rm);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
    }

    if xs.iter().all(in_gate) && !xs.is_empty() && xs.iter().any(|x| *x != 0u32) {
        // the complete exact-Rational oracle
        let (sum_alt, o_alt) = rational_oracle(&xs, prec, rm);
        assert_eq!(ComparableFloat(sum_alt), ComparableFloat(sum.clone()));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let rug_xs: Vec<rug::Float> = xs.iter().map(rug::Float::exact_from).collect();
            let (rug_sum, rug_o) = rug_sum_prec_round(&rug_xs, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_sum)),
                ComparableFloatRef(&sum)
            );
            assert_eq!(rug_o, o);
        }

        if o == Equal {
            for rm in exhaustive_rounding_modes() {
                let (s, oo) = Float::sum_prec_round(&xs, prec, rm);
                assert_eq!(
                    ComparableFloat(s.abs_negative_zero()),
                    ComparableFloat(sum.abs_negative_zero_ref())
                );
                assert_eq!(oo, Equal);
            }
        } else {
            assert_panic!(Float::sum_prec_round(&xs, prec, Exact));
        }
    }
}

#[test]
fn sum_prec_round_properties() {
    float_vec_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(xs, prec, rm)| {
        sum_prec_round_properties_helper(xs, prec, rm);
    });

    float_vec_unsigned_rounding_mode_triple_gen_var_2().test_properties(|(xs, prec, rm)| {
        sum_prec_round_properties_helper(xs, prec, rm);
    });
}

#[test]
fn sum_prec_properties() {
    float_vec_unsigned_pair_gen_var_1().test_properties(|(xs, prec)| {
        let (sum, o) = Float::sum_prec(&xs, prec);
        assert!(sum.is_valid());
        let (sum_alt, o_alt) = Float::sum_prec_round(&xs, prec, Nearest);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn sum_round_properties() {
    float_vec_rounding_mode_pair_gen_var_1().test_properties(|(xs, rm)| {
        let (sum, o) = Float::sum_round(&xs, rm);
        assert!(sum.is_valid());
        let prec = xs
            .iter()
            .map(SignificantBits::significant_bits)
            .max()
            .unwrap_or(1);
        let (sum_alt, o_alt) = Float::sum_prec_round(&xs, prec, rm);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
    });

    float_vec_rounding_mode_pair_gen_var_2().test_properties(|(xs, rm)| {
        let (sum, o) = Float::sum_round(&xs, rm);
        assert!(sum.is_valid());
        let prec = xs
            .iter()
            .map(SignificantBits::significant_bits)
            .max()
            .unwrap_or(1);
        let (sum_alt, o_alt) = Float::sum_prec_round(&xs, prec, rm);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn sum_properties_helper(xs: Vec<Float>) {
    let sum = Float::sum(xs.iter().cloned());
    assert!(sum.is_valid());
    let sum_alt = Float::sum(xs.iter());
    assert!(sum_alt.is_valid());
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

    let prec = xs
        .iter()
        .map(SignificantBits::significant_bits)
        .max()
        .unwrap_or(1);
    let (sum_alt, _) = Float::sum_prec_round(&xs, prec, Nearest);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

    if xs.iter().all(in_gate) {
        let rug_xs: Vec<rug::Float> = xs.iter().map(rug::Float::exact_from).collect();
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_sum(&rug_xs))),
            ComparableFloatRef(&sum),
        );
    }
}

#[test]
fn sum_properties() {
    float_vec_gen().test_properties(sum_properties_helper);
    float_vec_gen_var_1().test_properties(sum_properties_helper);
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_sum() {
    fn test<T: PrimitiveFloat>(xs: &[T], out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_sum(xs)), NiceFloat(out));
    }
    test::<f64>(&[], 0.0);
    test::<f64>(&[f64::NAN, 1.0], f64::NAN);
    test::<f64>(&[f64::INFINITY, f64::NEGATIVE_INFINITY], f64::NAN);
    test::<f64>(&[f64::INFINITY, 1.0], f64::INFINITY);
    test::<f64>(&[-0.0, -0.0], -0.0);
    test::<f64>(&[-0.0, 0.0], 0.0);
    test::<f64>(&[1.0, -1.0], 0.0);
    // only a single rounding is performed
    test::<f64>(&[0.1; 10], 1.0);
    // the overflow boundary: max + ulp/2 ties to infinity, and anything less rounds back
    test::<f64>(&[f64::MAX_FINITE, 2.0f64.powi(970)], f64::INFINITY);
    test::<f64>(&[f64::MAX_FINITE, 2.0f64.powi(969)], f64::MAX_FINITE);
    test::<f64>(&[f64::MAX_FINITE, f64::MAX_FINITE], f64::INFINITY);
    // sums of subnormals are exact on the subnormal grid
    test::<f64>(&[5e-324, 5e-324], 1.0e-323);
    test::<f32>(&[0.1; 10], 1.0);
    test::<f32>(&[f32::MAX_FINITE, f32::MAX_FINITE], f32::INFINITY);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_sum_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
    T: RoundingFrom<Rational>,
    Rational: ExactFrom<T>,
{
    primitive_float_vec_gen_var_1::<T>().test_properties(|xs| {
        let sum = primitive_float_sum(&xs);
        if xs.iter().all(|x| x.is_finite()) {
            let exact: Rational = xs.iter().map(|&x| Rational::exact_from(x)).sum();
            if exact != 0u32 {
                let (sum_alt, _) = T::rounding_from(exact, Nearest);
                assert_eq!(NiceFloat(sum_alt), NiceFloat(sum));
            }
        }
    });
}

#[test]
fn primitive_float_sum_properties() {
    apply_fn_to_primitive_floats!(primitive_float_sum_properties_helper);
}
