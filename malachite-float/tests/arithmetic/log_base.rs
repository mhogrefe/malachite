// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{IsPowerOf2, LogBase, LogBaseAssign};
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeZero, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::test_util::arithmetic::log_base::{rug_log_base, rug_log_base_prec_round};
use malachite_float::test_util::common::rug_round_try_from_rounding_mode;
use malachite_float::test_util::generators::{
    float_unsigned_rounding_mode_triple_gen_var_27, float_unsigned_rounding_mode_triple_gen_var_28,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_5,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_6,
};
use malachite_float::{ComparableFloatRef, Float};
use std::panic::catch_unwind;

// Cross-checks the by-value, by-reference, and assigning variants against each other and against
// the rug oracle, and checks delegation when `base` is a power of 2. Returns the computed `(Float,
// Ordering)`.
fn check(x: &Float, base: u64, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let (log, o) = x.clone().log_base_prec_round(base, prec, rm);
    assert!(log.is_valid());

    let (log_alt, o_alt) = x.log_base_prec_round_ref(base, prec, rm);
    assert!(log_alt.is_valid());
    assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.log_base_prec_round_assign(base, prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    // When the base is a power of 2 the result must match log_base_power_of_2.
    if base.is_power_of_2() {
        let (alt, o2) =
            x.log_base_power_of_2_prec_round_ref(i64::from(base.trailing_zeros()), prec, rm);
        assert_eq!(ComparableFloatRef(&alt), ComparableFloatRef(&log));
        assert_eq!(o2, o);
    }

    if log.is_normal() {
        assert_eq!(log.get_prec(), Some(prec));
    }

    // Cross-check against the independent rug oracle, including on extreme inputs: with rational
    // results detected up front (see rational_log_base), the ln(x) / ln(base) bracketing stays
    // bounded and fast even there, matching log_base_2.
    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_log, rug_o) =
            rug_log_base_prec_round(&rug::Float::exact_from(x), base, prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log)),
            ComparableFloatRef(&log),
        );
        assert_eq!(rug_o, o);
    }
    (log, o)
}

#[test]
fn test_log_base_prec_round() {
    let test = |x: Float, base: u64, prec: u64, rm: RoundingMode, out: &str, o_out: Ordering| {
        let (log, o) = check(&x, base, prec, rm);
        assert_eq!(log.to_string(), out);
        assert_eq!(o, o_out);
    };
    // Exact powers: log_base(base^n) = n.
    test(Float::from(1000), 10, 10, Exact, "3.0", Equal);
    test(Float::from(100), 10, 10, Exact, "2.0", Equal);
    test(Float::from(81), 3, 10, Exact, "4.0", Equal);
    test(Float::from(243), 3, 10, Nearest, "5.0", Equal); // 3^5
    test(Float::from(16807), 7, 10, Nearest, "5.0", Equal); // 7^5
    test(Float::from(1), 10, 10, Exact, "0.0", Equal);
    // Base a power of 2: delegates to log_base_power_of_2.
    test(Float::from(64), 4, 10, Nearest, "3.0", Equal); // log_4(64) = 3
    test(Float::from(8), 2, 10, Exact, "3.0", Equal);
    test(Float::from(1000000), 8, 10, Nearest, "6.64", Less); // log_8(10^6), inexact
    // Inexact.
    test(Float::from(50), 10, 10, Floor, "1.697", Less);
    test(Float::from(50), 10, 10, Ceiling, "1.699", Greater);
    test(Float::from(50), 10, 10, Nearest, "1.699", Greater);
    test(Float::from(2), 3, 20, Nearest, "0.63093", Greater);
    // x in (0, 1): negative result.
    test(Float::from(1) >> 3, 3, 20, Nearest, "-1.89279", Less); // log_3(1/8)
    // Special cases.
    test(Float::NAN, 10, 10, Nearest, "NaN", Equal);
    test(Float::INFINITY, 10, 10, Nearest, "Infinity", Equal);
    test(Float::NEGATIVE_INFINITY, 10, 10, Nearest, "NaN", Equal);
    test(Float::ZERO, 10, 10, Nearest, "-Infinity", Equal);
    test(Float::NEGATIVE_ZERO, 10, 10, Nearest, "-Infinity", Equal);
    test(-Float::from(5), 10, 10, Nearest, "NaN", Equal);
}

#[test]
fn test_log_base_directed_consistency() {
    // For a curated set of inputs, exercise every rounding mode (cross-checked against the oracle
    // inside `check`), and confirm Floor <= Nearest <= Ceiling for finite results.
    let inputs: &[(Float, u64, u64)] = &[
        (Float::from(1000), 10, 30),
        (Float::from(50), 10, 30),
        (Float::from(81), 3, 30),
        (Float::from(2), 3, 30),
        (Float::from(7), 5, 40),
        (Float::from(8), 2, 30),      // power-of-2 base
        (Float::from(64), 4, 30),     // power-of-2 base
        (Float::from(1) >> 5, 3, 30), // 1/32, negative result
    ];
    for (x, base, prec) in inputs {
        let (floor, _) = check(x, *base, *prec, Floor);
        let (ceiling, _) = check(x, *base, *prec, Ceiling);
        let (nearest, _) = check(x, *base, *prec, Nearest);
        let _ = check(x, *base, *prec, Down);
        let _ = check(x, *base, *prec, Up);
        if floor.is_normal() && ceiling.is_normal() {
            assert!(
                floor <= nearest,
                "{x} base {base}: floor {floor} > nearest {nearest}"
            );
            assert!(
                nearest <= ceiling,
                "{x} base {base}: nearest {nearest} > ceiling {ceiling}"
            );
        }
    }
}

