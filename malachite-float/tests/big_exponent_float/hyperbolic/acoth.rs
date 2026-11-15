// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::Two;
use malachite_base::num::conversion::traits::FromStringBase;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::big_exponent_float::hyperbolic::asinh::{
    InverseHyperbolicFunction, inverse_hyperbolic_prec_round_generic,
    inverse_hyperbolic_rational_prec_round_generic,
};
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, float_to_big_exponent_float,
};
use malachite_float::test_util::generators::*;
use malachite_float::{BigExponentFloat, Float};
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};
use std::str::FromStr;

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

fn same_pair(x: &(BigExponentFloat, Ordering), y: &(BigExponentFloat, Ordering)) -> bool {
    same(&x.0, &y.0) && x.1 == y.1
}

fn float_result_comparable(x: &Float, o: Ordering) -> bool {
    x.get_exponent().map_or_else(
        // an inexact zero or infinity is an underflow or overflow
        || o == Equal,
        |e| e > Float::MIN_EXPONENT + 1 && e < Float::MAX_EXPONENT - 1,
    )
}

fn hex(s: &str) -> BigExponentFloat {
    BigExponentFloat::from_string_base(16, s).unwrap()
}

// A `NaN`, zero, or infinite result is exact; any other result has at most `prec` bits.
fn check_result(r: &(BigExponentFloat, Ordering), prec: u64) {
    if r.0.is_nan() || r.0.is_zero() || r.0.is_infinite() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
}

