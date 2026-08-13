// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::max;
use malachite_base::num::arithmetic::traits::{Average, AverageAssign, PowerOf2};
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeZero, One, Two, Zero,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::test_util::generators::float_float_unsigned_rounding_mode_quadruple_gen_var_15;
use malachite_float::{ComparableFloatRef, Float};
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};

// The largest exponent a `Float` may have is `Float::MAX_EXPONENT`, and a power of two $2^k$ has
// exponent $k+1$, so these are the largest and smallest powers of two.
const MAX_POW: i64 = (1 << 30) - 2;
const MIN_POW: i64 = -(1 << 30);

fn expect(x: Float, y: Float, prec: u64, rm: RoundingMode, out: Float, o: Ordering) {
    // the output of `average_prec_round` has precision `prec`; each expected value here is exactly
    // representable at that precision, so re-rounding it only relabels the precision
    let (out, out_o) = Float::from_float_prec(out, prec);
    assert_eq!(out_o, Equal);
    let (a, ao) = x.clone().average_prec_round(y.clone(), prec, rm);
    assert_eq!(
        (ComparableFloatRef(&a), ao),
        (ComparableFloatRef(&out), o),
        "average({x}, {y}, {prec}, {rm})"
    );
    // averaging is symmetric
    let (b, bo) = y.average_prec_round(x, prec, rm);
    assert_eq!((ComparableFloatRef(&b), bo), (ComparableFloatRef(&out), o));
}

#[test]
fn test_average_extremes() {
    let max_pow = Float::power_of_2(MAX_POW);
    let min_pow = Float::power_of_2(MIN_POW);
    for rm in [Floor, Ceiling, Down, Up, Nearest, Exact] {
        // - the sum would overflow, but the average is exactly representable
        expect(
            max_pow.clone(),
            max_pow.clone(),
            10,
            rm,
            Float::power_of_2(MAX_POW),
            Equal,
        );
        expect(
            -max_pow.clone(),
            -max_pow.clone(),
            10,
            rm,
            -Float::power_of_2(MAX_POW),
            Equal,
        );
        // - halving would underflow, but the average is exactly representable
        expect(
            min_pow.clone(),
            min_pow.clone(),
            10,
            rm,
            Float::power_of_2(MIN_POW),
            Equal,
        );
        // - the average of adjacent extreme powers of two is exact at precision 2
        expect(
            max_pow.clone(),
            Float::power_of_2(MAX_POW - 1),
            2,
            rm,
            Float::power_of_2(MAX_POW - 1) * Float::from(1.5),
            Equal,
        );
        expect(
            min_pow.clone(),
            Float::power_of_2(MIN_POW + 1),
            2,
            rm,
            Float::power_of_2(MIN_POW) * Float::from(1.5),
            Equal,
        );
    }

    // - exact cancellation keeps the sum's zero, which is negative only under Floor
    for rm in [Ceiling, Down, Up, Nearest] {
        expect(
            min_pow.clone(),
            -min_pow.clone(),
            10,
            rm,
            Float::ZERO,
            Equal,
        );
        expect(
            max_pow.clone(),
            -max_pow.clone(),
            10,
            rm,
            Float::ZERO,
            Equal,
        );
    }
    expect(
        min_pow.clone(),
        -min_pow.clone(),
        10,
        Floor,
        Float::NEGATIVE_ZERO,
        Equal,
    );

    // - the average of the smallest positive value and zero underflows; it is exactly half the
    //   smallest value, so Nearest breaks the tie toward zero
    expect(min_pow.clone(), Float::ZERO, 10, Floor, Float::ZERO, Less);
    expect(min_pow.clone(), Float::ZERO, 10, Down, Float::ZERO, Less);
    expect(min_pow.clone(), Float::ZERO, 10, Nearest, Float::ZERO, Less);
    expect(
        min_pow.clone(),
        Float::ZERO,
        10,
        Ceiling,
        Float::power_of_2(MIN_POW),
        Greater,
    );
    expect(
        min_pow.clone(),
        Float::ZERO,
        10,
        Up,
        Float::power_of_2(MIN_POW),
        Greater,
    );

    // - a huge value paired with the smallest positive one: the tiny value survives only as a
    //   sticky bit, nudging the result off the exact half
    expect(
        max_pow.clone(),
        min_pow.clone(),
        10,
        Floor,
        Float::power_of_2(MAX_POW - 1),
        Less,
    );
    expect(
        max_pow.clone(),
        min_pow.clone(),
        10,
        Nearest,
        Float::power_of_2(MAX_POW - 1),
        Less,
    );
    expect(
        max_pow.clone(),
        min_pow.clone(),
        10,
        Ceiling,
        // the next `Float` above 2^(MAX_POW-1) at precision 10
        Float::power_of_2(MAX_POW - 1)
            .add_prec(Float::power_of_2(MAX_POW - 10), 10)
            .0,
        Greater,
    );
}

