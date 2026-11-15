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
use malachite_float::big_exponent_float::hyperbolic::sinh_cosh::{
    HyperbolicFunction, hyperbolic_prec_round_generic,
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

fn coth_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().coth_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.coth_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.coth_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().coth_prec(prec), &r));
        assert!(same_pair(&x.coth_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.coth_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn coth_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::coth_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::coth_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::coth_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::coth_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_coth_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = coth_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &hyperbolic_prec_round_generic(HyperbolicFunction::Coth, &x, prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "1.0", "0x1.0", Equal);
    test("-Infinity", "-Infinity", 10, Floor, "-1.0", "-0x1.0", Equal);
    test("0.0", "0x0", 10, Exact, "NaN", "NaN", Equal);
    test("1.0", "0x1", 1, Nearest, "1.0", "0x1.0", Less);
    test("1.0", "0x1", 10, Floor, "1.31", "0x1.5", Less);
    test("1.0", "0x1", 10, Ceiling, "1.3145", "0x1.508", Greater);
    test(
        "1.0",
        "0x1",
        100,
        Nearest,
        "1.3130352854993313036361612469313",
        "0x1.50231499b6b1d49f813a188fa",
        Greater,
    );
    test("-1.0", "-0x1", 1, Nearest, "-1.0", "-0x1.0", Greater);
    test("-1.0", "-0x1", 10, Floor, "-1.3145", "-0x1.508", Less);
    test("-1.0", "-0x1", 10, Ceiling, "-1.31", "-0x1.5", Greater);
    test(
        "-1.0",
        "-0x1",
        100,
        Nearest,
        "-1.3130352854993313036361612469313",
        "-0x1.50231499b6b1d49f813a188fa",
        Less,
    );
    test("0.00024", "0x0.001", 1, Nearest, "4.1e3", "0x1.0E+3", Less);
    test("0.00024", "0x0.001", 10, Floor, "4.1e3", "0x1.0E+3", Less);
    test(
        "0.00024", "0x0.001", 10, Ceiling, "4104.0", "0x1008.0", Greater,
    );
    test(
        "0.00024",
        "0x0.001",
        100,
        Nearest,
        "4096.0000813802080099574412053106",
        "0x1000.0005555554fa4fa502fada",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        1,
        Nearest,
        "-1.0",
        "-0x1.0",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Floor,
        "-1.004",
        "-0x1.01",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Ceiling,
        "-1.0020",
        "-0x1.008",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        100,
        Nearest,
        "-1.003741873197321289119762001281",
        "-0x1.00f53a37021e3cddb880cf7b4",
        Less,
    );
    test("20.0", "0x14", 1, Nearest, "1.0", "0x1.0", Less);
    test("20.0", "0x14", 10, Floor, "1.0", "0x1.0", Less);
    test("20.0", "0x14", 10, Ceiling, "1.0020", "0x1.008", Greater);
    test(
        "20.0",
        "0x14",
        100,
        Nearest,
        "1.0000000000000000084967085105839",
        "0x1.000000000000009cbc924cd8e",
        Greater,
    );
    test("-712.0", "-0x2c8", 1, Nearest, "-1.0", "-0x1.0", Greater);
    test("-712.0", "-0x2c8", 10, Floor, "-1.0020", "-0x1.008", Less);
    test("-712.0", "-0x2c8", 10, Ceiling, "-1.0", "-0x1.0", Greater);
    test("-712.0", "-0x2c8", 100, Nearest, "-1.0", "-0x1.0", Greater);
    test("44.50", "0x2c.8", 1, Nearest, "1.0", "0x1.0", Less);
    test("44.50", "0x2c.8", 10, Floor, "1.0", "0x1.0", Less);
    test("44.50", "0x2c.8", 10, Ceiling, "1.0020", "0x1.008", Greater);
    test("44.50", "0x2c.8", 100, Nearest, "1.0", "0x1.0", Less);
    test("4.3e9", "0x1.0E+8", 1, Nearest, "1.0", "0x1.0", Less);
    test("4.3e9", "0x1.0E+8", 10, Floor, "1.0", "0x1.0", Less);
    test(
        "4.3e9", "0x1.0E+8", 10, Ceiling, "1.0020", "0x1.008", Greater,
    );
    test("4.3e9", "0x1.0E+8", 100, Nearest, "1.0", "0x1.0", Less);
    test("-4.3e9", "-0x1.0E+8", 1, Nearest, "-1.0", "-0x1.0", Greater);
    test(
        "-4.3e9",
        "-0x1.0E+8",
        10,
        Floor,
        "-1.0020",
        "-0x1.008",
        Less,
    );
    test(
        "-4.3e9",
        "-0x1.0E+8",
        10,
        Ceiling,
        "-1.0",
        "-0x1.0",
        Greater,
    );
    test(
        "-4.3e9",
        "-0x1.0E+8",
        100,
        Nearest,
        "-1.0",
        "-0x1.0",
        Greater,
    );
    test("3.3e12", "0x3.0E+10", 1, Nearest, "1.0", "0x1.0", Less);
    test("3.3e12", "0x3.0E+10", 10, Floor, "1.0", "0x1.0", Less);
    test(
        "3.3e12",
        "0x3.0E+10",
        10,
        Ceiling,
        "1.0020",
        "0x1.008",
        Greater,
    );
    test("3.3e12", "0x3.0E+10", 100, Nearest, "1.0", "0x1.0", Less);
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        1,
        Nearest,
        "3.6e12041199826",
        "0x1.0E+10000000000",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        10,
        Floor,
        "3.6e12041199826",
        "0x1.0E+10000000000",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        10,
        Ceiling,
        "3.6316e12041199826",
        "0x1.008E+10000000000",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        100,
        Nearest,
        "3.6e12041199826",
        "0x1.0E+10000000000",
        Less,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        1,
        Nearest,
        "-9.1e12041199825",
        "-0x4.0E+9999999999",
        Greater,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        10,
        Floor,
        "-1.2088e12041199826",
        "-0x5.56E+9999999999",
        Less,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        10,
        Ceiling,
        "-1.207e12041199826",
        "-0x5.54E+9999999999",
        Greater,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        100,
        Nearest,
        "-1.2081658445059049654194101037121e12041199826",
        "-0x5.5555555555555555555555558E+9999999999",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456",
        1,
        Nearest,
        "4.2e323228496",
        "0x1.0E+268435456",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456",
        10,
        Floor,
        "4.2e323228496",
        "0x1.0E+268435456",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456",
        10,
        Ceiling,
        "4.2054e323228496",
        "0x1.008E+268435456",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456",
        100,
        Nearest,
        "4.2e323228496",
        "0x1.0E+268435456",
        Less,
    );
    test(
        "1.2e-1209",
        "0x0.0001E-1000",
        1,
        Nearest,
        "8.6e1208",
        "0x1.0E+1004",
        Less,
    );
    test(
        "1.2e-1209",
        "0x0.0001E-1000",
        10,
        Floor,
        "8.6e1208",
        "0x1.0E+1004",
        Less,
    );
    test(
        "1.2e-1209",
        "0x0.0001E-1000",
        10,
        Ceiling,
        "8.6559e1208",
        "0x1.008E+1004",
        Greater,
    );
    test(
        "1.2e-1209",
        "0x0.0001E-1000",
        100,
        Nearest,
        "8.6e1208",
        "0x1.0E+1004",
        Less,
    );
}

