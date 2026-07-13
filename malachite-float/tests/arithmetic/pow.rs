// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::apply_fn_to_primitive_floats;
use malachite_base::assert_panic;
use malachite_base::num::arithmetic::traits::{CheckedRoot, IsPowerOf2, Pow, PowAssign, PowerOf2};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, Zero,
};
use malachite_base::num::conversion::traits::{ExactFrom, IsInteger, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    primitive_float_pair_gen, primitive_float_unsigned_pair_gen_var_1,
    primitive_float_unsigned_pair_gen_var_4,
};
use malachite_float::arithmetic::pow::{
    primitive_float_pow, primitive_float_pow_integer, primitive_float_pow_u,
    primitive_float_rational_pow, primitive_float_unsigned_pow,
};
use malachite_float::test_util::arithmetic::pow::{
    rug_pow, rug_pow_integer, rug_pow_integer_prec, rug_pow_integer_prec_round,
    rug_pow_integer_round, rug_pow_prec, rug_pow_prec_round, rug_pow_round, rug_pow_s,
    rug_pow_s_prec, rug_pow_s_prec_round, rug_pow_s_round, rug_pow_u, rug_pow_u_prec,
    rug_pow_u_prec_round, rug_pow_u_round, rug_unsigned_pow_rational_prec_round,
};
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::generators::{
    float_float_unsigned_rounding_mode_quadruple_gen_var_9,
    float_float_unsigned_rounding_mode_quadruple_gen_var_10, float_float_unsigned_triple_gen_var_1,
    float_integer_pair_gen, float_integer_unsigned_rounding_mode_quadruple_gen_var_1,
    float_integer_unsigned_rounding_mode_quadruple_gen_var_2,
    float_integer_unsigned_triple_gen_var_1, float_pair_gen, float_pair_gen_var_10,
    float_rational_unsigned_rounding_mode_quadruple_gen_var_1,
    float_rational_unsigned_triple_gen_var_1, float_signed_pair_gen,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_11,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_12,
    float_signed_unsigned_triple_gen_var_1, float_unsigned_pair_gen,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_9,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_10,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_11,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_12,
    float_unsigned_unsigned_triple_gen_var_1,
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_2,
    unsigned_unsigned_unsigned_rounding_mode_quadruple_gen_var_1,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::integer_primitive_float_pair_gen;
use malachite_q::Rational;
use malachite_q::test_util::generators::rational_primitive_float_pair_gen;
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_pow_special_values() {
    let test = |x: Float, y: Float, out: Float, o_out: Ordering| {
        let (p, o) = x.pow_prec_round_ref_ref(&y, 10, Nearest);
        assert_eq!(ComparableFloat(p), ComparableFloat(out));
        assert_eq!(o, o_out);
    };
    let one = || Float::one_prec(10);
    // pow(x, 0) = 1 for any x, even NaN
    test(Float::NAN, Float::ZERO, one(), Equal);
    test(Float::INFINITY, Float::ZERO, one(), Equal);
    test(Float::NEGATIVE_ZERO, Float::ZERO, one(), Equal);
    test(Float::from(3.0), Float::ZERO, one(), Equal);
    // pow(+1, y) = 1 for any y, even NaN
    test(Float::ONE, Float::NAN, one(), Equal);
    test(Float::ONE, Float::INFINITY, one(), Equal);
    // NaN propagation
    test(Float::NAN, Float::ONE, Float::NAN, Equal);
    test(Float::from(2.0), Float::NAN, Float::NAN, Equal);
    // pow(-1, +/-inf) = 1
    test(-Float::ONE, Float::INFINITY, one(), Equal);
    test(-Float::ONE, Float::NEGATIVE_INFINITY, one(), Equal);
    // y = +/-inf against |x| <> 1
    test(Float::from(2.0), Float::INFINITY, Float::INFINITY, Equal);
    test(
        Float::from(2.0),
        Float::NEGATIVE_INFINITY,
        Float::ZERO,
        Equal,
    );
    test(Float::from(0.5), Float::INFINITY, Float::ZERO, Equal);
    test(
        Float::from(0.5),
        Float::NEGATIVE_INFINITY,
        Float::INFINITY,
        Equal,
    );
    // x = +/-inf
    test(Float::INFINITY, Float::from(2.0), Float::INFINITY, Equal);
    test(Float::INFINITY, Float::from(-2.0), Float::ZERO, Equal);
    test(
        Float::NEGATIVE_INFINITY,
        Float::from(3.0),
        Float::NEGATIVE_INFINITY,
        Equal,
    );
    test(
        Float::NEGATIVE_INFINITY,
        Float::from(2.0),
        Float::INFINITY,
        Equal,
    );
    test(
        Float::NEGATIVE_INFINITY,
        Float::from(-3.0),
        Float::NEGATIVE_ZERO,
        Equal,
    );
    test(
        Float::NEGATIVE_INFINITY,
        Float::from(-2.0),
        Float::ZERO,
        Equal,
    );
    // x = +/-0
    test(Float::ZERO, Float::from(3.0), Float::ZERO, Equal);
    test(
        Float::NEGATIVE_ZERO,
        Float::from(3.0),
        Float::NEGATIVE_ZERO,
        Equal,
    );
    test(Float::NEGATIVE_ZERO, Float::from(2.0), Float::ZERO, Equal);
    test(Float::ZERO, Float::from(-3.0), Float::INFINITY, Equal);
    test(
        Float::NEGATIVE_ZERO,
        Float::from(-3.0),
        Float::NEGATIVE_INFINITY,
        Equal,
    );
    test(
        Float::NEGATIVE_ZERO,
        Float::from(-2.0),
        Float::INFINITY,
        Equal,
    );
    // negative base, non-integer exponent
    test(Float::from(-2.0), Float::from(0.5), Float::NAN, Equal);
}

#[test]
fn test_pow() {
    let test = |s, s_hex, t, t_hex, prec: u64, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);
        let (p, o) = x.pow_prec_round(y, prec, rm);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);
        assert_eq!(o, o_out);
    };
    test(
        "2.0",
        "0x2.0#1",
        "0.5",
        "0x0.8#1",
        53,
        Nearest,
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        "100.0",
        "0x64.0#5",
        53,
        Nearest,
        "5.153775207320113e47",
        "0x5.a4653ca673768E+39#53",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        "10.0",
        "0xa.0#3",
        53,
        Nearest,
        "1024.0",
        "0x400.00000000000#53",
        Equal,
    );
    test(
        "0.5",
        "0x0.8#1",
        "2.0",
        "0x2.0#1",
        53,
        Nearest,
        "0.25",
        "0x0.40000000000000#53",
        Equal,
    );
    test(
        "10.0",
        "0xa.0#3",
        "-1.0",
        "-0x1.0#1",
        53,
        Nearest,
        "0.10000000000000001",
        "0x0.1999999999999a#53",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        "0.5",
        "0x0.8#1",
        53,
        Floor,
        "1.4142135623730949",
        "0x1.6a09e667f3bcc#53",
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        "1.5",
        "0x1.8#2",
        53,
        Nearest,
        "1.8371173070873836",
        "0x1.d64d51e0db1c6#53",
        Greater,
    );
    test(
        "-2.0",
        "-0x2.0#1",
        "3.0",
        "0x3.0#2",
        53,
        Nearest,
        "-8.0",
        "-0x8.0000000000000#53",
        Equal,
    );
    test(
        "-2.0",
        "-0x2.0#1",
        "-3.0",
        "-0x3.0#2",
        53,
        Nearest,
        "-0.125",
        "-0x0.20000000000000#53",
        Equal,
    );
}

// The nearest-mode underflow boundary for integer exponents: x^z landing near 2^(emin - 2) must
// choose between 0 and the minimum positive Float, which requires the pow_general fallback of
// `mpfr_pow_pos_z` (with its bottom-binade 2^k rescue and the double-rounding guard on the final
// scaling). Each case is cross-checked against MPFR via rug.
#[test]
fn test_pow_integer_underflow_boundary() {
    let emin = i64::from(Float::MIN_EXPONENT);
    let x = Float::from(0.75f64);
    let log2_x = 0.75f64.log2();
    #[allow(clippy::cast_possible_truncation)]
    let z_mid = (((emin as f64) - 1.5) / log2_x) as i64;
    for dz in [-8i64, -2, -1, 0, 1, 2, 8] {
        let z = z_mid + dz;
        let y = Float::exact_from(&malachite_nz::integer::Integer::from(z));
        let (p, o) = x.pow_prec_round_ref_ref(&y, 5, Nearest);
        let (rug_p, rug_o) = rug_pow_prec_round(
            &rug::Float::exact_from(&x),
            &rug::Float::exact_from(&y),
            5,
            rug::float::Round::Nearest,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_p)),
            ComparableFloatRef(&p)
        );
        assert_eq!(rug_o, o);
    }
}

// An exact power in the bottom binade with non-integer y: x = 25 * 2^(2j), y = 5/2, so that x^y =
// 5^5 * 2^(5j) exactly with exponent exactly MIN_EXPONENT. Exercises pow_general's bottom-binade
// 2^k rescue followed by the pow_is_exact break, which must NOT apply the final 2^k scaling (the
// exact result is already at true scale).
#[test]
fn test_pow_exact_bottom_binade() {
    let j: i64 = -214748367;
    let (x, o) = Float::from_unsigned_prec(25u32, 5);
    assert_eq!(o, Equal);
    let x = x << (2 * j);
    let y = Float::from(2.5f64);
    for prec in [12u64, 13, 20, 53] {
        let (p, o) = x.pow_prec_round_ref_ref(&y, prec, Nearest);
        assert_eq!(o, Equal);
        assert_eq!(
            i64::from(p.get_exponent().unwrap()),
            i64::from(Float::MIN_EXPONENT)
        );
        let (rug_p, rug_o) = rug_pow_prec_round(
            &rug::Float::exact_from(&x),
            &rug::Float::exact_from(&y),
            prec,
            rug::float::Round::Nearest,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_p)),
            ComparableFloatRef(&p)
        );
        assert_eq!(rug_o, o);
    }
}

// 4^(-2^29) = 2^(-2^30), the minimum positive Float exactly: the early-underflow exponent bound (ex
// - 1) * y is achieved with equality, so the nextabove bump ported from mpfr_pow is required to
// keep directed modes from misreporting a representable result as underflow.
#[test]
fn test_pow_early_underflow_bound_equality() {
    let x = Float::from(4.0f64);
    let y = -(Float::exact_from(&malachite_nz::natural::Natural::from(1u32)) << 29u32);
    for rm in [Floor, Ceiling, Down, Up, Nearest, Exact] {
        let (p, o) = x.pow_prec_round_ref_ref(&y, 10, rm);
        assert_eq!(o, Equal);
        assert_eq!(
            i64::from(p.get_exponent().unwrap()),
            i64::from(Float::MIN_EXPONENT)
        );
        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_p, rug_o) = rug_pow_prec_round(
                &rug::Float::exact_from(&x),
                &rug::Float::exact_from(&y),
                10,
                rug_rm,
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_p)),
                ComparableFloatRef(&p)
            );
            assert_eq!(rug_o, o);
        }
    }
}

// Branch-coverage cases discovered by instrumenting every interesting branch of pow.rs and running
// the unit and property suites plus handcrafted probes (the porting workflow's manual coverage
// step). Each case is annotated with the branch it exercises and was verified against MPFR via rug
// when generated.
#[test]
fn test_pow_coverage() {
    let test = |s, s_hex, t, t_hex, prec: u64, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);
        let (p, o) = x.pow_prec_round(y, prec, rm);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);
        assert_eq!(o, o_out);
    };
    // - e_tiny + tiny_one_above: y*log2(x) tiny, Nearest rounds to exactly 1
    test(
        "1.5",
        "0x1.8#2",
        "6.0e-61",
        "0x1.0E-50#1",
        10,
        Nearest,
        "1.0",
        "0x1.000#10",
        Less,
    );
    // - tiny_up_above: 1+ulp for Up when the true result is above 1
    test(
        "1.5",
        "0x1.8#2",
        "6.0e-61",
        "0x1.0E-50#1",
        10,
        Up,
        "1.002",
        "0x1.008#10",
        Greater,
    );
    // - tiny_down_below: 1-ulp for Floor when the true result is below 1
    test(
        "1.5",
        "0x1.8#2",
        "-6.0e-61",
        "-0x1.0E-50#1",
        10,
        Floor,
        "0.999",
        "0x0.ffc#10",
        Less,
    );
    // - tiny_one_below: Nearest rounds to exactly 1 from below
    test(
        "1.5",
        "0x1.8#2",
        "-6.0e-61",
        "-0x1.0E-50#1",
        10,
        Nearest,
        "1.0",
        "0x1.000#10",
        Greater,
    );
    // - pg_neg_result: negative x with huge odd integer y routes through pow_general
    test(
        "-1.0000000000000000000000000000000000000000000000000000000000000000000000000000000000005",
        "-0x1.0000000000000000000000000000000000000000000000000000000000000000000001#281",
        "2037035976334486086268445688409378161051468393665936250636140449354381299763336706183397\
         377.0",
        "0x1000000000000000000000000000000000000000000000000000000000000000000000000001.0#301",
        10,
        Nearest,
        "-too_big",
        "-0xa.84E+378193#10",
        Greater,
    );
    // - pi_pow2_underflow: (2^b)^z with result exponent below the range
    test(
        "2.0",
        "0x2.0#1",
        "-1073741825.0",
        "-0x40000001.0#31",
        5,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );
    // - pg_overflow_spurious + pg_k_subtract + pg_shl_k: exp upper bound overflows, 2^k rescue
    test(
        "3.0",
        "0x3.0#2",
        "677453079.5",
        "0x28611d17.8#32",
        2,
        Nearest,
        "too_big",
        "0x8.0E+268434431#2",
        Less,
    );
    // - pie_y_neg: pow_is_exact bails on negative y
    test(
        "3.0",
        "0x3.0#2",
        "-1048576.5",
        "-0x100000.8#22",
        10,
        Nearest,
        "too_small",
        "0x2.f7E-415489#10",
        Less,
    );
    // - pie_b_odd_shift + pie_sqrt_fail: odd exponent shift, then non-square mantissa
    test(
        "1.5",
        "0x1.8#2",
        "1048576.5",
        "0x100000.8#22",
        10,
        Nearest,
        "too_big",
        "0x3.d1E+153344#10",
        Greater,
    );
    // - pi_pos_zero_directed: deep positive-exponent underflow, Floor gives 0
    test(
        "0.8",
        "0x0.c#2",
        "2587095930.0",
        "0x9a33f37a.0#31",
        10,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    // - pi_pos_zero_directed: deep positive-exponent underflow, Up gives min positive
    test(
        "0.8",
        "0x0.c#2",
        "2587095930.0",
        "0x9a33f37a.0#31",
        10,
        Up,
        "too_small",
        "0x1.000E-268435456#10",
        Greater,
    );
    // - pi_neg_zero_nearest + pp_saturation_zeroed + pp_noncr: negative-z deep underflow, Nearest
    test(
        "1.5",
        "0x1.8#2",
        "-1835573822.0",
        "-0x6d68a23e.0#30",
        10,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );
    // - pi_neg_zero_directed: negative-z deep underflow, Floor
    test(
        "1.5",
        "0x1.8#2",
        "-1835573822.0",
        "-0x6d68a23e.0#30",
        10,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    // - pi_prebound_underflow: estimate far below the exponent range
    test(
        "0.8",
        "0x0.c#2",
        "9.0e9",
        "0x2.0E+8#1",
        10,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );
}

#[allow(clippy::needless_pass_by_value)]
fn pow_prec_round_properties_helper(
    x: Float,
    y: Float,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) {
    if rm == Exact {
        // Exact is only allowed when the result is exactly representable; otherwise panic.
        let (p, o) = x.pow_prec_round_ref_ref(&y, prec, Nearest);
        if o == Equal {
            let (pe, oe) = x.pow_prec_round_ref_ref(&y, prec, Exact);
            assert_eq!(ComparableFloatRef(&pe), ComparableFloatRef(&p));
            assert_eq!(oe, Equal);
        } else {
            assert_panic!(x.pow_prec_round_ref_ref(&y, prec, Exact));
        }
        return;
    }
    let (p, o) = x.clone().pow_prec_round(y.clone(), prec, rm);
    assert!(p.is_valid());
    let (p_alt, o_alt) = x.clone().pow_prec_round_val_ref(&y, prec, rm);
    assert!(p_alt.is_valid());
    assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
    assert_eq!(o_alt, o);
    let (p_alt, o_alt) = x.pow_prec_round_ref_val(y.clone(), prec, rm);
    assert!(p_alt.is_valid());
    assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
    assert_eq!(o_alt, o);
    let (p_alt, o_alt) = x.pow_prec_round_ref_ref(&y, prec, rm);
    assert!(p_alt.is_valid());
    assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.pow_prec_round_assign(y.clone(), prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&p));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.pow_prec_round_assign_ref(&y, prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&p));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_p, rug_o) = rug_pow_prec_round(
            &rug::Float::exact_from(&x),
            &rug::Float::exact_from(&y),
            prec,
            rug_rm,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_p)),
            ComparableFloatRef(&p),
        );
        assert_eq!(rug_o, o);
    }

    // x < 0 with non-integer finite y gives NaN
    if x.is_normal() && x.is_sign_negative() && y.is_normal() && !(&y).is_integer() {
        assert!(p.is_nan());
    }
    if p.is_normal() && !extreme {
        assert_eq!(p.get_prec(), Some(prec));
    }
}