fn acoth_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().acoth_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.acoth_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.acoth_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().acoth_prec(prec), &r));
        assert!(same_pair(&x.acoth_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.acoth_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn acoth_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::acoth_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::acoth_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::acoth_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::acoth_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_acoth_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = acoth_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &inverse_hyperbolic_prec_round_generic(InverseHyperbolicFunction::Acoth, &x, prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "0.0", "0x0.0", Equal);
    test("-Infinity", "-Infinity", 10, Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0", 10, Exact, "NaN", "NaN", Equal);
    test("1.0", "0x1", 10, Exact, "Infinity", "Infinity", Equal);
    test("-1.0", "-0x1", 10, Exact, "-Infinity", "-Infinity", Equal);
    test("0.50", "0x0.8", 10, Exact, "NaN", "NaN", Equal);
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        10,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test("2.0", "0x2", 1, Nearest, "0.50", "0x0.8", Less);
    test("2.0", "0x2", 10, Floor, "0.5488", "0x0.8c8", Less);
    test("2.0", "0x2", 10, Ceiling, "0.54980", "0x0.8cc", Greater);
    test(
        "2.0",
        "0x2",
        100,
        Nearest,
        "0.5493061443340548456976226184611",
        "0x0.8c9f53d5681854bb520cc6aa8",
        Less,
    );
    test("-2.0", "-0x2", 1, Nearest, "-0.50", "-0x0.8", Greater);
    test("-2.0", "-0x2", 10, Floor, "-0.54980", "-0x0.8cc", Less);
    test("-2.0", "-0x2", 10, Ceiling, "-0.5488", "-0x0.8c8", Greater);
    test(
        "-2.0",
        "-0x2",
        100,
        Nearest,
        "-0.5493061443340548456976226184611",
        "-0x0.8c9f53d5681854bb520cc6aa8",
        Greater,
    );
    test(
        "1.0000000002",
        "0x1.00000001",
        1,
        Nearest,
        "8.0",
        "0x8.0",
        Less,
    );
    test(
        "1.0000000002",
        "0x1.00000001",
        10,
        Floor,
        "11.422",
        "0xb.6c",
        Less,
    );
    test(
        "1.0000000002",
        "0x1.00000001",
        10,
        Ceiling,
        "11.44",
        "0xb.7",
        Greater,
    );
    test(
        "1.0000000002",
        "0x1.00000001",
        100,
        Nearest,
        "11.43692847929730526629440927949",
        "0xb.6fda8b7945df5792732d134c",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        1,
        Nearest,
        "0.25",
        "0x0.4",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Floor,
        "0.32959",
        "0x0.546",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Ceiling,
        "0.3301",
        "0x0.548",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        100,
        Nearest,
        "0.3297653149566991214250947637126",
        "0x0.546b7feb181b06a1ebc34fec2",
        Greater,
    );
    test("4.3e9", "0x1.0E+8", 1, Nearest, "2.3e-10", "0x1.0E-8", Less);
    test("4.3e9", "0x1.0E+8", 10, Floor, "2.3e-10", "0x1.0E-8", Less);
    test(
        "4.3e9",
        "0x1.0E+8",
        10,
        Ceiling,
        "2.3329e-10",
        "0x1.008E-8",
        Greater,
    );
    test(
        "4.3e9",
        "0x1.0E+8",
        100,
        Nearest,
        "2.3283064365386962891045725816130e-10",
        "0x1.0000000000000000555555556E-8",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        1,
        Nearest,
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        10,
        Floor,
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        10,
        Ceiling,
        "2.7644e-12041199827",
        "0x1.008E-10000000000",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        100,
        Nearest,
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        Less,
    );
    test(
        "1.1e12041199827",
        "0x3.0E+10000000000",
        1,
        Nearest,
        "6.9e-12041199828",
        "0x4.0E-10000000001",
        Less,
    );
    test(
        "1.1e12041199827",
        "0x3.0E+10000000000",
        10,
        Floor,
        "9.188e-12041199828",
        "0x5.54E-10000000001",
        Less,
    );
    test(
        "1.1e12041199827",
        "0x3.0E+10000000000",
        10,
        Ceiling,
        "9.2012e-12041199828",
        "0x5.56E-10000000001",
        Greater,
    );
    test(
        "1.1e12041199827",
        "0x3.0E+10000000000",
        100,
        Nearest,
        "9.1966770635327334409489410646665e-12041199828",
        "0x5.5555555555555555555555558E-10000000001",
        Greater,
    );
    test(
        "-1.1e12041199827",
        "-0x3.0E+10000000000",
        1,
        Nearest,
        "-6.9e-12041199828",
        "-0x4.0E-10000000001",
        Greater,
    );
    test(
        "-1.1e12041199827",
        "-0x3.0E+10000000000",
        10,
        Floor,
        "-9.2012e-12041199828",
        "-0x5.56E-10000000001",
        Less,
    );
    test(
        "-1.1e12041199827",
        "-0x3.0E+10000000000",
        10,
        Ceiling,
        "-9.188e-12041199828",
        "-0x5.54E-10000000001",
        Greater,
    );
    test(
        "-1.1e12041199827",
        "-0x3.0E+10000000000",
        100,
        Nearest,
        "-9.1966770635327334409489410646665e-12041199828",
        "-0x5.5555555555555555555555558E-10000000001",
        Less,
    );
}

#[test]
fn test_acoth_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let x = Rational::from_str(x).unwrap();
        let r = acoth_rational_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &inverse_hyperbolic_rational_prec_round_generic(
                InverseHyperbolicFunction::Acoth,
                &x,
                prec,
                rm
            ),
            &r
        ));
    };
    test("1", 10, Exact, "Infinity", "Infinity", Equal);
    test("-1", 10, Exact, "-Infinity", "-Infinity", Equal);
    test("0", 10, Exact, "NaN", "NaN", Equal);
    test("1/2", 10, Exact, "NaN", "NaN", Equal);
    test("3", 1, Nearest, "0.25", "0x0.4", Less);
    test("3", 10, Floor, "0.34619", "0x0.58a", Less);
    test("3", 10, Ceiling, "0.3467", "0x0.58c", Greater);
    test(
        "3",
        100,
        Nearest,
        "0.34657359027997265470861606073",
        "0x0.58b90bfbe8e7bcd5e4f1d9cc",
        Less,
    );
    test("-3", 1, Nearest, "-0.25", "-0x0.4", Greater);
    test("-3", 10, Floor, "-0.3467", "-0x0.58c", Less);
    test("-3", 10, Ceiling, "-0.34619", "-0x0.58a", Greater);
    test(
        "-3",
        100,
        Nearest,
        "-0.34657359027997265470861606073",
        "-0x0.58b90bfbe8e7bcd5e4f1d9cc",
        Greater,
    );
    test("22/7", 1, Nearest, "0.25", "0x0.4", Less);
    test("22/7", 10, Floor, "0.32959", "0x0.546", Less);
    test("22/7", 10, Ceiling, "0.3301", "0x0.548", Greater);
    test(
        "22/7",
        100,
        Nearest,
        "0.3296228144421319805936337311067",
        "0x0.546229280b5ffbc3284155d02",
        Greater,
    );
    test("100001/100000", 1, Nearest, "8.0", "0x8.0", Greater);
    test("100001/100000", 10, Floor, "6.1016", "0x6.1a", Less);
    test("100001/100000", 10, Ceiling, "6.109", "0x6.1c", Greater);
    test(
        "100001/100000",
        100,
        Nearest,
        "6.103038822758836885586849906089",
        "0x6.1a60c095f7b10616c5b3ada1",
        Greater,
    );
    test(
        "-1000000000000",
        1,
        Nearest,
        "-9.1e-13",
        "-0x1.0E-10",
        Greater,
    );
    test(
        "-1000000000000",
        10,
        Floor,
        "-1.0001e-12",
        "-0x1.198E-10",
        Less,
    );
    test(
        "-1000000000000",
        10,
        Ceiling,
        "-9.983e-13",
        "-0x1.19E-10",
        Greater,
    );
    test(
        "-1000000000000",
        100,
        Nearest,
        "-1.00000000000000000000000033333e-12",
        "-0x1.19799812dea11197f27f80dcE-10",
        Less,
    );
}