#[test]
fn test_coth_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let r = coth_rational_prec_round_all(&Rational::from_str(x).unwrap(), prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("0", 10, Exact, "NaN", "NaN", Equal);
    test("1/3", 1, Nearest, "4.0", "0x4.0", Greater);
    test("1/3", 10, Floor, "3.109", "0x3.1c", Less);
    test("1/3", 10, Ceiling, "3.1133", "0x3.1d", Greater);
    test(
        "1/3",
        100,
        Nearest,
        "3.11029667961944370137837523142",
        "0x3.1c3c6737d2a99a2538dde3dc",
        Greater,
    );
    test("-22/7", 1, Nearest, "-1.0", "-0x1.0", Greater);
    test("-22/7", 10, Floor, "-1.004", "-0x1.01", Less);
    test("-22/7", 10, Ceiling, "-1.0020", "-0x1.008", Greater);
    test(
        "-22/7",
        100,
        Nearest,
        "-1.003732404398477118693835025096",
        "-0x1.00f49b5ad3b018d5106a69234",
        Less,
    );
    test("1000000000000", 1, Nearest, "1.0", "0x1.0", Less);
    test("1000000000000", 10, Floor, "1.0", "0x1.0", Less);
    test("1000000000000", 10, Ceiling, "1.0020", "0x1.008", Greater);
    test("1000000000000", 100, Nearest, "1.0", "0x1.0", Less);
    test("-7/2", 1, Nearest, "-1.0", "-0x1.0", Greater);
    test("-7/2", 10, Floor, "-1.0020", "-0x1.008", Less);
    test("-7/2", 10, Ceiling, "-1.0", "-0x1.0", Greater);
    test(
        "-7/2",
        100,
        Nearest,
        "-1.0018254285064434673298498027695",
        "-0x1.0077a19bbc82247d05eae44aa",
        Greater,
    );
    test(
        "1/100000000000000000000000",
        1,
        Nearest,
        "7.6e22",
        "0x1.0E+19",
        Less,
    );
    test(
        "1/100000000000000000000000",
        10,
        Floor,
        "9.9908e22",
        "0x1.528E+19",
        Less,
    );
    test(
        "1/100000000000000000000000",
        10,
        Ceiling,
        "1.001e23",
        "0x1.53E+19",
        Greater,
    );
    test(
        "1/100000000000000000000000",
        100,
        Nearest,
        "1.00000000000000000e23",
        "0x1.52d02c7e14af68E+19",
        Less,
    );
}

