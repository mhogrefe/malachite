// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{
    LogBase10Of1PlusX, LogBase10Of1PlusXAssign, PowerOf2,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeOne, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::{
    primitive_float_gen, unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::arithmetic::log_base_1_plus_x::primitive_float_log_base_1_plus_x;
use malachite_float::arithmetic::log_base_10_1_plus_x::primitive_float_log_base_10_1_plus_x;
use malachite_float::test_util::arithmetic::log_base_10_1_plus_x::{
    rug_log_base_10_1_plus_x, rug_log_base_10_1_plus_x_prec, rug_log_base_10_1_plus_x_prec_round,
};
use malachite_float::test_util::common::{rug_round_try_from_rounding_mode, to_hex_string};
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_12, float_rounding_mode_pair_gen_var_45,
    float_rounding_mode_pair_gen_var_46, float_unsigned_pair_gen_var_1,
    float_unsigned_pair_gen_var_4, float_unsigned_rounding_mode_triple_gen_var_34,
    float_unsigned_rounding_mode_triple_gen_var_35,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::panic::catch_unwind;

// Cross-checks the by-value/by-reference/assigning variants, precision, and the rug oracle. Returns
// the computed `(Float, Ordering)`. Because the oracle is a single native `log10_1p` MPFR call (not
// a bracketed quotient), it is fast even for extreme inputs and reproduces underflow at the same
// threshold as `Float`, so it is checked on every input.
fn check(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let (log, o) = x.clone().log_base_10_1_plus_x_prec_round(prec, rm);
    assert!(log.is_valid());

    let (log_alt, o_alt) = x.log_base_10_1_plus_x_prec_round_ref(prec, rm);
    assert!(log_alt.is_valid());
    assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.log_base_10_1_plus_x_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    if log.is_normal() {
        assert_eq!(log.get_prec(), Some(prec));
    }

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_log, rug_o) =
            rug_log_base_10_1_plus_x_prec_round(&rug::Float::exact_from(x), prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log)),
            ComparableFloatRef(&log),
        );
        assert_eq!(rug_o, o);
    }
    (log, o)
}

fn log_base_10_1_plus_x_prec_round_properties_helper(x: &Float, prec: u64, rm: RoundingMode) {
    let (log, o) = check(x, prec, rm);

    // Must match the general `log_base_1_plus_x` with base 10 (independently implemented, with its
    // own oracle).
    let (alt, o_alt) = x.log_base_1_plus_x_prec_round_ref(10, prec, rm);
    assert_eq!(ComparableFloatRef(&alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    // x < 0 (i.e. 1 + x < 1) gives a negative log; x > 0 gives positive; x = 0 gives 0.
    if log.is_normal() {
        if *x > 0u32 && o > Less {
            assert!(log > 0u32);
        } else if *x < 0u32 && *x > -1i32 && o < Greater {
            assert!(log < 0u32);
        }
    }

    // An exact result does not depend on the rounding mode; an inexact one cannot be produced with
    // `Exact`.
    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (alt, o_alt) = x.log_base_10_1_plus_x_prec_round_ref(prec, rm);
            assert_eq!(
                ComparableFloat(alt.abs_negative_zero()),
                ComparableFloat(log.abs_negative_zero_ref())
            );
            assert_eq!(o_alt, Equal);
        }
    } else {
        assert_panic!(x.log_base_10_1_plus_x_prec_round_ref(prec, Exact));
    }
}

#[test]
fn log_base_10_1_plus_x_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_34().test_properties(|(x, prec, rm)| {
        log_base_10_1_plus_x_prec_round_properties_helper(&x, prec, rm);
    });
    float_unsigned_rounding_mode_triple_gen_var_35().test_properties(|(x, prec, rm)| {
        log_base_10_1_plus_x_prec_round_properties_helper(&x, prec, rm);
    });

    // The special cases hold for every precision and rounding mode.
    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let f = |x: Float| x.log_base_10_1_plus_x_prec_round(prec, rm);
        assert!(f(Float::NAN).0.is_nan());
        assert_eq!(f(Float::INFINITY), (Float::INFINITY, Equal));
        assert!(f(Float::NEGATIVE_INFINITY).0.is_nan());
        // log_10(1 + 0) = 0, with the sign of the zero preserved.
        assert_eq!(
            ComparableFloat(f(Float::ZERO).0),
            ComparableFloat(Float::ZERO)
        );
        assert_eq!(
            ComparableFloat(f(-Float::ZERO).0),
            ComparableFloat(-Float::ZERO)
        );
        // 1 + (-1) = 0, so log_10(0) = -infinity.
        assert_eq!(f(Float::NEGATIVE_ONE), (Float::NEGATIVE_INFINITY, Equal));
        // x < -1 is outside the domain.
        assert!(f(Float::from(-2)).0.is_nan());
    });
}