#[test]
#[should_panic]
fn acoth_prec_round_fail_1() {
    BigExponentFloat::TWO.acoth_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn acoth_prec_round_fail_2() {
    BigExponentFloat::TWO.acoth_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn acoth_prec_round_fail_3() {
    BigExponentFloat::acoth_rational_prec_round(Rational::from(3), 0, Floor);
}

#[test]
#[should_panic]
fn acoth_prec_round_fail_4() {
    BigExponentFloat::acoth_rational_prec_round(Rational::from(3), 10, Exact);
}

#[test]
fn acoth_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_23().test_properties(
        |(x, prec, rm)| {
            let r = acoth_prec_round_all(&x, prec, rm);
            // `Float`'s functions agree with log(1 + t).
            assert!(same_pair(
                &inverse_hyperbolic_prec_round_generic(
                    InverseHyperbolicFunction::Acoth,
                    &x,
                    prec,
                    rm
                ),
                &r
            ));
            check_result(&r, prec);
            if x.is_finite() && !x.is_zero() {
                // acoth is odd
                let (s, o) = acoth_prec_round_all(&-&x, prec, -rm);
                assert!(same(&s, &-&r.0));
                assert_eq!(o, r.1.reverse());
            }
            if big_exponent_float_exponent_is_small(&x)
                && let Ok(q) = Rational::try_from(&x)
            {
                assert!(same_pair(&acoth_rational_prec_round_all(&q, prec, rm), &r));
            }
        },
    );
}

#[test]
fn acoth_prec_round_float_agreement_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        // skip the `Float` generator's `Exact` cases
        if x.is_zero() || rm == Exact {
            return;
        }
        let (y, o) = x.acoth_prec_round_ref(prec, rm);
        if float_result_comparable(&y, o) {
            let r = acoth_prec_round_all(&float_to_big_exponent_float(&x), prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
    });
    float_unsigned_rounding_mode_triple_gen_var_55().test_properties(|(x, prec, rm)| {
        // skip the `Float` generator's `Exact` cases
        if x.is_zero() || rm == Exact {
            return;
        }
        let (y, o) = x.acoth_prec_round_ref(prec, rm);
        if float_result_comparable(&y, o) {
            let r = acoth_prec_round_all(&float_to_big_exponent_float(&x), prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
    });
}

#[test]
fn acoth_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_18().test_properties(|(x, prec, rm)| {
        let r = acoth_rational_prec_round_all(&x, prec, rm);
        check_result(&r, prec);
        // `Float`'s functions agree with log(1 + t).
        assert!(same_pair(
            &inverse_hyperbolic_rational_prec_round_generic(
                InverseHyperbolicFunction::Acoth,
                &x,
                prec,
                rm
            ),
            &r
        ));
        if x != 0u32 {
            let (y, o) = Float::acoth_rational_prec_round_ref(&x, prec, rm);
            if float_result_comparable(&y, o) {
                assert!(same(&r.0, &float_to_big_exponent_float(&y)));
                assert_eq!(r.1, o);
            }
        }
    });
}
