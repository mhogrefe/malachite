// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::One;
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

fn acsch_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().acsch_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.acsch_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.acsch_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().acsch_prec(prec), &r));
        assert!(same_pair(&x.acsch_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.acsch_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn acsch_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::acsch_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::acsch_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::acsch_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::acsch_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_acsch_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = acsch_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &inverse_hyperbolic_prec_round_generic(InverseHyperbolicFunction::Acsch, &x, prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "0.0", "0x0.0", Equal);
    test("-Infinity", "-Infinity", 10, Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0", 10, Exact, "NaN", "NaN", Equal);
    test("1.0", "0x1", 1, Nearest, "1.0", "0x1.0", Greater);
    test("1.0", "0x1", 10, Floor, "0.8809", "0x0.e18", Less);
    test("1.0", "0x1", 10, Ceiling, "0.88184", "0x0.e1c", Greater);
    test(
        "1.0",
        "0x1",
        100,
        Nearest,
        "0.88137358701954302523260932497968",
        "0x0.e1a1b30bcea13660d8f99e8dd",
        Less,
    );
    test("-1.0", "-0x1", 1, Nearest, "-1.0", "-0x1.0", Less);
    test("-1.0", "-0x1", 10, Floor, "-0.88184", "-0x0.e1c", Less);
    test("-1.0", "-0x1", 10, Ceiling, "-0.8809", "-0x0.e18", Greater);
    test(
        "-1.0",
        "-0x1",
        100,
        Nearest,
        "-0.88137358701954302523260932497968",
        "-0x0.e1a1b30bcea13660d8f99e8dd",
        Greater,
    );
    test("0.00024", "0x0.001", 1, Nearest, "8.0", "0x8.0", Less);
    test("0.00024", "0x0.001", 10, Floor, "9.00", "0x9.0", Less);
    test(
        "0.00024", "0x0.001", 10, Ceiling, "9.0156", "0x9.04", Greater,
    );
    test(
        "0.00024",
        "0x0.001",
        100,
        Nearest,
        "9.0109133621804498832047774704774",
        "0x9.02cb37d5a78915b9409d740d",
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
        "0.31299",
        "0x0.502",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Ceiling,
        "0.3135",
        "0x0.504",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        100,
        Nearest,
        "0.313165880450868387695588582822",
        "0x0.502ba39ec27042b2fa0529e7",
        Less,
    );
    test(
        "4.3e9", "0x1.0E+8", 1, Nearest, "2.3e-10", "0x1.0E-8", Greater,
    );
    test(
        "4.3e9",
        "0x1.0E+8",
        10,
        Floor,
        "2.3260e-10",
        "0xf.fcE-9",
        Less,
    );
    test(
        "4.3e9", "0x1.0E+8", 10, Ceiling, "2.3e-10", "0x1.0E-8", Greater,
    );
    test(
        "4.3e9",
        "0x1.0E+8",
        100,
        Nearest,
        "2.3283064365386962890414637091935e-10",
        "0xf.fffffffffffffffd55555555E-9",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        1,
        Nearest,
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        10,
        Floor,
        "2.7563e-12041199827",
        "0xf.fcE-10000000001",
        Less,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        10,
        Ceiling,
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        Greater,
    );
    test(
        "3.6e12041199826",
        "0x1.0E+10000000000",
        100,
        Nearest,
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        Greater,
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
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        1,
        Nearest,
        "3.4e10",
        "0x8.0E+8",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        10,
        Floor,
        "2.772e10",
        "0x6.74E+8",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        10,
        Ceiling,
        "2.7750e10",
        "0x6.76E+8",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        100,
        Nearest,
        "27725887223.090959557249230167745",
        "0x6749776f7.17492023a4e2a4818",
        Greater,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        1,
        Nearest,
        "-3.4e10",
        "-0x8.0E+8",
        Less,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        10,
        Floor,
        "-2.7750e10",
        "-0x6.76E+8",
        Less,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        10,
        Ceiling,
        "-2.772e10",
        "-0x6.74E+8",
        Greater,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        100,
        Nearest,
        "-27725887221.99234726858112047636",
        "-0x6749776f5.fe0a7878d4b1fb0b",
        Less,
    );
}

#[test]
fn test_acsch_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let x = Rational::from_str(x).unwrap();
        let r = acsch_rational_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &inverse_hyperbolic_rational_prec_round_generic(
                InverseHyperbolicFunction::Acsch,
                &x,
                prec,
                rm
            ),
            &r
        ));
    };
    test("0", 10, Exact, "NaN", "NaN", Equal);
    test("1/3", 1, Nearest, "2.0", "0x2.0", Greater);
    test("1/3", 10, Floor, "1.8184", "0x1.d18", Less);
    test("1/3", 10, Ceiling, "1.820", "0x1.d2", Greater);
    test(
        "1/3",
        100,
        Nearest,
        "1.8184464592320668234836989635607",
        "0x1.d185b507edc0dfdf653c5b01e",
        Greater,
    );
    test("-22/7", 1, Nearest, "-0.25", "-0x0.4", Greater);
    test("-22/7", 10, Floor, "-0.3135", "-0x0.504", Less);
    test("-22/7", 10, Ceiling, "-0.31299", "-0x0.502", Greater);
    test(
        "-22/7",
        100,
        Nearest,
        "-0.31304384340830898212066562005260",
        "-0x0.5023a42da72003b7db346297b8",
        Less,
    );
    test("1000000000000", 1, Nearest, "9.1e-13", "0x1.0E-10", Less);
    test("1000000000000", 10, Floor, "9.983e-13", "0x1.19E-10", Less);
    test(
        "1000000000000",
        10,
        Ceiling,
        "1.0001e-12",
        "0x1.198E-10",
        Greater,
    );
    test(
        "1000000000000",
        100,
        Nearest,
        "9.9999999999999999999999983333398e-13",
        "0x1.19799812dea11197f27ed6b7eE-10",
        Greater,
    );
    test("-7/2", 1, Nearest, "-0.25", "-0x0.4", Greater);
    test("-7/2", 10, Floor, "-0.2822", "-0x0.484", Less);
    test("-7/2", 10, Ceiling, "-0.28174", "-0x0.482", Greater);
    test(
        "-7/2",
        100,
        Nearest,
        "-0.28196323918998291614041121278267",
        "-0x0.482ebe2afec591874de23ee198",
        Greater,
    );
    test(
        "1/100000000000000000000000",
        1,
        Nearest,
        "64.0",
        "0x4.0E+1",
        Greater,
    );
    test(
        "1/100000000000000000000000",
        10,
        Floor,
        "53.62",
        "0x35.a",
        Less,
    );
    test(
        "1/100000000000000000000000",
        10,
        Ceiling,
        "53.688",
        "0x35.b",
        Greater,
    );
    test(
        "1/100000000000000000000000",
        100,
        Nearest,
        "53.652604319422996041831035579211",
        "0x35.a71113a1266d51e80118ab3c",
        Greater,
    );
}