#[test]
fn pow_prec_round_properties() {
    float_float_unsigned_rounding_mode_quadruple_gen_var_9().test_properties(|(x, y, prec, rm)| {
        pow_prec_round_properties_helper(x, y, prec, rm, false);
    });

    float_float_unsigned_rounding_mode_quadruple_gen_var_10().test_properties(
        |(x, y, prec, rm)| {
            pow_prec_round_properties_helper(x, y, prec, rm, true);
        },
    );
}

#[test]
fn pow_prec_properties() {
    float_float_unsigned_triple_gen_var_1().test_properties(|(x, y, prec)| {
        let (p, o) = x.clone().pow_prec(y.clone(), prec);
        assert!(p.is_valid());
        let (p_alt, o_alt) = x.pow_prec_ref_ref(&y, prec);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
        assert_eq!(o_alt, o);
        let (p_alt, o_alt) = x.pow_prec_round_ref_ref(&y, prec, Nearest);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
        assert_eq!(o_alt, o);
        let (rug_p, rug_o) = rug_pow_prec(
            &rug::Float::exact_from(&x),
            &rug::Float::exact_from(&y),
            prec,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_p)),
            ComparableFloatRef(&p),
        );
        assert_eq!(rug_o, o);
    });
}

#[test]
fn pow_properties() {
    float_pair_gen().test_properties(|(x, y)| {
        let p = x.clone().pow(y.clone());
        assert!(p.is_valid());
        let p_alt = x.clone().pow(&y);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
        let p_alt = (&x).pow(y.clone());
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
        let p_alt = (&x).pow(&y);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));

        let mut x_alt = x.clone();
        x_alt.pow_assign(y.clone());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&p));
        let mut x_alt = x.clone();
        x_alt.pow_assign(&y);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&p));

        let rug_p = rug_pow(&rug::Float::exact_from(&x), &rug::Float::exact_from(&y));
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_p)),
            ComparableFloatRef(&p),
        );

        // x^1 == x (rounded to the working precision)
        let prec = x.significant_bits().max(y.significant_bits());
        let (p1, _) = x.pow_prec_round_ref_val(Float::ONE, prec, Nearest);
        let (x_rounded, _) = Float::from_float_prec_round_ref(&x, prec, Nearest);
        assert_eq!(ComparableFloatRef(&p1), ComparableFloatRef(&x_rounded));
    });

    float_pair_gen_var_10().test_properties(|(x, y)| {
        let p = (&x).pow(&y);
        assert!(p.is_valid());
        let rug_p = rug_pow(&rug::Float::exact_from(&x), &rug::Float::exact_from(&y));
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_p)),
            ComparableFloatRef(&p),
        );
    });
}

#[test]
fn pow_round_properties() {
    float_pair_gen().test_properties(|(x, y)| {
        for rm in [Floor, Ceiling, Down, Up, Nearest] {
            let (p, o) = x.clone().pow_round(y.clone(), rm);
            assert!(p.is_valid());
            let (p_alt, o_alt) = x.pow_round_ref_ref(&y, rm);
            assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
            assert_eq!(o_alt, o);
            if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
                let (rug_p, rug_o) = rug_pow_round(
                    &rug::Float::exact_from(&x),
                    &rug::Float::exact_from(&y),
                    rug_rm,
                );
                assert_eq!(
                    ComparableFloatRef(&Float::from(&rug_p)),
                    ComparableFloatRef(&p),
                );
                assert_eq!(rug_o, o);
            }
        }
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_pow() {
    fn test<T: PrimitiveFloat>(x: T, y: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_pow(x, y)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN, f32::NAN);
    test::<f32>(f32::NAN, f32::INFINITY, f32::NAN);
    test::<f32>(f32::NAN, f32::NEGATIVE_INFINITY, f32::NAN);
    test::<f32>(f32::NAN, 0.0, 1.0);
    test::<f32>(f32::NAN, -0.0, 1.0);
    test::<f32>(f32::NAN, 1.0, f32::NAN);
    test::<f32>(f32::NAN, -1.0, f32::NAN);
    test::<f32>(f32::NAN, 2.0, f32::NAN);
    test::<f32>(f32::NAN, -3.0, f32::NAN);

    test::<f32>(f32::INFINITY, f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, f32::INFINITY, f32::INFINITY);
    test::<f32>(f32::INFINITY, f32::NEGATIVE_INFINITY, 0.0);
    test::<f32>(f32::INFINITY, 0.0, 1.0);
    test::<f32>(f32::INFINITY, -0.0, 1.0);
    test::<f32>(f32::INFINITY, 1.0, f32::INFINITY);
    test::<f32>(f32::INFINITY, -1.0, 0.0);
    test::<f32>(f32::INFINITY, 2.0, f32::INFINITY);
    test::<f32>(f32::INFINITY, -3.0, 0.0);

    test::<f32>(f32::NEGATIVE_INFINITY, f32::NAN, f32::NAN);
    test::<f32>(f32::NEGATIVE_INFINITY, f32::INFINITY, f32::INFINITY);
    test::<f32>(f32::NEGATIVE_INFINITY, f32::NEGATIVE_INFINITY, 0.0);
    test::<f32>(f32::NEGATIVE_INFINITY, 0.0, 1.0);
    test::<f32>(f32::NEGATIVE_INFINITY, -0.0, 1.0);
    test::<f32>(f32::NEGATIVE_INFINITY, 1.0, f32::NEGATIVE_INFINITY);
    test::<f32>(f32::NEGATIVE_INFINITY, -1.0, -0.0);
    test::<f32>(f32::NEGATIVE_INFINITY, 2.0, f32::INFINITY);
    test::<f32>(f32::NEGATIVE_INFINITY, -3.0, -0.0);

    test::<f32>(0.0, f32::NAN, f32::NAN);
    test::<f32>(0.0, f32::INFINITY, 0.0);
    test::<f32>(0.0, f32::NEGATIVE_INFINITY, f32::INFINITY);
    test::<f32>(0.0, 0.0, 1.0);
    test::<f32>(0.0, -0.0, 1.0);
    test::<f32>(0.0, 1.0, 0.0);
    test::<f32>(0.0, -1.0, f32::INFINITY);
    test::<f32>(0.0, 2.0, 0.0);
    test::<f32>(0.0, -3.0, f32::INFINITY);

    test::<f32>(-0.0, f32::NAN, f32::NAN);
    test::<f32>(-0.0, f32::INFINITY, 0.0);
    test::<f32>(-0.0, f32::NEGATIVE_INFINITY, f32::INFINITY);
    test::<f32>(-0.0, 0.0, 1.0);
    test::<f32>(-0.0, -0.0, 1.0);
    test::<f32>(-0.0, 1.0, -0.0);
    test::<f32>(-0.0, -1.0, f32::NEGATIVE_INFINITY);
    test::<f32>(-0.0, 2.0, 0.0);
    test::<f32>(-0.0, -3.0, f32::NEGATIVE_INFINITY);

    test::<f32>(1.0, f32::NAN, 1.0);
    test::<f32>(1.0, f32::INFINITY, 1.0);
    test::<f32>(1.0, f32::NEGATIVE_INFINITY, 1.0);
    test::<f32>(1.0, 0.0, 1.0);
    test::<f32>(1.0, -0.0, 1.0);
    test::<f32>(1.0, 1.0, 1.0);
    test::<f32>(1.0, -1.0, 1.0);
    test::<f32>(1.0, 2.0, 1.0);
    test::<f32>(1.0, -3.0, 1.0);

    test::<f32>(-1.0, f32::NAN, f32::NAN);
    test::<f32>(-1.0, f32::INFINITY, 1.0);
    test::<f32>(-1.0, f32::NEGATIVE_INFINITY, 1.0);
    test::<f32>(-1.0, 0.0, 1.0);
    test::<f32>(-1.0, -0.0, 1.0);
    test::<f32>(-1.0, 1.0, -1.0);
    test::<f32>(-1.0, -1.0, -1.0);
    test::<f32>(-1.0, 2.0, 1.0);
    test::<f32>(-1.0, -3.0, -1.0);

    test::<f32>(2.0, f32::NAN, f32::NAN);
    test::<f32>(2.0, f32::INFINITY, f32::INFINITY);
    test::<f32>(2.0, f32::NEGATIVE_INFINITY, 0.0);
    test::<f32>(2.0, 0.0, 1.0);
    test::<f32>(2.0, -0.0, 1.0);
    test::<f32>(2.0, 1.0, 2.0);
    test::<f32>(2.0, -1.0, 0.5);
    test::<f32>(2.0, 2.0, 4.0);
    test::<f32>(2.0, -3.0, 0.125);

    test::<f32>(-3.0, f32::NAN, f32::NAN);
    test::<f32>(-3.0, f32::INFINITY, f32::INFINITY);
    test::<f32>(-3.0, f32::NEGATIVE_INFINITY, 0.0);
    test::<f32>(-3.0, 0.0, 1.0);
    test::<f32>(-3.0, -0.0, 1.0);
    test::<f32>(-3.0, 1.0, -3.0);
    test::<f32>(-3.0, -1.0, -0.33333334);
    test::<f32>(-3.0, 2.0, 9.0);
    test::<f32>(-3.0, -3.0, -0.037037037);

    test::<f32>(3.0, 2.5, 15.588457);
    test::<f32>(2.0, 0.5, core::f32::consts::SQRT_2);
    test::<f32>(1.5, 100.0, 4.065612e17);
    // overflow and underflow
    test::<f32>(2.0, 128.0, f32::INFINITY);
    test::<f32>(2.0, -150.0, 0.0);

    test::<f64>(3.0, 2.5, 15.588457268119896);
    test::<f64>(2.0, 0.5, core::f64::consts::SQRT_2);
    test::<f64>(10.0, -0.5, 0.31622776601683794);
    test::<f64>(0.5, 0.5, core::f64::consts::FRAC_1_SQRT_2);
    test::<f64>(1.5, 100.0, 4.065611775352152e17);
    test::<f64>(0.9999999999, 10000000000.0, 0.36787941071456814);
    test::<f64>(-2.0, 3.0, -8.0);
    test::<f64>(-2.0, 0.5, f64::NAN);
    // overflow and underflow
    test::<f64>(1.0e300, 2.0, f64::INFINITY);
    test::<f64>(1.0e-300, 2.0, 0.0);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_pow_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_pair_gen::<T>().test_properties(|(x, y)| {
        primitive_float_pow(x, y);
    });
}

#[test]
fn primitive_float_pow_properties() {
    apply_fn_to_primitive_floats!(primitive_float_pow_properties_helper);
}

// A `Float` base that is a sliver of 1 -- within a couple of binades of the smallest positive
// `Float`, requiring a precision near 2^30 -- has a logarithm below the smallest positive `Float`.
// Such a base once made `Float::pow` panic (its `ln` underflowed on `x - 1`); it is now delegated
// to the exact-`Rational` power. These small-exponent cases take the tiny-result shortcut (x^y
// rounds to 1 +/- ulp), so they avoid the 128-MB log2 brackets and stay cheap. Run under
// `--release`.
#[test]
fn test_pow_sliver_of_one() {
    let p = u64::try_from(-i64::from(Float::MIN_EXPONENT) + 3).unwrap();
    let eps = Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2);
    let sliver = |xr: Rational| Float::from_rational_prec_round(xr, p, Exact).0;
    let test = |x: Float, y: Float, out: &str, out_hex: &str, o_out| {
        let (r, o) = x.pow_prec_round_ref_ref(&y, 64, Nearest);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(to_hex_string(&r), out_hex);
        assert_eq!(o, o_out);
    };
    let y = Float::power_of_2(60i64);
    // 1 + 2^(MIN_EXPONENT - 2), just above 1
    test(
        sliver(Rational::ONE + &eps),
        y.clone(),
        "1.0",
        "0x1.0000000000000000#64",
        Less,
    );
    // 1 - 2^(MIN_EXPONENT - 2), just below 1
    test(
        sliver(Rational::ONE - &eps),
        y.clone(),
        "1.0",
        "0x1.0000000000000000#64",
        Greater,
    );
    // negative base, even integer exponent
    test(
        sliver(-(Rational::ONE + &eps)),
        y,
        "1.0",
        "0x1.0000000000000000#64",
        Less,
    );
    // negative base, odd integer exponent
    let y_odd =
        Float::from_rational_prec_round(Rational::power_of_2(60i64) + Rational::ONE, 61, Exact).0;
    test(
        sliver(-(Rational::ONE + &eps)),
        y_odd,
        "-1.0",
        "-0x1.0000000000000000#64",
        Greater,
    );
}

#[test]
fn test_rational_pow() {
    let test = |s: &str,
                t: &str,
                t_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out| {
        let x = Rational::from_str(s).unwrap();
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);
        let (p, o) = Float::rational_pow_prec_round_ref_ref(&x, &y, prec, rm);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);
        assert_eq!(o, o_out);
    };
    // - ordinary squeeze: non-dyadic base
    test(
        "3/2",
        "0.5",
        "0x0.8#1",
        20,
        Nearest,
        "1.224745",
        "0x1.3988e#20",
        Less,
    );
    // - ordinary squeeze, directed
    test(
        "2/3",
        "2.5",
        "0x2.8#3",
        20,
        Floor,
        "0.3628869",
        "0x0.5ce628#20",
        Less,
    );
    // - negative base, odd integer y
    test(
        "-3/2",
        "3.0",
        "0x3.0#2",
        20,
        Nearest,
        "-3.375",
        "-0x3.60000#20",
        Equal,
    );
    // - negative base, non-integer y is NaN
    test("-3/2", "0.5", "0x0.8#1", 20, Nearest, "NaN", "NaN", Equal);
    // - exact dyadic result via descent: (9/4)^(1/2) = 3/2
    test(
        "9/4",
        "0.5",
        "0x0.8#1",
        20,
        Nearest,
        "1.5",
        "0x1.80000#20",
        Equal,
    );
    // - power-of-2 base delegates to power_of_2_rational
    test(
        "1/4",
        "0.8",
        "0x0.c#2",
        20,
        Nearest,
        "0.3535533",
        "0x0.5a8278#20",
        Less,
    );
    // - small integer y materializes exactly
    test(
        "3/7",
        "4.0e1",
        "0x28.0#3",
        30,
        Nearest,
        "1.909539244e-15",
        "0x8.998c820E-13#30",
        Greater,
    );
    // - dyadic in-range base delegates to Float::pow
    test(
        "5/4",
        "0.5",
        "0x0.8#1",
        20,
        Nearest,
        "1.118034",
        "0x1.1e378#20",
        Greater,
    );
    // - deep underflow, Nearest
    test(
        "2/3",
        "1.0e12",
        "0x1.0E+10#1",
        32,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );
    // - deep underflow, Up gives the minimum positive value
    test(
        "2/3",
        "1.0e12",
        "0x1.0E+10#1",
        32,
        Up,
        "too_small",
        "0x1.00000000E-268435456#32",
        Greater,
    );

    // Bracket ends that land exactly on a representable power in the x-space squeeze: a base just
    // below a perfect power makes the upper bracket round to it (sqx_hi_eq), and just above makes
    // the lower bracket round to it (sqx_lo_eq).
    let test2 = |x: Rational, t_hex: &str, prec: u64, out_hex: &str, o_out| {
        let y = parse_hex_string(t_hex);
        let (r, o) = Float::rational_pow_prec_round_ref_ref(&x, &y, prec, Nearest);
        assert!(r.is_valid());
        assert_eq!(to_hex_string(&r), out_hex);
        assert_eq!(o, o_out);
    };
    let tiny = Rational::from_unsigneds(1u32, 3u32).pow(100i64);
    // - sqx_hi_eq: (4 - 3^-100)^(1/2), upper bracket rounds to 4 so 4^(1/2) = 2 is exact
    test2(
        Rational::from(4u32) - &tiny,
        "0x0.8#1",
        5,
        "0x2.0#5",
        Greater,
    );
    // - sqx_lo_eq: (9 + 3^-100)^(1/2), lower bracket rounds to 9 so 9^(1/2) = 3 is exact
    test2(Rational::from(9u32) + &tiny, "0x0.8#1", 5, "0x3.0#5", Less);
}