#[test]
fn test_log_base() {
    // The `LogBase` trait: rounds to the input's precision, to nearest.
    let test = |x: Float, base: u64, out: &str| {
        let log = x.clone().log_base(base);
        assert!(log.is_valid());
        assert_eq!(log.to_string(), out);

        let log_alt = (&x).log_base(base);
        assert!(log_alt.is_valid());
        assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));

        let mut x_alt = x.clone();
        x_alt.log_base_assign(base);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log));

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log_base(
                &rug::Float::exact_from(&x),
                base
            ))),
            ComparableFloatRef(&log),
        );
        let _ = x.significant_bits();
    };
    test(Float::from(1000), 10, "3.0");
    test(Float::from(81), 3, "4.0");
    test(Float::from(1), 10, "0.0");
    test(Float::NAN, 10, "NaN");
    test(Float::INFINITY, 10, "Infinity");
    test(Float::ZERO, 10, "-Infinity");
}

#[test]
fn log_base_prec_round_fail() {
    // Precision must be nonzero.
    assert_panic!(Float::from(10).log_base_prec_round(10, 0, Nearest));
    assert_panic!(Float::from(10).log_base_prec_round_ref(10, 0, Nearest));
    // Base must be greater than 1.
    assert_panic!(Float::from(10).log_base_prec_round(1, 10, Nearest));
    assert_panic!(Float::from(10).log_base_prec_round(0, 10, Nearest));
    assert_panic!(Float::from(10).log_base(1));
    // Exact rounding of an inexact result panics.
    assert_panic!(Float::from(50).log_base_prec_round(10, 10, Exact));
    assert_panic!(Float::from(2).log_base_prec_round(3, 10, Exact));
}

// Verifies the `_prec` variant against `_prec_round(..., Nearest)`, plus by-value/ref/assign
// consistency.
fn check_prec(x: &Float, base: u64, prec: u64) {
    let (log, o) = x.clone().log_base_prec(base, prec);
    assert!(log.is_valid());
    let (log_ref, o_ref) = x.log_base_prec_ref(base, prec);
    assert_eq!(ComparableFloatRef(&log_ref), ComparableFloatRef(&log));
    assert_eq!(o_ref, o);
    let mut x_alt = x.clone();
    let o_alt = x_alt.log_base_prec_assign(base, prec);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);
    let (expected, eo) = check(x, base, prec, Nearest);
    assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&expected));
    assert_eq!(o, eo);
}

// Verifies the `_round` variant (precision = input precision) against the oracle and the
// `_prec_round` variant, plus by-value/ref/assign consistency.
fn check_round(x: &Float, base: u64, rm: RoundingMode) {
    let prec = x.significant_bits();
    let (expected, eo) = check(x, base, prec, rm);
    let (log, o) = x.clone().log_base_round(base, rm);
    assert!(log.is_valid());
    assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&expected));
    assert_eq!(o, eo);
    let (log_ref, o_ref) = x.log_base_round_ref(base, rm);
    assert_eq!(ComparableFloatRef(&log_ref), ComparableFloatRef(&log));
    assert_eq!(o_ref, o);
    let mut x_alt = x.clone();
    let o_alt = x_alt.log_base_round_assign(base, rm);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);
}

#[test]
fn log_base_prec_round_properties() {
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_5().test_properties(
        |(x, base, prec, rm)| {
            check(&x, base, prec, rm);
        },
    );
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_6().test_properties(
        |(x, base, prec, rm)| {
            check(&x, base, prec, rm);
        },
    );
}

#[test]
fn log_base_prec_properties() {
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_5().test_properties(
        |(x, base, prec, _rm)| {
            check_prec(&x, base, prec);
        },
    );
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_6().test_properties(
        |(x, base, prec, _rm)| {
            check_prec(&x, base, prec);
        },
    );
}

#[test]
fn log_base_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_27().test_properties(|(x, base, rm)| {
        check_round(&x, base, rm);
    });
    float_unsigned_rounding_mode_triple_gen_var_28().test_properties(|(x, base, rm)| {
        check_round(&x, base, rm);
    });
}

#[test]
fn log_base_properties() {
    let f = |x: Float, base: u64| {
        let prec = x.significant_bits();
        let (expected, _) = check(&x, base, prec, Nearest);
        let log = x.clone().log_base(base);
        assert!(log.is_valid());
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&expected));
        let log_ref = (&x).log_base(base);
        assert_eq!(ComparableFloatRef(&log_ref), ComparableFloatRef(&log));
        let mut x_alt = x.clone();
        x_alt.log_base_assign(base);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log));
    };
    float_unsigned_rounding_mode_triple_gen_var_27().test_properties(|(x, base, _rm)| {
        f(x, base);
    });
    float_unsigned_rounding_mode_triple_gen_var_28().test_properties(|(x, base, _rm)| {
        f(x, base);
    });
}
