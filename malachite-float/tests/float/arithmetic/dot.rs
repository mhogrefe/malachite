// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::apply_fn_to_primitive_floats;
use malachite_base::num::arithmetic::traits::{NegAssign, PowerOf2};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_float::float::arithmetic::dot::primitive_float_dot;
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::float::arithmetic::dot::{
    naive_dot, naive_dot_prec, naive_dot_prec_round, naive_dot_round, rug_dot_prec_round,
};
use malachite_float::test_util::generators::{
    float_vec_gen, float_vec_gen_var_1, float_vec_pair_gen_var_1, float_vec_pair_gen_var_2,
    float_vec_pair_rounding_mode_triple_gen_var_1, float_vec_pair_rounding_mode_triple_gen_var_2,
    float_vec_pair_unsigned_rounding_mode_quadruple_gen_var_1,
    float_vec_pair_unsigned_rounding_mode_quadruple_gen_var_2,
    float_vec_pair_unsigned_triple_gen_var_1, primitive_float_vec_pair_gen_var_1,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn parse_hex_strings(xs_hex: &[&str]) -> Vec<Float> {
    xs_hex.iter().copied().map(parse_hex_string).collect()
}

// Whether every term's exponent is far enough inside the range that exact products are
// representable as `Float`s, which both the naive oracle and MPFR's `mpfr_dot` require. (MPFR
// aborts on inputs outside this gate, so rug must never see them.)
fn term_gate(xs: &[Float], ys: &[Float]) -> bool {
    xs.iter().zip(ys.iter()).all(|(x, y)| {
        let e = |f: &Float| f.get_exponent().map_or(0i64, i64::from);
        e(x).abs() < 1 << 15 && e(y).abs() < 1 << 15
    })
}

#[test]
fn test_dot() {
    let test = |xs_hex: &[&str], ys_hex: &[&str], out: &str, out_hex: &str| {
        let xs = parse_hex_strings(xs_hex);
        let ys = parse_hex_strings(ys_hex);

        let dot = Float::dot(&xs, &ys);
        assert!(dot.is_valid());
        assert_eq!(dot.to_string(), out);
        assert_eq!(to_hex_string(&dot), out_hex);

        if term_gate(&xs, &ys) {
            let dot_alt = naive_dot(&xs, &ys);
            assert_eq!(ComparableFloatRef(&dot_alt), ComparableFloatRef(&dot));
        }
    };
    test(&[], &[], "0.0", "0x0.0");
    test(&["NaN"], &["0x2.0#1"], "NaN", "NaN");
    test(
        &["0x1.0#1", "0x2.0#1", "0x3.0#2"],
        &["0x4.0#3", "0x5.0#3", "0x6.0#3"],
        "32.0",
        "0x20.0#3",
    );
    test(
        &["0x1.0#1", "-0x1.0#1"],
        &["0x1.0#1", "0x1.0#1"],
        "0.0",
        "0x0.0",
    );
}

#[test]
fn test_dot_prec() {
    let test =
        |xs_hex: &[&str], ys_hex: &[&str], prec: u64, out: &str, out_hex: &str, o_out: Ordering| {
            let xs = parse_hex_strings(xs_hex);
            let ys = parse_hex_strings(ys_hex);

            let (dot, o) = Float::dot_prec(&xs, &ys, prec);
            assert!(dot.is_valid());
            assert_eq!(dot.to_string(), out);
            assert_eq!(to_hex_string(&dot), out_hex);
            assert_eq!(o, o_out);

            if term_gate(&xs, &ys) {
                let (dot_alt, o_alt) = naive_dot_prec(&xs, &ys, prec);
                assert_eq!(ComparableFloatRef(&dot_alt), ComparableFloatRef(&dot));
                assert_eq!(o_alt, o);
            }
        };
    test(&[], &[], 10, "0.0", "0x0.0", Equal);
    test(
        &["0x1.0#1", "0x2.0#1", "0x3.0#2"],
        &["0x4.0#3", "0x5.0#3", "0x6.0#3"],
        10,
        "32.000",
        "0x20.0#10",
        Equal,
    );
    test(
        &["0x1.0#1", "0x2.0#1", "0x3.0#2"],
        &["0x4.0#3", "0x5.0#3", "0x6.0#3"],
        3,
        "32.0",
        "0x20.0#3",
        Equal,
    );
}

#[test]
fn dot_prec_fail() {
    assert_panic!(Float::dot_prec(&[Float::from(3)], &[Float::from(4)], 0));
    assert_panic!(Float::dot_prec(&[Float::from(3)], &[], 5));
}

#[test]
fn test_dot_round() {
    let test = |xs_hex: &[&str],
                ys_hex: &[&str],
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let xs = parse_hex_strings(xs_hex);
        let ys = parse_hex_strings(ys_hex);

        let (dot, o) = Float::dot_round(&xs, &ys, rm);
        assert!(dot.is_valid());
        assert_eq!(dot.to_string(), out);
        assert_eq!(to_hex_string(&dot), out_hex);
        assert_eq!(o, o_out);

        if term_gate(&xs, &ys) {
            let (dot_alt, o_alt) = naive_dot_round(&xs, &ys, rm);
            assert_eq!(ComparableFloatRef(&dot_alt), ComparableFloatRef(&dot));
            assert_eq!(o_alt, o);
        }
    };
    test(
        &["0x1.0#1", "0x2.0#1", "0x3.0#2"],
        &["0x4.0#3", "0x5.0#3", "0x6.0#3"],
        Exact,
        "32.0",
        "0x20.0#3",
        Equal,
    );
    test(
        &["0x0.4#1", "0x2.0#1"],
        &["0x0.4#1", "0x5.0#3"],
        Floor,
        "10.0",
        "0xa.0#3",
        Less,
    );
    test(
        &["0x0.4#1", "0x2.0#1"],
        &["0x0.4#1", "0x5.0#3"],
        Ceiling,
        "12.0",
        "0xc.0#3",
        Greater,
    );
}