// Bases at or beyond the Float exponent range, and bases so close to 1 that their logarithm is at
// or below the smallest positive Float: the regimes that need the exact-Rational t-space squeeze,
// the dyadic-result descent, or the sliver reroute. Inputs are constructed from
// `Float::MAX_EXPONENT` and `Float::MIN_EXPONENT` (their Rational forms occupy ~128 MB), as
// elsewhere in the extreme tests; expected values were generated through the implementation and
// cross-checked by hand against the closed forms in the comments. Run under `--release`.
#[test]
fn test_rational_pow_extreme() {
    let test = |x: &Rational,
                t: &str,
                t_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out| {
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);
        let (p, o) = Float::rational_pow_prec_round_ref_ref(x, &y, prec, rm);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);
        assert_eq!(o, o_out);
    };
    let e = i64::from(Float::MAX_EXPONENT) + 1;
    let mi = i64::from(Float::MIN_EXPONENT);
    // - (3 * 2^(MAX_EXPONENT + 1))^(1/4) = 3^(1/4) * 2^(2^28): base beyond the range, in-range
    //   inexact result via the directed-log t-squeeze
    test(
        &(Rational::from(3u32) * Rational::power_of_2(e)),
        "0.2",
        "0x0.4#1",
        20,
        Nearest,
        "too_big",
        "0x1.50ea4E+67108864#20",
        Greater,
    );
    // - (81 * 2^(MAX_EXPONENT + 1))^(1/4) = 3 * 2^(2^28): base beyond the range, exact via the
    //   square-root descent
    test(
        &(Rational::from(81u32) * Rational::power_of_2(e)),
        "0.2",
        "0x0.4#1",
        20,
        Nearest,
        "too_big",
        "0x3.00000E+67108864#20",
        Equal,
    );
    // - (5 / 2^(MAX_EXPONENT + 1))^(-1/2) = 5^(-1/2) * 2^(2^30): tiny base, negative power,
    //   in-range result
    test(
        &(Rational::from(5u32) * Rational::power_of_2(-e)),
        "-0.5",
        "-0x0.8#1",
        20,
        Nearest,
        "too_big",
        "0x7.27c98E+134217727#20",
        Greater,
    );
    // - (1 - 2^(MIN_EXPONENT - 2))^(2^60): a sliver of 1 from below, whose Float logarithm would
    //   underflow; the sliver reroute sends it to the t-squeeze (via the directed branch on 2x)
    test(
        &(Rational::ONE - Rational::power_of_2(mi - 2)),
        "1.0e18",
        "0x1.0E+15#1",
        64,
        Nearest,
        "1.0",
        "0x1.0000000000000000#64",
        Greater,
    );
    // - ((1 + 3^-500) * 2^(MAX_EXPONENT + 1))^(1/4): base beyond the range with a mantissa a sliver
    //   above 1, exercising the atanh-series log2 brackets
    test(
        &((Rational::ONE + Rational::from_unsigneds(1u32, 3u32).pow(500i64))
            * Rational::power_of_2(e)),
        "0.2",
        "0x0.4#1",
        20,
        Nearest,
        "too_big",
        "0x1.00000E+67108864#20",
        Less,
    );
    // - (1 + 2^-70000)^(2^60): a moderate dyadic sliver, handled on the Float::pow path (its
    //   logarithm is well within range)
    test(
        &(Rational::ONE + Rational::power_of_2(-70000i64)),
        "1.0e18",
        "0x1.0E+15#1",
        64,
        Nearest,
        "1.0",
        "0x1.0000000000000000#64",
        Less,
    );
    // - (1 + 3^-500)^(2^700): a moderate non-dyadic sliver, in-range x-space squeeze
    test(
        &(Rational::ONE + Rational::from_unsigneds(1u32, 3u32).pow(500i64)),
        "5.0e210",
        "0x1.0E+175#1",
        64,
        Nearest,
        "1.0",
        "0x1.0000000000000000#64",
        Less,
    );
    // - ((1 - 3^-500) * 2^(MAX_EXPONENT + 1))^(1/4): base beyond the range with a mantissa a sliver
    //   BELOW 1, exercising the signed (negative-argument) atanh-series log2 brackets
    test(
        &((Rational::ONE - Rational::from_unsigneds(1u32, 3u32).pow(500i64))
            * Rational::power_of_2(e)),
        "0.2",
        "0x0.4#1",
        20,
        Nearest,
        "too_big",
        "0x1.00000E+67108864#20",
        Greater,
    );
}

// Regression test: `rational_pow_exact`'s representability rejection formerly used an upper bound
// on sb(m^z) where soundness requires a lower bound, so exactly-representable results and `Nearest`
// ties with sb(m) * z > prec + 2 > sb(m^z) leaked past the exact route into the squeeze, which
// never terminates on them.
#[test]
fn test_rational_pow_exact_bound_regression() {
    // - a `Nearest` tie that leaked: (4/17^4)^(-3/2) = 17^6/8, whose odd part has 25 = prec + 1
    //   bits (formerly hung)
    let x = Rational::from_unsigneds(4u32, 83521u32);
    let y = Float::from(-1.5);
    let exact = Rational::from_unsigneds(24137569u32, 8u32);
    for prec in [21u64, 24, 25, 30] {
        for rm in [Floor, Ceiling, Down, Up, Nearest] {
            let (p, o) = Float::rational_pow_prec_round_ref_ref(&x, &y, prec, rm);
            let (ep, eo) = Float::from_rational_prec_round(exact.clone(), prec, rm);
            assert_eq!(ComparableFloatRef(&p), ComparableFloatRef(&ep));
            assert_eq!(o, eo);
        }
    }
    // - an exactly representable value that leaked: (4/17^2)^(-5/2) = 17^5/32 with a 21-bit odd
    //   part at prec 21; `Exact` formerly hung instead of succeeding
    let x = Rational::from_unsigneds(4u32, 289u32);
    let y = Float::from(-2.5);
    let exact = Rational::from_unsigneds(1419857u32, 32u32);
    for prec in [21u64, 25] {
        let (p, o) = Float::rational_pow_prec_round_ref_ref(&x, &y, prec, Exact);
        let (ep, _) = Float::from_rational_prec_round(exact.clone(), prec, Exact);
        assert_eq!(ComparableFloatRef(&p), ComparableFloatRef(&ep));
        assert_eq!(o, Equal);
    }
    // - the same leak in the extreme regime, reaching the shared `pow_squeeze_t` instead: (17^4 *
    //   2^(-2^30 - 100))^(3/4) = 17^3 * 2^(-805306443), a tie at prec 12 (formerly hung)
    let e = -(1i64 << 30) - 100;
    let x = Rational::from(83521u32) << e;
    let y = Float::from(0.75);
    for prec in [12u64, 13, 20] {
        for rm in [Floor, Ceiling, Down, Up, Nearest] {
            let (p, o) = Float::rational_pow_prec_round_ref_ref(&x, &y, prec, rm);
            let (ep, eo) = Float::from(4913u32).shl_prec_round(3 * e / 4, prec, rm);
            assert_eq!(ComparableFloatRef(&p), ComparableFloatRef(&ep));
            assert_eq!(o, eo);
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn rational_pow_prec_round_properties_helper(x: Rational, y: Float, prec: u64, rm: RoundingMode) {
    if rm == Exact {
        let (p, o) = Float::rational_pow_prec_round_ref_ref(&x, &y, prec, Nearest);
        if o == Equal {
            let (pe, oe) = Float::rational_pow_prec_round_ref_ref(&x, &y, prec, Exact);
            assert_eq!(ComparableFloatRef(&pe), ComparableFloatRef(&p));
            assert_eq!(oe, Equal);
        } else {
            assert_panic!(Float::rational_pow_prec_round_ref_ref(&x, &y, prec, Exact));
        }
        return;
    }
    let (p, o) = Float::rational_pow_prec_round_ref_ref(&x, &y, prec, rm);
    assert!(p.is_valid());
    let (p_alt, o_alt) = Float::rational_pow_prec_round(x.clone(), y.clone(), prec, rm);
    assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
    assert_eq!(o_alt, o);
    let (p_alt, o_alt) = Float::rational_pow_prec_round_val_ref(x.clone(), &y, prec, rm);
    assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
    assert_eq!(o_alt, o);
    let (p_alt, o_alt) = Float::rational_pow_prec_round_ref_val(&x, y.clone(), prec, rm);
    assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
    assert_eq!(o_alt, o);

    // A dyadic base that converts exactly must agree with Float::pow.
    if x != 0u32 && y.is_finite() {
        let bits = x.numerator_ref().significant_bits() + x.denominator_ref().significant_bits();
        if x.denominator_ref().is_power_of_2() && bits < 10000 {
            let xf = Float::from_rational_prec_round_ref(&x, bits, Floor).0;
            if Rational::exact_from(&xf) == x {
                let (p_alt, o_alt) = xf.pow_prec_round_val_ref(&y, prec, rm);
                assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
                assert_eq!(o_alt, o);
            }
        }
    }
    // A small integer exponent must agree with exact Rational powering.
    if y.is_finite()
        && !y.is_zero()
        && (&y).is_integer()
        && y.significant_bits() < 20
        && let Ok(z) = i64::try_from(&Integer::rounding_from(&y, Nearest).0)
        && z.unsigned_abs() < 100
        && x != 0u32
    {
        let (p_alt, o_alt) = Float::from_rational_prec_round((&x).pow(z), prec, rm);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
        assert_eq!(o_alt, o);
    }
    // Exactness implies rounding-mode invariance.
    if o == Equal && p.is_normal() {
        for rm2 in [Floor, Ceiling, Down, Up, Nearest] {
            let (p_alt, o_alt) = Float::rational_pow_prec_round_ref_ref(&x, &y, prec, rm2);
            assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
            assert_eq!(o_alt, Equal);
        }
    }
    // Negative base with non-integer finite y is NaN.
    if x < 0u32 && y.is_normal() && !(&y).is_integer() {
        assert!(p.is_nan());
    }
    if p.is_normal() {
        assert_eq!(p.get_prec(), Some(prec));
    }
}

#[test]
fn rational_pow_prec_round_properties() {
    float_rational_unsigned_rounding_mode_quadruple_gen_var_1().test_properties(
        |(y, x, prec, rm)| {
            rational_pow_prec_round_properties_helper(x, y, prec, rm);
        },
    );
}

#[test]
fn rational_pow_prec_properties() {
    float_rational_unsigned_triple_gen_var_1::<u64>().test_properties(|(y, x, prec)| {
        let (p, o) = Float::rational_pow_prec_ref_ref(&x, &y, prec);
        assert!(p.is_valid());
        let (p_alt, o_alt) = Float::rational_pow_prec(x.clone(), y.clone(), prec);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
        assert_eq!(o_alt, o);
        let (p_alt, o_alt) = Float::rational_pow_prec_val_ref(x.clone(), &y, prec);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
        assert_eq!(o_alt, o);
        let (p_alt, o_alt) = Float::rational_pow_prec_ref_val(&x, y.clone(), prec);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
        assert_eq!(o_alt, o);
        let (p_alt, o_alt) = Float::rational_pow_prec_round_ref_ref(&x, &y, prec, Nearest);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
        assert_eq!(o_alt, o);
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_rational_pow() {
    fn test<T: PrimitiveFloat>(s: &str, y: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        let x = Rational::from_str(s).unwrap();
        assert_eq!(
            NiceFloat(primitive_float_rational_pow(&x, y)),
            NiceFloat(out)
        );
    }
    test::<f32>("0", f32::NAN, f32::NAN);
    test::<f32>("0", f32::INFINITY, 0.0);
    test::<f32>("0", f32::NEGATIVE_INFINITY, f32::INFINITY);
    test::<f32>("0", 0.0, 1.0);
    test::<f32>("0", -0.0, 1.0);
    test::<f32>("0", 2.0, 0.0);
    test::<f32>("0", -3.0, f32::INFINITY);
    test::<f32>("1", f32::NAN, 1.0);
    test::<f32>("1", f32::INFINITY, 1.0);
    test::<f32>("1", f32::NEGATIVE_INFINITY, 1.0);
    test::<f32>("1", 0.0, 1.0);
    test::<f32>("1", -0.0, 1.0);
    test::<f32>("1", 2.0, 1.0);
    test::<f32>("1", -3.0, 1.0);
    test::<f32>("-1", f32::NAN, f32::NAN);
    test::<f32>("-1", f32::INFINITY, 1.0);
    test::<f32>("-1", f32::NEGATIVE_INFINITY, 1.0);
    test::<f32>("-1", 0.0, 1.0);
    test::<f32>("-1", -0.0, 1.0);
    test::<f32>("-1", 2.0, 1.0);
    test::<f32>("-1", -3.0, -1.0);
    test::<f32>("2", f32::NAN, f32::NAN);
    test::<f32>("2", f32::INFINITY, f32::INFINITY);
    test::<f32>("2", f32::NEGATIVE_INFINITY, 0.0);
    test::<f32>("2", 0.0, 1.0);
    test::<f32>("2", -0.0, 1.0);
    test::<f32>("2", 2.0, 4.0);
    test::<f32>("2", -3.0, 0.125);
    test::<f32>("1/2", f32::NAN, f32::NAN);
    test::<f32>("1/2", f32::INFINITY, 0.0);
    test::<f32>("1/2", f32::NEGATIVE_INFINITY, f32::INFINITY);
    test::<f32>("1/2", 0.0, 1.0);
    test::<f32>("1/2", -0.0, 1.0);
    test::<f32>("1/2", 2.0, 0.25);
    test::<f32>("1/2", -3.0, 8.0);

    test::<f64>("3/2", 2.5, 2.7556759606310752);
    test::<f64>("2/3", -2.5, 2.7556759606310752);
    test::<f64>("9/4", 0.5, 1.5);
    test::<f64>("1/4", 0.75, 0.3535533905932738);
    test::<f64>("-3/2", 3.0, -3.375);
    test::<f64>("-3/2", 0.5, f64::NAN);
    // overflow and underflow
    test::<f64>("3/2", 1000.0, 1.2338405969061735e176);
    test::<f64>("2/3", 1000.0, 8.104774656527566e-177);
    test::<f32>("3/2", 2.5, 2.755676);
    test::<f32>("2", 200.0, f32::INFINITY);
    test::<f32>("1/2", 200.0, 0.0);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_rational_pow_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_primitive_float_pair_gen::<T>().test_properties(|(x, y)| {
        primitive_float_rational_pow::<T>(&x, y);
    });
}

#[test]
fn primitive_float_rational_pow_properties() {
    apply_fn_to_primitive_floats!(primitive_float_rational_pow_properties_helper);
}

#[allow(clippy::needless_pass_by_value)]
fn pow_integer_prec_round_properties_helper(
    x: Float,
    z: Integer,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) {
    if rm == Exact {
        // Exact is only allowed when the result is exactly representable; otherwise panic.
        let (p, o) = x.pow_integer_prec_round_ref_ref(&z, prec, Nearest);
        if o == Equal {
            let (pe, oe) = x.pow_integer_prec_round_ref_ref(&z, prec, Exact);
            assert_eq!(ComparableFloatRef(&pe), ComparableFloatRef(&p));
            assert_eq!(oe, Equal);
        } else {
            assert_panic!(x.pow_integer_prec_round_ref_ref(&z, prec, Exact));
        }
        return;
    }
    let (p, o) = x.clone().pow_integer_prec_round(z.clone(), prec, rm);
    assert!(p.is_valid());
    let (p_alt, o_alt) = x.clone().pow_integer_prec_round_val_ref(&z, prec, rm);
    assert!(p_alt.is_valid());
    assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
    assert_eq!(o_alt, o);
    let (p_alt, o_alt) = x.pow_integer_prec_round_ref_val(z.clone(), prec, rm);
    assert!(p_alt.is_valid());
    assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
    assert_eq!(o_alt, o);
    let (p_alt, o_alt) = x.pow_integer_prec_round_ref_ref(&z, prec, rm);
    assert!(p_alt.is_valid());
    assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.pow_integer_prec_round_assign(z.clone(), prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&p));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.pow_integer_prec_round_assign_ref(&z, prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&p));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_p, rug_o) = rug_pow_integer_prec_round(
            &rug::Float::exact_from(&x),
            &rug::Integer::from(&z),
            prec,
            rug_rm,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_p)),
            ComparableFloatRef(&p)
        );
        assert_eq!(rug_o, o);
    }

    if p.is_normal() && !extreme {
        assert_eq!(p.get_prec(), Some(prec));
    }
}

