// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use core::str::FromStr;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{NegativeInfinity, One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::unsigned_rounding_mode_pair_gen_var_3;
use malachite_float::arithmetic::log_base::primitive_float_log_base_rational;
use malachite_float::arithmetic::log_base_rational_rational_base::*;
use malachite_float::test_util::arithmetic::log_base_rational_rational_base::{
    rug_log_base_rational_rational_base_prec, rug_log_base_rational_rational_base_prec_round,
};
use malachite_float::test_util::common::rug_round_try_from_rounding_mode;
use malachite_float::test_util::generators::*;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use std::panic::catch_unwind;

// Cross-checks the by-value and by-reference variants, the precision, and (for small enough
// operands) the rug oracle, returning the computed `(Float, Ordering)`. The oracle is skipped for
// large `x` or `base`, whose ln(x)/ln(base) bracketing uses a working precision that grows with
// their bit length.
fn check(x: &Rational, base: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let (log, o) =
        Float::log_base_rational_rational_base_prec_round(x.clone(), base.clone(), prec, rm);
    assert!(log.is_valid());

    let (log_alt, o_alt) = Float::log_base_rational_rational_base_prec_round_ref(x, base, prec, rm);
    assert!(log_alt.is_valid());
    assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    if log.is_normal() {
        assert_eq!(log.get_prec(), Some(prec));
    }

    if base.significant_bits() <= 128
        && x.significant_bits() <= 128
        && let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
    {
        let (rug_log, rug_o) =
            rug_log_base_rational_rational_base_prec_round(x, base, prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log)),
            ComparableFloatRef(&log),
        );
        assert_eq!(rug_o, o);
    }
    (log, o)
}

#[test]
fn test_log_base_rational_rational_base_prec_round() {
    let test =
        |xn: i64, xd: u64, bn: u64, bd: u64, prec: u64, rm: RoundingMode, out: &str, o_out| {
            let x = Rational::from_signeds(xn, i64::exact_from(xd));
            let base = Rational::from_unsigneds(bn, bd);
            let (log, o) = check(&x, &base, prec, rm);
            assert_eq!(log.to_string(), out);
            assert_eq!(o, o_out);
        };
    // Exact, integer base.
    test(9, 1, 3, 1, 10, Exact, "2.0", Equal); // log_3(9) = 2
    test(8, 1, 2, 1, 10, Exact, "3.0", Equal); // log_2(8) = 3
    test(27, 1, 3, 1, 10, Exact, "3.0", Equal); // log_3(27) = 3
    // Exact, rational base.
    test(8, 1, 4, 1, 10, Exact, "1.5", Equal); // log_4(8) = 3/2
    test(2, 1, 4, 1, 10, Exact, "0.5", Equal); // log_4(2) = 1/2
    test(9, 4, 3, 2, 10, Exact, "2.0", Equal); // log_{3/2}(9/4) = 2
    test(3, 2, 9, 4, 10, Exact, "0.5", Equal); // log_{9/4}(3/2) = 1/2
    // Non-dyadic rational x.
    test(1, 3, 3, 1, 10, Exact, "-1.0", Equal); // log_3(1/3) = -1
    test(1, 9, 3, 1, 10, Exact, "-2.0", Equal); // log_3(1/9) = -2
    // x < 1 (negative result).
    test(1, 4, 4, 1, 10, Exact, "-1.0", Equal); // log_4(1/4) = -1
    test(1, 8, 2, 1, 10, Exact, "-3.0", Equal); // log_2(1/8) = -3
    // Irrational (cross-checked against the oracle).
    test(2, 1, 3, 1, 20, Floor, "0.630929", Less); // log_3(2)
    test(2, 1, 3, 1, 20, Ceiling, "0.63093", Greater);
    test(2, 1, 3, 1, 20, Nearest, "0.63093", Greater);
    test(2, 1, 3, 2, 20, Nearest, "1.709511", Less); // log_{3/2}(2)
    test(1, 3, 3, 2, 20, Nearest, "-2.709511", Greater); // log_{3/2}(1/3)
    // x = 1.
    test(1, 1, 3, 1, 10, Exact, "0.0", Equal);
    test(1, 1, 7, 3, 10, Exact, "0.0", Equal);
}

#[test]
fn test_log_base_rational_rational_base_specials() {
    let three = Rational::from(3u32);
    let nearest = |x: Rational| Float::log_base_rational_rational_base_prec(x, three.clone(), 10);

    assert_eq!(nearest(Rational::ZERO), (Float::NEGATIVE_INFINITY, Equal));
    assert_eq!(nearest(Rational::ONE), (Float::ZERO, Equal)); // log_3(1) = 0
    assert_eq!(nearest(Rational::from(-2)).0.to_string(), "NaN"); // x < 0
    assert_eq!(
        nearest(Rational::from_signeds(-1, 2)).0.to_string(),
        "NaN" // x < 0
    );
}