#[test]
fn dot_round_fail() {
    assert_panic!(Float::dot_round(
        &[Float::from(3), Float::ONE],
        &[Float::from(5), Float::ONE >> 5u32],
        Exact
    ));
}

#[test]
fn test_dot_prec_round() {
    let test = |xs_hex: &[&str],
                ys_hex: &[&str],
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let xs = parse_hex_strings(xs_hex);
        let ys = parse_hex_strings(ys_hex);

        let (dot, o) = Float::dot_prec_round(&xs, &ys, prec, rm);
        assert!(dot.is_valid());
        assert_eq!(dot.to_string(), out);
        assert_eq!(to_hex_string(&dot), out_hex);
        assert_eq!(o, o_out);

        if term_gate(&xs, &ys) {
            let (dot_alt, o_alt) = naive_dot_prec_round(&xs, &ys, prec, rm);
            assert_eq!(ComparableFloatRef(&dot_alt), ComparableFloatRef(&dot));
            assert_eq!(o_alt, o);

            if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
                let rug_xs: Vec<rug::Float> = xs.iter().map(rug::Float::exact_from).collect();
                let rug_ys: Vec<rug::Float> = ys.iter().map(rug::Float::exact_from).collect();
                let (rug_dot, rug_o) = rug_dot_prec_round(&rug_xs, &rug_ys, prec, rug_rm);
                assert_eq!(
                    ComparableFloatRef(&Float::from(&rug_dot)),
                    ComparableFloatRef(&dot)
                );
                assert_eq!(rug_o, o);
            }
        }
    };
    // - the singular rules: NaN inputs, a zero times an infinity, infinite terms with and without
    //   sign conflicts
    test(
        &["NaN", "0x1.0#1", "0x2.0#1"],
        &["0x1.0#1", "0x2.0#1", "0x3.0#2"],
        5,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        &["Infinity", "0x3.0#2", "0x1.0#1"],
        &["0x0.0", "0x1.0#1", "0x1.0#1"],
        5,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        &["Infinity", "Infinity", "0x1.0#1"],
        &["0x1.0#1", "-0x2.0#1", "0x1.0#1"],
        5,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        &["Infinity", "0x3.0#2", "0x1.0#1"],
        &["-0x2.0#1", "0x1.0#1", "0x1.0#1"],
        5,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        &["-Infinity", "-0x3.0#2", "0x1.0#1"],
        &["-0x2.0#1", "0x1.0#1", "0x1.0#1"],
        5,
        Nearest,
        "Infinity",
        "Infinity",
        Equal,
    );
    // - all-zero terms: same signs, and mixed signs under Nearest and Floor
    test(
        &["0x0.0", "-0x3.0#2", "0x0.0"],
        &["-0x3.0#2", "0x0.0", "0x5.0#3"],
        5,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        &["0x0.0", "-0x3.0#2"],
        &["-0x3.0#2", "0x0.0"],
        5,
        Nearest,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        &["0x0.0", "-0x3.0#2"],
        &["-0x3.0#2", "0x0.0"],
        5,
        Floor,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    // - delegations: a single pair, a single regular term, and two regular terms (which use the
    //   fused multiply-add-multiply)
    test(
        &["0x3.0#2"],
        &["0x5.0#3"],
        10,
        Nearest,
        "15.000",
        "0xf.00#10",
        Equal,
    );
    test(
        &["0x3.0#2", "0x0.0", "0x5.0#3"],
        &["0x5.0#3", "0x7.0#3", "0x0.0"],
        10,
        Nearest,
        "15.000",
        "0xf.00#10",
        Equal,
    );
    test(
        &["0x3.0#2", "0x5.0#3", "0x0.0"],
        &["0x5.0#3", "0x7.0#3", "0xb.0#4"],
        10,
        Nearest,
        "50.000",
        "0x32.0#10",
        Equal,
    );
    // - an exact dot product through the kernel, including under the Exact rounding mode
    test(
        &["0x1.0#1", "0x2.0#1", "0x3.0#2"],
        &["0x4.0#1", "0x5.0#3", "0x6.0#2"],
        10,
        Nearest,
        "32.000",
        "0x20.0#10",
        Equal,
    );
    test(
        &["0x1.0#1", "0x2.0#1", "0x3.0#2"],
        &["0x4.0#1", "0x5.0#3", "0x6.0#2"],
        10,
        Exact,
        "32.000",
        "0x20.0#10",
        Equal,
    );
    // - inexact dot products through the kernel
    test(
        &["0x0.55555555558#40", "0xe.492492492492492#64", "-0x3.243f6f0243f6f0243f6f02440#100"],
        &["0xe.492492492492492#64", "-0x3.243f6f0243f6f0243f6f02440#100", "0x0.55555555558#40"],
        20,
        Floor,
        "-41.165222",
        "-0x29.2a4c#20",
        Less,
    );
    test(
        &["0x0.55555555558#40", "0xe.492492492492492#64", "-0x3.243f6f0243f6f0243f6f02440#100"],
        &["0xe.492492492492492#64", "-0x3.243f6f0243f6f0243f6f02440#100", "0x0.55555555558#40"],
        20,
        Ceiling,
        "-41.165161",
        "-0x29.2a48#20",
        Greater,
    );
    test(
        &["0x0.55555555558#40", "0xe.492492492492492#64", "-0x3.243f6f0243f6f0243f6f02440#100"],
        &["0xe.492492492492492#64", "-0x3.243f6f0243f6f0243f6f02440#100", "0x0.55555555558#40"],
        20,
        Down,
        "-41.165161",
        "-0x29.2a48#20",
        Greater,
    );
    test(
        &["0x0.55555555558#40", "0xe.492492492492492#64", "-0x3.243f6f0243f6f0243f6f02440#100"],
        &["0xe.492492492492492#64", "-0x3.243f6f0243f6f0243f6f02440#100", "0x0.55555555558#40"],
        20,
        Up,
        "-41.165222",
        "-0x29.2a4c#20",
        Less,
    );
    test(
        &["0x0.55555555558#40", "0xe.492492492492492#64", "-0x3.243f6f0243f6f0243f6f02440#100"],
        &["0xe.492492492492492#64", "-0x3.243f6f0243f6f0243f6f02440#100", "0x0.55555555558#40"],
        20,
        Nearest,
        "-41.165222",
        "-0x29.2a4c#20",
        Less,
    );
    // - exact cancellation of regular terms: +0, except -0 under Floor
    test(
        &["0x3.0#2", "0x5.0#3", "-0x3.0#2"],
        &["0x2.0#1", "0x0.0", "0x2.0#1"],
        5,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        &["0x3.0#2", "0x5.0#3", "-0x3.0#2"],
        &["0x2.0#1", "0x0.0", "0x2.0#1"],
        5,
        Floor,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    // - zero terms mixed with regular terms
    test(
        &["0x3.0#2", "0x0.0", "0x5.0#3"],
        &["0x2.0#1", "0x7.0#3", "0x1.0#1"],
        10,
        Nearest,
        "11.000",
        "0xb.00#10",
        Equal,
    );
    // - overflow and underflow of the final result, in both signs
    test(
        &["0x4.0E+268435455#1", "0x4.0E+268435455#1", "0x4.0E+268435455#1"],
        &["0x2.0#1", "0x2.0#1", "0x2.0#1"],
        5,
        Floor,
        "2.03e323228496",
        "0x7.cE+268435455#5",
        Less,
    );
    test(
        &["0x4.0E+268435455#1", "0x4.0E+268435455#1", "0x4.0E+268435455#1"],
        &["0x2.0#1", "0x2.0#1", "0x2.0#1"],
        5,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        &["0x4.0E+268435455#1", "0x4.0E+268435455#1", "0x4.0E+268435455#1"],
        &["0x2.0#1", "0x2.0#1", "0x2.0#1"],
        5,
        Down,
        "2.03e323228496",
        "0x7.cE+268435455#5",
        Less,
    );
    test(
        &["0x4.0E+268435455#1", "0x4.0E+268435455#1", "0x4.0E+268435455#1"],
        &["0x2.0#1", "0x2.0#1", "0x2.0#1"],
        5,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        &["0x4.0E+268435455#1", "0x4.0E+268435455#1", "0x4.0E+268435455#1"],
        &["0x2.0#1", "0x2.0#1", "0x2.0#1"],
        5,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        &["-0x4.0E+268435455#1", "-0x4.0E+268435455#1", "-0x4.0E+268435455#1"],
        &["0x2.0#1", "0x2.0#1", "0x2.0#1"],
        5,
        Floor,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        &["-0x4.0E+268435455#1", "-0x4.0E+268435455#1", "-0x4.0E+268435455#1"],
        &["0x2.0#1", "0x2.0#1", "0x2.0#1"],
        5,
        Ceiling,
        "-2.03e323228496",
        "-0x7.cE+268435455#5",
        Greater,
    );
    test(
        &["-0x4.0E+268435455#1", "-0x4.0E+268435455#1", "-0x4.0E+268435455#1"],
        &["0x2.0#1", "0x2.0#1", "0x2.0#1"],
        5,
        Nearest,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        &["0x2.0E-268435456#1", "0x2.0E-268435456#1", "0x2.0E-268435456#1"],
        &["0x2.0E-268435456#1", "0x2.0E-268435456#1", "-0x2.0E-268435456#1"],
        5,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        &["0x2.0E-268435456#1", "0x2.0E-268435456#1", "0x2.0E-268435456#1"],
        &["0x2.0E-268435456#1", "0x2.0E-268435456#1", "-0x2.0E-268435456#1"],
        5,
        Ceiling,
        "2.38e-323228497",
        "0x1.0E-268435456#5",
        Greater,
    );
    test(
        &["0x2.0E-268435456#1", "0x2.0E-268435456#1", "0x2.0E-268435456#1"],
        &["0x2.0E-268435456#1", "0x2.0E-268435456#1", "-0x2.0E-268435456#1"],
        5,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );
    // Step-4 branch-coverage rows.
    // - a NaN in the second slice, and repeated same-sign infinite terms
    test(
        &["0x1.0#1", "0x2.0#1", "0x3.0#2"],
        &["0x1.0#1", "NaN", "0x3.0#2"],
        5,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        &["Infinity", "Infinity", "0x1.0#1"],
        &["0x1.0#1", "0x2.0#1", "0x3.0#2"],
        5,
        Nearest,
        "Infinity",
        "Infinity",
        Equal,
    );
    // - all-zero terms whose first zero is positive, and genuinely mixed-sign zero terms under
    //   Floor and Nearest
    test(
        &["0x0.0", "0x3.0#2"],
        &["0x5.0#3", "0x0.0"],
        5,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        &["0x0.0", "-0x3.0#2"],
        &["0x5.0#3", "0x0.0"],
        5,
        Floor,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        &["0x0.0", "-0x3.0#2"],
        &["0x5.0#3", "0x0.0"],
        5,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
    );
    // - exact cancellation of three or more regular terms, through the summation kernel
    test(
        &["0x3.0#2", "0x5.0#3", "-0x8.0#1"],
        &["0x2.0#1", "0x2.0#1", "0x2.0#1"],
        5,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        &["0x3.0#2", "0x5.0#3", "-0x8.0#1"],
        &["0x2.0#1", "0x2.0#1", "0x2.0#1"],
        5,
        Floor,
        "-0.0",
        "-0x0.0",
        Equal,
    );
}

#[test]
fn dot_prec_round_fail() {
    assert_panic!(Float::dot_prec_round(
        &[Float::from(3)],
        &[Float::from(4)],
        0,
        Floor
    ));
    assert_panic!(Float::dot_prec_round(&[Float::from(3)], &[], 5, Floor));
    assert_panic!(Float::dot_prec_round(
        &[Float::from(3), Float::ONE],
        &[Float::from(5), Float::ONE >> 5u32],
        3,
        Exact
    ));
}

// The showcase cases that MPFR's `mpfr_dot` cannot handle (it aborts on them): intermediate
// products far outside the exponent range, with the final dot product back in range. The expected
// values are analytic: X * X - X * X + 3 * 1 = 3, t * t - t * t + 1 = 1, and 2 * t * t + 1 is
// slightly more than 1.
#[test]
fn dot_beyond_mpfr() {
    let x = Float::power_of_2(1073741822i64);
    let xs = [x.clone(), x.clone(), Float::from(3)];
    let ys = [x.clone(), -x.clone(), Float::ONE];
    for rm in exhaustive_rounding_modes() {
        let (d, o) = Float::dot_prec_round(&xs, &ys, 10, rm);
        assert_eq!(d.to_string(), "3.0000", "{rm}");
        assert_eq!(o, Equal);
    }
    let t = Float::power_of_2(-1073741823i64);
    let xs = [t.clone(), t.clone(), Float::ONE];
    let ys = [t.clone(), -t.clone(), Float::ONE];
    let (d, o) = Float::dot_prec_round(&xs, &ys, 10, Exact);
    assert_eq!(d.to_string(), "1.0000");
    assert_eq!(o, Equal);
    let xs = [t.clone(), t.clone(), Float::ONE];
    let ys = [t.clone(), t.clone(), Float::ONE];
    let (d, o) = Float::dot_prec_round(&xs, &ys, 10, Ceiling);
    assert_eq!(d.to_string(), "1.0020");
    assert_eq!(o, Greater);
    let (d, o) = Float::dot_prec_round(&xs, &ys, 10, Nearest);
    assert_eq!(d.to_string(), "1.0000");
    assert_eq!(o, Less);
}

#[allow(clippy::needless_pass_by_value)]
fn dot_prec_round_properties_helper(xs: Vec<Float>, ys: Vec<Float>, prec: u64, rm: RoundingMode) {
    let (dot, o) = Float::dot_prec_round(&xs, &ys, prec, rm);
    assert!(dot.is_valid());

    // reversing both slices changes nothing
    let xs_rev: Vec<Float> = xs.iter().rev().cloned().collect();
    let ys_rev: Vec<Float> = ys.iter().rev().cloned().collect();
    let (dot_alt, o_alt) = Float::dot_prec_round(&xs_rev, &ys_rev, prec, rm);
    assert_eq!(ComparableFloatRef(&dot_alt), ComparableFloatRef(&dot));
    assert_eq!(o_alt, o);

    // swapping the slices changes nothing
    let (dot_alt, o_alt) = Float::dot_prec_round(&ys, &xs, prec, rm);
    assert_eq!(ComparableFloatRef(&dot_alt), ComparableFloatRef(&dot));
    assert_eq!(o_alt, o);

    // negating one slice negates the dot product, with the rounding mode reflected
    let mut xs_neg = xs.clone();
    for x in &mut xs_neg {
        x.neg_assign();
    }
    let (dot_alt, o_alt) = Float::dot_prec_round(&xs_neg, &ys, prec, -rm);
    if dot.is_nan() {
        assert!(dot_alt.is_nan());
    } else if dot == 0u32 {
        // Zero results are not antisymmetric: the IEEE zero-sign rules prefer +0 for every rounding
        // mode except Floor, in both the negated and non-negated computations. Only the magnitude
        // and the (reflected) ternary are preserved.
        assert_eq!(dot_alt, 0u32);
        assert_eq!(o_alt, o.reverse());
    } else {
        assert_eq!(ComparableFloat(dot_alt), ComparableFloat(-dot.clone()));
        assert_eq!(o_alt, o.reverse());
    }

    // appending a pair with a zero changes nothing, unless every term is zero or the result is NaN
    if !xs.is_empty()
        && !dot.is_nan()
        && xs
            .iter()
            .zip(ys.iter())
            .any(|(x, y)| *x != 0u32 && *y != 0u32)
    {
        let mut xs_app = xs.clone();
        let mut ys_app = ys.clone();
        xs_app.push(Float::ZERO);
        ys_app.push(Float::from(3));
        let (dot_alt, o_alt) = Float::dot_prec_round(&xs_app, &ys_app, prec, rm);
        assert_eq!(ComparableFloatRef(&dot_alt), ComparableFloatRef(&dot));
        assert_eq!(o_alt, o);
    }

    // consistency with the delegations
    if xs.len() == 1 {
        let (dot_alt, o_alt) = xs[0].mul_prec_round_ref_ref(&ys[0], prec, rm);
        assert_eq!(ComparableFloatRef(&dot_alt), ComparableFloatRef(&dot));
        assert_eq!(o_alt, o);
    }

    if term_gate(&xs, &ys) {
        // the naive oracle: exact term products summed with the naive summation oracle
        let (dot_alt, o_alt) = naive_dot_prec_round(&xs, &ys, prec, rm);
        assert_eq!(ComparableFloat(dot_alt), ComparableFloat(dot.clone()));
        assert_eq!(o_alt, o);

        // rug, where its rounding modes allow (and only inside the term gate: mpfr_dot aborts on
        // intermediate overflow or underflow)
        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let rug_xs: Vec<rug::Float> = xs.iter().map(rug::Float::exact_from).collect();
            let rug_ys: Vec<rug::Float> = ys.iter().map(rug::Float::exact_from).collect();
            let (rug_dot, rug_o) = rug_dot_prec_round(&rug_xs, &rug_ys, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_dot)),
                ComparableFloatRef(&dot)
            );
            assert_eq!(rug_o, o);
        }

        // the complete exact-Rational oracle, for nonzero finite results
        if xs.iter().chain(ys.iter()).all(Float::is_finite) {
            let exact: Rational = xs
                .iter()
                .zip(ys.iter())
                .map(|(x, y)| Rational::exact_from(x) * Rational::exact_from(y))
                .sum();
            if exact != 0u32 {
                let (dot_alt, o_alt) = Float::from_rational_prec_round(exact, prec, rm);
                assert_eq!(ComparableFloat(dot_alt), ComparableFloat(dot.clone()));
                assert_eq!(o_alt, o);

                if o == Equal {
                    for rm in exhaustive_rounding_modes() {
                        let (d, oo) = Float::dot_prec_round(&xs, &ys, prec, rm);
                        assert_eq!(ComparableFloat(d), ComparableFloat(dot.clone()));
                        assert_eq!(oo, Equal);
                    }
                } else {
                    assert_panic!(Float::dot_prec_round(&xs, &ys, prec, Exact));
                }
            }
        }
    }
}