#[test]
fn pow_integer_prec_round_properties() {
    float_integer_unsigned_rounding_mode_quadruple_gen_var_1().test_properties(
        |(x, z, prec, rm)| {
            pow_integer_prec_round_properties_helper(x, z, prec, rm, false);
        },
    );

    float_integer_unsigned_rounding_mode_quadruple_gen_var_2().test_properties(
        |(x, z, prec, rm)| {
            pow_integer_prec_round_properties_helper(x, z, prec, rm, true);
        },
    );
}

#[test]
fn pow_integer_prec_properties() {
    float_integer_unsigned_triple_gen_var_1::<u64>().test_properties(|(x, z, prec)| {
        let (p, o) = x.clone().pow_integer_prec(z.clone(), prec);
        assert!(p.is_valid());
        let (p_alt, o_alt) = x.clone().pow_integer_prec_val_ref(&z, prec);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
        assert_eq!(o_alt, o);
        let (p_alt, o_alt) = x.pow_integer_prec_ref_val(z.clone(), prec);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
        assert_eq!(o_alt, o);
        let (p_alt, o_alt) = x.pow_integer_prec_ref_ref(&z, prec);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
        assert_eq!(o_alt, o);
        let (p_alt, o_alt) = x.pow_integer_prec_round_ref_ref(&z, prec, Nearest);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
        assert_eq!(o_alt, o);
        let (rug_p, rug_o) =
            rug_pow_integer_prec(&rug::Float::exact_from(&x), &rug::Integer::from(&z), prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_p)),
            ComparableFloatRef(&p)
        );
        assert_eq!(rug_o, o);
    });
}

#[test]
fn pow_integer_round_properties() {
    float_integer_pair_gen().test_properties(|(x, z)| {
        for rm in [Floor, Ceiling, Down, Up, Nearest] {
            let (p, o) = x.clone().pow_integer_round(z.clone(), rm);
            assert!(p.is_valid());
            let (p_alt, o_alt) = x.clone().pow_integer_round_val_ref(&z, rm);
            assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
            assert_eq!(o_alt, o);
            let (p_alt, o_alt) = x.pow_integer_round_ref_val(z.clone(), rm);
            assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
            assert_eq!(o_alt, o);
            let (p_alt, o_alt) = x.pow_integer_round_ref_ref(&z, rm);
            assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
            assert_eq!(o_alt, o);

            let mut x_alt = x.clone();
            let o_alt = x_alt.pow_integer_round_assign(z.clone(), rm);
            assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&p));
            assert_eq!(o_alt, o);
            let mut x_alt = x.clone();
            let o_alt = x_alt.pow_integer_round_assign_ref(&z, rm);
            assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&p));
            assert_eq!(o_alt, o);

            if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
                let (rug_p, rug_o) = rug_pow_integer_round(
                    &rug::Float::exact_from(&x),
                    &rug::Integer::from(&z),
                    rug_rm,
                );
                assert_eq!(
                    ComparableFloatRef(&Float::from(&rug_p)),
                    ComparableFloatRef(&p)
                );
                assert_eq!(rug_o, o);
            }
        }
    });
}

#[test]
fn pow_integer_properties() {
    float_integer_pair_gen().test_properties(|(x, z)| {
        let p = x.clone().pow(z.clone());
        assert!(p.is_valid());
        let p_alt = x.clone().pow(&z);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
        let p_alt = (&x).pow(z.clone());
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
        let p_alt = (&x).pow(&z);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));

        let mut x_alt = x.clone();
        x_alt.pow_assign(z.clone());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&p));
        let mut x_alt = x.clone();
        x_alt.pow_assign(&z);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&p));

        // The trait rounds to the nearest value at the base's precision.
        let (p_alt, _) = x.pow_integer_round_ref_ref(&z, Nearest);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));

        let rug_p = rug_pow_integer(&rug::Float::exact_from(&x), &rug::Integer::from(&z));
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_p)),
            ComparableFloatRef(&p)
        );
    });
}

#[test]
fn test_pow_integer() {
    let test = |s, s_hex, z: i64, prec: u64, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let (p, o) = x.pow_integer_prec_round(Integer::from(z), prec, rm);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);
        assert_eq!(o, o_out);
    };
    test(
        "3.0",
        "0x3.0#2",
        5,
        20,
        Nearest,
        "243.0",
        "0xf3.000#20",
        Equal,
    );
    test(
        "2.0",
        "0x2.0#1",
        10,
        10,
        Nearest,
        "1024.0",
        "0x400.0#10",
        Equal,
    );
    // - negative base, odd exponent gives a negative result
    test(
        "-2.0",
        "-0x2.0#1",
        3,
        10,
        Nearest,
        "-8.0",
        "-0x8.00#10",
        Equal,
    );
    // - negative base, even exponent gives a positive result
    test(
        "-2.0",
        "-0x2.0#1",
        4,
        10,
        Nearest,
        "16.0",
        "0x10.00#10",
        Equal,
    );
    test("1.5", "0x1.8#2", 2, 10, Nearest, "2.25", "0x2.40#10", Equal);
    // - inexact, rounded down
    test(
        "3.0",
        "0x3.0#2",
        -2,
        10,
        Floor,
        "0.1111",
        "0x0.1c70#10",
        Less,
    );
    // - inexact, rounded up
    test(
        "3.0",
        "0x3.0#2",
        -2,
        10,
        Ceiling,
        "0.1112",
        "0x0.1c78#10",
        Greater,
    );
    test("3.0", "0x3.0#2", 5, 2, Floor, "2.0e2", "0xc.0E+1#2", Less);
    test(
        "3.0",
        "0x3.0#2",
        5,
        2,
        Ceiling,
        "3.0e2",
        "0x1.0E+2#2",
        Greater,
    );
    // - negative exponent, reciprocal power
    test(
        "-3.0",
        "-0x3.0#2",
        -1,
        10,
        Nearest,
        "-0.3335",
        "-0x0.556#10",
        Less,
    );
    // - zero exponent gives 1 for any base
    test("5.0", "0x5.0#3", 0, 10, Nearest, "1.0", "0x1.000#10", Equal);
    test(
        "-5.0",
        "-0x5.0#3",
        0,
        10,
        Nearest,
        "1.0",
        "0x1.000#10",
        Equal,
    );
    // - |base| == 1: sign follows exponent parity
    test("-1.0", "-0x1.0#1", 7, 5, Nearest, "-1.0", "-0x1.0#5", Equal);
    test("-1.0", "-0x1.0#1", 8, 5, Nearest, "1.0", "0x1.0#5", Equal);
    // - overflow
    test(
        "2.0",
        "0x2.0#3",
        100000000000,
        5,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    // - underflow to zero (Nearest)
    test(
        "2.0",
        "0x2.0#3",
        -100000000000,
        5,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );
    // - underflow to smallest positive (Up)
    test(
        "2.0",
        "0x2.0#3",
        -100000000000,
        5,
        Up,
        "too_small",
        "0x1.0E-268435456#5",
        Greater,
    );
}

#[test]
fn test_pow_integer_special_values() {
    let test = |base: Float, z: i64, out: &str, out_hex: &str| {
        let (p, o) = base.pow_integer_prec_round(Integer::from(z), 1, Nearest);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);
        assert_eq!(o, Equal);
    };
    test(Float::NAN, 0, "1.0", "0x1.0#1");
    test(Float::NAN, 2, "NaN", "NaN");
    test(Float::NAN, 3, "NaN", "NaN");
    test(Float::NAN, -2, "NaN", "NaN");
    test(Float::NAN, -3, "NaN", "NaN");

    test(Float::INFINITY, 0, "1.0", "0x1.0#1");
    test(Float::INFINITY, 2, "Infinity", "Infinity");
    test(Float::INFINITY, 3, "Infinity", "Infinity");
    test(Float::INFINITY, -2, "0.0", "0x0.0");
    test(Float::INFINITY, -3, "0.0", "0x0.0");

    test(Float::NEGATIVE_INFINITY, 0, "1.0", "0x1.0#1");
    test(Float::NEGATIVE_INFINITY, 2, "Infinity", "Infinity");
    test(Float::NEGATIVE_INFINITY, 3, "-Infinity", "-Infinity");
    test(Float::NEGATIVE_INFINITY, -2, "0.0", "0x0.0");
    test(Float::NEGATIVE_INFINITY, -3, "-0.0", "-0x0.0");

    test(Float::ZERO, 0, "1.0", "0x1.0#1");
    test(Float::ZERO, 2, "0.0", "0x0.0");
    test(Float::ZERO, 3, "0.0", "0x0.0");
    test(Float::ZERO, -2, "Infinity", "Infinity");
    test(Float::ZERO, -3, "Infinity", "Infinity");

    test(Float::NEGATIVE_ZERO, 0, "1.0", "0x1.0#1");
    test(Float::NEGATIVE_ZERO, 2, "0.0", "0x0.0");
    test(Float::NEGATIVE_ZERO, 3, "-0.0", "-0x0.0");
    test(Float::NEGATIVE_ZERO, -2, "Infinity", "Infinity");
    test(Float::NEGATIVE_ZERO, -3, "-Infinity", "-Infinity");

    test(Float::ONE, 0, "1.0", "0x1.0#1");
    test(Float::ONE, 2, "1.0", "0x1.0#1");
    test(Float::ONE, 3, "1.0", "0x1.0#1");
    test(Float::ONE, -2, "1.0", "0x1.0#1");
    test(Float::ONE, -3, "1.0", "0x1.0#1");

    test(Float::NEGATIVE_ONE, 0, "1.0", "0x1.0#1");
    test(Float::NEGATIVE_ONE, 2, "1.0", "0x1.0#1");
    test(Float::NEGATIVE_ONE, 3, "-1.0", "-0x1.0#1");
    test(Float::NEGATIVE_ONE, -2, "1.0", "0x1.0#1");
    test(Float::NEGATIVE_ONE, -3, "-1.0", "-0x1.0#1");
}

#[test]
fn test_pow_integer_extreme() {
    let max_e = i64::from(Float::MAX_EXPONENT);
    let min_e = i64::from(Float::MIN_EXPONENT);
    let test = |base: Float, z: i64, prec: u64, rm, out: &str, out_hex: &str, o_out| {
        let (p, o) = base.pow_integer_prec_round(Integer::from(z), prec, rm);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);
        assert_eq!(o, o_out);
    };
    // 2^(MAX_EXPONENT - 1) is the largest finite power of 2; raised to 1 it is itself
    test(
        Float::power_of_2(max_e - 1),
        1,
        10,
        Nearest,
        "too_big",
        "0x4.00E+268435455#10",
        Equal,
    );
    // squaring it overflows
    test(
        Float::power_of_2(max_e - 1),
        2,
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    // overflow with Down gives the largest finite value
    test(
        Float::power_of_2(max_e - 1),
        2,
        5,
        Down,
        "too_big",
        "0x7.cE+268435455#5",
        Less,
    );
    // 2^MIN_EXPONENT squared underflows
    test(
        Float::power_of_2(min_e),
        2,
        10,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );
    // underflow with Up gives the smallest positive value
    test(
        Float::power_of_2(min_e),
        2,
        5,
        Up,
        "too_small",
        "0x1.0E-268435456#5",
        Greater,
    );
    // a negative exponent on the smallest value overflows
    test(
        Float::power_of_2(min_e),
        -1,
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_pow_integer() {
    fn test<T: PrimitiveFloat>(x: T, z: i64, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_pow_integer(x, &Integer::from(z))),
            NiceFloat(out)
        );
    }
    test::<f32>(f32::NAN, 0, 1.0);
    test::<f32>(f32::NAN, 2, f32::NAN);
    test::<f32>(f32::NAN, 3, f32::NAN);
    test::<f32>(f32::NAN, -2, f32::NAN);
    test::<f32>(f32::NAN, -3, f32::NAN);

    test::<f32>(f32::INFINITY, 0, 1.0);
    test::<f32>(f32::INFINITY, 2, f32::INFINITY);
    test::<f32>(f32::INFINITY, 3, f32::INFINITY);
    test::<f32>(f32::INFINITY, -2, 0.0);
    test::<f32>(f32::INFINITY, -3, 0.0);

    test::<f32>(f32::NEGATIVE_INFINITY, 0, 1.0);
    test::<f32>(f32::NEGATIVE_INFINITY, 2, f32::INFINITY);
    test::<f32>(f32::NEGATIVE_INFINITY, 3, f32::NEGATIVE_INFINITY);
    test::<f32>(f32::NEGATIVE_INFINITY, -2, 0.0);
    test::<f32>(f32::NEGATIVE_INFINITY, -3, -0.0);

    test::<f32>(0.0, 0, 1.0);
    test::<f32>(0.0, 2, 0.0);
    test::<f32>(0.0, 3, 0.0);
    test::<f32>(0.0, -2, f32::INFINITY);
    test::<f32>(0.0, -3, f32::INFINITY);

    test::<f32>(-0.0, 0, 1.0);
    test::<f32>(-0.0, 2, 0.0);
    test::<f32>(-0.0, 3, -0.0);
    test::<f32>(-0.0, -2, f32::INFINITY);
    test::<f32>(-0.0, -3, f32::NEGATIVE_INFINITY);

    test::<f32>(1.0, 0, 1.0);
    test::<f32>(1.0, 2, 1.0);
    test::<f32>(1.0, 3, 1.0);
    test::<f32>(1.0, -2, 1.0);
    test::<f32>(1.0, -3, 1.0);

    test::<f32>(-1.0, 0, 1.0);
    test::<f32>(-1.0, 2, 1.0);
    test::<f32>(-1.0, 3, -1.0);
    test::<f32>(-1.0, -2, 1.0);
    test::<f32>(-1.0, -3, -1.0);

    test::<f32>(3.0, 5, 243.0);
    test::<f32>(2.0, 10, 1024.0);
    test::<f32>(-2.0, 3, -8.0);
    test::<f32>(2.0, -3, 0.125);
    test::<f32>(2.0, 200, f32::INFINITY);
    test::<f32>(0.5, 200, 0.0);

    test::<f64>(3.0, 5, 243.0);
    test::<f64>(2.0, -3, 0.125);
    test::<f64>(1.5, 100, 4.065611775352152e17);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_pow_integer_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    integer_primitive_float_pair_gen::<T>().test_properties(|(z, x)| {
        primitive_float_pow_integer::<T>(x, &z);
    });
}

