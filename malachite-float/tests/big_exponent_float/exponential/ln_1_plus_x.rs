// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, One, Two, Zero,
};
use malachite_base::num::conversion::traits::FromStringBase;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::float_to_big_exponent_float;
use malachite_float::test_util::generators::{
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_16,
    float_unsigned_rounding_mode_triple_gen_var_21,
};
use malachite_float::{BigExponentFloat, Float};
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

fn ln_1_plus_x_prec_round_all(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().ln_1_plus_x_prec_round(prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.ln_1_plus_x_prec_round_ref(prec, rm), &r));
    let mut z = x.clone();
    let o = z.ln_1_plus_x_prec_round_assign(prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().ln_1_plus_x_prec(prec), &r));
        assert!(same_pair(&x.ln_1_plus_x_prec_ref(prec), &r));
        let mut z = x.clone();
        let o = z.ln_1_plus_x_prec_assign(prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

#[test]
fn test_ln_1_plus_x_prec_round() {
    let nan = BigExponentFloat::NAN;
    let inf = BigExponentFloat::INFINITY;
    let ninf = BigExponentFloat::NEGATIVE_INFINITY;
    let zero = BigExponentFloat::ZERO;
    for rm in exhaustive_rounding_modes() {
        assert!(same_pair(
            &ln_1_plus_x_prec_round_all(&nan, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &ln_1_plus_x_prec_round_all(&inf, 10, rm),
            &(inf.clone(), Equal)
        ));
        assert!(same_pair(
            &ln_1_plus_x_prec_round_all(&ninf, 10, rm),
            &(nan.clone(), Equal)
        ));
        assert!(same_pair(
            &ln_1_plus_x_prec_round_all(&zero, 10, rm),
            &(zero.clone(), Equal)
        ));
        assert!(same_pair(
            &ln_1_plus_x_prec_round_all(&BigExponentFloat::NEGATIVE_ONE, 10, rm),
            &(ninf.clone(), Equal)
        ));
        assert!(same_pair(
            &ln_1_plus_x_prec_round_all(&-BigExponentFloat::TWO, 10, rm),
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
        let r = ln_1_plus_x_prec_round_all(&hex(x_hex), prec, rm);
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
        "1.4199",
        "0x1.6b8",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Ceiling,
        "1.422",
        "0x1.6c",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Down,
        "1.4199",
        "0x1.6b8",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Up,
        "1.422",
        "0x1.6c",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        10,
        Nearest,
        "1.422",
        "0x1.6c",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        100,
        Nearest,
        "1.421080412794292603484306978989",
        "0x1.6bcbed09f00aece2ea800b6b",
        Greater,
    );
    test("0.50", "0x0.8", 1, Nearest, "0.50", "0x0.8", Greater);
    test("0.50", "0x0.8", 10, Floor, "0.4053", "0x0.67c", Less);
    test("0.50", "0x0.8", 10, Ceiling, "0.40576", "0x0.67e", Greater);
    test("0.50", "0x0.8", 10, Down, "0.4053", "0x0.67c", Less);
    test("0.50", "0x0.8", 10, Up, "0.40576", "0x0.67e", Greater);
    test("0.50", "0x0.8", 10, Nearest, "0.4053", "0x0.67c", Less);
    test(
        "0.50",
        "0x0.8",
        100,
        Nearest,
        "0.405465108108164381978013115464",
        "0x0.67cc8fb2fe612fcada35d9bd",
        Less,
    );
    test("3.0", "0x3", 1, Nearest, "1.0", "0x1.0", Less);
    test("3.0", "0x3", 10, Floor, "1.3848", "0x1.628", Less);
    test("3.0", "0x3", 10, Ceiling, "1.387", "0x1.63", Greater);
    test("3.0", "0x3", 10, Down, "1.3848", "0x1.628", Less);
    test("3.0", "0x3", 10, Up, "1.387", "0x1.63", Greater);
    test("3.0", "0x3", 10, Nearest, "1.387", "0x1.63", Greater);
    test(
        "3.0",
        "0x3",
        100,
        Nearest,
        "1.3862943611198906188344642429",
        "0x1.62e42fefa39ef35793c7673",
        Less,
    );
    test("7.0", "0x7", 1, Nearest, "2.0", "0x2.0", Less);
    test("7.0", "0x7", 10, Floor, "2.078", "0x2.14", Less);
    test("7.0", "0x7", 10, Ceiling, "2.0820", "0x2.15", Greater);
    test("7.0", "0x7", 10, Down, "2.078", "0x2.14", Less);
    test("7.0", "0x7", 10, Up, "2.0820", "0x2.15", Greater);
    test("7.0", "0x7", 10, Nearest, "2.078", "0x2.14", Less);
    test(
        "7.0",
        "0x7",
        100,
        Nearest,
        "2.07944154167983592825169636437",
        "0x2.145647e7756e6d035dab1ac8",
        Less,
    );
    test("100.0", "0x64", 1, Nearest, "4.0", "0x4.0", Less);
    test("100.0", "0x64", 10, Floor, "4.609", "0x4.9c", Less);
    test("100.0", "0x64", 10, Ceiling, "4.6172", "0x4.9e", Greater);
    test("100.0", "0x64", 10, Down, "4.609", "0x4.9c", Less);
    test("100.0", "0x64", 10, Up, "4.6172", "0x4.9e", Greater);
    test("100.0", "0x64", 10, Nearest, "4.6172", "0x4.9e", Greater);
    test(
        "100.0",
        "0x64",
        100,
        Nearest,
        "4.6151205168412594508841982669101",
        "0x4.9d7889c6ee8c2f8e3679653f8",
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
        "6.907755278982137052053974364051",
        "0x6.e862a663ffe81112fc120a0a",
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
        "6.9087547793152205852207837629719",
        "0x6.e8a427393e190c0f51e66f788",
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
        "23.0258509300404568401749145469",
        "0x17.069e2aa3184e62fbe4ebc0a",
        Greater,
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
        "55.4517744447956247533785705438",
        "0x37.73a77d7190d605af17291f8",
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
        "56.14492162535557006279580225169",
        "0x38.2519956962a57f5ae10c5318",
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
        "3.9e-121",
        "0x1.0E-100",
        Greater,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Floor,
        "3.8688e-121",
        "0xf.fcE-101",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Ceiling,
        "3.9e-121",
        "0x1.0E-100",
        Greater,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Down,
        "3.8688e-121",
        "0xf.fcE-101",
        Less,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Up,
        "3.9e-121",
        "0x1.0E-100",
        Greater,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        10,
        Nearest,
        "3.9e-121",
        "0x1.0E-100",
        Greater,
    );
    test(
        "3.9e-121",
        "0x1.0E-100",
        100,
        Nearest,
        "3.9e-121",
        "0x1.0E-100",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        1,
        Nearest,
        "1.6e-361235995",
        "0x1.0E-300000000",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Floor,
        "2.3919e-361235995",
        "0x1.7f8E-300000000",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Ceiling,
        "2.4e-361235995",
        "0x1.8E-300000000",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Down,
        "2.3919e-361235995",
        "0x1.7f8E-300000000",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Up,
        "2.4e-361235995",
        "0x1.8E-300000000",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        10,
        Nearest,
        "2.4e-361235995",
        "0x1.8E-300000000",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        100,
        Nearest,
        "2.4e-361235995",
        "0x1.8E-300000000",
        Greater,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        1,
        Nearest,
        "-3.2e-361235995",
        "-0x2.0E-300000000",
        Less,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Floor,
        "-2.3982e-361235995",
        "-0x1.808E-300000000",
        Less,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Ceiling,
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        Greater,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Down,
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        Greater,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Up,
        "-2.3982e-361235995",
        "-0x1.808E-300000000",
        Less,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        10,
        Nearest,
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        Greater,
    );
    test(
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        100,
        Nearest,
        "-2.4e-361235995",
        "-0x1.8E-300000000",
        Greater,
    );
    test("-0.50", "-0x0.8", 1, Nearest, "-0.50", "-0x0.8", Greater);
    test("-0.50", "-0x0.8", 10, Floor, "-0.6934", "-0x0.b18", Less);
    test(
        "-0.50", "-0x0.8", 10, Ceiling, "-0.69238", "-0x0.b14", Greater,
    );
    test("-0.50", "-0x0.8", 10, Down, "-0.69238", "-0x0.b14", Greater);
    test("-0.50", "-0x0.8", 10, Up, "-0.6934", "-0x0.b18", Less);
    test("-0.50", "-0x0.8", 10, Nearest, "-0.6934", "-0x0.b18", Less);
    test(
        "-0.50",
        "-0x0.8",
        100,
        Nearest,
        "-0.69314718055994530941723212146",
        "-0x0.b17217f7d1cf79abc9e3b398",
        Greater,
    );
    test("-3.0", "-0x3", 1, Nearest, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", 10, Floor, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", 10, Ceiling, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", 10, Down, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", 10, Up, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", 10, Nearest, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", 100, Nearest, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", 300, Exact, "NaN", "NaN", Equal);
    test("1.0", "0x1.0", 1, Nearest, "0.50", "0x0.8", Less);
    test("1.0", "0x1.0", 10, Floor, "0.69238", "0x0.b14", Less);
    test("1.0", "0x1.0", 10, Ceiling, "0.6934", "0x0.b18", Greater);
    test("1.0", "0x1.0", 10, Down, "0.69238", "0x0.b14", Less);
    test("1.0", "0x1.0", 10, Up, "0.6934", "0x0.b18", Greater);
    test("1.0", "0x1.0", 10, Nearest, "0.6934", "0x0.b18", Greater);
    test(
        "1.0",
        "0x1.0",
        100,
        Nearest,
        "0.69314718055994530941723212146",
        "0x0.b17217f7d1cf79abc9e3b398",
        Less,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        1,
        Nearest,
        "0.50",
        "0x0.8",
        Less,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        10,
        Floor,
        "0.69238",
        "0x0.b14",
        Less,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        10,
        Ceiling,
        "0.6934",
        "0x0.b18",
        Greater,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        10,
        Down,
        "0.69238",
        "0x0.b14",
        Less,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        10,
        Up,
        "0.6934",
        "0x0.b18",
        Greater,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        10,
        Nearest,
        "0.6934",
        "0x0.b18",
        Greater,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        100,
        Nearest,
        "0.69314718044352998758352104343",
        "0x0.b17217f751cf79aba9e3b398",
        Greater,
    );
}

#[test]
#[should_panic]
fn ln_1_plus_x_prec_round_fail_1() {
    BigExponentFloat::from(3).ln_1_plus_x_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn ln_1_plus_x_prec_round_fail_2() {
    BigExponentFloat::from(core::f64::consts::PI).ln_1_plus_x_prec_round(10, Exact);
}

fn ln_1_plus_x_prec_round_properties_helper(x: &BigExponentFloat, prec: u64, rm: RoundingMode) {
    let r = ln_1_plus_x_prec_round_all(x, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else if !r.0.is_nan() {
        // The result lies between the results rounded down and up.
        let floor = ln_1_plus_x_prec_round_all(x, prec, Floor);
        let ceiling = ln_1_plus_x_prec_round_all(x, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0);
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(&ln_1_plus_x_prec_round_all(x, prec, Exact), &r));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = ln_1_plus_x_prec_round_all(x, prec + 16, rm);
            let (z, oz) = y.round_to_prec_round(prec, rm);
            assert!(same(&z, &r.0));
            assert_eq!(if o == Equal { oz } else { o }, r.1);
        }
    }
    // log_b(1 + x) is log_b of the exact sum 1 + x.
    if x.is_finite() && exponent_is_moderate(x) {
        let z = x + BigExponentFloat::ONE;
        assert!(same_pair(&z.ln_prec_round_ref(prec, rm), &r));
    }
}

#[test]
fn ln_1_plus_x_prec_round_properties() {
    big_exponent_float_unsigned_rounding_mode_triple_gen_var_16().test_properties(
        |(x, prec, rm)| {
            ln_1_plus_x_prec_round_properties_helper(&x, prec, rm);
        },
    );
}

#[test]
fn ln_1_plus_x_prec_round_float_agreement_properties() {
    float_unsigned_rounding_mode_triple_gen_var_21().test_properties(|(x, prec, rm)| {
        if rm == Exact {
            return;
        }
        let (f, fo) = x.ln_1_plus_x_prec_round_ref(prec, rm);
        if float_result_comparable(&f, fo) {
            let r = ln_1_plus_x_prec_round_all(&float_to_big_exponent_float(&x), prec, rm);
            assert!(same(&r.0, &float_to_big_exponent_float(&f)));
            assert_eq!(r.1, fo);
        }
    });
}
