// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

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

fn atanh_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().atanh_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.atanh_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.atanh_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().atanh_prec(prec), &r));
        assert!(same_pair(&x.atanh_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.atanh_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn atanh_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::atanh_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::atanh_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::atanh_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::atanh_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_atanh_prec_round() {
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let x = hex(x_hex);
        let r = atanh_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &inverse_hyperbolic_prec_round_generic(InverseHyperbolicFunction::Atanh, &x, prec, rm),
            &r
        ));
    };
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0", 10, Exact, "0.0", "0x0.0", Equal);
    test("1.0", "0x1", 10, Exact, "Infinity", "Infinity", Equal);
    test("-1.0", "-0x1", 10, Exact, "-Infinity", "-Infinity", Equal);
    test("2.0", "0x2", 10, Exact, "NaN", "NaN", Equal);
    test(
        "1.1e12041199827",
        "0x3.0E+10000000000",
        10,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test("0.50", "0x0.8", 1, Nearest, "0.50", "0x0.8", Less);
    test("0.50", "0x0.8", 10, Floor, "0.5488", "0x0.8c8", Less);
    test("0.50", "0x0.8", 10, Ceiling, "0.54980", "0x0.8cc", Greater);
    test(
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "0.5493061443340548456976226184611",
        "0x0.8c9f53d5681854bb520cc6aa8",
        Less,
    );
    test("-0.50", "-0x0.8", 1, Nearest, "-0.50", "-0x0.8", Greater);
    test("-0.50", "-0x0.8", 10, Floor, "-0.54980", "-0x0.8cc", Less);
    test(
        "-0.50", "-0x0.8", 10, Ceiling, "-0.5488", "-0x0.8c8", Greater,
    );
    test(
        "-0.50",
        "-0x0.8",
        100,
        Nearest,
        "-0.5493061443340548456976226184611",
        "-0x0.8c9f53d5681854bb520cc6aa8",
        Greater,
    );
    test("0.00024", "0x0.001", 1, Nearest, "0.00024", "0x0.001", Less);
    test("0.00024", "0x0.001", 10, Floor, "0.00024", "0x0.001", Less);
    test(
        "0.00024",
        "0x0.001",
        10,
        Ceiling,
        "0.00024462",
        "0x0.001008",
        Greater,
    );
    test(
        "0.00024",
        "0x0.001",
        100,
        Nearest,
        "0.0002441406298506385829279722521023",
        "0x0.001000000555555888888ad1ad1c",
        Less,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        1,
        Nearest,
        "8.0",
        "0x8.0",
        Less,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        10,
        Floor,
        "11.422",
        "0xb.6c",
        Less,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        10,
        Ceiling,
        "11.44",
        "0xb.7",
        Greater,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        100,
        Nearest,
        "11.43692847918088994446747446504",
        "0xb.6fda8b78c5df5792732d134c",
        Less,
    );
    test(
        "-0.99999999977",
        "-0x0.ffffffff",
        1,
        Nearest,
        "-8.0",
        "-0x8.0",
        Greater,
    );
    test(
        "-0.99999999977",
        "-0x0.ffffffff",
        10,
        Floor,
        "-11.44",
        "-0xb.7",
        Less,
    );
    test(
        "-0.99999999977",
        "-0x0.ffffffff",
        10,
        Ceiling,
        "-11.422",
        "-0xb.6c",
        Greater,
    );
    test(
        "-0.99999999977",
        "-0x0.ffffffff",
        100,
        Nearest,
        "-11.43692847918088994446747446504",
        "-0xb.6fda8b78c5df5792732d134c",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        1,
        Nearest,
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        10,
        Floor,
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        Less,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        10,
        Ceiling,
        "2.7644e-12041199827",
        "0x1.008E-10000000000",
        Greater,
    );
    test(
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        100,
        Nearest,
        "2.8e-12041199827",
        "0x1.0E-10000000000",
        Less,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        1,
        Nearest,
        "-1.1e-12041199826",
        "-0x4.0E-10000000000",
        Less,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        10,
        Floor,
        "-8.2878e-12041199827",
        "-0x3.01E-10000000000",
        Less,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        10,
        Ceiling,
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        Greater,
    );
    test(
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        100,
        Nearest,
        "-8.3e-12041199827",
        "-0x3.0E-10000000000",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456",
        1,
        Nearest,
        "2.4e-323228497",
        "0x1.0E-268435456",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456",
        10,
        Floor,
        "2.4e-323228497",
        "0x1.0E-268435456",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456",
        10,
        Ceiling,
        "2.3872e-323228497",
        "0x1.008E-268435456",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456",
        100,
        Nearest,
        "2.4e-323228497",
        "0x1.0E-268435456",
        Less,
    );
}