#[test]
fn primitive_float_pow_integer_properties() {
    apply_fn_to_primitive_floats!(primitive_float_pow_integer_properties_helper);
}

#[test]
fn test_pow_u() {
    let test = |s, s_hex, n: u64, prec: u64, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let (p, o) = x.pow_u_prec_round(n, prec, rm);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);
        assert_eq!(o, o_out);
    };
    // - n == 0 fast path: x^0 = 1
    test("3.0", "0x3.0#2", 0, 10, Nearest, "1.0", "0x1.000#10", Equal);
    // - n == 1 fast path: x^1 = x
    test("3.0", "0x3.0#2", 1, 10, Nearest, "3.0", "0x3.00#10", Equal);
    // - n == 1 fast path with rounding
    test("1.2", "0x1.4#3", 1, 2, Nearest, "1.0", "0x1.0#2", Less);
    // - n == 2 fast path: x^2 = sqr(x)
    test("3.0", "0x3.0#2", 2, 10, Nearest, "9.0", "0x9.00#10", Equal);
    test("3.0", "0x3.0#2", 2, 2, Nearest, "8.0", "0x8.0#2", Less);
    // - n >= 3 square-and-multiply
    test(
        "3.0",
        "0x3.0#2",
        5,
        20,
        Nearest,
        "243.0",
        "0xf3.000#20",
        Equal,
    );
    test("3.0", "0x3.0#2", 5, 2, Floor, "2.0e2", "0xc.0E+1#2", Less);
    test(
        "3.0",
        "0x3.0#2",
        5,
        2,
        Ceiling,
        "3.0e2",
        "0x1.0E+2#2",
        Greater,
    );
    test(
        "1.5",
        "0x1.8#2",
        3,
        10,
        Nearest,
        "3.375",
        "0x3.60#10",
        Equal,
    );
    test(
        "1.5",
        "0x1.8#4",
        10,
        20,
        Nearest,
        "57.66504",
        "0x39.aa40#20",
        Equal,
    );
    // - negative base, odd exponent gives a negative result
    test(
        "-2.0",
        "-0x2.0#1",
        3,
        10,
        Nearest,
        "-8.0",
        "-0x8.00#10",
        Equal,
    );
    // - negative base, even exponent gives a positive result
    test(
        "-2.0",
        "-0x2.0#1",
        4,
        10,
        Nearest,
        "16.0",
        "0x10.00#10",
        Equal,
    );
    test(
        "-3.0",
        "-0x3.0#2",
        3,
        10,
        Nearest,
        "-27.0",
        "-0x1b.00#10",
        Equal,
    );
    // - inexact base: rounding accumulates (sqrt(2)^2 is just above 2)
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        2,
        53,
        Nearest,
        "2.0000000000000004",
        "0x2.0000000000002#53",
        Greater,
    );
    // - overflow
    test(
        "2.0",
        "0x2.0#3",
        100000000000,
        5,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    // - overflow with Down gives the largest finite value
    test(
        "2.0",
        "0x2.0#3",
        100000000000,
        5,
        Down,
        "too_big",
        "0x7.cE+268435455#5",
        Less,
    );
    // - underflow to zero
    test(
        "0.5",
        "0x0.8#3",
        100000000000,
        5,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );
    // - underflow with Up gives the smallest positive value
    test(
        "0.5",
        "0x0.8#3",
        100000000000,
        5,
        Up,
        "too_small",
        "0x1.0E-268435456#5",
        Greater,
    );
    // - canround: an inexact power that the initial working precision already rounds correctly
    test(
        "-269104312292334.303",
        "-0xf4bfbaf113ee.4d8#57",
        11,
        123,
        Floor,
        "-5.3595364566060205383890667672492462806e158",
        "-0x9.c253fc20bc736c88c0172ae629606cE+131#123",
        Less,
    );
    // - ziv: a hard-to-round power that needs the working precision to grow
    test(
        "-0.0078124999999999999999999999999999",
        "-0x0.01ffffffffffffffffffffffffff8#106",
        72,
        5,
        Down,
        "1.85e-152",
        "0xf.8E-127#5",
        Less,
    );
}

#[test]
fn test_pow_u_special_values() {
    let test = |base: Float, n: u64, out: &str, out_hex: &str| {
        let (p, o) = base.pow_u_prec_round(n, 1, Nearest);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);
        assert_eq!(o, Equal);
    };
    test(Float::NAN, 0, "1.0", "0x1.0#1");
    test(Float::NAN, 1, "NaN", "NaN");
    test(Float::NAN, 2, "NaN", "NaN");
    test(Float::NAN, 3, "NaN", "NaN");
    test(Float::NAN, 4, "NaN", "NaN");

    test(Float::INFINITY, 0, "1.0", "0x1.0#1");
    test(Float::INFINITY, 1, "Infinity", "Infinity");
    test(Float::INFINITY, 2, "Infinity", "Infinity");
    test(Float::INFINITY, 3, "Infinity", "Infinity");
    test(Float::INFINITY, 4, "Infinity", "Infinity");

    test(Float::NEGATIVE_INFINITY, 0, "1.0", "0x1.0#1");
    test(Float::NEGATIVE_INFINITY, 1, "-Infinity", "-Infinity");
    test(Float::NEGATIVE_INFINITY, 2, "Infinity", "Infinity");
    test(Float::NEGATIVE_INFINITY, 3, "-Infinity", "-Infinity");
    test(Float::NEGATIVE_INFINITY, 4, "Infinity", "Infinity");

    test(Float::ZERO, 0, "1.0", "0x1.0#1");
    test(Float::ZERO, 1, "0.0", "0x0.0");
    test(Float::ZERO, 2, "0.0", "0x0.0");
    test(Float::ZERO, 3, "0.0", "0x0.0");
    test(Float::ZERO, 4, "0.0", "0x0.0");

    test(Float::NEGATIVE_ZERO, 0, "1.0", "0x1.0#1");
    test(Float::NEGATIVE_ZERO, 1, "-0.0", "-0x0.0");
    test(Float::NEGATIVE_ZERO, 2, "0.0", "0x0.0");
    test(Float::NEGATIVE_ZERO, 3, "-0.0", "-0x0.0");
    test(Float::NEGATIVE_ZERO, 4, "0.0", "0x0.0");

    test(Float::ONE, 0, "1.0", "0x1.0#1");
    test(Float::ONE, 1, "1.0", "0x1.0#1");
    test(Float::ONE, 2, "1.0", "0x1.0#1");
    test(Float::ONE, 3, "1.0", "0x1.0#1");
    test(Float::ONE, 4, "1.0", "0x1.0#1");

    test(Float::NEGATIVE_ONE, 0, "1.0", "0x1.0#1");
    test(Float::NEGATIVE_ONE, 1, "-1.0", "-0x1.0#1");
    test(Float::NEGATIVE_ONE, 2, "1.0", "0x1.0#1");
    test(Float::NEGATIVE_ONE, 3, "-1.0", "-0x1.0#1");
    test(Float::NEGATIVE_ONE, 4, "1.0", "0x1.0#1");
}

#[test]
fn test_pow_u_extreme() {
    let max_e = i64::from(Float::MAX_EXPONENT);
    let min_e = i64::from(Float::MIN_EXPONENT);
    let test = |base: Float, n: u64, prec: u64, rm, out: &str, out_hex: &str, o_out| {
        let (p, o) = base.pow_u_prec_round(n, prec, rm);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);
        assert_eq!(o, o_out);
    };
    // 2^(MAX_EXPONENT - 1) is the largest finite power of 2; raised to 1 it is itself
    test(
        Float::power_of_2(max_e - 1),
        1,
        10,
        Nearest,
        "too_big",
        "0x4.00E+268435455#10",
        Equal,
    );
    // squaring it overflows
    test(
        Float::power_of_2(max_e - 1),
        2,
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    // overflow with Down gives the largest finite value
    test(
        Float::power_of_2(max_e - 1),
        2,
        5,
        Down,
        "too_big",
        "0x7.cE+268435455#5",
        Less,
    );
    test(
        Float::power_of_2(max_e - 1),
        3,
        10,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    // 2^MIN_EXPONENT squared underflows
    test(
        Float::power_of_2(min_e),
        2,
        10,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );
    // underflow with Up gives the smallest positive value
    test(
        Float::power_of_2(min_e),
        2,
        5,
        Up,
        "too_small",
        "0x1.0E-268435456#5",
        Greater,
    );
    test(
        Float::power_of_2(min_e),
        3,
        10,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );
    // negative base, odd exponent: overflow to -Infinity
    test(
        -Float::power_of_2(max_e - 1),
        3,
        10,
        Nearest,
        "-Infinity",
        "-Infinity",
        Less,
    );
}

#[allow(clippy::needless_pass_by_value)]
fn pow_u_prec_round_properties_helper(
    x: Float,
    n: u64,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) {
    if rm == Exact {
        // Exact is only allowed when the result is exactly representable; otherwise panic.
        let (p, o) = x.pow_u_prec_round_ref(n, prec, Nearest);
        if o == Equal {
            let (pe, oe) = x.pow_u_prec_round_ref(n, prec, Exact);
            assert_eq!(ComparableFloatRef(&pe), ComparableFloatRef(&p));
            assert_eq!(oe, Equal);
        } else {
            assert_panic!(x.pow_u_prec_round_ref(n, prec, Exact));
        }
        return;
    }
    let (p, o) = x.clone().pow_u_prec_round(n, prec, rm);
    assert!(p.is_valid());
    let (p_alt, o_alt) = x.pow_u_prec_round_ref(n, prec, rm);
    assert!(p_alt.is_valid());
    assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.pow_u_prec_round_assign(n, prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&p));
    assert_eq!(o_alt, o);

    // pow_u (mpfr_pow_ui) must agree with pow_integer (mpfr_pow_z).
    let (pi, oi) = x.pow_integer_prec_round_ref_ref(&Integer::from(n), prec, rm);
    assert_eq!(ComparableFloatRef(&pi), ComparableFloatRef(&p));
    assert_eq!(oi, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_p, rug_o) = rug_pow_u_prec_round(&rug::Float::exact_from(&x), n, prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_p)),
            ComparableFloatRef(&p)
        );
        assert_eq!(rug_o, o);
    }

    if p.is_normal() && !extreme {
        assert_eq!(p.get_prec(), Some(prec));
    }
}

#[test]
fn pow_u_prec_round_properties() {
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_9().test_properties(
        |(x, n, prec, rm)| {
            pow_u_prec_round_properties_helper(x, n, prec, rm, false);
        },
    );

    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_10().test_properties(
        |(x, n, prec, rm)| {
            pow_u_prec_round_properties_helper(x, n, prec, rm, true);
        },
    );
}

#[test]
fn pow_u_prec_properties() {
    float_unsigned_unsigned_triple_gen_var_1::<u64, u64>().test_properties(|(x, n, prec)| {
        let (p, o) = x.clone().pow_u_prec(n, prec);
        assert!(p.is_valid());
        let (p_alt, o_alt) = x.pow_u_prec_ref(n, prec);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
        assert_eq!(o_alt, o);
        let (p_alt, o_alt) = x.pow_u_prec_round_ref(n, prec, Nearest);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
        assert_eq!(o_alt, o);
        let (pi, oi) = x.pow_integer_prec_ref_ref(&Integer::from(n), prec);
        assert_eq!(ComparableFloatRef(&pi), ComparableFloatRef(&p));
        assert_eq!(oi, o);
        let (rug_p, rug_o) = rug_pow_u_prec(&rug::Float::exact_from(&x), n, prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_p)),
            ComparableFloatRef(&p)
        );
        assert_eq!(rug_o, o);
    });
}

#[test]
fn pow_u_round_properties() {
    float_unsigned_pair_gen::<u64>().test_properties(|(x, n)| {
        for rm in [Floor, Ceiling, Down, Up, Nearest] {
            let (p, o) = x.clone().pow_u_round(n, rm);
            assert!(p.is_valid());
            let (p_alt, o_alt) = x.pow_u_round_ref(n, rm);
            assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
            assert_eq!(o_alt, o);
            let mut x_alt = x.clone();
            let o_alt = x_alt.pow_u_round_assign(n, rm);
            assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&p));
            assert_eq!(o_alt, o);
            if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
                let (rug_p, rug_o) = rug_pow_u_round(&rug::Float::exact_from(&x), n, rug_rm);
                assert_eq!(
                    ComparableFloatRef(&Float::from(&rug_p)),
                    ComparableFloatRef(&p)
                );
                assert_eq!(rug_o, o);
            }
        }
    });
}

