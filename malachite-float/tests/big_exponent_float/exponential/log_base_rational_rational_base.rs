// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::One;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::float_to_big_exponent_float;
use malachite_float::test_util::generators::*;
use malachite_float::{BigExponentFloat, Float};
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

fn same_pair(x: &(BigExponentFloat, Ordering), y: &(BigExponentFloat, Ordering)) -> bool {
    same(&x.0, &y.0) && x.1 == y.1
}

fn float_result_comparable(x: &Float, o: Ordering) -> bool {
    x.get_exponent().map_or_else(
        || !x.is_zero() || o == Equal,
        |e| e > Float::MIN_EXPONENT + 1 && e < Float::MAX_EXPONENT - 1,
    )
}

fn log_base_rational_rational_base_prec_round_all(
    x: &Rational,
    b: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::log_base_rational_rational_base_prec_round(
        x.clone(),
        b.clone(),
        prec,
        rm,
    );
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::log_base_rational_rational_base_prec_round_ref(x, b, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::log_base_rational_rational_base_prec(x.clone(), b.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::log_base_rational_rational_base_prec_ref(x, b, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_log_base_rational_rational_base_prec_round() {
    // The expected values agree with FLINT's arb (see the `arf_logarithm_general` oracle mode).
    let cases: &[(&str, &str, u64, RoundingMode, &str, Ordering)] = &[
        ("3/5", "3/2", 1, Nearest, "-0x1.0", Greater),
        ("3/5", "3/2", 10, Floor, "-0x1.43", Less),
        ("3/5", "3/2", 10, Ceiling, "-0x1.428", Greater),
        ("3/5", "3/2", 10, Down, "-0x1.428", Greater),
        ("3/5", "3/2", 10, Up, "-0x1.43", Less),
        ("3/5", "3/2", 10, Nearest, "-0x1.428", Greater),
        (
            "3/5",
            "3/2",
            100,
            Nearest,
            "-0x1.4285986e701f27888ea9387de",
            Greater,
        ),
        ("3/5", "9", 1, Nearest, "-0x0.4", Less),
        ("3/5", "9", 10, Floor, "-0x0.3b9", Less),
        ("3/5", "9", 10, Ceiling, "-0x0.3b8", Greater),
        ("3/5", "9", 10, Down, "-0x0.3b8", Greater),
        ("3/5", "9", 10, Up, "-0x0.3b9", Less),
        ("3/5", "9", 10, Nearest, "-0x0.3b8", Greater),
        (
            "3/5",
            "9",
            100,
            Nearest,
            "-0x0.3b8440987ea72322ae1e4e54c4",
            Greater,
        ),
        ("9", "3/2", 1, Nearest, "0x4.0", Less),
        ("9", "3/2", 10, Floor, "0x5.6a", Less),
        ("9", "3/2", 10, Ceiling, "0x5.6c", Greater),
        ("9", "3/2", 10, Down, "0x5.6a", Less),
        ("9", "3/2", 10, Up, "0x5.6c", Greater),
        ("9", "3/2", 10, Nearest, "0x5.6c", Greater),
        (
            "9",
            "3/2",
            100,
            Nearest,
            "0x5.6b451060fe9464f4ec69fd848",
            Greater,
        ),
        ("9", "9", 1, Nearest, "0x1.0", Equal),
        ("9", "9", 10, Floor, "0x1.0", Equal),
        ("9", "9", 10, Ceiling, "0x1.0", Equal),
        ("9", "9", 10, Down, "0x1.0", Equal),
        ("9", "9", 10, Up, "0x1.0", Equal),
        ("9", "9", 10, Nearest, "0x1.0", Equal),
        ("9", "9", 100, Nearest, "0x1.0", Equal),
        ("9", "9", 300, Exact, "0x1.0", Equal),
        ("1/9", "3/2", 1, Nearest, "-0x4.0", Greater),
        ("1/9", "3/2", 10, Floor, "-0x5.6c", Less),
        ("1/9", "3/2", 10, Ceiling, "-0x5.6a", Greater),
        ("1/9", "3/2", 10, Down, "-0x5.6a", Greater),
        ("1/9", "3/2", 10, Up, "-0x5.6c", Less),
        ("1/9", "3/2", 10, Nearest, "-0x5.6c", Less),
        (
            "1/9",
            "3/2",
            100,
            Nearest,
            "-0x5.6b451060fe9464f4ec69fd848",
            Less,
        ),
        ("1/9", "9", 1, Nearest, "-0x1.0", Equal),
        ("1/9", "9", 10, Floor, "-0x1.0", Equal),
        ("1/9", "9", 10, Ceiling, "-0x1.0", Equal),
        ("1/9", "9", 10, Down, "-0x1.0", Equal),
        ("1/9", "9", 10, Up, "-0x1.0", Equal),
        ("1/9", "9", 10, Nearest, "-0x1.0", Equal),
        ("1/9", "9", 100, Nearest, "-0x1.0", Equal),
        ("1/9", "9", 300, Exact, "-0x1.0", Equal),
        ("1/3", "3/2", 1, Nearest, "-0x2.0", Greater),
        ("1/3", "3/2", 10, Floor, "-0x2.b6", Less),
        ("1/3", "3/2", 10, Ceiling, "-0x2.b5", Greater),
        ("1/3", "3/2", 10, Down, "-0x2.b5", Greater),
        ("1/3", "3/2", 10, Up, "-0x2.b6", Less),
        ("1/3", "3/2", 10, Nearest, "-0x2.b6", Less),
        (
            "1/3",
            "3/2",
            100,
            Nearest,
            "-0x2.b5a288307f4a327a7634fec24",
            Less,
        ),
        ("1/3", "9", 1, Nearest, "-0x0.8", Equal),
        ("1/3", "9", 10, Floor, "-0x0.8", Equal),
        ("1/3", "9", 10, Ceiling, "-0x0.8", Equal),
        ("1/3", "9", 10, Down, "-0x0.8", Equal),
        ("1/3", "9", 10, Up, "-0x0.8", Equal),
        ("1/3", "9", 10, Nearest, "-0x0.8", Equal),
        ("1/3", "9", 100, Nearest, "-0x0.8", Equal),
        ("1/3", "9", 300, Exact, "-0x0.8", Equal),
        ("10000000000/3", "3/2", 1, Nearest, "0x4.0E+1", Greater),
        ("10000000000/3", "3/2", 10, Floor, "0x36.1", Less),
        ("10000000000/3", "3/2", 10, Ceiling, "0x36.2", Greater),
        ("10000000000/3", "3/2", 10, Down, "0x36.1", Less),
        ("10000000000/3", "3/2", 10, Up, "0x36.2", Greater),
        ("10000000000/3", "3/2", 10, Nearest, "0x36.1", Less),
        (
            "10000000000/3",
            "3/2",
            100,
            Nearest,
            "0x36.14480fe9d3b94a6c588b1d54",
            Less,
        ),
        ("10000000000/3", "9", 1, Nearest, "0x8.0", Less),
        ("10000000000/3", "9", 10, Floor, "0x9.f8", Less),
        ("10000000000/3", "9", 10, Ceiling, "0x9.fc", Greater),
        ("10000000000/3", "9", 10, Down, "0x9.f8", Less),
        ("10000000000/3", "9", 10, Up, "0x9.fc", Greater),
        ("10000000000/3", "9", 10, Nearest, "0x9.fc", Greater),
        (
            "10000000000/3",
            "9",
            100,
            Nearest,
            "0x9.fac195bd42d8ede0e2687ca6",
            Greater,
        ),
        ("1", "3/2", 1, Nearest, "0x0.0", Equal),
        ("1", "3/2", 10, Floor, "0x0.0", Equal),
        ("1", "3/2", 10, Ceiling, "0x0.0", Equal),
        ("1", "3/2", 10, Down, "0x0.0", Equal),
        ("1", "3/2", 10, Up, "0x0.0", Equal),
        ("1", "3/2", 10, Nearest, "0x0.0", Equal),
        ("1", "3/2", 100, Nearest, "0x0.0", Equal),
        ("1", "3/2", 300, Exact, "0x0.0", Equal),
        ("1", "9", 1, Nearest, "0x0.0", Equal),
        ("1", "9", 10, Floor, "0x0.0", Equal),
        ("1", "9", 10, Ceiling, "0x0.0", Equal),
        ("1", "9", 10, Down, "0x0.0", Equal),
        ("1", "9", 10, Up, "0x0.0", Equal),
        ("1", "9", 10, Nearest, "0x0.0", Equal),
        ("1", "9", 100, Nearest, "0x0.0", Equal),
        ("1", "9", 300, Exact, "0x0.0", Equal),
        ("0", "3/2", 1, Nearest, "-Infinity", Equal),
        ("0", "3/2", 10, Floor, "-Infinity", Equal),
        ("0", "3/2", 10, Ceiling, "-Infinity", Equal),
        ("0", "3/2", 10, Down, "-Infinity", Equal),
        ("0", "3/2", 10, Up, "-Infinity", Equal),
        ("0", "3/2", 10, Nearest, "-Infinity", Equal),
        ("0", "3/2", 100, Nearest, "-Infinity", Equal),
        ("0", "3/2", 300, Exact, "-Infinity", Equal),
        ("0", "9", 1, Nearest, "-Infinity", Equal),
        ("0", "9", 10, Floor, "-Infinity", Equal),
        ("0", "9", 10, Ceiling, "-Infinity", Equal),
        ("0", "9", 10, Down, "-Infinity", Equal),
        ("0", "9", 10, Up, "-Infinity", Equal),
        ("0", "9", 10, Nearest, "-Infinity", Equal),
        ("0", "9", 100, Nearest, "-Infinity", Equal),
        ("0", "9", 300, Exact, "-Infinity", Equal),
        ("-1", "3/2", 1, Nearest, "NaN", Equal),
        ("-1", "3/2", 10, Floor, "NaN", Equal),
        ("-1", "3/2", 10, Ceiling, "NaN", Equal),
        ("-1", "3/2", 10, Down, "NaN", Equal),
        ("-1", "3/2", 10, Up, "NaN", Equal),
        ("-1", "3/2", 10, Nearest, "NaN", Equal),
        ("-1", "3/2", 100, Nearest, "NaN", Equal),
        ("-1", "3/2", 300, Exact, "NaN", Equal),
        ("-1", "9", 1, Nearest, "NaN", Equal),
        ("-1", "9", 10, Floor, "NaN", Equal),
        ("-1", "9", 10, Ceiling, "NaN", Equal),
        ("-1", "9", 10, Down, "NaN", Equal),
        ("-1", "9", 10, Up, "NaN", Equal),
        ("-1", "9", 10, Nearest, "NaN", Equal),
        ("-1", "9", 100, Nearest, "NaN", Equal),
        ("-1", "9", 300, Exact, "NaN", Equal),
    ];
    for &(x, b, prec, rm, out, o) in cases {
        let r = log_base_rational_rational_base_prec_round_all(
            &Rational::from_str(x).unwrap(),
            &Rational::from_str(b).unwrap(),
            prec,
            rm,
        );
        assert_eq!(format!("{:?}", r.0), out, "{x} {b} {prec} {rm}");
        assert_eq!(r.1, o, "{x} {b} {prec} {rm}");
    }
}

#[test]
#[should_panic]
fn log_base_rational_rational_base_prec_round_fail_1() {
    BigExponentFloat::log_base_rational_rational_base_prec_round(
        Rational::from_unsigneds(1u8, 3),
        Rational::from_unsigneds(3u8, 2),
        0,
        Floor,
    );
}

#[test]
#[should_panic]
fn log_base_rational_rational_base_prec_round_fail_2() {
    BigExponentFloat::log_base_rational_rational_base_prec_round(
        Rational::from_unsigneds(1u8, 5),
        Rational::from_unsigneds(3u8, 2),
        10,
        Exact,
    );
}

#[test]
#[should_panic]
fn log_base_rational_rational_base_prec_round_fail_3() {
    BigExponentFloat::log_base_rational_rational_base_prec_round(
        Rational::from_unsigneds(1u8, 3),
        Rational::ONE,
        10,
        Floor,
    );
}

fn log_base_rational_rational_base_prec_round_properties_helper(
    x: &Rational,
    b: &Rational,
    prec: u64,
    rm: RoundingMode,
) {
    let r = log_base_rational_rational_base_prec_round_all(x, b, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else if !r.0.is_nan() {
        // The result lies between the results rounded down and up.
        let floor = log_base_rational_rational_base_prec_round_all(x, b, prec, Floor);
        let ceiling = log_base_rational_rational_base_prec_round_all(x, b, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0);
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(
                &log_base_rational_rational_base_prec_round_all(x, b, prec, Exact),
                &r
            ));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = log_base_rational_rational_base_prec_round_all(x, b, prec + 16, rm);
            let (z, oz) = y.round_to_prec_round(prec, rm);
            assert!(same(&z, &r.0));
            assert_eq!(if o == Equal { oz } else { o }, r.1);
        }
    }
    // An integer base gives the same result as the `u64` base.
    if let Ok(k) = u64::try_from(b)
        && k > 1
    {
        assert!(same_pair(
            &r,
            &BigExponentFloat::log_base_rational_prec_round_ref(x, k, prec, rm)
        ));
    }
}

#[test]
fn log_base_rational_rational_base_prec_round_properties() {
    rational_rational_unsigned_rounding_mode_quadruple_gen_var_2().test_properties(
        |(x, b, prec, rm)| {
            log_base_rational_rational_base_prec_round_properties_helper(&x, &b, prec, rm);
        },
    );
}

#[test]
fn log_base_rational_rational_base_prec_round_float_agreement_properties() {
    rational_rational_unsigned_rounding_mode_quadruple_gen_var_2().test_properties(
        |(x, b, prec, rm)| {
            if rm == Exact {
                return;
            }
            let (f, fo) = Float::log_base_rational_rational_base_prec_round_ref(&x, &b, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = log_base_rational_rational_base_prec_round_all(&x.clone(), &b, prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
}
