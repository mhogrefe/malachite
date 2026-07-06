// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::assert_panic;
use malachite_base::num::arithmetic::traits::{Pow, PowAssign};
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeZero, One, Zero,
};
use malachite_base::num::conversion::traits::{ExactFrom, IsInteger};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::test_util::arithmetic::pow::{
    rug_pow, rug_pow_prec, rug_pow_prec_round, rug_pow_round,
};
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::generators::{
    float_float_unsigned_rounding_mode_quadruple_gen_var_9,
    float_float_unsigned_rounding_mode_quadruple_gen_var_10, float_float_unsigned_triple_gen_var_1,
    float_pair_gen, float_pair_gen_var_10,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

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