#[test]
fn pow_u_properties() {
    float_unsigned_pair_gen::<u64>().test_properties(|(x, n)| {
        let p = x.clone().pow(n);
        assert!(p.is_valid());
        let p_alt = (&x).pow(n);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));

        let mut x_alt = x.clone();
        x_alt.pow_assign(n);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&p));

        // The trait rounds to the nearest value at the base's precision.
        let (p_alt, _) = x.pow_u_round_ref(n, Nearest);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));

        let rug_p = rug_pow_u(&rug::Float::exact_from(&x), n);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_p)),
            ComparableFloatRef(&p)
        );
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_pow_u() {
    fn test<T: PrimitiveFloat>(x: T, n: u64, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_pow_u(x, n)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, 0, 1.0);
    test::<f32>(f32::NAN, 1, f32::NAN);
    test::<f32>(f32::NAN, 2, f32::NAN);
    test::<f32>(f32::NAN, 3, f32::NAN);
    test::<f32>(f32::NAN, 4, f32::NAN);

    test::<f32>(f32::INFINITY, 0, 1.0);
    test::<f32>(f32::INFINITY, 1, f32::INFINITY);
    test::<f32>(f32::INFINITY, 2, f32::INFINITY);
    test::<f32>(f32::INFINITY, 3, f32::INFINITY);
    test::<f32>(f32::INFINITY, 4, f32::INFINITY);

    test::<f32>(f32::NEGATIVE_INFINITY, 0, 1.0);
    test::<f32>(f32::NEGATIVE_INFINITY, 1, f32::NEGATIVE_INFINITY);
    test::<f32>(f32::NEGATIVE_INFINITY, 2, f32::INFINITY);
    test::<f32>(f32::NEGATIVE_INFINITY, 3, f32::NEGATIVE_INFINITY);
    test::<f32>(f32::NEGATIVE_INFINITY, 4, f32::INFINITY);

    test::<f32>(0.0, 0, 1.0);
    test::<f32>(0.0, 1, 0.0);
    test::<f32>(0.0, 2, 0.0);
    test::<f32>(0.0, 3, 0.0);
    test::<f32>(0.0, 4, 0.0);

    test::<f32>(-0.0, 0, 1.0);
    test::<f32>(-0.0, 1, -0.0);
    test::<f32>(-0.0, 2, 0.0);
    test::<f32>(-0.0, 3, -0.0);
    test::<f32>(-0.0, 4, 0.0);

    test::<f32>(1.0, 0, 1.0);
    test::<f32>(1.0, 1, 1.0);
    test::<f32>(1.0, 2, 1.0);
    test::<f32>(1.0, 3, 1.0);
    test::<f32>(1.0, 4, 1.0);

    test::<f32>(-1.0, 0, 1.0);
    test::<f32>(-1.0, 1, -1.0);
    test::<f32>(-1.0, 2, 1.0);
    test::<f32>(-1.0, 3, -1.0);
    test::<f32>(-1.0, 4, 1.0);

    test::<f32>(3.0, 5, 243.0);
    test::<f32>(2.0, 10, 1024.0);
    test::<f32>(-2.0, 3, -8.0);
    test::<f32>(1.1, 2, 1.21);
    test::<f32>(2.0, 200, f32::INFINITY);
    test::<f32>(0.5, 200, 0.0);

    test::<f64>(3.0, 5, 243.0);
    test::<f64>(-2.0, 3, -8.0);
    test::<f64>(1.1, 2, 1.2100000000000002);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_pow_u_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_unsigned_pair_gen_var_1::<T, u64>().test_properties(|(x, n)| {
        primitive_float_pow_u::<T>(x, n);
    });
}

#[test]
fn primitive_float_pow_u_properties() {
    apply_fn_to_primitive_floats!(primitive_float_pow_u_properties_helper);
}

#[test]
fn test_pow_s() {
    let test = |s, s_hex, n: i64, prec: u64, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let (p, o) = x.pow_s_prec_round(n, prec, rm);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);
        assert_eq!(o, o_out);
    };
    // - n == 0: x^0 = 1
    test("3.0", "0x3.0#2", 0, 10, Nearest, "1.0", "0x1.000#10", Equal);
    // - n == 1: x^1 = x
    test("3.0", "0x3.0#2", 1, 10, Nearest, "3.0", "0x3.00#10", Equal);
    // - n >= 0 delegates to pow_u (square-and-multiply)
    test(
        "3.0",
        "0x3.0#2",
        5,
        20,
        Nearest,
        "243.0",
        "0xf3.000#20",
        Equal,
    );
    test("3.0", "0x3.0#2", 5, 2, Floor, "2.0e2", "0xc.0E+1#2", Less);
    test(
        "3.0",
        "0x3.0#2",
        5,
        2,
        Ceiling,
        "3.0e2",
        "0x1.0E+2#2",
        Greater,
    );
    test(
        "-2.0",
        "-0x2.0#1",
        3,
        10,
        Nearest,
        "-8.0",
        "-0x8.00#10",
        Equal,
    );
    test(
        "-2.0",
        "-0x2.0#1",
        4,
        10,
        Nearest,
        "16.0",
        "0x10.00#10",
        Equal,
    );
    // - n < 0 delegates to pow_integer (reciprocal power)
    test(
        "2.0",
        "0x2.0#1",
        -3,
        10,
        Nearest,
        "0.125",
        "0x0.200#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        -2,
        10,
        Floor,
        "0.1111",
        "0x0.1c70#10",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        -2,
        10,
        Ceiling,
        "0.1112",
        "0x0.1c78#10",
        Greater,
    );
    // - negative base, odd negative exponent gives a negative result
    test(
        "-2.0",
        "-0x2.0#2",
        -3,
        10,
        Nearest,
        "-0.125",
        "-0x0.200#10",
        Equal,
    );
    // - negative base, even negative exponent gives a positive result
    test(
        "-2.0",
        "-0x2.0#2",
        -4,
        10,
        Nearest,
        "0.0625",
        "0x0.1000#10",
        Equal,
    );
    test(
        "1.5",
        "0x1.8#4",
        -10,
        20,
        Nearest,
        "0.01734152",
        "0x0.04707e8#20",
        Less,
    );
    test(
        "-3.0",
        "-0x3.0#2",
        -1,
        10,
        Nearest,
        "-0.3335",
        "-0x0.556#10",
        Less,
    );
    // - power-of-2 base with a negative exponent is an exact reciprocal
    test(
        "4.0",
        "0x4.0#1",
        -3,
        5,
        Nearest,
        "0.016",
        "0x0.040#5",
        Equal,
    );
    // - underflow (positive base, large negative exponent)
    test(
        "2.0",
        "0x2.0#3",
        -100000000000,
        5,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );
    // - overflow (base < 1, large negative exponent)
    test(
        "0.5",
        "0x0.8#3",
        -100000000000,
        5,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    // - overflow with Down gives the largest finite value
    test(
        "0.5",
        "0x0.8#3",
        -100000000000,
        5,
        Down,
        "too_big",
        "0x7.cE+268435455#5",
        Less,
    );
}

#[test]
fn test_pow_s_special_values() {
    let test = |base: Float, n: i64, out: &str, out_hex: &str| {
        let (p, o) = base.pow_s_prec_round(n, 1, Nearest);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);
        assert_eq!(o, Equal);
    };
    test(Float::NAN, 0, "1.0", "0x1.0#1");
    test(Float::NAN, 1, "NaN", "NaN");
    test(Float::NAN, 2, "NaN", "NaN");
    test(Float::NAN, 3, "NaN", "NaN");
    test(Float::NAN, -1, "NaN", "NaN");
    test(Float::NAN, -2, "NaN", "NaN");
    test(Float::NAN, -3, "NaN", "NaN");

    test(Float::INFINITY, 0, "1.0", "0x1.0#1");
    test(Float::INFINITY, 1, "Infinity", "Infinity");
    test(Float::INFINITY, 2, "Infinity", "Infinity");
    test(Float::INFINITY, 3, "Infinity", "Infinity");
    test(Float::INFINITY, -1, "0.0", "0x0.0");
    test(Float::INFINITY, -2, "0.0", "0x0.0");
    test(Float::INFINITY, -3, "0.0", "0x0.0");

    test(Float::NEGATIVE_INFINITY, 0, "1.0", "0x1.0#1");
    test(Float::NEGATIVE_INFINITY, 1, "-Infinity", "-Infinity");
    test(Float::NEGATIVE_INFINITY, 2, "Infinity", "Infinity");
    test(Float::NEGATIVE_INFINITY, 3, "-Infinity", "-Infinity");
    test(Float::NEGATIVE_INFINITY, -1, "-0.0", "-0x0.0");
    test(Float::NEGATIVE_INFINITY, -2, "0.0", "0x0.0");
    test(Float::NEGATIVE_INFINITY, -3, "-0.0", "-0x0.0");

    test(Float::ZERO, 0, "1.0", "0x1.0#1");
    test(Float::ZERO, 1, "0.0", "0x0.0");
    test(Float::ZERO, 2, "0.0", "0x0.0");
    test(Float::ZERO, 3, "0.0", "0x0.0");
    test(Float::ZERO, -1, "Infinity", "Infinity");
    test(Float::ZERO, -2, "Infinity", "Infinity");
    test(Float::ZERO, -3, "Infinity", "Infinity");

    test(Float::NEGATIVE_ZERO, 0, "1.0", "0x1.0#1");
    test(Float::NEGATIVE_ZERO, 1, "-0.0", "-0x0.0");
    test(Float::NEGATIVE_ZERO, 2, "0.0", "0x0.0");
    test(Float::NEGATIVE_ZERO, 3, "-0.0", "-0x0.0");
    test(Float::NEGATIVE_ZERO, -1, "-Infinity", "-Infinity");
    test(Float::NEGATIVE_ZERO, -2, "Infinity", "Infinity");
    test(Float::NEGATIVE_ZERO, -3, "-Infinity", "-Infinity");

    test(Float::ONE, 0, "1.0", "0x1.0#1");
    test(Float::ONE, 1, "1.0", "0x1.0#1");
    test(Float::ONE, 2, "1.0", "0x1.0#1");
    test(Float::ONE, 3, "1.0", "0x1.0#1");
    test(Float::ONE, -1, "1.0", "0x1.0#1");
    test(Float::ONE, -2, "1.0", "0x1.0#1");
    test(Float::ONE, -3, "1.0", "0x1.0#1");

    test(Float::NEGATIVE_ONE, 0, "1.0", "0x1.0#1");
    test(Float::NEGATIVE_ONE, 1, "-1.0", "-0x1.0#1");
    test(Float::NEGATIVE_ONE, 2, "1.0", "0x1.0#1");
    test(Float::NEGATIVE_ONE, 3, "-1.0", "-0x1.0#1");
    test(Float::NEGATIVE_ONE, -1, "-1.0", "-0x1.0#1");
    test(Float::NEGATIVE_ONE, -2, "1.0", "0x1.0#1");
    test(Float::NEGATIVE_ONE, -3, "-1.0", "-0x1.0#1");
}

#[test]
fn test_pow_s_extreme() {
    let max_e = i64::from(Float::MAX_EXPONENT);
    let min_e = i64::from(Float::MIN_EXPONENT);
    let test = |base: Float, n: i64, prec: u64, rm, out: &str, out_hex: &str, o_out| {
        let (p, o) = base.pow_s_prec_round(n, prec, rm);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);
        assert_eq!(o, o_out);
    };
    // 2^MIN_EXPONENT raised to -1 overflows (to 2^-MIN)
    test(
        Float::power_of_2(min_e),
        -1,
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    // (2^(MAX_EXPONENT - 1))^-2 underflows
    test(
        Float::power_of_2(max_e - 1),
        -2,
        10,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );
    // underflow with Up gives the smallest positive value
    test(
        Float::power_of_2(max_e - 1),
        -2,
        5,
        Up,
        "too_small",
        "0x1.0E-268435456#5",
        Greater,
    );
    // negative base, odd negative exponent: overflow to -Infinity
    test(
        -Float::power_of_2(min_e),
        -1,
        10,
        Nearest,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        Float::power_of_2(min_e),
        -3,
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
}

#[allow(clippy::needless_pass_by_value)]
fn pow_s_prec_round_properties_helper(
    x: Float,
    n: i64,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) {
    if rm == Exact {
        let (p, o) = x.pow_s_prec_round_ref(n, prec, Nearest);
        if o == Equal {
            let (pe, oe) = x.pow_s_prec_round_ref(n, prec, Exact);
            assert_eq!(ComparableFloatRef(&pe), ComparableFloatRef(&p));
            assert_eq!(oe, Equal);
        } else {
            assert_panic!(x.pow_s_prec_round_ref(n, prec, Exact));
        }
        return;
    }
    let (p, o) = x.clone().pow_s_prec_round(n, prec, rm);
    assert!(p.is_valid());
    let (p_alt, o_alt) = x.pow_s_prec_round_ref(n, prec, rm);
    assert!(p_alt.is_valid());
    assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.pow_s_prec_round_assign(n, prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&p));
    assert_eq!(o_alt, o);

    // pow_s (mpfr_pow_si) must agree with pow_integer (mpfr_pow_z).
    let (pi, oi) = x.pow_integer_prec_round_ref_ref(&Integer::from(n), prec, rm);
    assert_eq!(ComparableFloatRef(&pi), ComparableFloatRef(&p));
    assert_eq!(oi, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_p, rug_o) = rug_pow_s_prec_round(&rug::Float::exact_from(&x), n, prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_p)),
            ComparableFloatRef(&p)
        );
        assert_eq!(rug_o, o);
    }

    if p.is_normal() && !extreme {
        assert_eq!(p.get_prec(), Some(prec));
    }
}

#[test]
fn pow_s_prec_round_properties() {
    float_signed_unsigned_rounding_mode_quadruple_gen_var_11().test_properties(
        |(x, n, prec, rm)| {
            pow_s_prec_round_properties_helper(x, n, prec, rm, false);
        },
    );

    float_signed_unsigned_rounding_mode_quadruple_gen_var_12().test_properties(
        |(x, n, prec, rm)| {
            pow_s_prec_round_properties_helper(x, n, prec, rm, true);
        },
    );
}

#[test]
fn pow_s_prec_properties() {
    float_signed_unsigned_triple_gen_var_1::<i64, u64>().test_properties(|(x, n, prec)| {
        let (p, o) = x.clone().pow_s_prec(n, prec);
        assert!(p.is_valid());
        let (p_alt, o_alt) = x.pow_s_prec_ref(n, prec);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
        assert_eq!(o_alt, o);
        let (p_alt, o_alt) = x.pow_s_prec_round_ref(n, prec, Nearest);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
        assert_eq!(o_alt, o);
        let (pi, oi) = x.pow_integer_prec_ref_ref(&Integer::from(n), prec);
        assert_eq!(ComparableFloatRef(&pi), ComparableFloatRef(&p));
        assert_eq!(oi, o);
        let (rug_p, rug_o) = rug_pow_s_prec(&rug::Float::exact_from(&x), n, prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_p)),
            ComparableFloatRef(&p)
        );
        assert_eq!(rug_o, o);
    });
}

#[test]
fn pow_s_round_properties() {
    float_signed_pair_gen::<i64>().test_properties(|(x, n)| {
        for rm in [Floor, Ceiling, Down, Up, Nearest] {
            let (p, o) = x.clone().pow_s_round(n, rm);
            assert!(p.is_valid());
            let (p_alt, o_alt) = x.pow_s_round_ref(n, rm);
            assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
            assert_eq!(o_alt, o);
            let mut x_alt = x.clone();
            let o_alt = x_alt.pow_s_round_assign(n, rm);
            assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&p));
            assert_eq!(o_alt, o);
            if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
                let (rug_p, rug_o) = rug_pow_s_round(&rug::Float::exact_from(&x), n, rug_rm);
                assert_eq!(
                    ComparableFloatRef(&Float::from(&rug_p)),
                    ComparableFloatRef(&p)
                );
                assert_eq!(rug_o, o);
            }
        }
    });
}

#[test]
fn pow_s_properties() {
    float_signed_pair_gen::<i64>().test_properties(|(x, n)| {
        let p = x.clone().pow(n);
        assert!(p.is_valid());
        let p_alt = (&x).pow(n);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));

        let mut x_alt = x.clone();
        x_alt.pow_assign(n);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&p));

        let (p_alt, _) = x.pow_s_round_ref(n, Nearest);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));

        let rug_p = rug_pow_s(&rug::Float::exact_from(&x), n);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_p)),
            ComparableFloatRef(&p)
        );
    });
}

