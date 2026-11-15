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

fn asinh_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().asinh_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.asinh_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.asinh_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().asinh_prec(prec), &r));
        assert!(same_pair(&x.asinh_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.asinh_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn asinh_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::asinh_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::asinh_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::asinh_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::asinh_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_asinh_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = asinh_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &inverse_hyperbolic_prec_round_generic(InverseHyperbolicFunction::Asinh, &x, prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test(
        "Infinity", "Infinity", 10, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        10,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("0.0", "0x0", 10, Exact, "0.0", "0x0.0", Equal);
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
    test(
        "0.00024", "0x0.001", 1, Nearest, "0.00024", "0x0.001", Greater,
    );
    test(
        "0.00024",
        "0x0.001",
        10,
        Floor,
        "0.00024390",
        "0x0.000ffc",
        Less,
    );
    test(
        "0.00024", "0x0.001", 10, Ceiling, "0.00024", "0x0.001", Greater,
    );
    test(
        "0.00024",
        "0x0.001",
        100,
        Nearest,
        "0.00024414062257468086032431940670858",
        "0x0.000ffffffd555556888887d1ad1b",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        1,
        Nearest,
        "-2.0",
        "-0x2.0",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Floor,
        "-1.863",
        "-0x1.dd",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Ceiling,
        "-1.8613",
        "-0x1.dc8",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        100,
        Nearest,
        "-1.8622957433108481827430517141269",
        "-0x1.dcbf69f10006cbe9c11ca9a5e",
        Less,
    );
    test("20.0", "0x14", 1, Nearest, "4.0", "0x4.0", Greater);
    test("20.0", "0x14", 10, Floor, "3.69", "0x3.b", Less);
    test("20.0", "0x14", 10, Ceiling, "3.6914", "0x3.b1", Greater);
    test(
        "20.0",
        "0x14",
        100,
        Nearest,
        "3.6895038689889056408216535709602",
        "0x3.b0835357c5d76bc90ffbbfe44",
        Less,
    );
    test("4.3e9", "0x1.0E+8", 1, Nearest, "16.0", "0x1.0E+1", Less);
    test("4.3e9", "0x1.0E+8", 10, Floor, "22.844", "0x16.d8", Less);
    test("4.3e9", "0x1.0E+8", 10, Ceiling, "22.88", "0x16.e", Greater);
    test(
        "4.3e9",
        "0x1.0E+8",
        100,
        Nearest,
        "22.87385695847819521078221253527",
        "0x16.dfb516f20bbeaf25465a2698",
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
        "-1.1e12041199827",
        "-0x3.0E+10000000000",
        1,
        Nearest,
        "-3.4e10",
        "-0x8.0E+8",
        Less,
    );
    test(
        "-1.1e12041199827",
        "-0x3.0E+10000000000",
        10,
        Floor,
        "-2.7750e10",
        "-0x6.76E+8",
        Less,
    );
    test(
        "-1.1e12041199827",
        "-0x3.0E+10000000000",
        10,
        Ceiling,
        "-2.772e10",
        "-0x6.74E+8",
        Greater,
    );
    test(
        "-1.1e12041199827",
        "-0x3.0E+10000000000",
        100,
        Nearest,
        "-27725887224.1895718459173398591",
        "-0x6749776f8.3087c7ce75134df8",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        1,
        Nearest,
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        10,
        Floor,
        "2.7563e-12041199827",
        "0xf.fcE-10000000001",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        10,
        Ceiling,
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        100,
        Nearest,
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        Greater,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        1,
        Nearest,
        "-5.5e-12041199827",
        "-0x2.0E-10000000000",
        Greater,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        10,
        Floor,
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        Less,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        10,
        Ceiling,
        "-8.2662e-12041199827",
        "-0x2.ffE-10000000000",
        Greater,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        100,
        Nearest,
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456",
        1,
        Nearest,
        "2.4e-323228497",
        "0x1.0E-268435456",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456",
        10,
        Floor,
        "2.3802e-323228497",
        "0xf.fcE-268435457",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456",
        10,
        Ceiling,
        "2.4e-323228497",
        "0x1.0E-268435456",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456",
        100,
        Nearest,
        "2.4e-323228497",
        "0x1.0E-268435456",
        Greater,
    );
}

#[test]
fn test_asinh_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let x = Rational::from_str(x).unwrap();
        let r = asinh_rational_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &inverse_hyperbolic_rational_prec_round_generic(
                InverseHyperbolicFunction::Asinh,
                &x,
                prec,
                rm
            ),
            &r
        ));
    };
    test("0", 10, Exact, "0.0", "0x0.0", Equal);
    test("1/3", 1, Nearest, "0.25", "0x0.4", Less);
    test("1/3", 10, Floor, "0.3271", "0x0.53c", Less);
    test("1/3", 10, Ceiling, "0.32764", "0x0.53e", Greater);
    test(
        "1/3",
        100,
        Nearest,
        "0.32745015023725844332253525998813",
        "0x0.53d3c5e656dd20ec63b484c7e8",
        Less,
    );
    test("-22/7", 1, Nearest, "-2.0", "-0x2.0", Less);
    test("-22/7", 10, Floor, "-1.863", "-0x1.dd", Less);
    test("-22/7", 10, Ceiling, "-1.8613", "-0x1.dc8", Greater);
    test(
        "-22/7",
        100,
        Nearest,
        "-1.8626792111346099850190014829502",
        "-0x1.dcd88b76c3216f8eaa82640e6",
        Less,
    );
    test("1000000000000", 1, Nearest, "32.0", "0x2.0E+1", Greater);
    test("1000000000000", 10, Floor, "28.31", "0x1c.5", Less);
    test("1000000000000", 10, Ceiling, "28.344", "0x1c.58", Greater);
    test(
        "1000000000000",
        100,
        Nearest,
        "28.3241682964884935176331298277",
        "0x1c.52fcb187d16fbdf7ba2c292",
        Greater,
    );
    test("-7/2", 1, Nearest, "-2.0", "-0x2.0", Less);
    test("-7/2", 10, Floor, "-1.9668", "-0x1.f78", Less);
    test("-7/2", 10, Ceiling, "-1.965", "-0x1.f7", Greater);
    test(
        "-7/2",
        100,
        Nearest,
        "-1.9657204716496515212387563486700",
        "-0x1.f73974f2d01d63cd3093c3d92",
        Greater,
    );
    test(
        "1/100000000000000000000000",
        1,
        Nearest,
        "1.3e-23",
        "0x1.0E-19",
        Greater,
    );
    test(
        "1/100000000000000000000000",
        10,
        Floor,
        "9.9908e-24",
        "0xc.14E-20",
        Less,
    );
    test(
        "1/100000000000000000000000",
        10,
        Ceiling,
        "1.000e-23",
        "0xc.18E-20",
        Greater,
    );
    test(
        "1/100000000000000000000000",
        100,
        Nearest,
        "1.0000000000000000000000000000001e-23",
        "0xc.16d9a0095928a2775b7053c1E-20",
        Greater,
    );
}

