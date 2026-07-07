// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::apply_fn_to_primitive_floats;
use malachite_base::assert_panic;
use malachite_base::num::arithmetic::traits::{IsPowerOf2, Pow, PowAssign, PowerOf2};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, Zero,
};
use malachite_base::num::conversion::traits::{ExactFrom, IsInteger, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::primitive_float_pair_gen;
use malachite_float::arithmetic::pow::{
    primitive_float_pow, primitive_float_pow_integer, primitive_float_rational_pow,
};
use malachite_float::test_util::arithmetic::pow::{
    rug_pow, rug_pow_integer, rug_pow_integer_prec, rug_pow_integer_prec_round,
    rug_pow_integer_round, rug_pow_prec, rug_pow_prec_round, rug_pow_round, rug_pow_u,
    rug_pow_u_prec, rug_pow_u_prec_round, rug_pow_u_round,
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
    float_rational_unsigned_triple_gen_var_1, float_unsigned_pair_gen,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_9,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_10,
    float_unsigned_unsigned_triple_gen_var_1,
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