#[test]
fn test_unsigned_pow_unsigned() {
    let test = |x: u64, y: u64, prec: u64, rm, out: &str, out_hex: &str, o_out| {
        let (p, o) = Float::unsigned_pow_unsigned_prec_round(x, y, prec, rm);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);
        assert_eq!(o, o_out);
    };
    // - n == 0: k^0 = 1 for any k
    test(0, 0, 10, Nearest, "1.0", "0x1.000#10", Equal);
    test(5, 0, 10, Nearest, "1.0", "0x1.000#10", Equal);
    // - n == 1: k^1 = k
    test(5, 1, 10, Nearest, "5.0", "0x5.00#10", Equal);
    // - n == 1 with rounding
    test(5, 1, 2, Nearest, "4.0", "0x4.0#2", Less);
    // - k == 0: 0^n = 0 for n >= 1
    test(0, 3, 10, Nearest, "0.0", "0x0.0", Equal);
    // - k == 1: 1^n = 1
    test(1, 100, 10, Nearest, "1.0", "0x1.000#10", Equal);
    // - square-and-multiply, exact
    test(2, 10, 20, Nearest, "1024.0", "0x400.000#20", Equal);
    test(3, 5, 20, Nearest, "243.0", "0xf3.000#20", Equal);
    test(10, 3, 20, Nearest, "1000.0", "0x3e8.000#20", Equal);
    // - inexact, rounded both ways
    test(3, 5, 2, Floor, "2.0e2", "0xc.0E+1#2", Less);
    test(3, 5, 2, Ceiling, "3.0e2", "0x1.0E+2#2", Greater);
    test(
        7,
        20,
        30,
        Nearest,
        "7.97922663e16",
        "0x1.1b7aa4b8E+14#30",
        Less,
    );
    // - a large exact power
    test(
        12,
        24,
        200,
        Nearest,
        "79496847203390844133441536.0",
        "0x41c21cb8e1000000000000.00000000000000000000000000000#200",
        Equal,
    );
    // - overflow falls back to pow_integer
    test(3, 1000000000, 5, Nearest, "Infinity", "Infinity", Greater);
    // - overflow with Down gives the largest finite value
    test(
        3,
        1000000000,
        5,
        Down,
        "too_big",
        "0x7.cE+268435455#5",
        Less,
    );
    test(2, 10000000000, 10, Nearest, "Infinity", "Infinity", Greater);
    // - canround: an inexact power the initial working precision already rounds correctly
    test(3, 6, 1, Down, "5.0e2", "0x2.0E+2#1", Less);

    // - error-budget regressions: with MPFR's budget (which upstream mpfr_ui_pow_ui inherits),
    //   `float_can_round` certified wrongly rounded results at small precisions
    test(263, 15, 1, Nearest, "1.0e36", "0x1.0E+30#1", Less);
    test(281, 6, 2, Nearest, "4.0e14", "0x1.8E+12#2", Less);
    test(205, 63, 4, Down, "4.1e145", "0xd.0E+120#4", Less);
    test(205, 63, 4, Nearest, "4.4e145", "0xe.0E+120#4", Greater);
    test(410, 63, 4, Floor, "3.7e164", "0x6.8E+136#4", Less);
}

// The exact Float representation of a u64, for use as an oracle base.
fn u64_as_float(k: u64) -> Float {
    Float::from_unsigned_prec(k, k.significant_bits().max(1)).0
}

#[test]
fn unsigned_pow_unsigned_prec_round_properties() {
    unsigned_unsigned_unsigned_rounding_mode_quadruple_gen_var_1().test_properties(
        |(x, y, prec, rm)| {
            if rm == Exact {
                let (p, o) = Float::unsigned_pow_unsigned_prec_round(x, y, prec, Nearest);
                if o == Equal {
                    let (pe, oe) = Float::unsigned_pow_unsigned_prec_round(x, y, prec, Exact);
                    assert_eq!(ComparableFloatRef(&pe), ComparableFloatRef(&p));
                    assert_eq!(oe, Equal);
                } else {
                    assert_panic!(Float::unsigned_pow_unsigned_prec_round(x, y, prec, Exact));
                }
                return;
            }
            let (p, o) = Float::unsigned_pow_unsigned_prec_round(x, y, prec, rm);
            assert!(p.is_valid());

            // unsigned_pow_unsigned (mpfr_ui_pow_ui) must agree with pow_integer (mpfr_pow_z) on
            // the exact Float representation of the base.
            let kf = u64_as_float(x);
            let (pi, oi) = kf.pow_integer_prec_round_ref_ref(&Integer::from(y), prec, rm);
            assert_eq!(ComparableFloatRef(&pi), ComparableFloatRef(&p));
            assert_eq!(oi, o);

            if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
                let (rug_p, rug_o) = rug_pow_integer_prec_round(
                    &rug::Float::exact_from(&kf),
                    &rug::Integer::from(y),
                    prec,
                    rug_rm,
                );
                assert_eq!(
                    ComparableFloatRef(&Float::from(&rug_p)),
                    ComparableFloatRef(&p)
                );
                assert_eq!(rug_o, o);
            }

            if p.is_normal() {
                assert_eq!(p.get_prec(), Some(prec));
            }
        },
    );
}

#[test]
fn unsigned_pow_unsigned_prec_properties() {
    unsigned_unsigned_unsigned_rounding_mode_quadruple_gen_var_1().test_properties(
        |(x, y, prec, _)| {
            let (p, o) = Float::unsigned_pow_unsigned_prec(x, y, prec);
            assert!(p.is_valid());
            let (p_alt, o_alt) = Float::unsigned_pow_unsigned_prec_round(x, y, prec, Nearest);
            assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
            assert_eq!(o_alt, o);
            let kf = u64_as_float(x);
            let (pi, oi) = kf.pow_integer_prec_ref_ref(&Integer::from(y), prec);
            assert_eq!(ComparableFloatRef(&pi), ComparableFloatRef(&p));
            assert_eq!(oi, o);
        },
    );
}

#[test]
fn test_unsigned_pow() {
    let test = |x: u64, s, s_hex, prec: u64, rm, out: &str, out_hex: &str, o_out| {
        let y = parse_hex_string(s_hex);
        assert_eq!(y.to_string(), s);
        let (p, o) = Float::unsigned_pow_prec_round(x, y, prec, rm);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);
        assert_eq!(o, o_out);
    };
    // - a fractional exponent
    test(
        2,
        "0.5",
        "0x0.8#1",
        53,
        Nearest,
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        Greater,
    );
    test(
        3,
        "2.5",
        "0x2.8#3",
        53,
        Nearest,
        "15.588457268119896",
        "0xf.96a522b1ab200#53",
        Greater,
    );
    test(
        3,
        "2.5",
        "0x2.8#3",
        53,
        Floor,
        "15.588457268119894",
        "0xf.96a522b1ab1f8#53",
        Less,
    );
    // - a negative exponent (reciprocal)
    test(
        2,
        "-1.0",
        "-0x1.0#1",
        10,
        Nearest,
        "0.5",
        "0x0.800#10",
        Equal,
    );
    test(
        10,
        "-0.5",
        "-0x0.8#1",
        53,
        Nearest,
        "0.31622776601683794",
        "0x0.50f44d8921243c#53",
        Greater,
    );
    // - an integer-valued exponent
    test(
        3,
        "5.0",
        "0x5.0#3",
        20,
        Nearest,
        "243.0",
        "0xf3.000#20",
        Equal,
    );
    test(3, "5.0", "0x5.0#3", 2, Floor, "2.0e2", "0xc.0E+1#2", Less);
    test(
        2,
        "1.2",
        "0x1.4#3",
        10,
        Nearest,
        "2.379",
        "0x2.61#10",
        Greater,
    );
    // - overflow
    test(
        2,
        "1.0e12",
        "0x1.0E+10#1",
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    // - a tiny exponent gives a result near 1
    test(
        100,
        "9.0e-13",
        "0x1.0E-10#1",
        20,
        Nearest,
        "1.0",
        "0x1.00000#20",
        Less,
    );
}

#[test]
fn test_unsigned_pow_special_values() {
    let test = |x: u64, y: Float, out: &str, out_hex: &str| {
        let (p, o) = Float::unsigned_pow_prec_round(x, y, 1, Nearest);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);
        assert_eq!(o, Equal);
    };
    test(0, Float::NAN, "NaN", "NaN");
    test(0, Float::INFINITY, "0.0", "0x0.0");
    test(0, Float::NEGATIVE_INFINITY, "Infinity", "Infinity");
    test(0, Float::ZERO, "1.0", "0x1.0#1");
    test(0, Float::NEGATIVE_ZERO, "1.0", "0x1.0#1");

    test(1, Float::NAN, "1.0", "0x1.0#1");
    test(1, Float::INFINITY, "1.0", "0x1.0#1");
    test(1, Float::NEGATIVE_INFINITY, "1.0", "0x1.0#1");
    test(1, Float::ZERO, "1.0", "0x1.0#1");
    test(1, Float::NEGATIVE_ZERO, "1.0", "0x1.0#1");

    test(2, Float::NAN, "NaN", "NaN");
    test(2, Float::INFINITY, "Infinity", "Infinity");
    test(2, Float::NEGATIVE_INFINITY, "0.0", "0x0.0");
    test(2, Float::ZERO, "1.0", "0x1.0#1");
    test(2, Float::NEGATIVE_ZERO, "1.0", "0x1.0#1");
}

#[allow(clippy::needless_pass_by_value)]
fn unsigned_pow_prec_round_properties_helper(
    y: Float,
    x: u64,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) {
    if rm == Exact {
        let (p, o) = Float::unsigned_pow_prec_round_ref(x, &y, prec, Nearest);
        if o == Equal {
            let (pe, oe) = Float::unsigned_pow_prec_round_ref(x, &y, prec, Exact);
            assert_eq!(ComparableFloatRef(&pe), ComparableFloatRef(&p));
            assert_eq!(oe, Equal);
        } else {
            assert_panic!(Float::unsigned_pow_prec_round_ref(x, &y, prec, Exact));
        }
        return;
    }
    let (p, o) = Float::unsigned_pow_prec_round(x, y.clone(), prec, rm);
    assert!(p.is_valid());
    let (p_alt, o_alt) = Float::unsigned_pow_prec_round_ref(x, &y, prec, rm);
    assert!(p_alt.is_valid());
    assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
    assert_eq!(o_alt, o);

    // n^x is x^x on the exact Float representation of the base, computed by pow_prec_round.
    let kf = u64_as_float(x);
    let (pp, op) = kf.pow_prec_round_ref_ref(&y, prec, rm);
    assert_eq!(ComparableFloatRef(&pp), ComparableFloatRef(&p));
    assert_eq!(op, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_p, rug_o) = rug_pow_prec_round(
            &rug::Float::exact_from(&kf),
            &rug::Float::exact_from(&y),
            prec,
            rug_rm,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_p)),
            ComparableFloatRef(&p)
        );
        assert_eq!(rug_o, o);
    }

    if p.is_normal() && !extreme {
        assert_eq!(p.get_prec(), Some(prec));
    }
}

#[test]
fn unsigned_pow_prec_round_properties() {
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_11().test_properties(
        |(y, x, prec, rm)| {
            unsigned_pow_prec_round_properties_helper(y, x, prec, rm, false);
        },
    );

    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_12().test_properties(
        |(y, x, prec, rm)| {
            unsigned_pow_prec_round_properties_helper(y, x, prec, rm, true);
        },
    );
}

#[test]
fn unsigned_pow_prec_properties() {
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_11().test_properties(
        |(y, x, prec, _)| {
            let (p, o) = Float::unsigned_pow_prec(x, y.clone(), prec);
            assert!(p.is_valid());
            let (p_alt, o_alt) = Float::unsigned_pow_prec_ref(x, &y, prec);
            assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
            assert_eq!(o_alt, o);
            let (p_alt, o_alt) = Float::unsigned_pow_prec_round_ref(x, &y, prec, Nearest);
            assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
            assert_eq!(o_alt, o);
        },
    );
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_unsigned_pow() {
    fn test<T: PrimitiveFloat>(x: u64, y: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_unsigned_pow(x, y)),
            NiceFloat(out)
        );
    }
    test::<f32>(0, f32::NAN, f32::NAN);
    test::<f32>(0, f32::INFINITY, 0.0);
    test::<f32>(0, f32::NEGATIVE_INFINITY, f32::INFINITY);
    test::<f32>(0, 0.0, 1.0);
    test::<f32>(0, -0.0, 1.0);
    test::<f32>(0, 0.5, 0.0);
    test::<f32>(0, -1.0, f32::INFINITY);
    test::<f32>(0, 2.0, 0.0);

    test::<f32>(1, f32::NAN, 1.0);
    test::<f32>(1, f32::INFINITY, 1.0);
    test::<f32>(1, f32::NEGATIVE_INFINITY, 1.0);
    test::<f32>(1, 0.0, 1.0);
    test::<f32>(1, -0.0, 1.0);
    test::<f32>(1, 0.5, 1.0);
    test::<f32>(1, -1.0, 1.0);
    test::<f32>(1, 2.0, 1.0);

    test::<f32>(2, f32::NAN, f32::NAN);
    test::<f32>(2, f32::INFINITY, f32::INFINITY);
    test::<f32>(2, f32::NEGATIVE_INFINITY, 0.0);
    test::<f32>(2, 0.0, 1.0);
    test::<f32>(2, -0.0, 1.0);
    test::<f32>(2, 0.5, std::f32::consts::SQRT_2);
    test::<f32>(2, -1.0, 0.5);
    test::<f32>(2, 2.0, 4.0);
    test::<f32>(3, 2.5, 15.588457);
    test::<f32>(10, -0.5, 0.31622776);
    test::<f32>(2, 200.0, f32::INFINITY);
    test::<f32>(2, -200.0, 0.0);

    // - negative exponents with subnormal results, exercising the reduced-precision re-round
    test::<f32>(2, -140.0, 7.17e-43);
    test::<f32>(3, -80.0, 6.765496e-39);
    test::<f32>(3, -80.5, 3.90606e-39);
    test::<f64>(2, -1060.0, 8.095e-320);
    test::<f64>(3, -660.5, 7.26793875e-316);

    test::<f64>(2, 0.5, std::f64::consts::SQRT_2);
    test::<f64>(3, 2.5, 15.588457268119896);
    test::<f64>(2, -1.0, 0.5);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_unsigned_pow_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_unsigned_pair_gen_var_1::<T, u64>().test_properties(|(y, x)| {
        primitive_float_unsigned_pow::<T>(x, y);
    });
    // The var_1 floats are positive and finite; this generator adds negative, zero, infinite, and
    // NaN exponents, whose negative-and-large members exercise the subnormal and deep-underflow
    // result paths.
    primitive_float_unsigned_pair_gen_var_4::<T, u64>().test_properties(|(y, x)| {
        primitive_float_unsigned_pow::<T>(x, y);
    });
}

#[test]
fn primitive_float_unsigned_pow_properties() {
    apply_fn_to_primitive_floats!(primitive_float_unsigned_pow_properties_helper);
}