#[test]
fn dot_prec_round_properties() {
    float_vec_pair_unsigned_rounding_mode_quadruple_gen_var_1().test_properties(
        |(xs, ys, prec, rm)| {
            dot_prec_round_properties_helper(xs, ys, prec, rm);
        },
    );

    float_vec_pair_unsigned_rounding_mode_quadruple_gen_var_2().test_properties(
        |(xs, ys, prec, rm)| {
            dot_prec_round_properties_helper(xs, ys, prec, rm);
        },
    );
}

#[test]
fn dot_vs_sum_properties() {
    // A dot product against all-ones agrees exactly with the sum, including for extreme inputs
    // whose intermediate products the naive and rug oracles cannot handle.
    let helper = |xs: Vec<Float>| {
        let ones = vec![Float::ONE; xs.len()];
        for prec in [1u64, 10, 64] {
            for rm in [Floor, Ceiling, Down, Up, Nearest] {
                let (dot, o) = Float::dot_prec_round(&xs, &ones, prec, rm);
                let (sum, sum_o) = Float::sum_prec_round(&xs, prec, rm);
                assert_eq!(ComparableFloat(dot), ComparableFloat(sum));
                assert_eq!(o, sum_o);
            }
        }
    };
    float_vec_gen().test_properties(helper);
    float_vec_gen_var_1().test_properties(helper);
}

