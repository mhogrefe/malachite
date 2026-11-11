// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, OneHalf, Zero};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_negate, big_exponent_float_power_of_2, float_to_big_exponent_float,
    parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_pair_gen, big_exponent_float_rational_pair_gen, float_pair_gen,
    float_rational_pair_gen,
};
use malachite_float::{BigExponentFloat, Float};
use malachite_nz::integer::Integer;
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};

const PRECS: [u64; 5] = [1, 2, 10, 64, 100];
const RMS: [RoundingMode; 5] = [Floor, Ceiling, Down, Up, Nearest];

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

#[test]
fn test_min_max() {
    let big = Integer::from_str("100000000000000000000").unwrap();
    let huge = big_exponent_float_power_of_2(big.clone());
    let tiny = big_exponent_float_power_of_2(-big);
    let neg_huge = big_exponent_float_negate(huge.clone());
    let pi = parse_big_exponent_float_hex("0x3.243f6a8885a30#53");
    let test = |x: &BigExponentFloat,
                y: &BigExponentFloat,
                min: &BigExponentFloat,
                max: &BigExponentFloat| {
        assert!(same(&x.clone().min(y.clone()), min));
        assert!(same(&x.clone().min_val_ref(y), min));
        assert!(same(&x.min_ref_val(y.clone()), min));
        assert!(same(&x.min_ref_ref(y), min));
        assert!(same(&x.clone().max(y.clone()), max));
        assert!(same(&x.clone().max_val_ref(y), max));
        assert!(same(&x.max_ref_val(y.clone()), max));
        assert!(same(&x.max_ref_ref(y), max));
    };
    let nan = BigExponentFloat::NAN;
    test(&nan, &nan, &nan, &nan);
    test(&nan, &pi, &pi, &pi);
    test(&pi, &nan, &pi, &pi);
    test(&nan, &huge, &huge, &huge);
    test(&BigExponentFloat::ZERO, &pi, &BigExponentFloat::ZERO, &pi);
    test(&pi, &BigExponentFloat::ONE, &BigExponentFloat::ONE, &pi);
    test(&pi, &huge, &pi, &huge);
    test(&neg_huge, &tiny, &neg_huge, &tiny);
    test(
        &tiny,
        &BigExponentFloat::ZERO,
        &BigExponentFloat::ZERO,
        &tiny,
    );
    test(
        &BigExponentFloat::NEGATIVE_INFINITY,
        &BigExponentFloat::INFINITY,
        &BigExponentFloat::NEGATIVE_INFINITY,
        &BigExponentFloat::INFINITY,
    );
    test(
        &BigExponentFloat::ONE_HALF,
        &BigExponentFloat::ONE_HALF,
        &BigExponentFloat::ONE_HALF,
        &BigExponentFloat::ONE_HALF,
    );

    // - rounding the selected operand
    let (x, o) = pi.min_prec_round_ref_ref(&huge, 5, Floor);
    assert_eq!(format!("{x:?}"), "0x3.2");
    assert_eq!(o, Less);
    let (x, o) = pi.max_prec_round_ref_ref(&huge, 5, Floor);
    assert!(same(&x, &huge));
    assert_eq!(o, Equal);
    let (x, o) = nan.min_prec_ref_ref(&nan, 5);
    assert!(x.is_nan());
    assert_eq!(o, Equal);

    // - a Rational
    let third = Rational::from_signeds(1, 3);
    let (x, o) = tiny.min_rational_prec_round_ref_ref(&third, 10, Floor);
    assert!(same(&x, &tiny));
    assert_eq!(o, Equal);
    let (x, o) = tiny.max_rational_prec_round_ref_ref(&third, 10, Floor);
    assert_eq!(format!("{x:?}"), "0x0.554");
    assert_eq!(o, Less);
    let (x, o) = nan.max_rational_prec_ref_ref(&third, 10);
    assert_eq!(format!("{x:?}"), "0x0.556");
    assert_eq!(o, Greater);
    let (x, o) = huge.min_rational_prec_ref_ref(&-third.clone(), 10);
    assert_eq!(format!("{x:?}"), "-0x0.556");
    assert_eq!(o, Less);
    // - a tie selects the BigExponentFloat
    let (x, o) =
        BigExponentFloat::ONE_HALF.min_rational_prec_round_ref_ref(&Rational::ONE_HALF, 1, Exact);
    assert!(same(&x, &BigExponentFloat::ONE_HALF));
    assert_eq!(o, Equal);
    // - the comparison is exact: 0x0.554 is the 10-bit Floor of 1/3, but less than it
    let x = parse_big_exponent_float_hex("0x0.554#10");
    let y = x.max_rational_prec_round_ref_ref(&third, 64, Nearest).0;
    assert_eq!(format!("{y:?}"), "0x0.55555555555555558");
}