#[test]
fn test_average_specials() {
    // NaN, infinities, and zeros follow the sum, since halving leaves each unchanged
    for rm in [Floor, Ceiling, Down, Up, Nearest] {
        let f = |x: Float, y: Float| {
            let (a, o) = x.clone().average_prec_round(y.clone(), 10, rm);
            let (s, so) = x.add_prec_round(y, 10, rm);
            assert_eq!((ComparableFloatRef(&a), o), (ComparableFloatRef(&s), so));
        };
        f(Float::NAN, Float::ONE);
        f(Float::NAN, Float::NAN);
        f(Float::INFINITY, Float::ONE);
        f(Float::NEGATIVE_INFINITY, Float::ONE);
        f(Float::INFINITY, Float::NEGATIVE_INFINITY);
        f(Float::INFINITY, Float::INFINITY);
        f(Float::ZERO, Float::ZERO);
        f(Float::NEGATIVE_ZERO, Float::NEGATIVE_ZERO);
        f(Float::ZERO, Float::NEGATIVE_ZERO);
    }
    // a zero paired with a finite nonzero value halves that value
    for rm in [Floor, Ceiling, Down, Up, Nearest] {
        expect(Float::ZERO, Float::TWO, 10, rm, Float::ONE, Equal);
        expect(Float::NEGATIVE_ZERO, Float::TWO, 10, rm, Float::ONE, Equal);
        expect(Float::ZERO, -Float::TWO, 10, rm, -Float::ONE, Equal);
    }
}