#[test]
fn dot_prec_properties() {
    float_vec_pair_unsigned_triple_gen_var_1().test_properties(|(xs, ys, prec)| {
        let (dot, o) = Float::dot_prec(&xs, &ys, prec);
        assert!(dot.is_valid());
        let (dot_alt, o_alt) = Float::dot_prec_round(&xs, &ys, prec, Nearest);
        assert_eq!(ComparableFloatRef(&dot_alt), ComparableFloatRef(&dot));
        assert_eq!(o_alt, o);

        if term_gate(&xs, &ys) {
            let (dot_alt, o_alt) = naive_dot_prec(&xs, &ys, prec);
            assert_eq!(ComparableFloat(dot_alt), ComparableFloat(dot.clone()));
            assert_eq!(o_alt, o);
        }
    });
}

#[test]
fn dot_round_properties() {
    let helper = |xs: &[Float], ys: &[Float], rm: RoundingMode| {
        let (dot, o) = Float::dot_round(xs, ys, rm);
        assert!(dot.is_valid());
        let prec = xs
            .iter()
            .chain(ys.iter())
            .map(SignificantBits::significant_bits)
            .max()
            .unwrap_or(1);
        let (dot_alt, o_alt) = Float::dot_prec_round(xs, ys, prec, rm);
        assert_eq!(ComparableFloatRef(&dot_alt), ComparableFloatRef(&dot));
        assert_eq!(o_alt, o);

        if term_gate(xs, ys) {
            let (dot_alt, o_alt) = naive_dot_round(xs, ys, rm);
            assert_eq!(ComparableFloat(dot_alt), ComparableFloat(dot.clone()));
            assert_eq!(o_alt, o);
        }
    };
    float_vec_pair_rounding_mode_triple_gen_var_1().test_properties(|(xs, ys, rm)| {
        helper(&xs, &ys, rm);
    });

    float_vec_pair_rounding_mode_triple_gen_var_2().test_properties(|(xs, ys, rm)| {
        helper(&xs, &ys, rm);
    });
}

