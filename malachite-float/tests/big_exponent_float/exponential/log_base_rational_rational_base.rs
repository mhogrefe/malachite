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
    let test =
        |x: &str, b: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
            let r = log_base_rational_rational_base_prec_round_all(
                &Rational::from_str(x).unwrap(),
                &Rational::from_str(b).unwrap(),
                prec,
                rm,
            );
            assert_eq!(format!("{:?}", r.0), out_hex, "{x} {b} {prec} {rm}");
            assert_eq!(r.0.to_string(), out);
            assert_eq!(r.1, o, "{x} {b} {prec} {rm}");
        };
    test("3/5", "3/2", 1, Nearest, "-1.0", "-0x1.0", Greater);
    test("3/5", "3/2", 10, Floor, "-1.262", "-0x1.43", Less);
    test("3/5", "3/2", 10, Ceiling, "-1.2598", "-0x1.428", Greater);
    test("3/5", "3/2", 10, Down, "-1.2598", "-0x1.428", Greater);
    test("3/5", "3/2", 10, Up, "-1.262", "-0x1.43", Less);
    test("3/5", "3/2", 10, Nearest, "-1.2598", "-0x1.428", Greater);
    test(
        "3/5",
        "3/2",
        100,
        Nearest,
        "-1.2598510045646632506597788812136",
        "-0x1.4285986e701f27888ea9387de",
        Greater,
    );
    test("3/5", "9", 1, Nearest, "-0.25", "-0x0.4", Less);
    test("3/5", "9", 10, Floor, "-0.23267", "-0x0.3b9", Less);
    test("3/5", "9", 10, Ceiling, "-0.2324", "-0x0.3b8", Greater);
    test("3/5", "9", 10, Down, "-0.2324", "-0x0.3b8", Greater);
    test("3/5", "9", 10, Up, "-0.23267", "-0x0.3b9", Less);
    test("3/5", "9", 10, Nearest, "-0.2324", "-0x0.3b8", Greater);
    test(
        "3/5",
        "9",
        100,
        Nearest,
        "-0.23248676035896358359852020383927",
        "-0x0.3b8440987ea72322ae1e4e54c4",
        Greater,
    );
    test("9", "3/2", 1, Nearest, "4.0", "0x4.0", Less);
    test("9", "3/2", 10, Floor, "5.4141", "0x5.6a", Less);
    test("9", "3/2", 10, Ceiling, "5.422", "0x5.6c", Greater);
    test("9", "3/2", 10, Down, "5.4141", "0x5.6a", Less);
    test("9", "3/2", 10, Up, "5.422", "0x5.6c", Greater);
    test("9", "3/2", 10, Nearest, "5.422", "0x5.6c", Greater);
    test(
        "9",
        "3/2",
        100,
        Nearest,
        "5.4190225827029095539523805243486",
        "0x5.6b451060fe9464f4ec69fd848",
        Greater,
    );
    test("9", "9", 1, Nearest, "1.0", "0x1.0", Equal);
    test("9", "9", 10, Floor, "1.0", "0x1.0", Equal);
    test("9", "9", 10, Ceiling, "1.0", "0x1.0", Equal);
    test("9", "9", 10, Down, "1.0", "0x1.0", Equal);
    test("9", "9", 10, Up, "1.0", "0x1.0", Equal);
    test("9", "9", 10, Nearest, "1.0", "0x1.0", Equal);
    test("9", "9", 100, Nearest, "1.0", "0x1.0", Equal);
    test("9", "9", 300, Exact, "1.0", "0x1.0", Equal);
    test("1/9", "3/2", 1, Nearest, "-4.0", "-0x4.0", Greater);
    test("1/9", "3/2", 10, Floor, "-5.422", "-0x5.6c", Less);
    test("1/9", "3/2", 10, Ceiling, "-5.4141", "-0x5.6a", Greater);
    test("1/9", "3/2", 10, Down, "-5.4141", "-0x5.6a", Greater);
    test("1/9", "3/2", 10, Up, "-5.422", "-0x5.6c", Less);
    test("1/9", "3/2", 10, Nearest, "-5.422", "-0x5.6c", Less);
    test(
        "1/9",
        "3/2",
        100,
        Nearest,
        "-5.4190225827029095539523805243486",
        "-0x5.6b451060fe9464f4ec69fd848",
        Less,
    );
    test("1/9", "9", 1, Nearest, "-1.0", "-0x1.0", Equal);
    test("1/9", "9", 10, Floor, "-1.0", "-0x1.0", Equal);
    test("1/9", "9", 10, Ceiling, "-1.0", "-0x1.0", Equal);
    test("1/9", "9", 10, Down, "-1.0", "-0x1.0", Equal);
    test("1/9", "9", 10, Up, "-1.0", "-0x1.0", Equal);
    test("1/9", "9", 10, Nearest, "-1.0", "-0x1.0", Equal);
    test("1/9", "9", 100, Nearest, "-1.0", "-0x1.0", Equal);
    test("1/9", "9", 300, Exact, "-1.0", "-0x1.0", Equal);
    test("1/3", "3/2", 1, Nearest, "-2.0", "-0x2.0", Greater);
    test("1/3", "3/2", 10, Floor, "-2.711", "-0x2.b6", Less);
    test("1/3", "3/2", 10, Ceiling, "-2.7070", "-0x2.b5", Greater);
    test("1/3", "3/2", 10, Down, "-2.7070", "-0x2.b5", Greater);
    test("1/3", "3/2", 10, Up, "-2.711", "-0x2.b6", Less);
    test("1/3", "3/2", 10, Nearest, "-2.711", "-0x2.b6", Less);
    test(
        "1/3",
        "3/2",
        100,
        Nearest,
        "-2.7095112913514547769761902621743",
        "-0x2.b5a288307f4a327a7634fec24",
        Less,
    );
    test("1/3", "9", 1, Nearest, "-0.50", "-0x0.8", Equal);
    test("1/3", "9", 10, Floor, "-0.50", "-0x0.8", Equal);
    test("1/3", "9", 10, Ceiling, "-0.50", "-0x0.8", Equal);
    test("1/3", "9", 10, Down, "-0.50", "-0x0.8", Equal);
    test("1/3", "9", 10, Up, "-0.50", "-0x0.8", Equal);
    test("1/3", "9", 10, Nearest, "-0.50", "-0x0.8", Equal);
    test("1/3", "9", 100, Nearest, "-0.50", "-0x0.8", Equal);
    test("1/3", "9", 300, Exact, "-0.50", "-0x0.8", Equal);
    test(
        "10000000000/3",
        "3/2",
        1,
        Nearest,
        "64.0",
        "0x4.0E+1",
        Greater,
    );
    test("10000000000/3", "3/2", 10, Floor, "54.062", "0x36.1", Less);
    test(
        "10000000000/3",
        "3/2",
        10,
        Ceiling,
        "54.12",
        "0x36.2",
        Greater,
    );
    test("10000000000/3", "3/2", 10, Down, "54.062", "0x36.1", Less);
    test("10000000000/3", "3/2", 10, Up, "54.12", "0x36.2", Greater);
    test(
        "10000000000/3",
        "3/2",
        10,
        Nearest,
        "54.062",
        "0x36.1",
        Less,
    );
    test(
        "10000000000/3",
        "3/2",
        100,
        Nearest,
        "54.079224581324273269145403793429",
        "0x36.14480fe9d3b94a6c588b1d54",
        Less,
    );
    test("10000000000/3", "9", 1, Nearest, "8.0", "0x8.0", Less);
    test("10000000000/3", "9", 10, Floor, "9.969", "0x9.f8", Less);
    test(
        "10000000000/3",
        "9",
        10,
        Ceiling,
        "9.9844",
        "0x9.fc",
        Greater,
    );
    test("10000000000/3", "9", 10, Down, "9.969", "0x9.f8", Less);
    test("10000000000/3", "9", 10, Up, "9.9844", "0x9.fc", Greater);
    test(
        "10000000000/3",
        "9",
        10,
        Nearest,
        "9.9844",
        "0x9.fc",
        Greater,
    );
    test(
        "10000000000/3",
        "9",
        100,
        Nearest,
        "9.979516371446923021482837610113",
        "0x9.fac195bd42d8ede0e2687ca6",
        Greater,
    );
    test("1", "3/2", 1, Nearest, "0.0", "0x0.0", Equal);
    test("1", "3/2", 10, Floor, "0.0", "0x0.0", Equal);
    test("1", "3/2", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("1", "3/2", 10, Down, "0.0", "0x0.0", Equal);
    test("1", "3/2", 10, Up, "0.0", "0x0.0", Equal);
    test("1", "3/2", 10, Nearest, "0.0", "0x0.0", Equal);
    test("1", "3/2", 100, Nearest, "0.0", "0x0.0", Equal);
    test("1", "3/2", 300, Exact, "0.0", "0x0.0", Equal);
    test("1", "9", 1, Nearest, "0.0", "0x0.0", Equal);
    test("1", "9", 10, Floor, "0.0", "0x0.0", Equal);
    test("1", "9", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("1", "9", 10, Down, "0.0", "0x0.0", Equal);
    test("1", "9", 10, Up, "0.0", "0x0.0", Equal);
    test("1", "9", 10, Nearest, "0.0", "0x0.0", Equal);
    test("1", "9", 100, Nearest, "0.0", "0x0.0", Equal);
    test("1", "9", 300, Exact, "0.0", "0x0.0", Equal);
    test("0", "3/2", 1, Nearest, "-Infinity", "-Infinity", Equal);
    test("0", "3/2", 10, Floor, "-Infinity", "-Infinity", Equal);
    test("0", "3/2", 10, Ceiling, "-Infinity", "-Infinity", Equal);
    test("0", "3/2", 10, Down, "-Infinity", "-Infinity", Equal);
    test("0", "3/2", 10, Up, "-Infinity", "-Infinity", Equal);
    test("0", "3/2", 10, Nearest, "-Infinity", "-Infinity", Equal);
    test("0", "3/2", 100, Nearest, "-Infinity", "-Infinity", Equal);
    test("0", "3/2", 300, Exact, "-Infinity", "-Infinity", Equal);
    test("0", "9", 1, Nearest, "-Infinity", "-Infinity", Equal);
    test("0", "9", 10, Floor, "-Infinity", "-Infinity", Equal);
    test("0", "9", 10, Ceiling, "-Infinity", "-Infinity", Equal);
    test("0", "9", 10, Down, "-Infinity", "-Infinity", Equal);
    test("0", "9", 10, Up, "-Infinity", "-Infinity", Equal);
    test("0", "9", 10, Nearest, "-Infinity", "-Infinity", Equal);
    test("0", "9", 100, Nearest, "-Infinity", "-Infinity", Equal);
    test("0", "9", 300, Exact, "-Infinity", "-Infinity", Equal);
    test("-1", "3/2", 1, Nearest, "NaN", "NaN", Equal);
    test("-1", "3/2", 10, Floor, "NaN", "NaN", Equal);
    test("-1", "3/2", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1", "3/2", 10, Down, "NaN", "NaN", Equal);
    test("-1", "3/2", 10, Up, "NaN", "NaN", Equal);
    test("-1", "3/2", 10, Nearest, "NaN", "NaN", Equal);
    test("-1", "3/2", 100, Nearest, "NaN", "NaN", Equal);
    test("-1", "3/2", 300, Exact, "NaN", "NaN", Equal);
    test("-1", "9", 1, Nearest, "NaN", "NaN", Equal);
    test("-1", "9", 10, Floor, "NaN", "NaN", Equal);
    test("-1", "9", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1", "9", 10, Down, "NaN", "NaN", Equal);
    test("-1", "9", 10, Up, "NaN", "NaN", Equal);
    test("-1", "9", 10, Nearest, "NaN", "NaN", Equal);
    test("-1", "9", 100, Nearest, "NaN", "NaN", Equal);
    test("-1", "9", 300, Exact, "NaN", "NaN", Equal);
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