#[test]
fn log_base_10_1_plus_x_prec_properties() {
    let f = |x: Float, prec: u64| {
        let (log, o) = x.clone().log_base_10_1_plus_x_prec(prec);
        assert!(log.is_valid());
        let (log_ref, o_ref) = x.log_base_10_1_plus_x_prec_ref(prec);
        assert_eq!(ComparableFloatRef(&log_ref), ComparableFloatRef(&log));
        assert_eq!(o_ref, o);
        let (expected, eo) = check(&x, prec, Nearest);
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&expected));
        assert_eq!(o, eo);
    };
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| f(x, prec));
    float_unsigned_pair_gen_var_4().test_properties(|(x, prec)| f(x, prec));
}

#[test]
fn log_base_10_1_plus_x_round_properties() {
    let f = |x: Float, rm: RoundingMode| {
        let (log, o) = x.clone().log_base_10_1_plus_x_round(rm);
        assert!(log.is_valid());
        let (log_ref, o_ref) = x.log_base_10_1_plus_x_round_ref(rm);
        assert_eq!(ComparableFloatRef(&log_ref), ComparableFloatRef(&log));
        assert_eq!(o_ref, o);
        let (expected, eo) = check(&x, x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&expected));
        assert_eq!(o, eo);
    };
    float_rounding_mode_pair_gen_var_45().test_properties(|(x, rm)| f(x, rm));
    float_rounding_mode_pair_gen_var_46().test_properties(|(x, rm)| f(x, rm));
}

#[test]
fn log_base_10_1_plus_x_properties() {
    let f = |x: Float| {
        let prec = x.significant_bits();
        let (expected, _) = check(&x, prec, Nearest);
        let log = x.clone().log_base_10_1_plus_x();
        assert!(log.is_valid());
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&expected));
        let log_ref = (&x).log_base_10_1_plus_x();
        assert_eq!(ComparableFloatRef(&log_ref), ComparableFloatRef(&log));
        let mut x_alt = x.clone();
        x_alt.log_base_10_1_plus_x_assign();
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log));
    };
    float_gen().test_properties(&f);
    float_gen_var_12().test_properties(f);
}

#[test]
fn log_base_10_1_plus_x_prec_round_fail() {
    // Precision must be nonzero.
    assert_panic!(Float::from(7).log_base_10_1_plus_x_prec_round(0, Nearest));
    // Exact is not allowed when the result is not exactly representable.
    assert_panic!(Float::from(1).log_base_10_1_plus_x_prec_round(10, Exact));
}

