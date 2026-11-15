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

fn acosh_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().acosh_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.acosh_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.acosh_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().acosh_prec(prec), &r));
        assert!(same_pair(&x.acosh_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.acosh_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn acosh_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::acosh_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::acosh_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::acosh_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::acosh_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_acosh_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = acosh_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &inverse_hyperbolic_prec_round_generic(InverseHyperbolicFunction::Acosh, &x, prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test(
        "Infinity", "Infinity", 10, Floor, "Infinity", "Infinity", Equal,
    );
    test("-Infinity", "-Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0", 10, Exact, "NaN", "NaN", Equal);
    test("1.0", "0x1", 10, Exact, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1", 10, Exact, "NaN", "NaN", Equal);
    test("0.50", "0x0.8", 10, Exact, "NaN", "NaN", Equal);
    test("-2.0", "-0x2", 10, Exact, "NaN", "NaN", Equal);
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        10,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test("2.0", "0x2", 1, Nearest, "1.0", "0x1.0", Less);
    test("2.0", "0x2", 10, Floor, "1.316", "0x1.51", Less);
    test("2.0", "0x2", 10, Ceiling, "1.3184", "0x1.518", Greater);
    test(
        "2.0",
        "0x2",
        100,
        Nearest,
        "1.316957896924816708625046347307",
        "0x1.51242719804349be684bd0188",
        Less,
    );
    test("1.5", "0x1.8", 1, Nearest, "1.0", "0x1.0", Greater);
    test("1.5", "0x1.8", 10, Floor, "0.96191", "0x0.f64", Less);
    test("1.5", "0x1.8", 10, Ceiling, "0.9629", "0x0.f68", Greater);
    test(
        "1.5",
        "0x1.8",
        100,
        Nearest,
        "0.96242365011920689499551782684852",
        "0x0.f661657628b04ca5f0210254b",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        1,
        Nearest,
        "2.0",
        "0x2.0",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Floor,
        "1.8105",
        "0x1.cf8",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Ceiling,
        "1.81",
        "0x1.d",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        100,
        Nearest,
        "1.8115262724608530659013198626248",
        "0x1.cfc02f90106c17a3fd778845e",
        Greater,
    );
    test(
        "1.0000000002",
        "0x1.00000001",
        1,
        Nearest,
        "0.000015",
        "0x0.0001",
        Less,
    );
    test(
        "1.0000000002",
        "0x1.00000001",
        10,
        Floor,
        "0.00002158",
        "0x0.00016a",
        Less,
    );
    test(
        "1.0000000002",
        "0x1.00000001",
        10,
        Ceiling,
        "0.000021607",
        "0x0.00016a8",
        Greater,
    );
    test(
        "1.0000000002",
        "0x1.00000001",
        100,
        Nearest,
        "0.000021579186437159052952634372225597",
        "0x0.00016a09e667d5914b2abaca766fa",
        Greater,
    );
    test("4.3e9", "0x1.0E+8", 1, Nearest, "16.0", "0x1.0E+1", Less);
    test("4.3e9", "0x1.0E+8", 10, Floor, "22.844", "0x16.d8", Less);
    test("4.3e9", "0x1.0E+8", 10, Ceiling, "22.88", "0x16.e", Greater);
    test(
        "4.3e9",
        "0x1.0E+8",
        100,
        Nearest,
        "22.87385695847819521075510748096",
        "0x16.dfb516f20bbeaf24c65a2698",
        Less,
    );
    test(
        "1.1e12041199827",
        "0x3.0E+10000000000",
        1,
        Nearest,
        "3.4e10",
        "0x8.0E+8",
        Greater,
    );
    test(
        "1.1e12041199827",
        "0x3.0E+10000000000",
        10,
        Floor,
        "2.772e10",
        "0x6.74E+8",
        Less,
    );
    test(
        "1.1e12041199827",
        "0x3.0E+10000000000",
        10,
        Ceiling,
        "2.7750e10",
        "0x6.76E+8",
        Greater,
    );
    test(
        "1.1e12041199827",
        "0x3.0E+10000000000",
        100,
        Nearest,
        "27725887224.1895718459173398591",
        "0x6749776f8.3087c7ce75134df8",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        1,
        Nearest,
        "3.4e10",
        "0x8.0E+8",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        10,
        Floor,
        "2.772e10",
        "0x6.74E+8",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        10,
        Ceiling,
        "2.7750e10",
        "0x6.76E+8",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        100,
        Nearest,
        "27725887223.090959557249230167745",
        "0x6749776f7.17492023a4e2a4818",
        Greater,
    );
}

#[test]
fn test_acosh_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let x = Rational::from_str(x).unwrap();
        let r = acosh_rational_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &inverse_hyperbolic_rational_prec_round_generic(
                InverseHyperbolicFunction::Acosh,
                &x,
                prec,
                rm
            ),
            &r
        ));
    };
    test("1", 10, Exact, "0.0", "0x0.0", Equal);
    test("0", 10, Exact, "NaN", "NaN", Equal);
    test("1/2", 10, Exact, "NaN", "NaN", Equal);
    test("-3", 10, Exact, "NaN", "NaN", Equal);
    test("3", 1, Nearest, "2.0", "0x2.0", Greater);
    test("3", 10, Floor, "1.762", "0x1.c3", Less);
    test("3", 10, Ceiling, "1.7637", "0x1.c38", Greater);
    test(
        "3",
        100,
        Nearest,
        "1.7627471740390860504652186499594",
        "0x1.c34366179d426cc1b1f33d1ba",
        Less,
    );
    test("22/7", 1, Nearest, "2.0", "0x2.0", Greater);
    test("22/7", 10, Floor, "1.8105", "0x1.cf8", Less);
    test("22/7", 10, Ceiling, "1.81", "0x1.d", Greater);
    test(
        "22/7",
        100,
        Nearest,
        "1.8119507608214135562359728131092",
        "0x1.cfdc014bb0b6cd086e591b3d6",
        Less,
    );
    test("1000000000000", 1, Nearest, "32.0", "0x2.0E+1", Greater);
    test("1000000000000", 10, Floor, "28.31", "0x1c.5", Less);
    test("1000000000000", 10, Ceiling, "28.344", "0x1c.58", Greater);
    test(
        "1000000000000",
        100,
        Nearest,
        "28.324168296488493517633129327677",
        "0x1c.52fcb187d16fbdf7ba2b8e62",
        Greater,
    );
    test(
        "100000000001/100000000000",
        1,
        Nearest,
        "3.8e-6",
        "0x0.00004",
        Less,
    );
    test(
        "100000000001/100000000000",
        10,
        Floor,
        "4.470e-6",
        "0x0.00004b",
        Less,
    );
    test(
        "100000000001/100000000000",
        10,
        Ceiling,
        "4.4778e-6",
        "0x0.00004b2",
        Greater,
    );
    test(
        "100000000001/100000000000",
        100,
        Nearest,
        "4.472135954995852612855856073222e-6",
        "0x0.00004b07ad7bc7832674cb00453e9",
        Less,
    );
}