#[test]
fn test_log_base_rational_rational_base_prec() {
    // The `_prec` methods round to nearest; cross-checked against the rug oracle.
    let test = |xn: i64, xd: u64, bn: u64, bd: u64, prec: u64, out: &str, o_out| {
        let x = Rational::from_signeds(xn, i64::exact_from(xd));
        let base = Rational::from_unsigneds(bn, bd);

        let (log, o) = Float::log_base_rational_rational_base_prec(x.clone(), base.clone(), prec);
        assert!(log.is_valid());
        assert_eq!(log.to_string(), out);
        assert_eq!(o, o_out);

        let (log_alt, o_alt) = Float::log_base_rational_rational_base_prec_ref(&x, &base, prec);
        assert!(log_alt.is_valid());
        assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
        assert_eq!(o_alt, o);

        let (rug_log, rug_o) = rug_log_base_rational_rational_base_prec(&x, &base, prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log)),
            ComparableFloatRef(&log),
        );
        assert_eq!(rug_o, o);
    };
    test(9, 1, 3, 1, 10, "2.0", Equal); // log_3(9) = 2
    test(8, 1, 4, 1, 10, "1.5", Equal); // log_4(8) = 3/2
    test(1, 9, 3, 1, 10, "-2.0", Equal); // log_3(1/9) = -2
    test(9, 4, 3, 2, 10, "2.0", Equal); // log_{3/2}(9/4) = 2
    test(2, 1, 3, 1, 20, "0.63093", Greater); // log_3(2)
    test(2, 1, 3, 2, 20, "1.709511", Less); // log_{3/2}(2)
    test(1, 1, 3, 1, 10, "0.0", Equal); // log_3(1) = 0
}

// The result must match the independently-implemented integer-base `log_base_rational` (which
// computes ln(x) / ln(base) rather than log_2(x) / log_2(base)) whenever the base is a `u64`
// integer.
fn check_integer_base_agreement(
    x: &Rational,
    base: &Rational,
    prec: u64,
    rm: RoundingMode,
    log: &Float,
    o: Ordering,
) {
    if let Ok(n) = u64::try_from(base) {
        let (alt, o_alt) = Float::log_base_rational_prec_round_ref(x, n, prec, rm);
        assert_eq!(ComparableFloatRef(&alt), ComparableFloatRef(log));
        assert_eq!(o_alt, o);
    }
}

fn log_base_rational_rational_base_prec_round_properties_helper(
    x: &Rational,
    base: &Rational,
    prec: u64,
    rm: RoundingMode,
) {
    let (log, o) = check(x, base, prec, rm);

    check_integer_base_agreement(x, base, prec, rm, &log, o);

    // The sign of the result follows whether x is on the same side of 1 as the base (> 1): x > 1
    // gives a positive log, 0 < x < 1 gives a negative log.
    if log.is_normal() {
        if *x > 1u32 && o > Less {
            assert!(log > 0u32);
        } else if *x > 0u32 && *x < 1u32 && o < Greater {
            assert!(log < 0u32);
        }
    }

    // An exact result does not depend on the rounding mode; an inexact one cannot be produced with
    // `Exact`.
    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (alt, o_alt) =
                Float::log_base_rational_rational_base_prec_round_ref(x, base, prec, rm);
            assert_eq!(
                ComparableFloat(alt.abs_negative_zero()),
                ComparableFloat(log.abs_negative_zero_ref())
            );
            assert_eq!(o_alt, Equal);
        }
    } else {
        assert_panic!(Float::log_base_rational_rational_base_prec_round_ref(
            x, base, prec, Exact
        ));
    }
}

#[test]
fn log_base_rational_rational_base_prec_round_properties() {
    rational_rational_unsigned_rounding_mode_quadruple_gen_var_2().test_properties_with_limit(
        20,
        |(x, base, prec, rm)| {
            log_base_rational_rational_base_prec_round_properties_helper(&x, &base, prec, rm);
        },
    );

    // The special cases hold for every base > 1, precision, and rounding mode.
    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        for base in [Rational::from(3u32), Rational::from_unsigneds(3u32, 2)] {
            let f = |x: Rational| {
                Float::log_base_rational_rational_base_prec_round(x, base.clone(), prec, rm)
            };
            assert_eq!(f(Rational::ZERO), (Float::NEGATIVE_INFINITY, Equal));
            assert_eq!(f(Rational::ONE), (Float::ZERO, Equal));
            assert!(f(Rational::from(-2)).0.is_nan());
            assert!(f(Rational::from_signeds(-1, 2)).0.is_nan());
        }
    });
}