#[test]
#[should_panic]
fn min_prec_round_fail() {
    BigExponentFloat::NAN.min_prec_round(BigExponentFloat::NAN, 0, Floor);
}

#[test]
#[should_panic]
fn max_prec_round_fail() {
    BigExponentFloat::from(3).max_prec_round(BigExponentFloat::ONE, 1, Exact);
}

#[test]
#[should_panic]
fn min_rational_prec_round_fail() {
    BigExponentFloat::NAN.min_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn max_rational_prec_round_fail() {
    BigExponentFloat::ZERO.max_rational_prec_round(Rational::from_signeds(1, 3), 10, Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn min_max_properties_helper(x: BigExponentFloat, y: BigExponentFloat) {
    let min = x.min_ref_ref(&y);
    assert!(same(&x.clone().min(y.clone()), &min));
    assert!(same(&x.clone().min_val_ref(&y), &min));
    assert!(same(&x.min_ref_val(y.clone()), &min));
    let max = x.max_ref_ref(&y);
    assert!(same(&x.clone().max(y.clone()), &max));
    assert!(same(&x.clone().max_val_ref(&y), &max));
    assert!(same(&x.max_ref_val(y.clone()), &max));

    assert!(same(&y.min_ref_ref(&x), &min));
    assert!(same(&y.max_ref_ref(&x), &max));
    assert!(same(&min, &x) || same(&min, &y));
    assert!(same(&max, &x) || same(&max, &y));
    if x.is_nan() || y.is_nan() {
        let other = if x.is_nan() { &y } else { &x };
        assert!(same(&min, other));
        assert!(same(&max, other));
    } else {
        assert!(min <= x && min <= y);
        assert!(max >= x && max >= y);
        assert!((same(&min, &x) && same(&max, &y)) || (same(&min, &y) && same(&max, &x)));
    }

    for prec in PRECS {
        for rm in RMS {
            let (r, o) = x.min_prec_round_ref_ref(&y, prec, rm);
            let (r_alt, o_alt) = min.round_to_prec_round_ref(prec, rm);
            assert!(same(&r, &r_alt));
            assert_eq!(o, o_alt);
            let (r_alt, o_alt) = x.clone().min_prec_round(y.clone(), prec, rm);
            assert!(same(&r, &r_alt));
            assert_eq!(o, o_alt);
            let (r_alt, o_alt) = x.clone().min_prec_round_val_ref(&y, prec, rm);
            assert!(same(&r, &r_alt));
            assert_eq!(o, o_alt);
            let (r_alt, o_alt) = x.min_prec_round_ref_val(y.clone(), prec, rm);
            assert!(same(&r, &r_alt));
            assert_eq!(o, o_alt);

            let (r, o) = x.max_prec_round_ref_ref(&y, prec, rm);
            let (r_alt, o_alt) = max.round_to_prec_round_ref(prec, rm);
            assert!(same(&r, &r_alt));
            assert_eq!(o, o_alt);
            let (r_alt, o_alt) = x.clone().max_prec_round(y.clone(), prec, rm);
            assert!(same(&r, &r_alt));
            assert_eq!(o, o_alt);
            let (r_alt, o_alt) = x.clone().max_prec_round_val_ref(&y, prec, rm);
            assert!(same(&r, &r_alt));
            assert_eq!(o, o_alt);
            let (r_alt, o_alt) = x.max_prec_round_ref_val(y.clone(), prec, rm);
            assert!(same(&r, &r_alt));
            assert_eq!(o, o_alt);
        }
        let (r, o) = x.min_prec_ref_ref(&y, prec);
        let (r_alt, o_alt) = x.min_prec_round_ref_ref(&y, prec, Nearest);
        assert!(same(&r, &r_alt));
        assert_eq!(o, o_alt);
        for (r_alt, o_alt) in [
            x.clone().min_prec(y.clone(), prec),
            x.clone().min_prec_val_ref(&y, prec),
            x.min_prec_ref_val(y.clone(), prec),
        ] {
            assert!(same(&r, &r_alt));
            assert_eq!(o, o_alt);
        }
        let (r, o) = x.max_prec_ref_ref(&y, prec);
        let (r_alt, o_alt) = x.max_prec_round_ref_ref(&y, prec, Nearest);
        assert!(same(&r, &r_alt));
        assert_eq!(o, o_alt);
        for (r_alt, o_alt) in [
            x.clone().max_prec(y.clone(), prec),
            x.clone().max_prec_val_ref(&y, prec),
            x.max_prec_ref_val(y.clone(), prec),
        ] {
            assert!(same(&r, &r_alt));
            assert_eq!(o, o_alt);
        }
    }
}

#[test]
fn min_max_properties() {
    big_exponent_float_pair_gen().test_properties(|(x, y)| {
        min_max_properties_helper(x, y);
    });

    // - agreement with `Float`, whose selection rules are the same apart from the sign of zero
    float_pair_gen().test_properties(|(x, y)| {
        let bx = float_to_big_exponent_float(&x);
        let by = float_to_big_exponent_float(&y);
        assert!(same(
            &bx.min_ref_ref(&by),
            &float_to_big_exponent_float(&x.min_ref_ref(&y).0)
        ));
        assert!(same(
            &bx.max_ref_ref(&by),
            &float_to_big_exponent_float(&x.max_ref_ref(&y).0)
        ));
    });
}

// The result is the winner of the exact comparison, rounded. A NaN `BigExponentFloat` loses, and a
// tie goes to the `BigExponentFloat`.
fn rational_alt(
    x: &BigExponentFloat,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
    is_max: bool,
) -> (BigExponentFloat, Ordering) {
    let rational_wins = x
        .partial_cmp(y)
        .is_none_or(|o| o == if is_max { Less } else { Greater });
    if rational_wins {
        BigExponentFloat::from_rational_prec_round_ref(y, prec, rm)
    } else {
        x.round_to_prec_round_ref(prec, rm)
    }
}

#[allow(clippy::needless_pass_by_value)]
fn min_max_rational_properties_helper(x: BigExponentFloat, y: Rational) {
    for prec in PRECS {
        for rm in RMS {
            let (r, o) = x.min_rational_prec_round_ref_ref(&y, prec, rm);
            let (r_alt, o_alt) = rational_alt(&x, &y, prec, rm, false);
            assert!(same(&r, &r_alt));
            assert_eq!(o, o_alt);
            for (r_alt, o_alt) in [
                x.clone().min_rational_prec_round(y.clone(), prec, rm),
                x.clone().min_rational_prec_round_val_ref(&y, prec, rm),
                x.min_rational_prec_round_ref_val(y.clone(), prec, rm),
            ] {
                assert!(same(&r, &r_alt));
                assert_eq!(o, o_alt);
            }
            if !x.is_nan() {
                assert!(r <= x || o == Greater);
            }

            let (r, o) = x.max_rational_prec_round_ref_ref(&y, prec, rm);
            let (r_alt, o_alt) = rational_alt(&x, &y, prec, rm, true);
            assert!(same(&r, &r_alt));
            assert_eq!(o, o_alt);
            for (r_alt, o_alt) in [
                x.clone().max_rational_prec_round(y.clone(), prec, rm),
                x.clone().max_rational_prec_round_val_ref(&y, prec, rm),
                x.max_rational_prec_round_ref_val(y.clone(), prec, rm),
            ] {
                assert!(same(&r, &r_alt));
                assert_eq!(o, o_alt);
            }
            if !x.is_nan() {
                assert!(r >= x || o == Less);
            }
        }
        let (r, o) = x.min_rational_prec_ref_ref(&y, prec);
        let (r_alt, o_alt) = x.min_rational_prec_round_ref_ref(&y, prec, Nearest);
        assert!(same(&r, &r_alt));
        assert_eq!(o, o_alt);
        for (r_alt, o_alt) in [
            x.clone().min_rational_prec(y.clone(), prec),
            x.clone().min_rational_prec_val_ref(&y, prec),
            x.min_rational_prec_ref_val(y.clone(), prec),
        ] {
            assert!(same(&r, &r_alt));
            assert_eq!(o, o_alt);
        }
        let (r, o) = x.max_rational_prec_ref_ref(&y, prec);
        let (r_alt, o_alt) = x.max_rational_prec_round_ref_ref(&y, prec, Nearest);
        assert!(same(&r, &r_alt));
        assert_eq!(o, o_alt);
        for (r_alt, o_alt) in [
            x.clone().max_rational_prec(y.clone(), prec),
            x.clone().max_rational_prec_val_ref(&y, prec),
            x.max_rational_prec_ref_val(y.clone(), prec),
        ] {
            assert!(same(&r, &r_alt));
            assert_eq!(o, o_alt);
        }
    }
}

#[test]
fn min_max_rational_properties() {
    big_exponent_float_rational_pair_gen().test_properties(|(x, y)| {
        min_max_rational_properties_helper(x, y);
    });

    // - agreement with `Float`, wherever `Float` neither overflows nor underflows
    float_rational_pair_gen().test_properties(|(x, y)| {
        let bx = float_to_big_exponent_float(&x);
        for prec in PRECS {
            for rm in RMS {
                for (b, f) in [
                    (
                        bx.min_rational_prec_round_ref_ref(&y, prec, rm),
                        x.min_rational_prec_round_ref_ref(&y, prec, rm),
                    ),
                    (
                        bx.max_rational_prec_round_ref_ref(&y, prec, rm),
                        x.max_rational_prec_round_ref_ref(&y, prec, rm),
                    ),
                ] {
                    let fb = float_to_big_exponent_float(&f.0);
                    if fb.is_normal() == b.0.is_normal() {
                        assert!(same(&b.0, &fb));
                        assert_eq!(b.1, f.1);
                    }
                }
            }
        }
    });
}

#[test]
fn min_max_float_agreement_properties() {
    // - agreement with `Float`'s rounding versions, wherever `Float` does not overflow
    float_pair_gen().test_properties(|(x, y)| {
        let bx = float_to_big_exponent_float(&x);
        let by = float_to_big_exponent_float(&y);
        for prec in PRECS {
            for rm in RMS {
                for (b, f) in [
                    (
                        bx.min_prec_round_ref_ref(&by, prec, rm),
                        Float::min_prec_round_ref_ref(&x, &y, prec, rm),
                    ),
                    (
                        bx.max_prec_round_ref_ref(&by, prec, rm),
                        Float::max_prec_round_ref_ref(&x, &y, prec, rm),
                    ),
                ] {
                    let fb = float_to_big_exponent_float(&f.0);
                    if fb.is_normal() == b.0.is_normal() {
                        assert!(same(&b.0, &fb));
                        assert_eq!(b.1, f.1);
                    }
                }
            }
        }
    });
}