#[test]
#[should_panic]
fn asinh_prec_round_fail_1() {
    BigExponentFloat::ONE.asinh_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn asinh_prec_round_fail_2() {
    BigExponentFloat::ONE.asinh_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn asinh_prec_round_fail_3() {
    BigExponentFloat::asinh_rational_prec_round(Rational::from(3), 0, Floor);
}

#[test]
#[should_panic]
fn asinh_prec_round_fail_4() {
    BigExponentFloat::asinh_rational_prec_round(Rational::from(3), 10, Exact);
}

#[test]
fn asinh_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_23().test_properties(
        |(x, prec, rm)| {
            let r = asinh_prec_round_all(&x, prec, rm);
            // `Float`'s functions agree with log(1 + t).
            assert!(same_pair(
                &inverse_hyperbolic_prec_round_generic(
                    InverseHyperbolicFunction::Asinh,
                    &x,
                    prec,
                    rm
                ),
                &r
            ));
            check_result(&r, prec);
            if x.is_finite() && !x.is_zero() {
                // asinh is odd
                let (s, o) = asinh_prec_round_all(&-&x, prec, -rm);
                assert!(same(&s, &-&r.0));
                assert_eq!(o, r.1.reverse());
            }
            if big_exponent_float_exponent_is_small(&x)
                && let Ok(q) = Rational::try_from(&x)
            {
                assert!(same_pair(&asinh_rational_prec_round_all(&q, prec, rm), &r));
            }
        },
    );
}

#[test]
fn asinh_prec_round_float_agreement_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        // skip the `Float` generator's `Exact` cases
        if x.is_zero() || rm == Exact {
            return;
        }
        let (y, o) = x.asinh_prec_round_ref(prec, rm);
        if float_result_comparable(&y, o) {
            let r = asinh_prec_round_all(&float_to_big_exponent_float(&x), prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
    });
}

#[test]
fn asinh_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        let r = asinh_rational_prec_round_all(&x, prec, rm);
        check_result(&r, prec);
        // `Float`'s functions agree with log(1 + t).
        assert!(same_pair(
            &inverse_hyperbolic_rational_prec_round_generic(
                InverseHyperbolicFunction::Asinh,
                &x,
                prec,
                rm
            ),
            &r
        ));
        if x != 0u32 {
            let (y, o) = Float::asinh_rational_prec_round_ref(&x, prec, rm);
            if float_result_comparable(&y, o) {
                assert!(same(&r.0, &float_to_big_exponent_float(&y)));
                assert_eq!(r.1, o);
            }
        }
    });
}
