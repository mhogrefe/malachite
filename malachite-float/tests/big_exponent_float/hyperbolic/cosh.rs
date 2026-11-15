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

fn cosh_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().cosh_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.cosh_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.cosh_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().cosh_prec(prec), &r));
        assert!(same_pair(&x.cosh_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.cosh_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn cosh_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::cosh_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::cosh_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::cosh_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::cosh_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_cosh_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = cosh_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &hyperbolic_prec_round_generic(HyperbolicFunction::Cosh, &x, prec, rm),
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
        "Infinity",
        "Infinity",
        Equal,
    );
    test("0.0", "0x0", 10, Exact, "1.0", "0x1.0", Equal);
    test("1.0", "0x1", 1, Nearest, "2.0", "0x2.0", Greater);
    test("1.0", "0x1", 10, Floor, "1.543", "0x1.8b", Less);
    test("1.0", "0x1", 10, Ceiling, "1.5449", "0x1.8b8", Greater);
    test(
        "1.0",
        "0x1",
        100,
        Nearest,
        "1.5430806348152437784779056207575",
        "0x1.8b07551d9f5504c2bd281001a",
        Greater,
    );
    test("-1.0", "-0x1", 1, Nearest, "2.0", "0x2.0", Greater);
    test("-1.0", "-0x1", 10, Floor, "1.543", "0x1.8b", Less);
    test("-1.0", "-0x1", 10, Ceiling, "1.5449", "0x1.8b8", Greater);
    test(
        "-1.0",
        "-0x1",
        100,
        Nearest,
        "1.5430806348152437784779056207575",
        "0x1.8b07551d9f5504c2bd281001a",
        Greater,
    );
    test("0.00024", "0x0.001", 1, Nearest, "1.0", "0x1.0", Less);
    test("0.00024", "0x0.001", 10, Floor, "1.0", "0x1.0", Less);
    test(
        "0.00024", "0x0.001", 10, Ceiling, "1.0020", "0x1.008", Greater,
    );
    test(
        "0.00024",
        "0x0.001",
        100,
        Nearest,
        "1.0000000298023225357250494107964",
        "0x1.0000008000000aaaaaab05b06",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        1,
        Nearest,
        "8.0",
        "0x8.0",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Floor,
        "11.578",
        "0xb.94",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        10,
        Ceiling,
        "11.59",
        "0xb.98",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30",
        100,
        Nearest,
        "11.591953275521519213439083247288",
        "0xb.978a3ff7200027b1b4b07db3",
        Greater,
    );
    test("20.0", "0x14", 1, Nearest, "2.7e8", "0x1.0E+7", Greater);
    test("20.0", "0x14", 10, Floor, "2.4248e8", "0xe.74E+6", Less);
    test("20.0", "0x14", 10, Ceiling, "2.427e8", "0xe.78E+6", Greater);
    test(
        "20.0",
        "0x14",
        100,
        Nearest,
        "242582597.70489514001513022648996",
        "0xe758445.b4740205796d617989",
        Less,
    );
    test(
        "-712.0",
        "-0x2c8",
        1,
        Nearest,
        "7.2e308",
        "0x4.0E+256",
        Less,
    );
    test(
        "-712.0",
        "-0x2c8",
        10,
        Floor,
        "8.2441e308",
        "0x4.96E+256",
        Less,
    );
    test(
        "-712.0",
        "-0x2c8",
        10,
        Ceiling,
        "8.258e308",
        "0x4.98E+256",
        Greater,
    );
    test(
        "-712.0",
        "-0x2c8",
        100,
        Nearest,
        "8.25355632594317134317929609942e308",
        "0x4.97586e6c35f929da5dd362eE+256",
        Greater,
    );
    test("44.50", "0x2c.8", 1, Nearest, "9.2e18", "0x8.0E+15", Less);
    test("44.50", "0x2c.8", 10, Floor, "1.059e19", "0x9.3E+15", Less);
    test(
        "44.50",
        "0x2c.8",
        10,
        Ceiling,
        "1.0610e19",
        "0x9.34E+15",
        Greater,
    );
    test(
        "44.50",
        "0x2c.8",
        100,
        Nearest,
        "10594353235538195474.46005490501",
        "0x9306b422ed43b412.75c628836",
        Less,
    );
    test(
        "4.3e9",
        "0x1.0E+8",
        1,
        Nearest,
        "2.5e1865280596",
        "0x4.0E+1549082004",
        Greater,
    );
    test(
        "4.3e9",
        "0x1.0E+8",
        10,
        Floor,
        "2.0262e1865280596",
        "0x3.4bE+1549082004",
        Less,
    );
    test(
        "4.3e9",
        "0x1.0E+8",
        10,
        Ceiling,
        "2.029e1865280596",
        "0x3.4cE+1549082004",
        Greater,
    );
    test(
        "4.3e9",
        "0x1.0E+8",
        100,
        Nearest,
        "2.0262845831873155231567957128889e1865280596",
        "0x3.4b0dbacab6e1019cb5de0349cE+1549082004",
        Greater,
    );
    test(
        "-4.3e9",
        "-0x1.0E+8",
        1,
        Nearest,
        "2.5e1865280596",
        "0x4.0E+1549082004",
        Greater,
    );
    test(
        "-4.3e9",
        "-0x1.0E+8",
        10,
        Floor,
        "2.0262e1865280596",
        "0x3.4bE+1549082004",
        Less,
    );
    test(
        "-4.3e9",
        "-0x1.0E+8",
        10,
        Ceiling,
        "2.029e1865280596",
        "0x3.4cE+1549082004",
        Greater,
    );
    test(
        "-4.3e9",
        "-0x1.0E+8",
        100,
        Nearest,
        "2.0262845831873155231567957128889e1865280596",
        "0x3.4b0dbacab6e1019cb5de0349cE+1549082004",
        Greater,
    );
    test(
        "3.3e12",
        "0x3.0E+10",
        1,
        Nearest,
        "1.8e1432535498194",
        "0x8.0E+1189694979593",
        Less,
    );
    test(
        "3.3e12",
        "0x3.0E+10",
        10,
        Floor,
        "2.7272e1432535498194",
        "0xb.ccE+1189694979593",
        Less,
    );
    test(
        "3.3e12",
        "0x3.0E+10",
        10,
        Ceiling,
        "2.731e1432535498194",
        "0xb.dE+1189694979593",
        Greater,
    );
    test(
        "3.3e12",
        "0x3.0E+10",
        100,
        Nearest,
        "2.728630374902043269116408089554e1432535498194",
        "0xb.cd9c748bd4fd70433bece80eE+1189694979593",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        10,
        Floor,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        10,
        Ceiling,
        "1.0020",
        "0x1.008",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        100,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        10,
        Floor,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        10,
        Ceiling,
        "1.0020",
        "0x1.008",
        Greater,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        100,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456",
        10,
        Floor,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456",
        10,
        Ceiling,
        "1.0020",
        "0x1.008",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456",
        100,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "1.2e-1209",
        "0x0.0001E-1000",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "1.2e-1209",
        "0x0.0001E-1000",
        10,
        Floor,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "1.2e-1209",
        "0x0.0001E-1000",
        10,
        Ceiling,
        "1.0020",
        "0x1.008",
        Greater,
    );
    test(
        "1.2e-1209",
        "0x0.0001E-1000",
        100,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
}

#[test]
fn test_cosh_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let r = cosh_rational_prec_round_all(&Rational::from_str(x).unwrap(), prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("0", 10, Exact, "1.0", "0x1.0", Equal);
    test("1/3", 1, Nearest, "1.0", "0x1.0", Less);
    test("1/3", 10, Floor, "1.055", "0x1.0e", Less);
    test("1/3", 10, Ceiling, "1.0566", "0x1.0e8", Greater);
    test(
        "1/3",
        100,
        Nearest,
        "1.0560718678299393895268647082638",
        "0x1.0e5ab9d68e232bca22825de22",
        Less,
    );
    test("-22/7", 1, Nearest, "8.0", "0x8.0", Less);
    test("-22/7", 10, Floor, "11.59", "0xb.98", Less);
    test("-22/7", 10, Ceiling, "11.609", "0xb.9c", Greater);
    test(
        "-22/7",
        100,
        Nearest,
        "11.606565803761966414401052945153",
        "0xb.9b47e5820793d583f2840c93",
        Greater,
    );
    test(
        "1000000000000",
        1,
        Nearest,
        "9.2e434294481902",
        "0x1.0E+360673760222",
        Greater,
    );
    test(
        "1000000000000",
        10,
        Floor,
        "8.926e434294481902",
        "0xf.98E+360673760221",
        Less,
    );
    test(
        "1000000000000",
        10,
        Ceiling,
        "8.9347e434294481902",
        "0xf.9cE+360673760221",
        Greater,
    );
    test(
        "1000000000000",
        100,
        Nearest,
        "8.9288937579627967444044753019089e434294481902",
        "0xf.996a49f86ac7c47d6aa7c617E+360673760221",
        Less,
    );
    test("-7/2", 1, Nearest, "16.0", "0x1.0E+1", Less);
    test("-7/2", 10, Floor, "16.56", "0x10.9", Less);
    test("-7/2", 10, Ceiling, "16.594", "0x10.98", Greater);
    test(
        "-7/2",
        100,
        Nearest,
        "16.572824671057316125696517821378",
        "0x10.92a4a33c887b3d940350b45e",
        Greater,
    );
    test(
        "1/100000000000000000000000",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "1/100000000000000000000000",
        10,
        Floor,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "1/100000000000000000000000",
        10,
        Ceiling,
        "1.0020",
        "0x1.008",
        Greater,
    );
    test(
        "1/100000000000000000000000",
        100,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
}

#[test]
#[should_panic]
fn cosh_prec_round_fail_1() {
    BigExponentFloat::ONE.cosh_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn cosh_prec_round_fail_2() {
    BigExponentFloat::ONE.cosh_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn cosh_prec_round_fail_3() {
    BigExponentFloat::cosh_rational_prec_round(Rational::from(3), 0, Floor);
}

#[test]
#[should_panic]
fn cosh_prec_round_fail_4() {
    BigExponentFloat::cosh_rational_prec_round(Rational::from(3), 10, Exact);
}

#[test]
fn cosh_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_9().test_properties(
        |(x, prec, rm)| {
            let r = cosh_prec_round_all(&x, prec, rm);
            // `Float`'s functions agree with the series and e^|x|.
            assert!(same_pair(
                &hyperbolic_prec_round_generic(HyperbolicFunction::Cosh, &x, prec, rm),
                &r
            ));
            check_result(&r, prec);
            if x.is_finite() && !x.is_zero() {
                // cosh is even
                assert!(same_pair(&cosh_prec_round_all(&-&x, prec, rm), &r));
            }
            if big_exponent_float_exponent_is_small(&x)
                && let Ok(q) = Rational::try_from(&x)
            {
                assert!(same_pair(&cosh_rational_prec_round_all(&q, prec, rm), &r));
            }
        },
    );
}

#[test]
fn cosh_prec_round_float_agreement_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        // the `Float` generator's `Exact` cases are those of exp
        if x.is_zero() || rm == Exact {
            return;
        }
        let (y, o) = x.cosh_prec_round_ref(prec, rm);
        if float_result_comparable(&y, o) {
            let r = cosh_prec_round_all(&float_to_big_exponent_float(&x), prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
    });
}

#[test]
fn cosh_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_19().test_properties(|(x, prec, rm)| {
        let r = cosh_rational_prec_round_all(&x, prec, rm);
        check_result(&r, prec);
        if x != 0u32 {
            let (y, o) = Float::cosh_rational_prec_round_ref(&x, prec, rm);
            if float_result_comparable(&y, o) {
                assert!(same(&r.0, &float_to_big_exponent_float(&y)));
                assert_eq!(r.1, o);
            }
        }
    });
}