#[test]
#[should_panic]
fn acsch_prec_round_fail_1() {
    BigExponentFloat::ONE.acsch_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn acsch_prec_round_fail_2() {
    BigExponentFloat::ONE.acsch_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn acsch_prec_round_fail_3() {
    BigExponentFloat::acsch_rational_prec_round(Rational::from(3), 0, Floor);
}

#[test]
#[should_panic]
fn acsch_prec_round_fail_4() {
    BigExponentFloat::acsch_rational_prec_round(Rational::from(3), 10, Exact);
}

#[test]
fn acsch_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_23().test_properties(
        |(x, prec, rm)| {
            let r = acsch_prec_round_all(&x, prec, rm);
            // `Float`'s functions agree with log(1 + t).
            assert!(same_pair(
                &inverse_hyperbolic_prec_round_generic(
                    InverseHyperbolicFunction::Acsch,
                    &x,
                    prec,
                    rm
                ),
                &r
            ));
            check_result(&r, prec);
            if x.is_finite() && !x.is_zero() {
                // acsch is odd
                let (s, o) = acsch_prec_round_all(&-&x, prec, -rm);
                assert!(same(&s, &-&r.0));
                assert_eq!(o, r.1.reverse());
            }
            if big_exponent_float_exponent_is_small(&x)
                && let Ok(q) = Rational::try_from(&x)
            {
                assert!(same_pair(&acsch_rational_prec_round_all(&q, prec, rm), &r));
            }
        },
    );
}

#[test]
fn acsch_prec_round_float_agreement_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        // skip the `Float` generator's `Exact` cases
        if x.is_zero() || rm == Exact {
            return;
        }
        let (y, o) = x.acsch_prec_round_ref(prec, rm);
        if float_result_comparable(&y, o) {
            let r = acsch_prec_round_all(&float_to_big_exponent_float(&x), prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
    });
}

#[test]
fn acsch_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        let r = acsch_rational_prec_round_all(&x, prec, rm);
        check_result(&r, prec);
        // `Float`'s functions agree with log(1 + t).
        assert!(same_pair(
            &inverse_hyperbolic_rational_prec_round_generic(
                InverseHyperbolicFunction::Acsch,
                &x,
                prec,
                rm
            ),
            &r
        ));
        if x != 0u32 {
            let (y, o) = Float::acsch_rational_prec_round_ref(&x, prec, rm);
            if float_result_comparable(&y, o) {
                assert!(same(&r.0, &float_to_big_exponent_float(&y)));
                assert_eq!(r.1, o);
            }
        }
    });
}