#[test]
fn dot_properties() {
    let helper = |xs: Vec<Float>, ys: Vec<Float>| {
        let dot = Float::dot(&xs, &ys);
        assert!(dot.is_valid());
        let prec = xs
            .iter()
            .chain(ys.iter())
            .map(SignificantBits::significant_bits)
            .max()
            .unwrap_or(1);
        let (dot_alt, _) = Float::dot_prec_round(&xs, &ys, prec, Nearest);
        assert_eq!(ComparableFloatRef(&dot_alt), ComparableFloatRef(&dot));

        if term_gate(&xs, &ys) {
            let dot_alt = naive_dot(&xs, &ys);
            assert_eq!(ComparableFloat(dot_alt), ComparableFloat(dot.clone()));
        }
    };
    float_vec_pair_gen_var_1().test_properties(|(xs, ys)| helper(xs, ys));
    float_vec_pair_gen_var_2().test_properties(|(xs, ys)| helper(xs, ys));
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_dot() {
    fn test<T: PrimitiveFloat>(xs: &[T], ys: &[T], out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_dot(xs, ys)), NiceFloat(out));
    }
    test::<f64>(&[], &[], 0.0);
    test::<f64>(&[f64::NAN], &[1.0], f64::NAN);
    test::<f64>(&[f64::INFINITY], &[0.0], f64::NAN);
    test::<f64>(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0], 32.0);
    // intermediate overflow does not occur
    test::<f64>(&[1.0e300, 1.0e300], &[1.0e300, -1.0e300], 0.0);
    // a tie on the subnormal grid rounds to even
    test::<f64>(
        &[2.0f64.powi(-537), 2.0f64.powi(-537)],
        &[2.0f64.powi(-537), 2.0f64.powi(-538)],
        1.0e-323,
    );
    // double rounding is avoided: the exact value is just below the tie, so the result rounds down,
    // even though the 54-bit intermediate rounds to the tie
    test::<f64>(
        &[2.0f64.powi(-537), 2.0f64.powi(-537), -2.0f64.powi(-587)],
        &[2.0f64.powi(-537), 2.0f64.powi(-538), 2.0f64.powi(-588)],
        5.0e-324,
    );
    test::<f32>(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0], 32.0);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_dot_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
    T: RoundingFrom<Rational>,
    Rational: ExactFrom<T>,
{
    primitive_float_vec_pair_gen_var_1::<T>().test_properties(|(xs, ys)| {
        let dot = primitive_float_dot(&xs, &ys);
        if xs.iter().chain(ys.iter()).all(|x| x.is_finite()) {
            let exact: Rational = xs
                .iter()
                .zip(ys.iter())
                .map(|(&x, &y)| Rational::exact_from(x) * Rational::exact_from(y))
                .sum();
            if exact != 0u32 {
                let (dot_alt, _) = T::rounding_from(exact, Nearest);
                assert_eq!(NiceFloat(dot_alt), NiceFloat(dot));
            }
        }
    });
}

#[test]
fn primitive_float_dot_properties() {
    apply_fn_to_primitive_floats!(primitive_float_dot_properties_helper);
}