// log_10(1 + x) can underflow: dividing log_2(1 + x) by log_2(10) > 1 can push the result below
// MIN_EXPONENT. These extreme-exponent inputs are rarely produced by the generators, so the clamp
// is locked in here. Behavior matches div_prec_round's per-rounding-mode clamping.
#[test]
fn log_base_10_1_plus_x_underflow() {
    let test_u = |x: Float, rm: RoundingMode, out: &str, out_hex: &str, o_out: Ordering| {
        let (log, o) = x.clone().log_base_10_1_plus_x_prec_round(1, rm);
        assert!(log.is_valid());
        assert_eq!(log.to_string(), out);
        assert_eq!(to_hex_string(&log), out_hex);
        assert_eq!(o, o_out);
        let (log_alt, o_alt) = x.log_base_10_1_plus_x_prec_round_ref(1, rm);
        assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
        assert_eq!(o_alt, o);
    };
    test_u(
        Float::power_of_2(i64::from(Float::MIN_EXPONENT)),
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test_u(
        Float::power_of_2(i64::from(Float::MIN_EXPONENT)),
        Ceiling,
        "too_small",
        "0x1.0E-268435456#1",
        Greater,
    );
    test_u(
        Float::power_of_2(i64::from(Float::MIN_EXPONENT)),
        Nearest,
        "too_small",
        "0x1.0E-268435456#1",
        Greater,
    );
    test_u(
        Float::power_of_2(i64::from(Float::MIN_EXPONENT)),
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test_u(
        Float::power_of_2(i64::from(Float::MIN_EXPONENT)),
        Up,
        "too_small",
        "0x1.0E-268435456#1",
        Greater,
    );
    test_u(
        -Float::power_of_2(i64::from(Float::MIN_EXPONENT)),
        Floor,
        "-too_small",
        "-0x1.0E-268435456#1",
        Less,
    );
    test_u(
        -Float::power_of_2(i64::from(Float::MIN_EXPONENT)),
        Ceiling,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_u(
        -Float::power_of_2(i64::from(Float::MIN_EXPONENT)),
        Nearest,
        "-too_small",
        "-0x1.0E-268435456#1",
        Less,
    );
    test_u(
        -Float::power_of_2(i64::from(Float::MIN_EXPONENT)),
        Down,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_u(
        -Float::power_of_2(i64::from(Float::MIN_EXPONENT)),
        Up,
        "-too_small",
        "-0x1.0E-268435456#1",
        Less,
    );
}

#[test]
fn test_log_base_10_1_plus_x_prec_round() {
    let test = |n: i64, d: u64, prec: u64, rm: RoundingMode, out: &str, o_out: Ordering| {
        let x = Float::exact_from(malachite_q::Rational::from_signeds(n, i64::exact_from(d)));
        let (log, o) = check(&x, prec, rm);
        assert_eq!(log.to_string(), out);
        assert_eq!(o, o_out);
    };
    test(9, 1, 10, Floor, "1.0", Equal);
    test(9, 1, 10, Ceiling, "1.0", Equal);
    test(9, 1, 10, Nearest, "1.0", Equal);
    test(9, 1, 10, Exact, "1.0", Equal);
    test(99, 1, 10, Floor, "2.0", Equal);
    test(99, 1, 10, Ceiling, "2.0", Equal);
    test(99, 1, 10, Nearest, "2.0", Equal);
    test(99, 1, 10, Exact, "2.0", Equal);
    test(999, 1, 10, Floor, "3.0", Equal);
    test(999, 1, 10, Nearest, "3.0", Equal);
    test(999, 1, 10, Exact, "3.0", Equal);
    test(0, 1, 10, Floor, "0.0", Equal);
    test(0, 1, 10, Ceiling, "0.0", Equal);
    test(0, 1, 10, Nearest, "0.0", Equal);
    test(0, 1, 10, Exact, "0.0", Equal);
    test(1, 1, 20, Floor, "0.3010297", Less);
    test(1, 1, 20, Ceiling, "0.3010302", Greater);
    test(1, 1, 20, Nearest, "0.3010302", Greater);
    test(7, 1, 30, Floor, "0.903089986", Less);
    test(7, 1, 30, Ceiling, "0.903089987", Greater);
    test(7, 1, 30, Nearest, "0.903089987", Greater);
    test(-1, 2, 20, Floor, "-0.3010302", Less);
    test(-1, 2, 20, Ceiling, "-0.3010297", Greater);
    test(-1, 2, 20, Nearest, "-0.3010302", Less);
    test(-1, 1, 10, Floor, "-Infinity", Equal);
    test(-1, 1, 10, Nearest, "-Infinity", Equal);
    test(-1, 1, 10, Exact, "-Infinity", Equal);
    test(-3, 1, 10, Floor, "NaN", Equal);
    test(-3, 1, 10, Nearest, "NaN", Equal);
    test(-3, 1, 10, Exact, "NaN", Equal);
}

#[test]
fn test_log_base_10_1_plus_x_prec() {
    // The `_prec` methods round to nearest; cross-checked against the rug oracle.
    let test = |n: i64, d: u64, prec: u64, out: &str, o_out: Ordering| {
        let x = Float::exact_from(malachite_q::Rational::from_signeds(n, i64::exact_from(d)));

        let (log, o) = x.clone().log_base_10_1_plus_x_prec(prec);
        assert!(log.is_valid());
        assert_eq!(log.to_string(), out);
        assert_eq!(o, o_out);

        let (log_alt, o_alt) = x.log_base_10_1_plus_x_prec_ref(prec);
        assert!(log_alt.is_valid());
        assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
        assert_eq!(o_alt, o);

        let mut log_alt = x.clone();
        let o_alt = log_alt.log_base_10_1_plus_x_prec_assign(prec);
        assert!(log_alt.is_valid());
        assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
        assert_eq!(o_alt, o);

        let (rug_log, rug_o) = rug_log_base_10_1_plus_x_prec(&rug::Float::exact_from(&x), prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log)),
            ComparableFloatRef(&log),
        );
        assert_eq!(rug_o, o);
    };
    test(9, 1, 10, "1.0", Equal); // log_10(10) = 1
    test(99, 1, 10, "2.0", Equal); // log_10(100) = 2
    test(999, 1, 10, "3.0", Equal); // log_10(1000) = 3
    test(0, 1, 10, "0.0", Equal);
    test(1, 1, 20, "0.3010302", Greater); // log_10(2)
    test(7, 1, 30, "0.903089987", Greater); // log_10(8)
    test(-1, 2, 20, "-0.3010302", Less); // log_10(1/2)
    test(-1, 1, 10, "-Infinity", Equal);
    test(-3, 1, 10, "NaN", Equal);
}

#[test]
fn test_log_base_10_1_plus_x() {
    // The `LogBase10Of1PlusX` trait: rounds to the input's precision, to nearest.
    let test = |n: i64, d: u64, out: &str| {
        let x = Float::exact_from(malachite_q::Rational::from_signeds(n, i64::exact_from(d)));
        let log = x.clone().log_base_10_1_plus_x();
        assert!(log.is_valid());
        assert_eq!(log.to_string(), out);

        let log_alt = (&x).log_base_10_1_plus_x();
        assert!(log_alt.is_valid());
        assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));

        let mut x_alt = x.clone();
        x_alt.log_base_10_1_plus_x_assign();
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log));

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log_base_10_1_plus_x(
                &rug::Float::exact_from(&x)
            ))),
            ComparableFloatRef(&log),
        );
    };
    test(9, 1, "1.0");
    test(99, 1, "2.0");
    test(999, 1, "3.0");
    test(0, 1, "0.0");
    test(1, 1, "0.2");
    test(7, 1, "0.9");
    test(-1, 2, "-0.2");
    test(-1, 1, "-Infinity");
    test(-3, 1, "NaN");
}