#[test]
#[should_panic]
fn coth_prec_round_fail_1() {
    BigExponentFloat::ONE.coth_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn coth_prec_round_fail_2() {
    BigExponentFloat::ONE.coth_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn coth_prec_round_fail_3() {
    BigExponentFloat::coth_rational_prec_round(Rational::from(3), 0, Floor);
}

#[test]
#[should_panic]
fn coth_prec_round_fail_4() {
    BigExponentFloat::coth_rational_prec_round(Rational::from(3), 10, Exact);
}

#[test]
fn coth_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_9().test_properties(
        |(x, prec, rm)| {
            let r = coth_prec_round_all(&x, prec, rm);
            // `Float`'s functions agree with the series and e^|x|.
            assert!(same_pair(
                &hyperbolic_prec_round_generic(HyperbolicFunction::Coth, &x, prec, rm),
                &r
            ));
            check_result(&r, prec);
            if x.is_finite() && !x.is_zero() {
                // coth is odd
                let (s, o) = coth_prec_round_all(&-&x, prec, -rm);
                assert!(same(&s, &-&r.0));
                assert_eq!(o, r.1.reverse());
            }
            if big_exponent_float_exponent_is_small(&x)
                && let Ok(q) = Rational::try_from(&x)
            {
                assert!(same_pair(&coth_rational_prec_round_all(&q, prec, rm), &r));
            }
        },
    );
}

#[test]
fn coth_prec_round_float_agreement_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        // the `Float` generator's `Exact` cases are those of exp
        if x.is_zero() || rm == Exact {
            return;
        }
        let (y, o) = x.coth_prec_round_ref(prec, rm);
        if float_result_comparable(&y, o) {
            let r = coth_prec_round_all(&float_to_big_exponent_float(&x), prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
    });
}

#[test]
fn coth_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_19().test_properties(|(x, prec, rm)| {
        let r = coth_rational_prec_round_all(&x, prec, rm);
        check_result(&r, prec);
        if x != 0u32 {
            let (y, o) = Float::coth_rational_prec_round_ref(&x, prec, rm);
            if float_result_comparable(&y, o) {
                assert!(same(&r.0, &float_to_big_exponent_float(&y)));
                assert_eq!(r.1, o);
            }
        }
    });
}
