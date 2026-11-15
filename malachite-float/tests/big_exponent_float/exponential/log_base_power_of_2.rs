// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeOne, One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, FromStringBase};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::float_to_big_exponent_float;
use malachite_float::test_util::generators::{
    big_exponent_float_signed_unsigned_rounding_mode_quadruple_gen_var_3,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_5,
    rational_signed_unsigned_rounding_mode_quadruple_gen_var_1,
};
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

fn hex(s: &str) -> BigExponentFloat {
    BigExponentFloat::from_string_base(16, s).unwrap()
}

// Whether the exponent of `x`, if it has one, is at most 256 in absolute value, so that exact
// conversions and sums involving x stay small.
fn exponent_is_moderate(x: &BigExponentFloat) -> bool {
    x.get_exponent()
        .is_none_or(|e| e.unsigned_abs_ref() <= &256u32)
}

fn log_base_power_of_2_prec_round_all(
    x: &BigExponentFloat,
    k: i64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().log_base_power_of_2_prec_round(k, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.log_base_power_of_2_prec_round_ref(k, prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.log_base_power_of_2_prec_round_assign(k, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().log_base_power_of_2_prec(k, prec), &r));
        assert!(same_pair(&x.log_base_power_of_2_prec_ref(k, prec), &r));
        let mut z = x.clone();
        let o = z.log_base_power_of_2_prec_assign(k, prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn log_base_power_of_2_rational_prec_round_all(
    x: &Rational,
    k: i64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::log_base_power_of_2_rational_prec_round(x.clone(), k, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::log_base_power_of_2_rational_prec_round_ref(x, k, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::log_base_power_of_2_rational_prec(x.clone(), k, prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::log_base_power_of_2_rational_prec_ref(x, k, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_log_base_power_of_2_prec_round() {
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let ninf = BigExponentFloat::NEGATIVE_INFINITY;
    let zero = BigExponentFloat::ZERO;
    let one = BigExponentFloat::ONE;
    for rm in exhaustive_rounding_modes() {
        assert!(same_pair(
            &log_base_power_of_2_prec_round_all(&nan, 3, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_power_of_2_prec_round_all(&inf, 3, 10, rm),
            &(inf.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_power_of_2_prec_round_all(&ninf, 3, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_power_of_2_prec_round_all(&zero, 3, 10, rm),
            &(ninf.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_power_of_2_prec_round_all(&one, 3, 10, rm),
            &(zero.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_power_of_2_prec_round_all(&BigExponentFloat::NEGATIVE_ONE, 3, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_power_of_2_prec_round_all(&nan, -3, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_power_of_2_prec_round_all(&inf, -3, 10, rm),
            &(ninf.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_power_of_2_prec_round_all(&ninf, -3, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_power_of_2_prec_round_all(&zero, -3, 10, rm),
            &(inf.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_power_of_2_prec_round_all(&one, -3, 10, rm),
            &(zero.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_power_of_2_prec_round_all(&BigExponentFloat::NEGATIVE_ONE, -3, 10, rm),
            &(nan.clone(), Equal)
        ));
    }
    // The expected values agree with MPFR's (computed through the base-2 logarithm divided by 3).
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let r = log_base_power_of_2_prec_round_all(&hex(x_hex), 3, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        1,
        Nearest,
        "0.50",
        "0x0.8",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Floor,
        "0.54980",
        "0x0.8cc",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Ceiling,
        "0.5508",
        "0x0.8d",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Down,
        "0.54980",
        "0x0.8cc",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Up,
        "0.5508",
        "0x0.8d",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Nearest,
        "0.5508",
        "0x0.8d",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        100,
        Nearest,
        "0.5504987098241062472681825273405",
        "0x0.8ced7bc32f4a7b367a6d28d28",
        Greater,
    );
    test("0.50", "0x0.8", 1, Nearest, "-0.25", "-0x0.4", Greater);
    test("0.50", "0x0.8", 10, Floor, "-0.33350", "-0x0.556", Less);
    test("0.50", "0x0.8", 10, Ceiling, "-0.3330", "-0x0.554", Greater);
    test("0.50", "0x0.8", 10, Down, "-0.3330", "-0x0.554", Greater);
    test("0.50", "0x0.8", 10, Up, "-0.33350", "-0x0.556", Less);
    test("0.50", "0x0.8", 10, Nearest, "-0.33350", "-0x0.556", Less);
    test(
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558",
        Less,
    );
    test("3.0", "0x3", 1, Nearest, "0.50", "0x0.8", Less);
    test("3.0", "0x3", 10, Floor, "0.52832", "0x0.874", Less);
    test("3.0", "0x3", 10, Ceiling, "0.5293", "0x0.878", Greater);
    test("3.0", "0x3", 10, Down, "0.52832", "0x0.874", Less);
    test("3.0", "0x3", 10, Up, "0.5293", "0x0.878", Greater);
    test("3.0", "0x3", 10, Nearest, "0.52832", "0x0.874", Less);
    test(
        "3.0",
        "0x3",
        100,
        Nearest,
        "0.528320833573718727151246314649",
        "0x0.874008bdfe9cd7dfe003b0ae",
        Less,
    );
    test("7.0", "0x7", 1, Nearest, "1.0", "0x1.0", Greater);
    test("7.0", "0x7", 10, Floor, "0.9355", "0x0.ef8", Less);
    test("7.0", "0x7", 10, Ceiling, "0.93652", "0x0.efc", Greater);
    test("7.0", "0x7", 10, Down, "0.9355", "0x0.ef8", Less);
    test("7.0", "0x7", 10, Up, "0.93652", "0x0.efc", Greater);
    test("7.0", "0x7", 10, Nearest, "0.9355", "0x0.ef8", Less);
    test(
        "7.0",
        "0x7",
        100,
        Nearest,
        "0.93578497401920136914732310574388",
        "0x0.ef8f9aa3802c89110e430136b",
        Less,
    );
    test("100.0", "0x64", 1, Nearest, "2.0", "0x2.0", Less);
    test("100.0", "0x64", 10, Floor, "2.211", "0x2.36", Less);
    test("100.0", "0x64", 10, Ceiling, "2.2148", "0x2.37", Greater);
    test("100.0", "0x64", 10, Down, "2.211", "0x2.36", Less);
    test("100.0", "0x64", 10, Up, "2.2148", "0x2.37", Greater);
    test("100.0", "0x64", 10, Nearest, "2.2148", "0x2.37", Greater);
    test(
        "100.0",
        "0x64",
        100,
        Nearest,
        "2.2146187299249082319135462863276",
        "0x2.36f140ca22f41d50c31fe7fe4",
        Greater,
    );
    test("999.00", "0x3e7", 1, Nearest, "4.0", "0x4.0", Greater);
    test("999.00", "0x3e7", 10, Floor, "3.320", "0x3.52", Less);
    test("999.00", "0x3e7", 10, Ceiling, "3.3242", "0x3.53", Greater);
    test("999.00", "0x3e7", 10, Down, "3.320", "0x3.52", Less);
    test("999.00", "0x3e7", 10, Up, "3.3242", "0x3.53", Greater);
    test("999.00", "0x3e7", 10, Nearest, "3.320", "0x3.52", Less);
    test(
        "999.00",
        "0x3e7",
        100,
        Nearest,
        "3.3214469559308061087396736698188",
        "0x3.524a59031f1eea3f8b74ef34c",
        Less,
    );
    test("1000.0", "0x3e8", 1, Nearest, "4.0", "0x4.0", Greater);
    test("1000.0", "0x3e8", 10, Floor, "3.320", "0x3.52", Less);
    test("1000.0", "0x3e8", 10, Ceiling, "3.3242", "0x3.53", Greater);
    test("1000.0", "0x3e8", 10, Down, "3.320", "0x3.52", Less);
    test("1000.0", "0x3e8", 10, Up, "3.3242", "0x3.53", Greater);
    test("1000.0", "0x3e8", 10, Nearest, "3.320", "0x3.52", Less);
    test(
        "1000.0",
        "0x3e8",
        100,
        Nearest,
        "3.3219280948873623478703194294898",
        "0x3.5269e12f346e2bf924afdbfd4",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        1,
        Nearest,
        "8.0",
        "0x8.0",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Floor,
        "11.06",
        "0xb.1",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Ceiling,
        "11.078",
        "0xb.14",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Down,
        "11.06",
        "0xb.1",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Up,
        "11.078",
        "0xb.14",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Nearest,
        "11.078",
        "0xb.14",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        100,
        Nearest,
        "11.073093649624541159567731431635",
        "0xb.12b643f2aec49293cf9f87f7",
        Greater,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        1,
        Nearest,
        "32.0",
        "0x2.0E+1",
        Greater,
    );
    test("1.2e24", "0x1.0E+20", 10, Floor, "26.656", "0x1a.a8", Less);
    test(
        "1.2e24",
        "0x1.0E+20",
        10,
        Ceiling,
        "26.69",
        "0x1a.b",
        Greater,
    );
    test("1.2e24", "0x1.0E+20", 10, Down, "26.656", "0x1a.a8", Less);
    test("1.2e24", "0x1.0E+20", 10, Up, "26.69", "0x1a.b", Greater);
    test(
        "1.2e24",
        "0x1.0E+20",
        10,
        Nearest,
        "26.656",
        "0x1a.a8",
        Less,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        100,
        Nearest,
        "26.666666666666666666666666666658",
        "0x1a.aaaaaaaaaaaaaaaaaaaaaaaa",
        Less,
    );
    test(
        "2.4e24",
        "0x2.0E+20",
        1,
        Nearest,
        "32.0",
        "0x2.0E+1",
        Greater,
    );
    test("2.4e24", "0x2.0E+20", 10, Floor, "27.0", "0x1b.0", Equal);
    test("2.4e24", "0x2.0E+20", 10, Ceiling, "27.0", "0x1b.0", Equal);
    test("2.4e24", "0x2.0E+20", 10, Down, "27.0", "0x1b.0", Equal);
    test("2.4e24", "0x2.0E+20", 10, Up, "27.0", "0x1b.0", Equal);
    test("2.4e24", "0x2.0E+20", 10, Nearest, "27.0", "0x1b.0", Equal);
    test("2.4e24", "0x2.0E+20", 100, Nearest, "27.0", "0x1b.0", Equal);
    test("2.4e24", "0x2.0E+20", 300, Exact, "27.0", "0x1b.0", Equal);
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        1,
        Nearest,
        "2.7e8",
        "0x1.0E+7",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        10,
        Floor,
        "3.995e8",
        "0x1.7dE+7",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        10,
        Ceiling,
        "4.0003e8",
        "0x1.7d8E+7",
        Greater,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        10,
        Down,
        "3.995e8",
        "0x1.7dE+7",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        10,
        Up,
        "4.0003e8",
        "0x1.7d8E+7",
        Greater,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        10,
        Nearest,
        "4.0003e8",
        "0x1.7d8E+7",
        Greater,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        100,
        Nearest,
        "4.000000e8",
        "0x1.7d784E+7",
        Equal,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        300,
        Exact,
        "4.000000e8",
        "0x1.7d784E+7",
        Equal,
    );
    test(
        "9.4e361235994",
        "0x1.8E+300000000",
        1,
        Nearest,
        "2.7e8",
        "0x1.0E+7",
        Less,
    );
    test(
        "9.4e361235994",
        "0x1.8E+300000000",
        10,
        Floor,
        "3.995e8",
        "0x1.7dE+7",
        Less,
    );
    test(
        "9.4e361235994",
        "0x1.8E+300000000",
        10,
        Ceiling,
        "4.0003e8",
        "0x1.7d8E+7",
        Greater,
    );
    test(
        "9.4e361235994",
        "0x1.8E+300000000",
        10,
        Down,
        "3.995e8",
        "0x1.7dE+7",
        Less,
    );
    test(
        "9.4e361235994",
        "0x1.8E+300000000",
        10,
        Up,
        "4.0003e8",
        "0x1.7d8E+7",
        Greater,
    );
    test(
        "9.4e361235994",
        "0x1.8E+300000000",
        10,
        Nearest,
        "4.0003e8",
        "0x1.7d8E+7",
        Greater,
    );
    test(
        "9.4e361235994",
        "0x1.8E+300000000",
        100,
        Nearest,
        "400000000.19498750024038539381777",
        "0x17d78400.31eab368a947828a8a",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        1,
        Nearest,
        "-1.3e2",
        "-0x8.0E+1",
        Greater,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Floor,
        "-133.5",
        "-0x85.8",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Ceiling,
        "-133.25",
        "-0x85.4",
        Greater,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Down,
        "-133.25",
        "-0x85.4",
        Greater,
    );
    test("3.9e-121", "0x1.0E-100", 10, Up, "-133.5", "-0x85.8", Less);
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Nearest,
        "-133.25",
        "-0x85.4",
        Greater,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        100,
        Nearest,
        "-133.33333333333333333333333333327",
        "-0x85.55555555555555555555555",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        1,
        Nearest,
        "-2.7e8",
        "-0x1.0E+7",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Floor,
        "-4.0003e8",
        "-0x1.7d8E+7",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Ceiling,
        "-3.995e8",
        "-0x1.7dE+7",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Down,
        "-3.995e8",
        "-0x1.7dE+7",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Up,
        "-4.0003e8",
        "-0x1.7d8E+7",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Nearest,
        "-4.0003e8",
        "-0x1.7d8E+7",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        100,
        Nearest,
        "-399999999.80501249975961460618223",
        "-0x17d783ff.ce154c9756b87d7576",
        Less,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        100,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        300,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test("-0.50", "-0x0.8", 1, Nearest, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8", 10, Floor, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8", 10, Ceiling, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8", 10, Down, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8", 10, Up, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8", 10, Nearest, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8", 100, Nearest, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8", 300, Exact, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", 1, Nearest, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", 10, Floor, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", 10, Ceiling, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", 10, Down, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", 10, Up, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", 10, Nearest, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", 100, Nearest, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", 300, Exact, "NaN", "NaN", Equal);
    test("1.0", "0x1.0", 1, Nearest, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0", 10, Floor, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0", 10, Down, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0", 10, Up, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0", 10, Nearest, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0", 100, Nearest, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0", 300, Exact, "0.0", "0x0.0", Equal);
    test(
        "0.99999999977",
        "0x0.ffffffff",
        1,
        Nearest,
        "-1.2e-10",
        "-0x8.0E-9",
        Less,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        10,
        Floor,
        "-1.1198e-10",
        "-0x7.b2E-9",
        Less,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        10,
        Ceiling,
        "-1.119e-10",
        "-0x7.bE-9",
        Greater,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        10,
        Down,
        "-1.119e-10",
        "-0x7.bE-9",
        Greater,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        10,
        Up,
        "-1.1198e-10",
        "-0x7.b2E-9",
        Less,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        10,
        Nearest,
        "-1.1198e-10",
        "-0x7.b2E-9",
        Less,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        100,
        Nearest,
        "-1.1196787166850914673175086802863e-10",
        "-0x7.b1c2771259e098b1c16b1ef48E-9",
        Less,
    );
}

#[test]
#[should_panic]
fn log_base_power_of_2_prec_round_fail_1() {
    BigExponentFloat::from(3).log_base_power_of_2_prec_round(3, 0, Floor);
}

#[test]
#[should_panic]
fn log_base_power_of_2_prec_round_fail_2() {
    BigExponentFloat::from(core::f64::consts::PI).log_base_power_of_2_prec_round(3, 10, Exact);
}

#[test]
#[should_panic]
fn log_base_power_of_2_prec_round_fail_3() {
    BigExponentFloat::from(3).log_base_power_of_2_prec_round(0, 10, Floor);
}

#[test]
fn test_log_base_power_of_2_rational_prec_round() {
    // The expected values agree with MPFR's.
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let r = log_base_power_of_2_rational_prec_round_all(
            &Rational::from_str(x).unwrap(),
            3,
            prec,
            rm,
        );
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("3/5", 1, Nearest, "-0.25", "-0x0.4", Less);
    test("3/5", 10, Floor, "-0.24585", "-0x0.3ef", Less);
    test("3/5", 10, Ceiling, "-0.2456", "-0x0.3ee", Greater);
    test("3/5", 10, Down, "-0.2456", "-0x0.3ee", Greater);
    test("3/5", 10, Up, "-0.24585", "-0x0.3ef", Less);
    test("3/5", 10, Nearest, "-0.2456", "-0x0.3ee", Greater);
    test(
        "3/5",
        100,
        Nearest,
        "-0.24565519805540205547219349518059",
        "-0x0.3ee34251bd87e1732c36edfbbc",
        Less,
    );
    test("1/3", 1, Nearest, "-0.50", "-0x0.8", Greater);
    test("1/3", 10, Floor, "-0.5293", "-0x0.878", Less);
    test("1/3", 10, Ceiling, "-0.52832", "-0x0.874", Greater);
    test("1/3", 10, Down, "-0.52832", "-0x0.874", Greater);
    test("1/3", 10, Up, "-0.5293", "-0x0.878", Less);
    test("1/3", 10, Nearest, "-0.52832", "-0x0.874", Greater);
    test(
        "1/3",
        100,
        Nearest,
        "-0.528320833573718727151246314649",
        "-0x0.874008bdfe9cd7dfe003b0ae",
        Greater,
    );
    test("100", 1, Nearest, "2.0", "0x2.0", Less);
    test("100", 10, Floor, "2.211", "0x2.36", Less);
    test("100", 10, Ceiling, "2.2148", "0x2.37", Greater);
    test("100", 10, Down, "2.211", "0x2.36", Less);
    test("100", 10, Up, "2.2148", "0x2.37", Greater);
    test("100", 10, Nearest, "2.2148", "0x2.37", Greater);
    test(
        "100",
        100,
        Nearest,
        "2.2146187299249082319135462863276",
        "0x2.36f140ca22f41d50c31fe7fe4",
        Greater,
    );
    test("1000", 1, Nearest, "4.0", "0x4.0", Greater);
    test("1000", 10, Floor, "3.320", "0x3.52", Less);
    test("1000", 10, Ceiling, "3.3242", "0x3.53", Greater);
    test("1000", 10, Down, "3.320", "0x3.52", Less);
    test("1000", 10, Up, "3.3242", "0x3.53", Greater);
    test("1000", 10, Nearest, "3.320", "0x3.52", Less);
    test(
        "1000",
        100,
        Nearest,
        "3.3219280948873623478703194294898",
        "0x3.5269e12f346e2bf924afdbfd4",
        Greater,
    );
    test("1/1000", 1, Nearest, "-4.0", "-0x4.0", Less);
    test("1/1000", 10, Floor, "-3.3242", "-0x3.53", Less);
    test("1/1000", 10, Ceiling, "-3.320", "-0x3.52", Greater);
    test("1/1000", 10, Down, "-3.320", "-0x3.52", Greater);
    test("1/1000", 10, Up, "-3.3242", "-0x3.53", Less);
    test("1/1000", 10, Nearest, "-3.320", "-0x3.52", Greater);
    test(
        "1/1000",
        100,
        Nearest,
        "-3.3219280948873623478703194294898",
        "-0x3.5269e12f346e2bf924afdbfd4",
        Less,
    );
    test("10000000000/3", 1, Nearest, "8.0", "0x8.0", Less);
    test("10000000000/3", 10, Floor, "10.53", "0xa.88", Less);
    test("10000000000/3", 10, Ceiling, "10.547", "0xa.8c", Greater);
    test("10000000000/3", 10, Down, "10.53", "0xa.88", Less);
    test("10000000000/3", 10, Up, "10.547", "0xa.8c", Greater);
    test("10000000000/3", 10, Nearest, "10.547", "0xa.8c", Greater);
    test(
        "10000000000/3",
        100,
        Nearest,
        "10.544772816050822432416485116986",
        "0xa.8b763b34b027bab3ef9bd749",
        Greater,
    );
    test("1", 1, Nearest, "0.0", "0x0.0", Equal);
    test("1", 10, Floor, "0.0", "0x0.0", Equal);
    test("1", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("1", 10, Down, "0.0", "0x0.0", Equal);
    test("1", 10, Up, "0.0", "0x0.0", Equal);
    test("1", 10, Nearest, "0.0", "0x0.0", Equal);
    test("1", 100, Nearest, "0.0", "0x0.0", Equal);
    test("1", 300, Exact, "0.0", "0x0.0", Equal);
    test("0", 1, Nearest, "-Infinity", "-Infinity", Equal);
    test("0", 10, Floor, "-Infinity", "-Infinity", Equal);
    test("0", 10, Ceiling, "-Infinity", "-Infinity", Equal);
    test("0", 10, Down, "-Infinity", "-Infinity", Equal);
    test("0", 10, Up, "-Infinity", "-Infinity", Equal);
    test("0", 10, Nearest, "-Infinity", "-Infinity", Equal);
    test("0", 100, Nearest, "-Infinity", "-Infinity", Equal);
    test("0", 300, Exact, "-Infinity", "-Infinity", Equal);
    test("-1", 1, Nearest, "NaN", "NaN", Equal);
    test("-1", 10, Floor, "NaN", "NaN", Equal);
    test("-1", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1", 10, Down, "NaN", "NaN", Equal);
    test("-1", 10, Up, "NaN", "NaN", Equal);
    test("-1", 10, Nearest, "NaN", "NaN", Equal);
    test("-1", 100, Nearest, "NaN", "NaN", Equal);
    test("-1", 300, Exact, "NaN", "NaN", Equal);
}

#[test]
#[should_panic]
fn log_base_power_of_2_rational_prec_round_fail() {
    BigExponentFloat::log_base_power_of_2_rational_prec_round(
        Rational::from_unsigneds(1u32, 3),
        3,
        10,
        Exact,
    );
}

fn log_base_power_of_2_prec_round_properties_helper(
    x: &BigExponentFloat,
    k: i64,
    prec: u64,
    rm: RoundingMode,
) {
    let r = log_base_power_of_2_prec_round_all(x, k, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else if !r.0.is_nan() {
        // The result lies between the results rounded down and up.
        let floor = log_base_power_of_2_prec_round_all(x, k, prec, Floor);
        let ceiling = log_base_power_of_2_prec_round_all(x, k, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0);
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(
                &log_base_power_of_2_prec_round_all(x, k, prec, Exact),
                &r
            ));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = log_base_power_of_2_prec_round_all(x, k, prec + 16, rm);
            let (z, oz) = y.round_to_prec_round(prec, rm);
            assert!(same(&z, &r.0));
            assert_eq!(if o == Equal { oz } else { o }, r.1);
        }
    }
    // A `BigExponentFloat` that is not too large or too small gives the same result as the equal
    // `Rational`.
    if x.is_finite() && exponent_is_moderate(x) {
        let q = Rational::exact_from(x);
        assert!(same_pair(
            &log_base_power_of_2_rational_prec_round_all(&q, k, prec, rm),
            &r
        ));
    }
    // log_(2^1) is log_2.
    assert!(same_pair(
        &x.log_base_power_of_2_prec_round_ref(1, prec, rm),
        &x.log_base_2_prec_round_ref(prec, rm)
    ));
}

#[test]
fn log_base_power_of_2_prec_round_properties() {
    big_exponent_float_signed_unsigned_rounding_mode_quadruple_gen_var_3().test_properties(
        |(x, k, prec, rm)| {
            log_base_power_of_2_prec_round_properties_helper(&x, k, prec, rm);
        },
    );
}

#[test]
fn log_base_power_of_2_prec_round_float_agreement_properties() {
    float_signed_unsigned_rounding_mode_quadruple_gen_var_5().test_properties(
        |(x, k, prec, rm)| {
            if rm == Exact {
                return;
            }
            let (f, fo) = x.log_base_power_of_2_prec_round_ref(k, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = log_base_power_of_2_prec_round_all(
                    &float_to_big_exponent_float(&x),
                    k,
                    prec,
                    rm,
                );
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
}

#[test]
fn log_base_power_of_2_rational_prec_round_float_agreement_properties() {
    rational_signed_unsigned_rounding_mode_quadruple_gen_var_1().test_properties(
        |(x, k, prec, rm)| {
            let (f, fo) = Float::log_base_power_of_2_rational_prec_round_ref(&x, k, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = log_base_power_of_2_rational_prec_round_all(&x, k, prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
}