#[test]
fn test_log_base_10_1_plus_x_round() {
    let test = |n: i64, d: u64, rm: RoundingMode, out: &str, o_out: Ordering| {
        let x = Float::exact_from(malachite_q::Rational::from_signeds(n, i64::exact_from(d)));
        // log_base_10_1_plus_x_round uses the input's precision; `check` cross-checks the oracle.
        let (log, o) = check(&x, x.significant_bits().max(1), rm);
        let (log2, o2) = x.clone().log_base_10_1_plus_x_round(rm);
        assert_eq!(ComparableFloatRef(&log2), ComparableFloatRef(&log));
        assert_eq!(o2, o);
        assert_eq!(log.to_string(), out);
        assert_eq!(o, o_out);
    };
    test(9, 1, Floor, "1.0", Equal);
    test(9, 1, Ceiling, "1.0", Equal);
    test(9, 1, Nearest, "1.0", Equal);
    test(9, 1, Down, "1.0", Equal);
    test(9, 1, Up, "1.0", Equal);
    test(99, 1, Floor, "2.0", Equal);
    test(99, 1, Nearest, "2.0", Equal);
    test(99, 1, Exact, "2.0", Equal);
    test(0, 1, Floor, "0.0", Equal);
    test(0, 1, Nearest, "0.0", Equal);
    test(0, 1, Exact, "0.0", Equal);
    test(-1, 1, Floor, "-Infinity", Equal);
    test(-1, 1, Nearest, "-Infinity", Equal);
    test(-1, 1, Exact, "-Infinity", Equal);
    test(-3, 1, Floor, "NaN", Equal);
    test(-3, 1, Nearest, "NaN", Equal);
    test(-3, 1, Exact, "NaN", Equal);
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_log_base_10_1_plus_x() {
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_log_base_10_1_plus_x(x)),
            NiceFloat(out)
        );
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, f32::INFINITY);
    test::<f32>(f32::NEGATIVE_INFINITY, f32::NAN);
    test::<f32>(0.0, 0.0);
    test::<f32>(-0.0, -0.0);
    test::<f32>(-1.0, f32::NEGATIVE_INFINITY);
    test::<f32>(-2.0, f32::NAN);
    test::<f32>(9.0, 1.0); // log_10(10)
    test::<f32>(99.0, 2.0); // log_10(100)
    test::<f32>(999.0, 3.0); // log_10(1000)
    test::<f32>(1.0, std::f32::consts::LOG10_2); // log_10(2)
    test::<f32>(-0.5, -std::f32::consts::LOG10_2); // log_10(1/2)
    test::<f32>(49.0, 1.6989699602127075); // log_10(50)

    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, f64::INFINITY);
    test::<f64>(f64::NEGATIVE_INFINITY, f64::NAN);
    test::<f64>(0.0, 0.0);
    test::<f64>(-0.0, -0.0);
    test::<f64>(-1.0, f64::NEGATIVE_INFINITY);
    test::<f64>(-2.0, f64::NAN);
    test::<f64>(9.0, 1.0); // log_10(10)
    test::<f64>(99.0, 2.0); // log_10(100)
    test::<f64>(999.0, 3.0); // log_10(1000)
    test::<f64>(1.0, std::f64::consts::LOG10_2); // log_10(2)
    test::<f64>(-0.5, -std::f64::consts::LOG10_2); // log_10(1/2)
    test::<f64>(49.0, 1.6989700043360187); // log_10(50)
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_log_base_10_1_plus_x_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        // log_base_10_1_plus_x agrees with log_base_1_plus_x with a base of 10.
        assert_eq!(
            NiceFloat(primitive_float_log_base_10_1_plus_x(x)),
            NiceFloat(primitive_float_log_base_1_plus_x(x, 10))
        );
    });
}

#[test]
fn primitive_float_log_base_10_1_plus_x_properties() {
    apply_fn_to_primitive_floats!(primitive_float_log_base_10_1_plus_x_properties_helper);
}