#[test]
#[should_panic]
fn acosh_prec_round_fail_1() {
    BigExponentFloat::TWO.acosh_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn acosh_prec_round_fail_2() {
    BigExponentFloat::TWO.acosh_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn acosh_prec_round_fail_3() {
    BigExponentFloat::acosh_rational_prec_round(Rational::from(3), 0, Floor);
}

#[test]
#[should_panic]
fn acosh_prec_round_fail_4() {
    BigExponentFloat::acosh_rational_prec_round(Rational::from(3), 10, Exact);
}

#[test]
fn acosh_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_23().test_properties(
        |(x, prec, rm)| {
            let r = acosh_prec_round_all(&x, prec, rm);
            // `Float`'s functions agree with log(1 + t).
            assert!(same_pair(
                &inverse_hyperbolic_prec_round_generic(
                    InverseHyperbolicFunction::Acosh,
                    &x,
                    prec,
                    rm
                ),
                &r
            ));
            check_result(&r, prec);
            if big_exponent_float_exponent_is_small(&x)
                && let Ok(q) = Rational::try_from(&x)
            {
                assert!(same_pair(&acosh_rational_prec_round_all(&q, prec, rm), &r));
            }
        },
    );
}

#[test]
fn acosh_prec_round_float_agreement_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        // skip the `Float` generator's `Exact` cases
        if x.is_zero() || rm == Exact {
            return;
        }
        let (y, o) = x.acosh_prec_round_ref(prec, rm);
        if float_result_comparable(&y, o) {
            let r = acosh_prec_round_all(&float_to_big_exponent_float(&x), prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
    });
}

#[test]
fn acosh_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_15().test_properties(|(x, prec, rm)| {
        let r = acosh_rational_prec_round_all(&x, prec, rm);
        check_result(&r, prec);
        // `Float`'s functions agree with log(1 + t).
        assert!(same_pair(
            &inverse_hyperbolic_rational_prec_round_generic(
                InverseHyperbolicFunction::Acosh,
                &x,
                prec,
                rm
            ),
            &r
        ));
        if x != 0u32 {
            let (y, o) = Float::acosh_rational_prec_round_ref(&x, prec, rm);
            if float_result_comparable(&y, o) {
                assert!(same(&r.0, &float_to_big_exponent_float(&y)));
                assert_eq!(r.1, o);
            }
        }
    });
}
