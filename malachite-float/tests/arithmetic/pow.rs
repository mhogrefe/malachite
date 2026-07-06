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
use malachite_float::test_util::common::rug_round_try_from_rounding_mode;
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
    let test = |x: f64, y: f64, prec: u64, rm: RoundingMode, out: &str, o_out: Ordering| {
        let (p, o) = Float::from(x).pow_prec_round(Float::from(y), prec, rm);
        assert_eq!(p.to_string(), out);
        assert_eq!(o, o_out);
    };
    test(2.0, 0.5, 53, Nearest, "1.4142135623730951", Greater);
    test(3.0, 100.0, 53, Nearest, "5.153775207320113e47", Less);
    test(2.0, 10.0, 53, Nearest, "1024.0", Equal);
    test(0.5, 2.0, 53, Nearest, "0.25", Equal);
    test(10.0, -1.0, 53, Nearest, "0.10000000000000001", Greater);
    test(2.0, 0.5, 53, Floor, "1.4142135623730949", Less);
    test(1.5, 1.5, 53, Nearest, "1.8371173070873836", Greater);
    test(-2.0, 3.0, 53, Nearest, "-8.0", Equal);
    test(-2.0, -3.0, 53, Nearest, "-0.125", Equal);
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
            ComparableFloatRef(&p),
            "boundary case dz={dz}"
        );
        assert_eq!(rug_o, o, "boundary ordering dz={dz}");
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
        assert_eq!(o, Equal, "prec {prec}");
        assert_eq!(
            i64::from(p.get_exponent().unwrap()),
            i64::from(Float::MIN_EXPONENT),
            "prec {prec}"
        );
        let (rug_p, rug_o) = rug_pow_prec_round(
            &rug::Float::exact_from(&x),
            &rug::Float::exact_from(&y),
            prec,
            rug::float::Round::Nearest,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_p)),
            ComparableFloatRef(&p),
            "prec {prec}"
        );
        assert_eq!(rug_o, o, "prec {prec}");
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
        assert_eq!(o, Equal, "rm {rm}");
        assert_eq!(
            i64::from(p.get_exponent().unwrap()),
            i64::from(Float::MIN_EXPONENT),
            "rm {rm}"
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
                ComparableFloatRef(&p),
                "rm {rm}"
            );
            assert_eq!(rug_o, o, "rm {rm}");
        }
    }
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
