// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Abs;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeOne, One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, FromStringBase};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::big_exponent_float::exponential::ln::ln_1_plus_x_series;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_power_of_2, float_to_big_exponent_float,
};
use malachite_float::test_util::generators::{
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_15, float_unsigned_pair_gen_var_1,
    float_unsigned_rounding_mode_triple_gen_var_19,
    rational_unsigned_rounding_mode_triple_gen_var_6,
    unsigned_unsigned_rounding_mode_triple_gen_var_8,
};
use malachite_float::{BigExponentFloat, Float};
use malachite_nz::integer::Integer;
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

fn ln_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().ln_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.ln_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.ln_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().ln_prec(prec), &r));
        assert!(same_pair(&x.ln_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.ln_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn ln_rational_prec_round_all(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::ln_rational_prec_round(x.clone(), prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::ln_rational_prec_round_ref(x, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::ln_rational_prec(x.clone(), prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::ln_rational_prec_ref(x, prec),
            &r
        ));
    }
    r
}

fn ln_unsigned_prec_round_all(n: u64, prec: u64, rm: RoundingMode) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::ln_unsigned_prec_round(n, prec, rm);
    assert!(r.0.is_valid());
    if rm == Nearest {
        assert!(same_pair(&BigExponentFloat::ln_unsigned_prec(n, prec), &r));
    }
    r
}

#[test]
fn test_ln_prec_round() {
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let ninf = BigExponentFloat::NEGATIVE_INFINITY;
    let zero = BigExponentFloat::ZERO;
    let one = BigExponentFloat::ONE;
    for rm in exhaustive_rounding_modes() {
        assert!(same_pair(
            &ln_prec_round_all(&nan, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &ln_prec_round_all(&inf, 10, rm),
            &(inf.clone(), Equal)
        ));
        assert!(same_pair(
            &ln_prec_round_all(&ninf, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &ln_prec_round_all(&zero, 10, rm),
            &(ninf.clone(), Equal)
        ));
        assert!(same_pair(
            &ln_prec_round_all(&one, 10, rm),
            &(zero.clone(), Equal)
        ));
        assert!(same_pair(
            &ln_prec_round_all(&BigExponentFloat::NEGATIVE_ONE, 10, rm),
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
        let r = ln_prec_round_all(&hex(x_hex), prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Floor,
        "1.145",
        "0x1.25",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Ceiling,
        "1.1465",
        "0x1.258",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Down,
        "1.145",
        "0x1.25",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Up,
        "1.1465",
        "0x1.258",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Nearest,
        "1.145",
        "0x1.25",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        100,
        Nearest,
        "1.144729885849400135161709026159",
        "0x1.250d048e7a1bcdee499fa5b0c",
        Less,
    );
    test("0.50", "0x0.8", 1, Nearest, "-0.50", "-0x0.8", Greater);
    test("0.50", "0x0.8", 10, Floor, "-0.6934", "-0x0.b18", Less);
    test(
        "0.50", "0x0.8", 10, Ceiling, "-0.69238", "-0x0.b14", Greater,
    );
    test("0.50", "0x0.8", 10, Down, "-0.69238", "-0x0.b14", Greater);
    test("0.50", "0x0.8", 10, Up, "-0.6934", "-0x0.b18", Less);
    test("0.50", "0x0.8", 10, Nearest, "-0.6934", "-0x0.b18", Less);
    test(
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "-0.69314718055994530941723212146",
        "-0x0.b17217f7d1cf79abc9e3b398",
        Greater,
    );
    test("3.0", "0x3", 1, Nearest, "1.0", "0x1.0", Less);
    test("3.0", "0x3", 10, Floor, "1.098", "0x1.19", Less);
    test("3.0", "0x3", 10, Ceiling, "1.0996", "0x1.198", Greater);
    test("3.0", "0x3", 10, Down, "1.098", "0x1.19", Less);
    test("3.0", "0x3", 10, Up, "1.0996", "0x1.198", Greater);
    test("3.0", "0x3", 10, Nearest, "1.098", "0x1.19", Less);
    test(
        "3.0",
        "0x3",
        100,
        Nearest,
        "1.098612288668109691395245236922",
        "0x1.193ea7aad030a976a4198d55",
        Less,
    );
    test("7.0", "0x7", 1, Nearest, "2.0", "0x2.0", Greater);
    test("7.0", "0x7", 10, Floor, "1.945", "0x1.f2", Less);
    test("7.0", "0x7", 10, Ceiling, "1.9473", "0x1.f28", Greater);
    test("7.0", "0x7", 10, Down, "1.945", "0x1.f2", Less);
    test("7.0", "0x7", 10, Up, "1.9473", "0x1.f28", Greater);
    test("7.0", "0x7", 10, Nearest, "1.945", "0x1.f2", Less);
    test(
        "7.0",
        "0x7",
        100,
        Nearest,
        "1.9459101490553133051053527434429",
        "0x1.f2272ae325a57546f69496cf2",
        Less,
    );
    test("100.0", "0x64", 1, Nearest, "4.0", "0x4.0", Less);
    test("100.0", "0x64", 10, Floor, "4.6016", "0x4.9a", Less);
    test("100.0", "0x64", 10, Ceiling, "4.609", "0x4.9c", Greater);
    test("100.0", "0x64", 10, Down, "4.6016", "0x4.9a", Less);
    test("100.0", "0x64", 10, Up, "4.609", "0x4.9c", Greater);
    test("100.0", "0x64", 10, Nearest, "4.6016", "0x4.9a", Less);
    test(
        "100.0",
        "0x64",
        100,
        Nearest,
        "4.605170185988091368035982909368",
        "0x4.9aec6eed554560b752b6b15c",
        Less,
    );
    test("999.00", "0x3e7", 1, Nearest, "8.0", "0x8.0", Greater);
    test("999.00", "0x3e7", 10, Floor, "6.906", "0x6.e8", Less);
    test("999.00", "0x3e7", 10, Ceiling, "6.9141", "0x6.ea", Greater);
    test("999.00", "0x3e7", 10, Down, "6.906", "0x6.e8", Less);
    test("999.00", "0x3e7", 10, Up, "6.9141", "0x6.ea", Greater);
    test("999.00", "0x3e7", 10, Nearest, "6.906", "0x6.e8", Less);
    test(
        "999.00",
        "0x3e7",
        100,
        Nearest,
        "6.906754778648553518553831381798",
        "0x6.e82114c7c989a356e9780bfb",
        Less,
    );
    test("1000.0", "0x3e8", 1, Nearest, "8.0", "0x8.0", Greater);
    test("1000.0", "0x3e8", 10, Floor, "6.906", "0x6.e8", Less);
    test("1000.0", "0x3e8", 10, Ceiling, "6.9141", "0x6.ea", Greater);
    test("1000.0", "0x3e8", 10, Down, "6.906", "0x6.e8", Less);
    test("1000.0", "0x3e8", 10, Up, "6.9141", "0x6.ea", Greater);
    test("1000.0", "0x3e8", 10, Nearest, "6.906", "0x6.e8", Less);
    test(
        "1000.0",
        "0x3e8",
        100,
        Nearest,
        "6.907755278982137052053974364051",
        "0x6.e862a663ffe81112fc120a0a",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        1,
        Nearest,
        "16.0",
        "0x1.0E+1",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Floor,
        "23.0",
        "0x17.0",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Ceiling,
        "23.031",
        "0x17.08",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Down,
        "23.0",
        "0x17.0",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Up,
        "23.031",
        "0x17.08",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        10,
        Nearest,
        "23.031",
        "0x17.08",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        100,
        Nearest,
        "23.02585092994045684017991454684",
        "0x17.069e2aa2aa5ae3949d9176cc",
        Less,
    );
    test(
        "1.2e24",
        "0x1.0E+20",
        1,
        Nearest,
        "64.0",
        "0x4.0E+1",
        Greater,
    );
    test("1.2e24", "0x1.0E+20", 10, Floor, "55.438", "0x37.7", Less);
    test(
        "1.2e24",
        "0x1.0E+20",
        10,
        Ceiling,
        "55.50",
        "0x37.8",
        Greater,
    );
    test("1.2e24", "0x1.0E+20", 10, Down, "55.438", "0x37.7", Less);
    test("1.2e24", "0x1.0E+20", 10, Up, "55.50", "0x37.8", Greater);
    test("1.2e24", "0x1.0E+20", 10, Nearest, "55.438", "0x37.7", Less);
    test(
        "1.2e24",
        "0x1.0E+20",
        100,
        Nearest,
        "55.4517744447956247533785697166",
        "0x37.73a77d7190d605af17281f8",
        Less,
    );
    test(
        "2.4e24",
        "0x2.0E+20",
        1,
        Nearest,
        "64.0",
        "0x4.0E+1",
        Greater,
    );
    test("2.4e24", "0x2.0E+20", 10, Floor, "56.12", "0x38.2", Less);
    test(
        "2.4e24",
        "0x2.0E+20",
        10,
        Ceiling,
        "56.188",
        "0x38.3",
        Greater,
    );
    test("2.4e24", "0x2.0E+20", 10, Down, "56.12", "0x38.2", Less);
    test("2.4e24", "0x2.0E+20", 10, Up, "56.188", "0x38.3", Greater);
    test("2.4e24", "0x2.0E+20", 10, Nearest, "56.12", "0x38.2", Less);
    test(
        "2.4e24",
        "0x2.0E+20",
        100,
        Nearest,
        "56.14492162535557006279580183810",
        "0x38.2519956962a57f5ae10bd318",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        1,
        Nearest,
        "1.1e9",
        "0x4.0E+7",
        Greater,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        10,
        Floor,
        "8.3152e8",
        "0x3.19E+7",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        10,
        Ceiling,
        "8.326e8",
        "0x3.1aE+7",
        Greater,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        10,
        Down,
        "8.3152e8",
        "0x3.19E+7",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        10,
        Up,
        "8.326e8",
        "0x3.1aE+7",
        Greater,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        10,
        Nearest,
        "8.3152e8",
        "0x3.19E+7",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        100,
        Nearest,
        "831776616.6719343713006785457496",
        "0x3193e768.ac03e415cb73c667b",
        Less,
    );
    test(
        "9.4e361235994",
        "0x1.8E+300000000",
        1,
        Nearest,
        "1.1e9",
        "0x4.0E+7",
        Greater,
    );
    test(
        "9.4e361235994",
        "0x1.8E+300000000",
        10,
        Floor,
        "8.3152e8",
        "0x3.19E+7",
        Less,
    );
    test(
        "9.4e361235994",
        "0x1.8E+300000000",
        10,
        Ceiling,
        "8.326e8",
        "0x3.1aE+7",
        Greater,
    );
    test(
        "9.4e361235994",
        "0x1.8E+300000000",
        10,
        Down,
        "8.3152e8",
        "0x3.19E+7",
        Less,
    );
    test(
        "9.4e361235994",
        "0x1.8E+300000000",
        10,
        Up,
        "8.326e8",
        "0x3.1aE+7",
        Greater,
    );
    test(
        "9.4e361235994",
        "0x1.8E+300000000",
        10,
        Nearest,
        "8.3152e8",
        "0x3.19E+7",
        Less,
    );
    test(
        "9.4e361235994",
        "0x1.8E+300000000",
        100,
        Nearest,
        "831776617.07739947940884292772802",
        "0x3193e769.13d073c8c9d4f6328c",
        Greater,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        1,
        Nearest,
        "-2.6e2",
        "-0x1.0E+2",
        Greater,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Floor,
        "-277.50",
        "-0x115.8",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Ceiling,
        "-277.0",
        "-0x115.0",
        Greater,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Down,
        "-277.0",
        "-0x115.0",
        Greater,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Up,
        "-277.50",
        "-0x115.8",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Nearest,
        "-277.50",
        "-0x115.8",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        100,
        Nearest,
        "-277.2588722239781237668928485832",
        "-0x115.42457337d42e1c6b73c89d8",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        1,
        Nearest,
        "-1.1e9",
        "-0x4.0E+7",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Floor,
        "-8.326e8",
        "-0x3.1aE+7",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Ceiling,
        "-8.3152e8",
        "-0x3.19E+7",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Down,
        "-8.3152e8",
        "-0x3.19E+7",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Up,
        "-8.326e8",
        "-0x3.1aE+7",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Nearest,
        "-8.3152e8",
        "-0x3.19E+7",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        100,
        Nearest,
        "-831776616.2664692631925141637721",
        "-0x3193e768.44375462cd12969cd8",
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
        "-2.3e-10",
        "-0x1.0E-8",
        Greater,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        10,
        Floor,
        "-2.3329e-10",
        "-0x1.008E-8",
        Less,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        10,
        Ceiling,
        "-2.3e-10",
        "-0x1.0E-8",
        Greater,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        10,
        Down,
        "-2.3e-10",
        "-0x1.0E-8",
        Greater,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        10,
        Up,
        "-2.3329e-10",
        "-0x1.008E-8",
        Less,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        10,
        Nearest,
        "-2.3e-10",
        "-0x1.0E-8",
        Greater,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        100,
        Nearest,
        "-2.3283064368097468322259486901222e-10",
        "-0x1.000000008000000055555555aE-8",
        Less,
    );
}

#[test]
#[should_panic]
fn ln_prec_round_fail_1() {
    BigExponentFloat::from(3).ln_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn ln_prec_round_fail_2() {
    BigExponentFloat::from(core::f64::consts::PI).ln_prec_round(10, Exact);
}

#[test]
fn test_ln_rational_prec_round() {
    // The expected values agree with MPFR's.
    let test = |x: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let r = ln_rational_prec_round_all(&Rational::from_str(x).unwrap(), prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {prec} {rm}");
    };
    test("3/5", 1, Nearest, "-0.50", "-0x0.8", Greater);
    test("3/5", 10, Floor, "-0.5117", "-0x0.83", Less);
    test("3/5", 10, Ceiling, "-0.51074", "-0x0.82c", Greater);
    test("3/5", 10, Down, "-0.51074", "-0x0.82c", Greater);
    test("3/5", 10, Up, "-0.5117", "-0x0.83", Less);
    test("3/5", 10, Nearest, "-0.51074", "-0x0.82c", Greater);
    test(
        "3/5",
        100,
        Nearest,
        "-0.510825623765990683205514096304",
        "-0x0.82c577d408a28d393b5e17c1",
        Greater,
    );
    test("1/3", 1, Nearest, "-1.0", "-0x1.0", Greater);
    test("1/3", 10, Floor, "-1.0996", "-0x1.198", Less);
    test("1/3", 10, Ceiling, "-1.098", "-0x1.19", Greater);
    test("1/3", 10, Down, "-1.098", "-0x1.19", Greater);
    test("1/3", 10, Up, "-1.0996", "-0x1.198", Less);
    test("1/3", 10, Nearest, "-1.098", "-0x1.19", Greater);
    test(
        "1/3",
        100,
        Nearest,
        "-1.098612288668109691395245236922",
        "-0x1.193ea7aad030a976a4198d55",
        Greater,
    );
    test("100", 1, Nearest, "4.0", "0x4.0", Less);
    test("100", 10, Floor, "4.6016", "0x4.9a", Less);
    test("100", 10, Ceiling, "4.609", "0x4.9c", Greater);
    test("100", 10, Down, "4.6016", "0x4.9a", Less);
    test("100", 10, Up, "4.609", "0x4.9c", Greater);
    test("100", 10, Nearest, "4.6016", "0x4.9a", Less);
    test(
        "100",
        100,
        Nearest,
        "4.605170185988091368035982909368",
        "0x4.9aec6eed554560b752b6b15c",
        Less,
    );
    test("1000", 1, Nearest, "8.0", "0x8.0", Greater);
    test("1000", 10, Floor, "6.906", "0x6.e8", Less);
    test("1000", 10, Ceiling, "6.9141", "0x6.ea", Greater);
    test("1000", 10, Down, "6.906", "0x6.e8", Less);
    test("1000", 10, Up, "6.9141", "0x6.ea", Greater);
    test("1000", 10, Nearest, "6.906", "0x6.e8", Less);
    test(
        "1000",
        100,
        Nearest,
        "6.907755278982137052053974364051",
        "0x6.e862a663ffe81112fc120a0a",
        Less,
    );
    test("1/1000", 1, Nearest, "-8.0", "-0x8.0", Less);
    test("1/1000", 10, Floor, "-6.9141", "-0x6.ea", Less);
    test("1/1000", 10, Ceiling, "-6.906", "-0x6.e8", Greater);
    test("1/1000", 10, Down, "-6.906", "-0x6.e8", Greater);
    test("1/1000", 10, Up, "-6.9141", "-0x6.ea", Less);
    test("1/1000", 10, Nearest, "-6.906", "-0x6.e8", Greater);
    test(
        "1/1000",
        100,
        Nearest,
        "-6.907755278982137052053974364051",
        "-0x6.e862a663ffe81112fc120a0a",
        Greater,
    );
    test("10000000000/3", 1, Nearest, "16.0", "0x1.0E+1", Less);
    test("10000000000/3", 10, Floor, "21.906", "0x15.e8", Less);
    test("10000000000/3", 10, Ceiling, "21.94", "0x15.f", Greater);
    test("10000000000/3", 10, Down, "21.906", "0x15.e8", Less);
    test("10000000000/3", 10, Up, "21.94", "0x15.f", Greater);
    test("10000000000/3", 10, Nearest, "21.94", "0x15.f", Greater);
    test(
        "10000000000/3",
        100,
        Nearest,
        "21.92723864127234714878466930993",
        "0x15.ed5f82f7da2a3a1df977e978",
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
fn ln_rational_prec_round_fail() {
    BigExponentFloat::ln_rational_prec_round(Rational::from_unsigneds(1u32, 3), 10, Exact);
}

#[test]
fn test_ln_unsigned_prec_round() {
    // The expected values agree with MPFR's.
    let test = |n: u64, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
        let r = ln_unsigned_prec_round_all(n, prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{n} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{n} {prec} {rm}");
    };
    test(0, 1, Nearest, "-Infinity", "-Infinity", Equal);
    test(0, 10, Floor, "-Infinity", "-Infinity", Equal);
    test(0, 10, Ceiling, "-Infinity", "-Infinity", Equal);
    test(0, 10, Down, "-Infinity", "-Infinity", Equal);
    test(0, 10, Up, "-Infinity", "-Infinity", Equal);
    test(0, 10, Nearest, "-Infinity", "-Infinity", Equal);
    test(0, 100, Nearest, "-Infinity", "-Infinity", Equal);
    test(0, 300, Exact, "-Infinity", "-Infinity", Equal);
    test(1, 1, Nearest, "0.0", "0x0.0", Equal);
    test(1, 10, Floor, "0.0", "0x0.0", Equal);
    test(1, 10, Ceiling, "0.0", "0x0.0", Equal);
    test(1, 10, Down, "0.0", "0x0.0", Equal);
    test(1, 10, Up, "0.0", "0x0.0", Equal);
    test(1, 10, Nearest, "0.0", "0x0.0", Equal);
    test(1, 100, Nearest, "0.0", "0x0.0", Equal);
    test(1, 300, Exact, "0.0", "0x0.0", Equal);
    test(2, 1, Nearest, "0.50", "0x0.8", Less);
    test(2, 10, Floor, "0.69238", "0x0.b14", Less);
    test(2, 10, Ceiling, "0.6934", "0x0.b18", Greater);
    test(2, 10, Down, "0.69238", "0x0.b14", Less);
    test(2, 10, Up, "0.6934", "0x0.b18", Greater);
    test(2, 10, Nearest, "0.6934", "0x0.b18", Greater);
    test(
        2,
        100,
        Nearest,
        "0.69314718055994530941723212146",
        "0x0.b17217f7d1cf79abc9e3b398",
        Less,
    );
    test(10, 1, Nearest, "2.0", "0x2.0", Less);
    test(10, 10, Floor, "2.3008", "0x2.4d", Less);
    test(10, 10, Ceiling, "2.305", "0x2.4e", Greater);
    test(10, 10, Down, "2.3008", "0x2.4d", Less);
    test(10, 10, Up, "2.305", "0x2.4e", Greater);
    test(10, 10, Nearest, "2.3008", "0x2.4d", Less);
    test(
        10,
        100,
        Nearest,
        "2.302585092994045684017991454684",
        "0x2.4d763776aaa2b05ba95b58ae",
        Less,
    );
    test(1000, 1, Nearest, "8.0", "0x8.0", Greater);
    test(1000, 10, Floor, "6.906", "0x6.e8", Less);
    test(1000, 10, Ceiling, "6.9141", "0x6.ea", Greater);
    test(1000, 10, Down, "6.906", "0x6.e8", Less);
    test(1000, 10, Up, "6.9141", "0x6.ea", Greater);
    test(1000, 10, Nearest, "6.906", "0x6.e8", Less);
    test(
        1000,
        100,
        Nearest,
        "6.907755278982137052053974364051",
        "0x6.e862a663ffe81112fc120a0a",
        Less,
    );
    test(18446744073709551615, 1, Nearest, "32.0", "0x2.0E+1", Less);
    test(18446744073709551615, 10, Floor, "44.312", "0x2c.5", Less);
    test(
        18446744073709551615,
        10,
        Ceiling,
        "44.38",
        "0x2c.6",
        Greater,
    );
    test(18446744073709551615, 10, Down, "44.312", "0x2c.5", Less);
    test(18446744073709551615, 10, Up, "44.38", "0x2c.6", Greater);
    test(
        18446744073709551615,
        10,
        Nearest,
        "44.38",
        "0x2c.6",
        Greater,
    );
    test(
        18446744073709551615,
        100,
        Nearest,
        "44.361419555836499802648645665",
        "0x2c.5c85fdf473de6af178ece6",
        Less,
    );
}

fn ln_prec_round_properties_helper(x: &BigExponentFloat, prec: u64, rm: RoundingMode) {
    let r = ln_prec_round_all(x, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else if !r.0.is_nan() {
        // The result lies between the results rounded down and up.
        let floor = ln_prec_round_all(x, prec, Floor);
        let ceiling = ln_prec_round_all(x, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0);
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(&ln_prec_round_all(x, prec, Exact), &r));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = ln_prec_round_all(x, prec + 16, rm);
            let (z, oz) = y.round_to_prec_round(prec, rm);
            assert!(same(&z, &r.0));
            assert_eq!(if o == Equal { oz } else { o }, r.1);
        }
    }
    // A `BigExponentFloat` that is not too large or too small gives the same result as the equal
    // `Rational`.
    if x.is_finite() && exponent_is_moderate(x) {
        let q = Rational::exact_from(x);
        assert!(same_pair(&ln_rational_prec_round_all(&q, prec, rm), &r));
    }
}

#[test]
fn ln_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_15().test_properties(
        |(x, prec, rm)| {
            ln_prec_round_properties_helper(&x, prec, rm);
        },
    );
}

#[test]
fn ln_prec_round_float_agreement_properties() {
    float_unsigned_rounding_mode_triple_gen_var_19().test_properties(|(x, prec, rm)| {
        if rm == Exact {
            return;
        }
        let (f, fo) = x.ln_prec_round_ref(prec, rm);
        if float_result_comparable(&f, fo) {
            let r = ln_prec_round_all(&float_to_big_exponent_float(&x), prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    });
}

#[test]
fn ln_rational_prec_round_float_agreement_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_6().test_properties(|(x, prec, rm)| {
        let (f, fo) = Float::ln_rational_prec_round_ref(&x, prec, rm);
        if float_result_comparable(&f, fo) {
            let r = ln_rational_prec_round_all(&x, prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    });
}

#[test]
fn ln_unsigned_prec_round_float_agreement_properties() {
    unsigned_unsigned_rounding_mode_triple_gen_var_8::<u64>().test_properties(|(n, prec, rm)| {
        let (f, fo) = Float::ln_unsigned_prec_round(n, prec, rm);
        let r = ln_unsigned_prec_round_all(n, prec, rm);
        assert!(same(&r.0, &float_to_big_exponent_float(&f)));
        assert_eq!(r.1, fo);
    });
}

#[test]
fn ln_1_plus_x_series_properties() {
    // The series approximation of ln(1 + t) is within its stated error bound of `Float`'s correctly
    // rounded value, computed with 64 more bits.
    float_unsigned_pair_gen_var_1::<u64>().test_properties(|(t, prec)| {
        if !t.is_finite() || t.is_zero() || t.get_exponent().unwrap() > -2 {
            return;
        }
        let wp = prec + 16;
        let (s, loss) = ln_1_plus_x_series(&float_to_big_exponent_float(&t), wp);
        let exact = float_to_big_exponent_float(&t.ln_1_plus_x_prec_round_ref(wp + 64, Nearest).0);
        let error = (s.clone() - exact).abs();
        let bound = big_exponent_float_power_of_2(
            s.get_exponent().unwrap() - Integer::from(wp) + Integer::from(loss),
        );
        assert!(error < bound, "{t:#x} {wp}");
    });
}