#[test]
fn test_unsigned_pow_rational() {
    let test = |x: u64, s: &str, prec: u64, rm, out: &str, out_hex: &str, o_out| {
        let q = Rational::from_str(s).unwrap();
        let (p, o) = Float::unsigned_pow_rational_prec_round(x, q, prec, rm);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);
        assert_eq!(o, o_out);
    };
    // - exact perfect powers: k = j^b gives k^(a/b) = j^a exactly
    test(8, "1/3", 20, Nearest, "2.0", "0x2.00000#20", Equal);
    test(27, "1/3", 20, Floor, "3.0", "0x3.00000#20", Equal);
    test(9, "1/2", 20, Nearest, "3.0", "0x3.00000#20", Equal);
    test(16, "3/4", 20, Nearest, "8.0", "0x8.0000#20", Equal);
    test(4, "-1/2", 20, Nearest, "0.5", "0x0.80000#20", Equal);
    test(1000000, "1/3", 20, Nearest, "100.0", "0x64.0000#20", Equal);
    test(64, "5/6", 30, Nearest, "32.0", "0x20.000000#30", Equal);
    // - power-of-2 base: k = 2^s gives k^q = 2^(s*q)
    test(
        2,
        "1/2",
        53,
        Nearest,
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        Greater,
    );
    test(8, "2/3", 20, Nearest, "4.0", "0x4.00000#20", Equal);
    test(2, "-1", 10, Nearest, "0.5", "0x0.800#10", Equal);
    test(4, "1/3", 30, Floor, "1.587401051", "0x1.965fea50#30", Less);
    // - irrational results
    test(
        3,
        "1/2",
        53,
        Nearest,
        "1.7320508075688772",
        "0x1.bb67ae8584caa#53",
        Less,
    );
    test(
        5,
        "2/3",
        53,
        Nearest,
        "2.924017738212866",
        "0x2.ec8c6d2e8c538#53",
        Less,
    );
    test(
        7,
        "-3/5",
        30,
        Nearest,
        "0.311129489",
        "0x0.4fa62ea4#30",
        Less,
    );
    test(3, "1/2", 2, Floor, "1.5", "0x1.8#2", Less);
    test(3, "1/2", 2, Ceiling, "2.0", "0x2.0#2", Greater);
    // - integer-valued exponents
    test(5, "3", 20, Nearest, "125.0", "0x7d.0000#20", Equal);
    test(2, "10", 20, Nearest, "1024.0", "0x400.000#20", Equal);
    // - overflow and underflow
    test(
        2,
        "1000000000000",
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(2, "-1000000000000", 10, Nearest, "0.0", "0x0.0", Less);
    test(
        3,
        "5000000000/7",
        5,
        Down,
        "too_big",
        "0x7.cE+268435455#5",
        Less,
    );
    // - Exact rounding of an exactly-representable result
    test(8, "1/3", 20, Exact, "2.0", "0x2.00000#20", Equal);
    test(27, "1/3", 30, Exact, "3.0", "0x3.0000000#30", Equal);
    // - a denominator exceeding u64::MAX (the perfect-power check is skipped)
    test(
        3,
        "1/18446744073709551617",
        20,
        Nearest,
        "1.0",
        "0x1.00000#20",
        Less,
    );
    test(
        3,
        "-1/18446744073709551617",
        20,
        Nearest,
        "1.0",
        "0x1.00000#20",
        Greater,
    );
    // - near-1 fast path: for a tiny q, k^q is within a few ulps of 1, so it is computed as exp(q *
    //   ln k) from a low-precision ln(k) rather than the full log2(k) squeeze. (See
    //   `power_of_10_rational_near_one_compute_huge` for the extreme-precision version.)
    test(
        10,
        "3/36028797018963968",
        53,
        Floor,
        "1.0",
        "0x1.0000000000000#53",
        Less,
    );
    test(
        10,
        "3/36028797018963968",
        53,
        Ceiling,
        "1.0000000000000002",
        "0x1.0000000000001#53",
        Greater,
    );
    test(
        10,
        "3/36028797018963968",
        53,
        Nearest,
        "1.0000000000000002",
        "0x1.0000000000001#53",
        Greater,
    );
    test(
        10,
        "-3/36028797018963968",
        53,
        Nearest,
        "0.9999999999999998",
        "0x0.fffffffffffff0#53",
        Less,
    );
    test(
        7,
        "-3/73786976294838206464",
        64,
        Nearest,
        "0.99999999999999999995",
        "0x0.ffffffffffffffff#64",
        Greater,
    );
    test(
        1000000,
        "3/4503599627370496",
        50,
        Nearest,
        "1.000000000000009",
        "0x1.0000000000028#50",
        Less,
    );

    // - Ziv growth in the squeeze: 6^(1 + 2^-300) lies within 2^-300 of the rounding boundary 6.0,
    //   so the initial working precision cannot separate the brackets and must grow
    let q = Rational::ONE + (Rational::ONE >> 300u32);
    let (p, o) = Float::unsigned_pow_rational_prec_round(6, q.clone(), 53, Floor);
    assert_eq!(p.to_string(), "6.0");
    assert_eq!(to_hex_string(&p), "0x6.0000000000000#53");
    assert_eq!(o, Less);
    let (p, o) = Float::unsigned_pow_rational_prec_round(6, q, 53, Ceiling);
    assert_eq!(p.to_string(), "6.000000000000001");
    assert_eq!(to_hex_string(&p), "0x6.0000000000004#53");
    assert_eq!(o, Greater);
}

#[test]
fn test_unsigned_pow_rational_special_values() {
    let test = |x: u64, s: &str, out: &str, out_hex: &str| {
        let q = Rational::from_str(s).unwrap();
        let (p, o) = Float::unsigned_pow_rational_prec_round(x, q, 1, Nearest);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);
        assert_eq!(o, Equal);
    };
    // - 0^q = 0 for q > 0, +Inf for q < 0, and 1 for q = 0
    test(0, "0", "1.0", "0x1.0#1");
    test(0, "1", "0.0", "0x0.0");
    test(0, "-1", "Infinity", "Infinity");
    test(0, "1/2", "0.0", "0x0.0");
    test(0, "-2", "Infinity", "Infinity");

    // - 1^q = 1 for any q
    test(1, "0", "1.0", "0x1.0#1");
    test(1, "1", "1.0", "0x1.0#1");
    test(1, "-1", "1.0", "0x1.0#1");
    test(1, "1/2", "1.0", "0x1.0#1");
    test(1, "-2", "1.0", "0x1.0#1");

    // - a normal base with exact integer/reciprocal exponents
    test(2, "0", "1.0", "0x1.0#1");
    test(2, "1", "2.0", "0x2.0#1");
    test(2, "-1", "0.5", "0x0.8#1");
    test(2, "-2", "0.2", "0x0.4#1");
}

#[allow(clippy::needless_pass_by_value)]
fn unsigned_pow_rational_prec_round_properties_helper(
    q: Rational,
    k: u64,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) {
    if rm == Exact {
        let (p, o) = Float::unsigned_pow_rational_prec_round_ref(k, &q, prec, Nearest);
        if o == Equal {
            let (pe, oe) = Float::unsigned_pow_rational_prec_round_ref(k, &q, prec, Exact);
            assert_eq!(ComparableFloatRef(&pe), ComparableFloatRef(&p));
            assert_eq!(oe, Equal);
        } else {
            assert_panic!(Float::unsigned_pow_rational_prec_round_ref(
                k, &q, prec, Exact
            ));
        }
        return;
    }
    let (p, o) = Float::unsigned_pow_rational_prec_round_ref(k, &q, prec, rm);
    assert!(p.is_valid());
    let (p_alt, o_alt) = Float::unsigned_pow_rational_prec_round(k, q.clone(), prec, rm);
    assert!(p_alt.is_valid());
    assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
    assert_eq!(o_alt, o);

    // For a dyadic exponent, q is exactly a Float, so k^q must match `rational_pow` (an independent
    // implementation) on the exact Rational base.
    if q.denominator_ref().is_power_of_2()
        && q.numerator_ref().significant_bits() < 4096
        && q.denominator_ref().significant_bits() < 4096
    {
        let yf = Float::exact_from(&q);
        let (pr, or) = Float::rational_pow_prec_round_ref_val(&Rational::from(k), yf, prec, rm);
        assert_eq!(ComparableFloatRef(&pr), ComparableFloatRef(&p));
        assert_eq!(or, o);
    }

    // The bracketing exp(q * ln k) oracle handles every regime -- boundaries and near-1 results
    // included -- except an exactly-rational k^q, which its escalating brackets could never
    // resolve. k^q is rational exactly when k is a perfect b-th power (b the denominator of q), for
    // example 7^1 or 8^(1/3); those cases are covered by the exact-value checks above.
    let k_perfect_power = u64::try_from(q.denominator_ref())
        .ok()
        .and_then(|b| k.checked_root(b))
        .is_some();
    if k >= 2
        && !k_perfect_power
        && let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
    {
        let (rp, ro) = rug_unsigned_pow_rational_prec_round(k, &q, prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rp)),
            ComparableFloatRef(&p)
        );
        assert_eq!(ro, o);
    }

    if p.is_normal() && !extreme {
        assert_eq!(p.get_prec(), Some(prec));
    }
}

#[test]
fn unsigned_pow_rational_prec_round_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_2().test_properties(
        |(q, k, prec, rm)| {
            unsigned_pow_rational_prec_round_properties_helper(q, k, prec, rm, false);
        },
    );
}

#[test]
fn unsigned_pow_rational_prec_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_2().test_properties(
        |(q, k, prec, _)| {
            let (p, o) = Float::unsigned_pow_rational_prec(k, q.clone(), prec);
            assert!(p.is_valid());
            let (p_alt, o_alt) = Float::unsigned_pow_rational_prec_ref(k, &q, prec);
            assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
            assert_eq!(o_alt, o);
            let (p_alt, o_alt) = Float::unsigned_pow_rational_prec_round_ref(k, &q, prec, Nearest);
            assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
            assert_eq!(o_alt, o);
        },
    );
}

// Regression test: `pow_integer`'s estimate-based overflow pre-bound leaves a window of true
// overflows just above MAX_EXPONENT that the entry check's 64-bit lower bound also misses. Such
// inputs formerly reached `pow_pos_natural`, whose magnitude-decreasing roundings saturated at the
// largest finite value -- which `float_can_round` can never certify -- and the Ziv loop grew
// forever. The window is now decided exactly.
#[test]
fn test_pow_integer_overflow_window_regression() {
    // - z * log2(x) in [MAX_EXPONENT + 2^-40, MAX_EXPONENT + 2^-33): x = 2^t rounded up at 200
    //   bits, t = (MAX_EXPONENT + 2^-40) / z, so z * log2(x) >= MAX_EXPONENT + 2^-40 (formerly
    //   hung)
    let z = (1u64 << 30) + 1;
    let t = (Rational::from(Float::MAX_EXPONENT) + (Rational::ONE >> 40u32)) / Rational::from(z);
    let x = Float::power_of_2_rational_prec_round(t, 200, Ceiling).0;
    let (v, o) = x.pow_integer_prec_round_ref_val(Integer::from(z), 53, Nearest);
    assert!(v.is_infinite());
    assert_eq!(o, Greater);
    let (v, o) = x.pow_integer_prec_round_ref_val(Integer::from(z), 53, Floor);
    assert!(!v.is_infinite());
    assert_eq!(v.get_exponent(), Some(Float::MAX_EXPONENT));
    assert_eq!(o, Less);
}

// Regression tests: the true product y * ln|x| below the Float exponent range formerly derailed
// pow_general (MPFR computes it in an extended exponent range, which malachite lacks). A negative
// product underflowed to -0.0, making exp return exactly 1, which `float_can_round` can never
// certify -- an infinite loop; a positive product saturated at the minimum positive value, whose
// unaccounted error let the loop certify a wrongly rounded result on the wrong side of the
// `Nearest` tie. Both are ~64-128 MB computations.
#[test]
fn test_pow_general_tiny_product_regression() {
    let min_exp = i64::from(Float::MIN_EXPONENT);
    // - negative product (formerly hung): x = 1 - 2^(MIN_EXPONENT + 300) at prec -MIN_EXPONENT -
    //   299, y = 2^-302; x^y = 1 - ~2^(MIN_EXPONENT - 2)
    let xk = u64::exact_from(-(min_exp + 300));
    let x = Float::from_rational_prec(Rational::ONE - (Rational::ONE >> xk), xk).0;
    let y = Float::power_of_2(-302i64);
    let (v, o) = Float::pow_prec_round_ref_ref(&x, &y, 301, Nearest);
    assert_eq!(v, 1u32);
    assert_eq!(o, Greater);
    let (v, o) = Float::pow_prec_round_ref_ref(&x, &y, 301, Floor);
    assert!(v < 1u32);
    assert_eq!(o, Less);
    // - positive product (formerly a silent wrong value, 1 + 2^(1 - 2^30) with ternary Greater): x
    //   = 1 + 2^(-2^29) at prec 2^29 + 1, y = 2^(-2^29 - 2), prec 2^30; the true result 1 +
    //   ~2^(-2^30 - 2) lies below the `Nearest` tie, so the answer is exactly 1 with Less
    let s29 = 1u64 << 29;
    let x = Float::from_rational_prec(Rational::ONE + (Rational::ONE >> s29), s29 + 1).0;
    let y = Float::power_of_2(-(1i64 << 29) - 2);
    let (v, o) = Float::pow_prec_round_ref_ref(&x, &y, 1u64 << 30, Nearest);
    assert_eq!(v, 1u32);
    assert_eq!(o, Less);
}

// The near-1 fast path: for x = +/-(1 + d) with |z||d| tiny, the result is rounded directly from
// +/-1 (or the Ziv loop is jump-started) instead of ballooning through ~log(|EXP(d)|) recomputes.
// Before the fast path, the pow_u rows here took ~milliseconds and the pow_integer rows ~seconds
// for B = 10^6; all are now microseconds.
#[test]
fn test_pow_near_one_fast_path() {
    // x = 1 +/- 2^-100, exactly representable with 101 bits
    let xp = Float::one_prec(2)
        .add_prec(Float::power_of_2(-100i64), 101)
        .0;
    let xm = Float::one_prec(2)
        .add_prec(-Float::power_of_2(-100i64), 101)
        .0;
    let test = |v: (Float, Ordering), out: &str, out_hex: &str, o_out| {
        assert!(v.0.is_valid());
        assert_eq!(v.0.to_string(), out);
        assert_eq!(to_hex_string(&v.0), out_hex);
        assert_eq!(v.1, o_out);
    };
    // - rounded directly from 1: all rounding modes, both signs of d
    test(
        xp.pow_u_prec_round_ref(3, 53, Floor),
        "1.0",
        "0x1.0000000000000#53",
        Less,
    );
    test(
        xp.pow_u_prec_round_ref(3, 53, Ceiling),
        "1.0000000000000002",
        "0x1.0000000000001#53",
        Greater,
    );
    test(
        xp.pow_u_prec_round_ref(3, 53, Nearest),
        "1.0",
        "0x1.0000000000000#53",
        Less,
    );
    test(
        xm.pow_u_prec_round_ref(65535, 53, Floor),
        "0.9999999999999999",
        "0x0.fffffffffffff8#53",
        Less,
    );
    test(
        xm.pow_u_prec_round_ref(65535, 53, Nearest),
        "1.0",
        "0x1.0000000000000#53",
        Greater,
    );
    // - negative z (the reciprocal-power path)
    test(
        xp.pow_integer_prec_round_ref_ref(&-Integer::from(1000), 53, Floor),
        "0.9999999999999999",
        "0x0.fffffffffffff8#53",
        Less,
    );
    test(
        xp.pow_integer_prec_round_ref_ref(&-Integer::from(1000), 53, Nearest),
        "1.0",
        "0x1.0000000000000#53",
        Greater,
    );
    // - negative x with odd z: rounded from -1 with the mirrored rounding mode
    test(
        (-xp.clone()).pow_u_prec_round_ref(3, 53, Floor),
        "-1.0000000000000002",
        "-0x1.0000000000001#53",
        Less,
    );
    test(
        (-xp.clone()).pow_u_prec_round_ref(3, 53, Nearest),
        "-1.0",
        "-0x1.0000000000000#53",
        Greater,
    );
    test(
        (-xm.clone()).pow_integer_prec_round_ref_ref(&-Integer::from(999), 53, Nearest),
        "-1.0",
        "-0x1.0000000000000#53",
        Greater,
    );
    // - jump start: the result's bits land within the output window ((1 + 2^-100)^3 = 1 + 3*2^-100
    //   + ... at prec 150), so the Ziv loop runs, starting at a working precision that covers the
    //   leading run of 0s or 9s
    test(
        xp.pow_u_prec_round_ref(3, 150, Nearest),
        "1.000000000000000000000000000002366582715663035",
        "0x1.00000000000000000000000030000000000000#150",
        Less,
    );
    test(
        xp.pow_integer_prec_round_ref_ref(&-Integer::from(3), 150, Nearest),
        "0.9999999999999999999999999999976334172843369646",
        "0x0.ffffffffffffffffffffffffd0000000000000#150",
        Less,
    );
}