#[test]
fn test_atanh_rational_prec_round() {
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let x = Rational::from_str(x).unwrap();
        let r = atanh_rational_prec_round_all(&x, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
        assert!(same_pair(
            &inverse_hyperbolic_rational_prec_round_generic(
                InverseHyperbolicFunction::Atanh,
                &x,
                prec,
                rm
            ),
            &r
        ));
    };
    test("0", 10, Exact, "0.0", "0x0.0", Equal);
    test("1", 10, Exact, "Infinity", "Infinity", Equal);
    test("-1", 10, Exact, "-Infinity", "-Infinity", Equal);
    test("3", 10, Exact, "NaN", "NaN", Equal);
    test("1/3", 1, Nearest, "0.25", "0x0.4", Less);
    test("1/3", 10, Floor, "0.34619", "0x0.58a", Less);
    test("1/3", 10, Ceiling, "0.3467", "0x0.58c", Greater);
    test(
        "1/3",
        100,
        Nearest,
        "0.34657359027997265470861606073",
        "0x0.58b90bfbe8e7bcd5e4f1d9cc",
        Less,
    );
    test("-1/3", 1, Nearest, "-0.25", "-0x0.4", Greater);
    test("-1/3", 10, Floor, "-0.3467", "-0x0.58c", Less);
    test("-1/3", 10, Ceiling, "-0.34619", "-0x0.58a", Greater);
    test(
        "-1/3",
        100,
        Nearest,
        "-0.34657359027997265470861606073",
        "-0x0.58b90bfbe8e7bcd5e4f1d9cc",
        Greater,
    );
    test("99999/100000", 1, Nearest, "8.0", "0x8.0", Greater);
    test("99999/100000", 10, Floor, "6.1016", "0x6.1a", Less);
    test("99999/100000", 10, Ceiling, "6.109", "0x6.1c", Greater);
    test(
        "99999/100000",
        100,
        Nearest,
        "6.103033822758836843920183238792",
        "0x6.1a606cb3218d7572665cfef9",
        Less,
    );
    test("-99999/100000", 1, Nearest, "-8.0", "-0x8.0", Less);
    test("-99999/100000", 10, Floor, "-6.109", "-0x6.1c", Less);
    test("-99999/100000", 10, Ceiling, "-6.1016", "-0x6.1a", Greater);
    test(
        "-99999/100000",
        100,
        Nearest,
        "-6.103033822758836843920183238792",
        "-0x6.1a606cb3218d7572665cfef9",
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
fn atanh_prec_round_fail_1() {
    BigExponentFloat::from(0.5).atanh_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn atanh_prec_round_fail_2() {
    BigExponentFloat::from(0.5).atanh_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn atanh_prec_round_fail_3() {
    BigExponentFloat::atanh_rational_prec_round(Rational::from_unsigneds(1u8, 3), 0, Floor);
}

#[test]
#[should_panic]
fn atanh_prec_round_fail_4() {
    BigExponentFloat::atanh_rational_prec_round(Rational::from_unsigneds(1u8, 3), 10, Exact);
}

#[test]
fn atanh_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_23().test_properties(
        |(x, prec, rm)| {
            let r = atanh_prec_round_all(&x, prec, rm);
            // `Float`'s functions agree with log(1 + t).
            assert!(same_pair(
                &inverse_hyperbolic_prec_round_generic(
                    InverseHyperbolicFunction::Atanh,
                    &x,
                    prec,
                    rm
                ),
                &r
            ));
            check_result(&r, prec);
            if x.is_finite() && !x.is_zero() {
                // atanh is odd
                let (s, o) = atanh_prec_round_all(&-&x, prec, -rm);
                assert!(same(&s, &-&r.0));
                assert_eq!(o, r.1.reverse());
            }
            if big_exponent_float_exponent_is_small(&x)
                && let Ok(q) = Rational::try_from(&x)
            {
                assert!(same_pair(&atanh_rational_prec_round_all(&q, prec, rm), &r));
            }
        },
    );
}

#[test]
fn atanh_prec_round_float_agreement_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        // skip the `Float` generator's `Exact` cases
        if x.is_zero() || rm == Exact {
            return;
        }
        let (y, o) = x.atanh_prec_round_ref(prec, rm);
        if float_result_comparable(&y, o) {
            let r = atanh_prec_round_all(&float_to_big_exponent_float(&x), prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&y)));
            assert_eq!(r.1, o);
        }
    });
}

#[test]
fn atanh_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_16().test_properties(|(x, prec, rm)| {
        let r = atanh_rational_prec_round_all(&x, prec, rm);
        check_result(&r, prec);
        // `Float`'s functions agree with log(1 + t).
        assert!(same_pair(
            &inverse_hyperbolic_rational_prec_round_generic(
                InverseHyperbolicFunction::Atanh,
                &x,
                prec,
                rm
            ),
            &r
        ));
        if x != 0u32 {
            let (y, o) = Float::atanh_rational_prec_round_ref(&x, prec, rm);
            if float_result_comparable(&y, o) {
                assert!(same(&r.0, &float_to_big_exponent_float(&y)));
                assert_eq!(r.1, o);
            }
        }
    });
}