#[test]
fn average_prec_round_properties() {
    float_float_unsigned_rounding_mode_quadruple_gen_var_15().test_properties(
        |(x, y, prec, rm)| {
            let (avg, o) = x.clone().average_prec_round(y.clone(), prec, rm);
            assert!(avg.is_valid());
            // every ownership variant agrees
            for (a, ao) in [
                x.clone().average_prec_round_val_ref(&y, prec, rm),
                x.average_prec_round_ref_val(y.clone(), prec, rm),
                x.average_prec_round_ref_ref(&y, prec, rm),
            ] {
                assert_eq!(ComparableFloatRef(&a), ComparableFloatRef(&avg));
                assert_eq!(ao, o);
            }
            // the in-place forms agree
            let mut m = x.clone();
            assert_eq!(m.average_prec_round_assign(y.clone(), prec, rm), o);
            assert_eq!(ComparableFloatRef(&m), ComparableFloatRef(&avg));
            let mut m = x.clone();
            assert_eq!(m.average_prec_round_assign_ref(&y, prec, rm), o);
            assert_eq!(ComparableFloatRef(&m), ComparableFloatRef(&avg));
            // Nearest is what the shorthands use
            if rm == Nearest {
                let (a, ao) = x.clone().average_prec(y.clone(), prec);
                assert_eq!(ComparableFloatRef(&a), ComparableFloatRef(&avg));
                assert_eq!(ao, o);
                let mut m = x.clone();
                assert_eq!(m.average_prec_assign(y.clone(), prec), o);
                assert_eq!(ComparableFloatRef(&m), ComparableFloatRef(&avg));
            }
            // the round-only form uses the maximum of the inputs' precisions
            if prec == max(x.significant_bits(), y.significant_bits()) {
                let (a, ao) = x.clone().average_round(y.clone(), rm);
                assert_eq!(ComparableFloatRef(&a), ComparableFloatRef(&avg));
                assert_eq!(ao, o);
                let mut m = x.clone();
                assert_eq!(m.average_round_assign(y.clone(), rm), o);
                assert_eq!(ComparableFloatRef(&m), ComparableFloatRef(&avg));
                if rm == Nearest {
                    // the trait is the round-only form with Nearest
                    assert_eq!(
                        ComparableFloatRef(&x.clone().average(y.clone())),
                        ComparableFloatRef(&avg)
                    );
                    assert_eq!(
                        ComparableFloatRef(&(&x).average(&y)),
                        ComparableFloatRef(&avg)
                    );
                    let mut m = x.clone();
                    m.average_assign(y.clone());
                    assert_eq!(ComparableFloatRef(&m), ComparableFloatRef(&avg));
                }
            }
            // averaging is symmetric, and the ordering is reported the same way
            let (avg_alt, o_alt) = y.clone().average_prec_round(x.clone(), prec, rm);
            assert_eq!(ComparableFloatRef(&avg_alt), ComparableFloatRef(&avg));
            assert_eq!(o_alt, o);
            if avg.is_finite() && avg != 0u32 {
                assert_eq!(avg.get_prec(), Some(prec));
            }
            if x.is_nan() || y.is_nan() {
                assert!(avg.is_nan());
                return;
            }
            // the exact average, when both inputs have moderate exponents; a rational recomputation
            // is prohibitively large otherwise
            let moderate = |f: &Float| f.get_exponent().is_none_or(|e| e.unsigned_abs() < 1000);
            if x.is_finite() && y.is_finite() && moderate(&x) && moderate(&y) {
                let q = (Rational::exact_from(&x) + Rational::exact_from(&y)) >> 1u64;
                if q == 0u32 {
                    assert_eq!(avg, 0u32);
                    assert_eq!(o, Equal);
                } else {
                    let (e, eo) = Float::from_rational_prec_round(q.clone(), prec, rm);
                    assert_eq!(ComparableFloatRef(&avg), ComparableFloatRef(&e));
                    assert_eq!(o, eo);
                    // the ordering describes the result's position relative to the exact average
                    assert_eq!(Rational::exact_from(&avg).cmp(&q), o);
                }
                // The exact average lies between the inputs. The rounded average need not: at a
                // coarse precision the nearest representable value can sit outside the interval.
                let (lo, hi) = if x <= y { (&x, &y) } else { (&y, &x) };
                assert!(q >= Rational::exact_from(lo));
                assert!(q <= Rational::exact_from(hi));
            }
            // an exact result is the same under every rounding mode; a zero is exempt, since the
            // sign of an exactly cancelling average is negative only under Floor
            if o == Equal && avg != 0u32 {
                for rm_alt in [Floor, Ceiling, Down, Up, Nearest, Exact] {
                    let (avg_alt, o_alt) = x.clone().average_prec_round(y.clone(), prec, rm_alt);
                    assert_eq!(ComparableFloatRef(&avg_alt), ComparableFloatRef(&avg));
                    assert_eq!(o_alt, Equal);
                }
            }
            // Negating both inputs negates the average and reverses both the mode and the ordering.
            // A zero result is exempt: the sign of a zero average follows the sign rule for a zero
            // sum, which is positive under every mode but Floor and so is not antisymmetric.
            if avg != 0u32 {
                let (neg, neg_o) = (-x).average_prec_round(-y, prec, -rm);
                assert_eq!(ComparableFloatRef(&neg), ComparableFloatRef(&-avg));
                assert_eq!(neg_o, o.reverse());
            }
        },
    );
}
