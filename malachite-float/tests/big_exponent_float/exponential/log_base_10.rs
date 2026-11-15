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
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_19,
    float_unsigned_rounding_mode_triple_gen_var_29,
    rational_unsigned_rounding_mode_triple_gen_var_9,
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

fn log_base_10_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().log_base_10_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.log_base_10_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.log_base_10_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().log_base_10_prec(prec), &r));
        assert!(same_pair(&x.log_base_10_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.log_base_10_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn log_base_10_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::log_base_10_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::log_base_10_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::log_base_10_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::log_base_10_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_log_base_10_prec_round() {
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let ninf = BigExponentFloat::NEGATIVE_INFINITY;
    let zero = BigExponentFloat::ZERO;
    let one = BigExponentFloat::ONE;
    for rm in exhaustive_rounding_modes() {
        assert!(same_pair(
            &log_base_10_prec_round_all(&nan, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_10_prec_round_all(&inf, 10, rm),
            &(inf.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_10_prec_round_all(&ninf, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_10_prec_round_all(&zero, 10, rm),
            &(ninf.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_10_prec_round_all(&one, 10, rm),
            &(zero.clone(), Equal)
        ));
        assert!(same_pair(
            &log_base_10_prec_round_all(&BigExponentFloat::NEGATIVE_ONE, 10, rm),
            &(nan.clone(), Equal)
        ));
    }
    // The expected values agree with MPFR's.
    let test = |x: &str,
                x_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let r = log_base_10_prec_round_all(&hex(x_hex), prec, rm);
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
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Floor,
        "0.4971",
        "0x0.7f4",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Ceiling,
        "0.49756",
        "0x0.7f6",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Down,
        "0.4971",
        "0x0.7f4",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Up,
        "0.49756",
        "0x0.7f6",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Nearest,
        "0.4971",
        "0x0.7f4",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        100,
        Nearest,
        "0.49714987269413383742172312455219",
        "0x0.7f4536cc6e8eeacae4696b9e08",
        Less,
    );
    test("0.50", "0x0.8", 1, Nearest, "-0.25", "-0x0.4", Greater);
    test("0.50", "0x0.8", 10, Floor, "-0.30127", "-0x0.4d2", Less);
    test("0.50", "0x0.8", 10, Ceiling, "-0.3008", "-0x0.4d", Greater);
    test("0.50", "0x0.8", 10, Down, "-0.3008", "-0x0.4d", Greater);
    test("0.50", "0x0.8", 10, Up, "-0.30127", "-0x0.4d2", Less);
    test("0.50", "0x0.8", 10, Nearest, "-0.30127", "-0x0.4d2", Less);
    test(
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "-0.30102999566398119521373889472460",
        "-0x0.4d104d427de7fbcc47c4acd608",
        Less,
    );
    test("3.0", "0x3", 1, Nearest, "0.50", "0x0.8", Greater);
    test("3.0", "0x3", 10, Floor, "0.47705", "0x0.7a2", Less);
    test("3.0", "0x3", 10, Ceiling, "0.4775", "0x0.7a4", Greater);
    test("3.0", "0x3", 10, Down, "0.47705", "0x0.7a2", Less);
    test("3.0", "0x3", 10, Up, "0.4775", "0x0.7a4", Greater);
    test("3.0", "0x3", 10, Nearest, "0.47705", "0x0.7a2", Less);
    test(
        "3.0",
        "0x3",
        100,
        Nearest,
        "0.47712125471966243729502790325519",
        "0x0.7a249e593f57f4230c0d0ea088",
        Greater,
    );
    test("7.0", "0x7", 1, Nearest, "1.0", "0x1.0", Greater);
    test("7.0", "0x7", 10, Floor, "0.84473", "0x0.d84", Less);
    test("7.0", "0x7", 10, Ceiling, "0.8457", "0x0.d88", Greater);
    test("7.0", "0x7", 10, Down, "0.84473", "0x0.d84", Less);
    test("7.0", "0x7", 10, Up, "0.8457", "0x0.d88", Greater);
    test("7.0", "0x7", 10, Nearest, "0.84473", "0x0.d84", Less);
    test(
        "7.0",
        "0x7",
        100,
        Nearest,
        "0.84509804001425683071221625859",
        "0x0.d858585bc661f94b692ff8a8",
        Less,
    );
    test("100.0", "0x64", 1, Nearest, "2.0", "0x2.0", Equal);
    test("100.0", "0x64", 10, Floor, "2.0", "0x2.0", Equal);
    test("100.0", "0x64", 10, Ceiling, "2.0", "0x2.0", Equal);
    test("100.0", "0x64", 10, Down, "2.0", "0x2.0", Equal);
    test("100.0", "0x64", 10, Up, "2.0", "0x2.0", Equal);
    test("100.0", "0x64", 10, Nearest, "2.0", "0x2.0", Equal);
    test("100.0", "0x64", 100, Nearest, "2.0", "0x2.0", Equal);
    test("100.0", "0x64", 300, Exact, "2.0", "0x2.0", Equal);
    test("999.00", "0x3e7", 1, Nearest, "2.0", "0x2.0", Less);
    test("999.00", "0x3e7", 10, Floor, "2.9961", "0x2.ff", Less);
    test("999.00", "0x3e7", 10, Ceiling, "3.0", "0x3.0", Greater);
    test("999.00", "0x3e7", 10, Down, "2.9961", "0x2.ff", Less);
    test("999.00", "0x3e7", 10, Up, "3.0", "0x3.0", Greater);
    test("999.00", "0x3e7", 10, Nearest, "3.0", "0x3.0", Greater);
    test(
        "999.00",
        "0x3e7",
        100,
        Nearest,
        "2.9995654882259823086935343993031",
        "0x2.ffe3861a240fdd50f6c1489bc",
        Less,
    );
    test("1000.0", "0x3e8", 1, Nearest, "4.0", "0x4.0", Greater);
    test("1000.0", "0x3e8", 10, Floor, "3.0", "0x3.0", Equal);
    test("1000.0", "0x3e8", 10, Ceiling, "3.0", "0x3.0", Equal);
    test("1000.0", "0x3e8", 10, Down, "3.0", "0x3.0", Equal);
    test("1000.0", "0x3e8", 10, Up, "3.0", "0x3.0", Equal);
    test("1000.0", "0x3e8", 10, Nearest, "3.0", "0x3.0", Equal);
    test("1000.0", "0x3e8", 100, Nearest, "3.0", "0x3.0", Equal);
    test("1000.0", "0x3e8", 300, Exact, "3.0", "0x3.0", Equal);
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
        "10.0",
        "0xa.0",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Ceiling,
        "10.0",
        "0xa.0",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Down,
        "10.0",
        "0xa.0",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Up,
        "10.0",
        "0xa.0",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Nearest,
        "10.0",
        "0xa.0",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        100,
        Nearest,
        "10.0",
        "0xa.0",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        300,
        Exact,
        "10.0",
        "0xa.0",
        Equal,
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
    test("1.2e24", "0x1.0E+20", 10, Floor, "24.06", "0x18.1", Less);
    test(
        "1.2e24",
        "0x1.0E+20",
        10,
        Ceiling,
        "24.094",
        "0x18.18",
        Greater,
    );
    test("1.2e24", "0x1.0E+20", 10, Down, "24.06", "0x18.1", Less);
    test("1.2e24", "0x1.0E+20", 10, Up, "24.094", "0x18.18", Greater);
    test(
        "1.2e24",
        "0x1.0E+20",
        10,
        Nearest,
        "24.094",
        "0x18.18",
        Greater,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        100,
        Nearest,
        "24.082399653118495617099111577962",
        "0x18.151824c7587eafd66d7602e2",
        Greater,
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
    test("2.4e24", "0x2.0E+20", 10, Floor, "24.38", "0x18.6", Less);
    test(
        "2.4e24",
        "0x2.0E+20",
        10,
        Ceiling,
        "24.406",
        "0x18.68",
        Greater,
    );
    test("2.4e24", "0x2.0E+20", 10, Down, "24.38", "0x18.6", Less);
    test("2.4e24", "0x2.0E+20", 10, Up, "24.406", "0x18.68", Greater);
    test("2.4e24", "0x2.0E+20", 10, Nearest, "24.38", "0x18.6", Less);
    test(
        "2.4e24",
        "0x2.0E+20",
        100,
        Nearest,
        "24.38342964878247681231285047269",
        "0x18.62287209d666aba2b53aafb8",
        Greater,
    );
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
        "3.6123e8",
        "0x1.588E+7",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        10,
        Ceiling,
        "3.618e8",
        "0x1.59E+7",
        Greater,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        10,
        Down,
        "3.6123e8",
        "0x1.588E+7",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        10,
        Up,
        "3.618e8",
        "0x1.59E+7",
        Greater,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        10,
        Nearest,
        "3.6123e8",
        "0x1.588E+7",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        100,
        Nearest,
        "361235994.79677743425648667366936",
        "0x1588061a.cbf99b1e5288d42162",
        Less,
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
        "3.6123e8",
        "0x1.588E+7",
        Less,
    );
    test(
        "9.4e361235994",
        "0x1.8E+300000000",
        10,
        Ceiling,
        "3.618e8",
        "0x1.59E+7",
        Greater,
    );
    test(
        "9.4e361235994",
        "0x1.8E+300000000",
        10,
        Down,
        "3.6123e8",
        "0x1.588E+7",
        Less,
    );
    test(
        "9.4e361235994",
        "0x1.8E+300000000",
        10,
        Up,
        "3.618e8",
        "0x1.59E+7",
        Greater,
    );
    test(
        "9.4e361235994",
        "0x1.8E+300000000",
        10,
        Nearest,
        "3.6123e8",
        "0x1.588E+7",
        Less,
    );
    test(
        "9.4e361235994",
        "0x1.8E+300000000",
        100,
        Nearest,
        "361235994.97286869331216791575059",
        "0x1588061a.f90dec3513f8cc7826",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        1,
        Nearest,
        "-1.3e2",
        "-0x8.0E+1",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Floor,
        "-120.5",
        "-0x78.8",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Ceiling,
        "-120.38",
        "-0x78.6",
        Greater,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Down,
        "-120.38",
        "-0x78.6",
        Greater,
    );
    test("3.9e-121", "0x1.0E-100", 10, Up, "-120.5", "-0x78.8", Less);
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Nearest,
        "-120.38",
        "-0x78.6",
        Greater,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        100,
        Nearest,
        "-120.41199826559247808549555788978",
        "-0x78.6978b7e4ba796f30234e0e68",
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
        "-3.618e8",
        "-0x1.59E+7",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Ceiling,
        "-3.6123e8",
        "-0x1.588E+7",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Down,
        "-3.6123e8",
        "-0x1.588E+7",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Up,
        "-3.618e8",
        "-0x1.59E+7",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Nearest,
        "-3.6123e8",
        "-0x1.588E+7",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        100,
        Nearest,
        "-361235994.62068617520080543158813",
        "-0x1588061a.9ee54a079118dbca9e",
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
        "-1.012e-10",
        "-0x6.f4E-9",
        Less,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        10,
        Ceiling,
        "-1.0107e-10",
        "-0x6.f2E-9",
        Greater,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        10,
        Down,
        "-1.0107e-10",
        "-0x6.f2E-9",
        Greater,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        10,
        Up,
        "-1.012e-10",
        "-0x6.f4E-9",
        Less,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        10,
        Nearest,
        "-1.0107e-10",
        "-0x6.f2E-9",
        Greater,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        100,
        Nearest,
        "-1.011170637686295340623253991831e-10",
        "-0x6.f2dec54d32b2ef50d8740842E-9",
        Less,
    );
}

#[test]
#[should_panic]
fn log_base_10_prec_round_fail_1() {
    BigExponentFloat::from(3).log_base_10_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn log_base_10_prec_round_fail_2() {
    BigExponentFloat::from(core::f64::consts::PI).log_base_10_prec_round(10, Exact);
}

#[test]
fn test_log_base_10_rational_prec_round() {
    // The expected values agree with MPFR's.
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let r = log_base_10_rational_prec_round_all(&Rational::from_str(x).unwrap(), prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("3/5", 1, Nearest, "-0.25", "-0x0.4", Less);
    test("3/5", 10, Floor, "-0.22192", "-0x0.38d", Less);
    test("3/5", 10, Ceiling, "-0.2217", "-0x0.38c", Greater);
    test("3/5", 10, Down, "-0.2217", "-0x0.38c", Greater);
    test("3/5", 10, Up, "-0.22192", "-0x0.38d", Less);
    test("3/5", 10, Nearest, "-0.22192", "-0x0.38d", Less);
    test(
        "3/5",
        100,
        Nearest,
        "-0.22184874961635636749123320202041",
        "-0x0.38cb146442c01010ac2e448974",
        Less,
    );
    test("1/3", 1, Nearest, "-0.50", "-0x0.8", Less);
    test("1/3", 10, Floor, "-0.4775", "-0x0.7a4", Less);
    test("1/3", 10, Ceiling, "-0.47705", "-0x0.7a2", Greater);
    test("1/3", 10, Down, "-0.47705", "-0x0.7a2", Greater);
    test("1/3", 10, Up, "-0.4775", "-0x0.7a4", Less);
    test("1/3", 10, Nearest, "-0.47705", "-0x0.7a2", Greater);
    test(
        "1/3",
        100,
        Nearest,
        "-0.47712125471966243729502790325519",
        "-0x0.7a249e593f57f4230c0d0ea088",
        Less,
    );
    test("100", 1, Nearest, "2.0", "0x2.0", Equal);
    test("100", 10, Floor, "2.0", "0x2.0", Equal);
    test("100", 10, Ceiling, "2.0", "0x2.0", Equal);
    test("100", 10, Down, "2.0", "0x2.0", Equal);
    test("100", 10, Up, "2.0", "0x2.0", Equal);
    test("100", 10, Nearest, "2.0", "0x2.0", Equal);
    test("100", 100, Nearest, "2.0", "0x2.0", Equal);
    test("100", 300, Exact, "2.0", "0x2.0", Equal);
    test("1000", 1, Nearest, "4.0", "0x4.0", Greater);
    test("1000", 10, Floor, "3.0", "0x3.0", Equal);
    test("1000", 10, Ceiling, "3.0", "0x3.0", Equal);
    test("1000", 10, Down, "3.0", "0x3.0", Equal);
    test("1000", 10, Up, "3.0", "0x3.0", Equal);
    test("1000", 10, Nearest, "3.0", "0x3.0", Equal);
    test("1000", 100, Nearest, "3.0", "0x3.0", Equal);
    test("1000", 300, Exact, "3.0", "0x3.0", Equal);
    test("1/1000", 1, Nearest, "-4.0", "-0x4.0", Less);
    test("1/1000", 10, Floor, "-3.0", "-0x3.0", Equal);
    test("1/1000", 10, Ceiling, "-3.0", "-0x3.0", Equal);
    test("1/1000", 10, Down, "-3.0", "-0x3.0", Equal);
    test("1/1000", 10, Up, "-3.0", "-0x3.0", Equal);
    test("1/1000", 10, Nearest, "-3.0", "-0x3.0", Equal);
    test("1/1000", 100, Nearest, "-3.0", "-0x3.0", Equal);
    test("1/1000", 300, Exact, "-3.0", "-0x3.0", Equal);
    test("10000000000/3", 1, Nearest, "8.0", "0x8.0", Less);
    test("10000000000/3", 10, Floor, "9.5156", "0x9.84", Less);
    test("10000000000/3", 10, Ceiling, "9.531", "0x9.88", Greater);
    test("10000000000/3", 10, Down, "9.5156", "0x9.84", Less);
    test("10000000000/3", 10, Up, "9.531", "0x9.88", Greater);
    test("10000000000/3", 10, Nearest, "9.5156", "0x9.84", Less);
    test(
        "10000000000/3",
        100,
        Nearest,
        "9.5228787452803375627049720967389",
        "0x9.85db61a6c0a80bdcf3f2f15f",
        Less,
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
fn log_base_10_rational_prec_round_fail() {
    BigExponentFloat::log_base_10_rational_prec_round(Rational::from_unsigneds(1u32, 3), 10, Exact);
}

fn log_base_10_prec_round_properties_helper(x: &BigExponentFloat, prec: u64, rm: RoundingMode) {
    let r = log_base_10_prec_round_all(x, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else if !r.0.is_nan() {
        // The result lies between the results rounded down and up.
        let floor = log_base_10_prec_round_all(x, prec, Floor);
        let ceiling = log_base_10_prec_round_all(x, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0);
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(&log_base_10_prec_round_all(x, prec, Exact), &r));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = log_base_10_prec_round_all(x, prec + 16, rm);
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
            &log_base_10_rational_prec_round_all(&q, prec, rm),
            &r
        ));
    }
}

#[test]
fn log_base_10_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_19().test_properties(
        |(x, prec, rm)| {
            log_base_10_prec_round_properties_helper(&x, prec, rm);
        },
    );
}

#[test]
fn log_base_10_prec_round_float_agreement_properties() {
    float_unsigned_rounding_mode_triple_gen_var_29().test_properties(|(x, prec, rm)| {
        if rm == Exact {
            return;
        }
        let (f, fo) = x.log_base_10_prec_round_ref(prec, rm);
        if float_result_comparable(&f, fo) {
            let r = log_base_10_prec_round_all(&float_to_big_exponent_float(&x), prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    });
}

#[test]
fn log_base_10_rational_prec_round_float_agreement_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_9().test_properties(|(x, prec, rm)| {
        let (f, fo) = Float::log_base_10_rational_prec_round_ref(&x, prec, rm);
        if float_result_comparable(&f, fo) {
            let r = log_base_10_rational_prec_round_all(&x, prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    });
}