#[test]
fn log_base_rational_rational_base_prec_properties() {
    rational_rational_unsigned_rounding_mode_quadruple_gen_var_2().test_properties_with_limit(
        20,
        |(x, base, prec, _rm)| {
            let (log, o) =
                Float::log_base_rational_rational_base_prec(x.clone(), base.clone(), prec);
            assert!(log.is_valid());
            let (log_ref, o_ref) = Float::log_base_rational_rational_base_prec_ref(&x, &base, prec);
            assert_eq!(ComparableFloatRef(&log_ref), ComparableFloatRef(&log));
            assert_eq!(o_ref, o);
            let (expected, eo) = check(&x, &base, prec, Nearest);
            assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&expected));
            assert_eq!(o, eo);
        },
    );
}

#[test]
fn log_base_rational_rational_base_fail() {
    // Precision must be nonzero.
    assert_panic!(Float::log_base_rational_rational_base_prec_round(
        Rational::from(8),
        Rational::from(3u32),
        0,
        Nearest
    ));
    // Base must be greater than 1.
    assert_panic!(Float::log_base_rational_rational_base_prec_round(
        Rational::from(8),
        Rational::ONE,
        10,
        Nearest
    ));
    assert_panic!(Float::log_base_rational_rational_base_prec_round(
        Rational::from(8),
        Rational::from_unsigneds(1u32, 2),
        10,
        Nearest
    ));
    // Exact is not allowed when the result is not exactly representable.
    assert_panic!(Float::log_base_rational_rational_base_prec_round(
        Rational::from(2),
        Rational::from(3u32),
        10,
        Exact
    ));
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_log_base_rational_rational_base() {
    fn test<T: PrimitiveFloat>(x: &str, base: &str, out: T)
    where
        Float: PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        let x = Rational::from_str(x).unwrap();
        let base = Rational::from_str(base).unwrap();
        assert_eq!(
            NiceFloat(primitive_float_log_base_rational_rational_base::<T>(
                &x, &base
            )),
            NiceFloat(out)
        );
    }
    test::<f32>("0", "10", f32::NEGATIVE_INFINITY);
    test::<f32>("1", "10", 0.0);
    test::<f32>("8", "4", 1.5); // log_4(8)
    test::<f32>("9", "3", 2.0); // log_3(9)
    test::<f32>("1000", "10", 3.0); // log_10(1000)
    test::<f32>("8", "2", 3.0); // log_2(8), power-of-2 base
    test::<f32>("9/4", "3/2", 2.0); // log_(3/2)(9/4)
    test::<f32>("2", "3/2", 1.7095113); // log_(3/2)(2)
    test::<f32>("50", "10", 1.6989699602127075); // log_10(50)
    test::<f32>("1/3", "10", -0.4771212637424469); // log_10(1/3)
    test::<f32>("-1", "10", f32::NAN);
    test::<f32>("-22/7", "10", f32::NAN);

    test::<f64>("0", "10", f64::NEGATIVE_INFINITY);
    test::<f64>("1", "10", 0.0);
    test::<f64>("8", "4", 1.5); // log_4(8)
    test::<f64>("9", "3", 2.0); // log_3(9)
    test::<f64>("1000", "10", 3.0); // log_10(1000)
    test::<f64>("8", "2", 3.0); // log_2(8), power-of-2 base
    test::<f64>("9/4", "3/2", 2.0); // log_(3/2)(9/4)
    test::<f64>("2", "3/2", 1.7095112913514547); // log_(3/2)(2)
    test::<f64>("50", "10", 1.6989700043360187); // log_10(50)
    test::<f64>("1/3", "10", -0.47712125471966244); // log_10(1/3)
    test::<f64>("-1", "10", f64::NAN);
    test::<f64>("-22/7", "10", f64::NAN);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_log_base_rational_rational_base_properties_helper<T: PrimitiveFloat>()
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    // The generator yields a positive `Rational` `x` and a `Rational` base greater than 1 (and an
    // unrelated precision and rounding mode).
    rational_rational_unsigned_rounding_mode_quadruple_gen_var_2().test_properties_with_limit(
        20,
        |(x, base, _, _)| {
            let y = primitive_float_log_base_rational_rational_base::<T>(&x, &base);
            // When the base is a positive integer, the result matches the integer-base function.
            if let Ok(b) = u64::try_from(&base) {
                assert_eq!(
                    NiceFloat(y),
                    NiceFloat(primitive_float_log_base_rational::<T>(&x, b))
                );
            }
        },
    );
}

#[test]
fn primitive_float_log_base_rational_rational_base_properties() {
    apply_fn_to_primitive_floats!(
        primitive_float_log_base_rational_rational_base_properties_helper
    );
}
