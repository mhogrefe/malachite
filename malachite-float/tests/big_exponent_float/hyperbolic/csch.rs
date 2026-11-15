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

fn csch_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().csch_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.csch_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.csch_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().csch_prec(prec), &r));
        assert!(same_pair(&x.csch_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.csch_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn csch_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::csch_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::csch_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::csch_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::csch_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_csch_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = csch_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &hyperbolic_prec_round_generic(HyperbolicFunction::Csch, &x, prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "0.0", "0x0.0", Equal);
    test("-Infinity", "-Infinity", 10, Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0", 10, Exact, "NaN", "NaN", Equal);
    test("1.0", "0x1", 1, Nearest, "1.0", "0x1.0", Greater);
    test("1.0", "0x1", 10, Floor, "0.85059", "0x0.d9c", Less);
    test("1.0", "0x1", 10, Ceiling, "0.8516", "0x0.da", Greater);
    test(
        "1.0",
        "0x1",
        100,
        Nearest,
        "0.850918128239321545133842763287",
        "0x0.d9d5c53c5c86070b36fe901e",
        Greater,
    );
    test("-1.0", "-0x1", 1, Nearest, "-1.0", "-0x1.0", Less);
    test("-1.0", "-0x1", 10, Floor, "-0.8516", "-0x0.da", Less);
    test("-1.0", "-0x1", 10, Ceiling, "-0.85059", "-0x0.d9c", Greater);
    test(
        "-1.0",
        "-0x1",
        100,
        Nearest,
        "-0.850918128239321545133842763287",
        "-0x0.d9d5c53c5c86070b36fe901e",
        Less,
    );
    test(
        "0.00024", "0x0.001", 1, Nearest, "4.1e3", "0x1.0E+3", Greater,
    );
    test("0.00024", "0x0.001", 10, Floor, "4092.0", "0xffc.0", Less);
    test(
        "0.00024", "0x0.001", 10, Ceiling, "4.1e3", "0x1.0E+3", Greater,
    );
    test(
        "0.00024",
        "0x0.001",
        100,
        Nearest,
        "4095.999959309896116287238773257",
        "0xfff.fffd555555a4fa4f9c9474",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        1,
        Nearest,
        "-0.062",
        "-0x0.1",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Floor,
        "-0.08667",
        "-0x0.163",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Ceiling,
        "-0.086548",
        "-0x0.1628",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        100,
        Nearest,
        "-0.086589537530046952472299189409366",
        "-0x0.162abb5fde08708a3696958e62",
        Greater,
    );
    test("20.0", "0x14", 1, Nearest, "3.7e-9", "0x1.0E-7", Less);
    test("20.0", "0x14", 10, Floor, "4.118e-9", "0x1.1bE-7", Less);
    test(
        "20.0",
        "0x14",
        10,
        Ceiling,
        "4.1255e-9",
        "0x1.1b8E-7",
        Greater,
    );
    test(
        "20.0",
        "0x14",
        100,
        Nearest,
        "4.1223072448771156734449022857046e-9",
        "0x1.1b48655f37266eb703a97f66eE-7",
        Less,
    );
    test(
        "-712.0",
        "-0x2c8",
        1,
        Nearest,
        "-1.4e-309",
        "-0x4.0E-257",
        Less,
    );
    test(
        "-712.0",
        "-0x2c8",
        10,
        Floor,
        "-1.2128e-309",
        "-0x3.7dE-257",
        Less,
    );
    test(
        "-712.0",
        "-0x2c8",
        10,
        Ceiling,
        "-1.211e-309",
        "-0x3.7cE-257",
        Greater,
    );
    test(
        "-712.0",
        "-0x2c8",
        100,
        Nearest,
        "-1.211598928399783424426631081937e-309",
        "-0x3.7c24905d969c434646475cf98E-257",
        Less,
    );
    test(
        "44.50",
        "0x2c.8",
        1,
        Nearest,
        "1.1e-19",
        "0x2.0E-16",
        Greater,
    );
    test(
        "44.50",
        "0x2c.8",
        10,
        Floor,
        "9.4338e-20",
        "0x1.bd8E-16",
        Less,
    );
    test(
        "44.50",
        "0x2c.8",
        10,
        Ceiling,
        "9.444e-20",
        "0x1.beE-16",
        Greater,
    );
    test(
        "44.50",
        "0x2c.8",
        100,
        Nearest,
        "9.4389905430522468327211693835322e-20",
        "0x1.bdbe64cf27cd47642ea875e5aE-16",
        Greater,
    );
    test(
        "4.3e9",
        "0x1.0E+8",
        1,
        Nearest,
        "4.1e-1865280597",
        "0x4.0E-1549082005",
        Less,
    );
    test(
        "4.3e9",
        "0x1.0E+8",
        10,
        Floor,
        "4.9281e-1865280597",
        "0x4.daE-1549082005",
        Less,
    );
    test(
        "4.3e9",
        "0x1.0E+8",
        10,
        Ceiling,
        "4.936e-1865280597",
        "0x4.dcE-1549082005",
        Greater,
    );
    test(
        "4.3e9",
        "0x1.0E+8",
        100,
        Nearest,
        "4.93514093872744601206381905049e-1865280597",
        "0x4.dbc8842f96902982a1f17618E-1549082005",
        Greater,
    );
    test(
        "-4.3e9",
        "-0x1.0E+8",
        1,
        Nearest,
        "-4.1e-1865280597",
        "-0x4.0E-1549082005",
        Greater,
    );
    test(
        "-4.3e9",
        "-0x1.0E+8",
        10,
        Floor,
        "-4.936e-1865280597",
        "-0x4.dcE-1549082005",
        Less,
    );
    test(
        "-4.3e9",
        "-0x1.0E+8",
        10,
        Ceiling,
        "-4.9281e-1865280597",
        "-0x4.daE-1549082005",
        Greater,
    );
    test(
        "-4.3e9",
        "-0x1.0E+8",
        100,
        Nearest,
        "-4.93514093872744601206381905049e-1865280597",
        "-0x4.dbc8842f96902982a1f17618E-1549082005",
        Less,
    );
    test(
        "3.3e12",
        "0x3.0E+10",
        1,
        Nearest,
        "2.7e-1432535498195",
        "0x1.0E-1189694979594",
        Less,
    );
    test(
        "3.3e12",
        "0x3.0E+10",
        10,
        Floor,
        "3.665e-1432535498195",
        "0x1.5bE-1189694979594",
        Less,
    );
    test(
        "3.3e12",
        "0x3.0E+10",
        10,
        Ceiling,
        "3.6699e-1432535498195",
        "0x1.5b8E-1189694979594",
        Greater,
    );
    test(
        "3.3e12",
        "0x3.0E+10",
        100,
        Nearest,
        "3.6648422930346496528426284413358e-1432535498195",
        "0x1.5b06848982a8e67a8d9d11ab6E-1189694979594",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        1,
        Nearest,
        "3.6e12041199826",
        "0x1.0E+10000000000",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        10,
        Floor,
        "3.6210e12041199826",
        "0xf.fcE+9999999999",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        10,
        Ceiling,
        "3.6e12041199826",
        "0x1.0E+10000000000",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        100,
        Nearest,
        "3.6e12041199826",
        "0x1.0E+10000000000",
        Greater,
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
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456",
        10,
        Floor,
        "4.1931e323228496",
        "0xf.fcE+268435455",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456",
        10,
        Ceiling,
        "4.2e323228496",
        "0x1.0E+268435456",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456",
        100,
        Nearest,
        "4.2e323228496",
        "0x1.0E+268435456",
        Greater,
    );
    test(
        "1.2e-1209",
        "0x0.0001E-1000",
        1,
        Nearest,
        "8.6e1208",
        "0x1.0E+1004",
        Greater,
    );
    test(
        "1.2e-1209",
        "0x0.0001E-1000",
        10,
        Floor,
        "8.6305e1208",
        "0xf.fcE+1003",
        Less,
    );
    test(
        "1.2e-1209",
        "0x0.0001E-1000",
        10,
        Ceiling,
        "8.6e1208",
        "0x1.0E+1004",
        Greater,
    );
    test(
        "1.2e-1209",
        "0x0.0001E-1000",
        100,
        Nearest,
        "8.6e1208",
        "0x1.0E+1004",
        Greater,
    );
}

#[test]
fn test_csch_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let r = csch_rational_prec_round_all(&Rational::from_str(x).unwrap(), prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("0", 10, Exact, "NaN", "NaN", Equal);
    test("1/3", 1, Nearest, "2.0", "0x2.0", Less);
    test("1/3", 10, Floor, "2.9414", "0x2.f1", Less);
    test("1/3", 10, Ceiling, "2.945", "0x2.f2", Greater);
    test(
        "1/3",
        100,
        Nearest,
        "2.9451562666948143476455860034951",
        "0x2.f1f5c2d7104d708fac22e7c6c",
        Less,
    );
    test("-22/7", 1, Nearest, "-0.062", "-0x0.1", Greater);
    test("-22/7", 10, Floor, "-0.086548", "-0x0.1628", Less);
    test("-22/7", 10, Ceiling, "-0.08643", "-0x0.162", Greater);
    test(
        "-22/7",
        100,
        Nearest,
        "-0.08647970651862787589103205270538",
        "-0x0.162388b743e325b1326cd6dab8",
        Greater,
    );
    test(
        "1000000000000",
        1,
        Nearest,
        "1.1e-434294481903",
        "0x1.0E-360673760222",
        Less,
    );
    test(
        "1000000000000",
        10,
        Floor,
        "1.1196e-434294481903",
        "0x1.068E-360673760222",
        Less,
    );
    test(
        "1000000000000",
        10,
        Ceiling,
        "1.122e-434294481903",
        "0x1.07E-360673760222",
        Greater,
    );
    test(
        "1000000000000",
        100,
        Nearest,
        "1.119959568460761401085773703948e-434294481903",
        "0x1.0693855b39f13e2cc52ee2958E-360673760222",
        Greater,
    );
    test("-7/2", 1, Nearest, "-0.062", "-0x0.1", Less);
    test("-7/2", 10, Floor, "-0.060486", "-0x0.0f7c", Less);
    test("-7/2", 10, Ceiling, "-0.06042", "-0x0.0f78", Greater);
    test(
        "-7/2",
        100,
        Nearest,
        "-0.06044989000915610506042891899828",
        "-0x0.0f79a4dca2d8ee75e5f3ce63b2",
        Less,
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
        Greater,
    );
}

#[test]
#[should_panic]
fn csch_prec_round_fail_1() {
    BigExponentFloat::ONE.csch_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn csch_prec_round_fail_2() {
    BigExponentFloat::ONE.csch_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn csch_prec_round_fail_3() {
    BigExponentFloat::csch_rational_prec_round(Rational::from(3), 0, Floor);
}

#[test]
#[should_panic]
fn csch_prec_round_fail_4() {
    BigExponentFloat::csch_rational_prec_round(Rational::from(3), 10, Exact);
}

#[test]
fn csch_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_9().test_properties(
        |(x, prec, rm)| {
            let r = csch_prec_round_all(&x, prec, rm);
            // `Float`'s functions agree with the series and e^|x|.
            assert!(same_pair(
                &hyperbolic_prec_round_generic(HyperbolicFunction::Csch, &x, prec, rm),
                &r
            ));
            check_result(&r, prec);
            if x.is_finite() && !x.is_zero() {
                // csch is odd
                let (s, o) = csch_prec_round_all(&-&x, prec, -rm);
                assert!(same(&s, &-&r.0));
                assert_eq!(o, r.1.reverse());
            }
            if big_exponent_float_exponent_is_small(&x)
                && let Ok(q) = Rational::try_from(&x)
            {
                assert!(same_pair(&csch_rational_prec_round_all(&q, prec, rm), &r));
            }
        },
    );
}

#[test]
fn csch_prec_round_float_agreement_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        // the `Float` generator's `Exact` cases are those of exp
        if x.is_zero() || rm == Exact {
            return;
        }
        let (y, o) = x.csch_prec_round_ref(prec, rm);
        if float_result_comparable(&y, o) {
            let r = csch_prec_round_all(&float_to_big_exponent_float(&x), prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
    });
}

#[test]
fn csch_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_19().test_properties(|(x, prec, rm)| {
        let r = csch_rational_prec_round_all(&x, prec, rm);
        check_result(&r, prec);
        if x != 0u32 {
            let (y, o) = Float::csch_rational_prec_round_ref(&x, prec, rm);
            if float_result_comparable(&y, o) {
                assert!(same(&r.0, &float_to_big_exponent_float(&y)));
                assert_eq!(r.1, o);
            }
        }
    });
}
