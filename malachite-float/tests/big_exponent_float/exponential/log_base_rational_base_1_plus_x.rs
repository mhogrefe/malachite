// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::FromStringBase;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::float_to_big_exponent_float;
use malachite_float::test_util::generators::{
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_14,
    float_rational_unsigned_rounding_mode_quadruple_gen_var_13,
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

fn log_base_rational_base_1_plus_x_prec_round_all(
    x: &BigExponentFloat,
    b: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x
        .clone()
        .log_base_rational_base_1_plus_x_prec_round(b, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.log_base_rational_base_1_plus_x_prec_round_ref(b, prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.log_base_rational_base_1_plus_x_prec_round_assign(b, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(
            &x.clone().log_base_rational_base_1_plus_x_prec(b, prec),
            &r
        ));
        assert!(same_pair(
            &x.log_base_rational_base_1_plus_x_prec_ref(b, prec),
            &r
        ));
        let mut z = x.clone();
        let o = z.log_base_rational_base_1_plus_x_prec_assign(b, prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

#[test]
fn test_log_base_rational_base_1_plus_x_prec_round() {
    // The expected values agree with FLINT's arb (see the `arf_logarithm_general` oracle mode).
    let test = |x: &str,
                x_hex: &str,
                b: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o: Ordering| {
        assert_eq!(hex(x_hex).to_string(), x);
        let r = log_base_rational_base_1_plus_x_prec_round_all(
            &hex(x_hex),
            &Rational::from_str(b).unwrap(),
            prec,
            rm,
        );
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {b} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {b} {prec} {rm}");
    };
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3/2",
        1,
        Nearest,
        "4.0",
        "0x4.0",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3/2",
        10,
        Floor,
        "3.5039",
        "0x3.81",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3/2",
        10,
        Ceiling,
        "3.508",
        "0x3.82",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3/2",
        10,
        Down,
        "3.5039",
        "0x3.81",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3/2",
        10,
        Up,
        "3.508",
        "0x3.82",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3/2",
        10,
        Nearest,
        "3.5039",
        "0x3.81",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3/2",
        100,
        Nearest,
        "3.504815542389892637555311944253",
        "0x3.813b976513bc34f6d07ff7de8",
        Less,
    );
    test("0.50", "0x0.8", "3/2", 1, Nearest, "1.0", "0x1.0", Equal);
    test("0.50", "0x0.8", "3/2", 10, Floor, "1.0", "0x1.0", Equal);
    test("0.50", "0x0.8", "3/2", 10, Ceiling, "1.0", "0x1.0", Equal);
    test("0.50", "0x0.8", "3/2", 10, Down, "1.0", "0x1.0", Equal);
    test("0.50", "0x0.8", "3/2", 10, Up, "1.0", "0x1.0", Equal);
    test("0.50", "0x0.8", "3/2", 10, Nearest, "1.0", "0x1.0", Equal);
    test("0.50", "0x0.8", "3/2", 100, Nearest, "1.0", "0x1.0", Equal);
    test("0.50", "0x0.8", "3/2", 300, Exact, "1.0", "0x1.0", Equal);
    test("-0.50", "-0x0.8", "3/2", 1, Nearest, "-2.0", "-0x2.0", Less);
    test(
        "-0.50", "-0x0.8", "3/2", 10, Floor, "-1.711", "-0x1.b6", Less,
    );
    test(
        "-0.50", "-0x0.8", "3/2", 10, Ceiling, "-1.7090", "-0x1.b58", Greater,
    );
    test(
        "-0.50", "-0x0.8", "3/2", 10, Down, "-1.7090", "-0x1.b58", Greater,
    );
    test("-0.50", "-0x0.8", "3/2", 10, Up, "-1.711", "-0x1.b6", Less);
    test(
        "-0.50", "-0x0.8", "3/2", 10, Nearest, "-1.7090", "-0x1.b58", Greater,
    );
    test(
        "-0.50",
        "-0x0.8",
        "3/2",
        100,
        Nearest,
        "-1.709511291351454776976190262174",
        "-0x1.b5a288307f4a327a7634fec24",
        Less,
    );
    test("2.0", "0x2", "3/2", 1, Nearest, "2.0", "0x2.0", Less);
    test("2.0", "0x2", "3/2", 10, Floor, "2.7070", "0x2.b5", Less);
    test("2.0", "0x2", "3/2", 10, Ceiling, "2.711", "0x2.b6", Greater);
    test("2.0", "0x2", "3/2", 10, Down, "2.7070", "0x2.b5", Less);
    test("2.0", "0x2", "3/2", 10, Up, "2.711", "0x2.b6", Greater);
    test("2.0", "0x2", "3/2", 10, Nearest, "2.711", "0x2.b6", Greater);
    test(
        "2.0",
        "0x2",
        "3/2",
        100,
        Nearest,
        "2.7095112913514547769761902621743",
        "0x2.b5a288307f4a327a7634fec24",
        Greater,
    );
    test("8.0", "0x8", "3/2", 1, Nearest, "4.0", "0x4.0", Less);
    test("8.0", "0x8", "3/2", 10, Floor, "5.4141", "0x5.6a", Less);
    test("8.0", "0x8", "3/2", 10, Ceiling, "5.422", "0x5.6c", Greater);
    test("8.0", "0x8", "3/2", 10, Down, "5.4141", "0x5.6a", Less);
    test("8.0", "0x8", "3/2", 10, Up, "5.422", "0x5.6c", Greater);
    test("8.0", "0x8", "3/2", 10, Nearest, "5.422", "0x5.6c", Greater);
    test(
        "8.0",
        "0x8",
        "3/2",
        100,
        Nearest,
        "5.4190225827029095539523805243486",
        "0x5.6b451060fe9464f4ec69fd848",
        Greater,
    );
    test("26.0", "0x1a", "3/2", 1, Nearest, "8.0", "0x8.0", Less);
    test("26.0", "0x1a", "3/2", 10, Floor, "8.125", "0x8.2", Less);
    test(
        "26.0", "0x1a", "3/2", 10, Ceiling, "8.1406", "0x8.24", Greater,
    );
    test("26.0", "0x1a", "3/2", 10, Down, "8.125", "0x8.2", Less);
    test("26.0", "0x1a", "3/2", 10, Up, "8.1406", "0x8.24", Greater);
    test("26.0", "0x1a", "3/2", 10, Nearest, "8.125", "0x8.2", Less);
    test(
        "26.0",
        "0x1a",
        "3/2",
        100,
        Nearest,
        "8.1285338740543643309285707865260",
        "0x8.20e798917dde976f629efc47",
        Greater,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "3/2",
        1,
        Nearest,
        "2.1e9",
        "0x8.0E+7",
        Greater,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "3/2",
        10,
        Floor,
        "2.051e9",
        "0x7.a4E+7",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "3/2",
        10,
        Ceiling,
        "2.0531e9",
        "0x7.a6E+7",
        Greater,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "3/2",
        10,
        Down,
        "2.051e9",
        "0x7.a4E+7",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "3/2",
        10,
        Up,
        "2.0531e9",
        "0x7.a6E+7",
        Greater,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "3/2",
        10,
        Nearest,
        "2.051e9",
        "0x7.a4E+7",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "3/2",
        100,
        Nearest,
        "2051413549.621745732371428314608",
        "0x7a46162d.9f2aba72f67d8b0a3",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3/2",
        1,
        Nearest,
        "6.4e-361235995",
        "0x4.0E-300000000",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3/2",
        10,
        Floor,
        "5.9065e-361235995",
        "0x3.b3E-300000000",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3/2",
        10,
        Ceiling,
        "5.913e-361235995",
        "0x3.b4E-300000000",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3/2",
        10,
        Down,
        "5.9065e-361235995",
        "0x3.b3E-300000000",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3/2",
        10,
        Up,
        "5.913e-361235995",
        "0x3.b4E-300000000",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3/2",
        10,
        Nearest,
        "5.9065e-361235995",
        "0x3.b3E-300000000",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3/2",
        100,
        Nearest,
        "5.906909776828210515123304511935e-361235995",
        "0x3.b30f7edd60a48c370c0f2d5bE-300000000",
        Less,
    );
    test("-3.0", "-0x3", "3/2", 1, Nearest, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", "3/2", 10, Floor, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", "3/2", 10, Ceiling, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", "3/2", 10, Down, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", "3/2", 10, Up, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", "3/2", 10, Nearest, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", "3/2", 100, Nearest, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", "3/2", 300, Exact, "NaN", "NaN", Equal);
    test("0.0", "0x0", "3/2", 1, Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0", "3/2", 10, Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0", "3/2", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("0.0", "0x0", "3/2", 10, Down, "0.0", "0x0.0", Equal);
    test("0.0", "0x0", "3/2", 10, Up, "0.0", "0x0.0", Equal);
    test("0.0", "0x0", "3/2", 10, Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0", "3/2", 100, Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0", "3/2", 300, Exact, "0.0", "0x0.0", Equal);
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "3/2",
        1,
        Nearest,
        "64.0",
        "0x4.0E+1",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "3/2",
        10,
        Floor,
        "56.75",
        "0x38.c",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "3/2",
        10,
        Ceiling,
        "56.812",
        "0x38.d",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "3/2",
        10,
        Down,
        "56.75",
        "0x38.c",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "3/2",
        10,
        Up,
        "56.812",
        "0x38.d",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "3/2",
        10,
        Nearest,
        "56.812",
        "0x38.d",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "3/2",
        100,
        Nearest,
        "56.78873587292235839234690570690",
        "0x38.c9ea981b622fc2446fb2c33",
        Less,
    );
    test(
        "6.8949262155132e354333927",
        "0x3.a0666ba2118E+294267957",
        "3/2",
        10,
        Nearest,
        "2.01e9",
        "0x7.8E+7",
        Greater,
    );
    test(
        "5.7739571204062e-323228558",
        "0x6.3b1994b6d4E-268435507",
        "3/2",
        10,
        Nearest,
        "1.425e-323228557",
        "0xf.6E-268435507",
        Greater,
    );
    test(
        "1.0383360817427e360981065",
        "0x3.081e8996b88E+299788285",
        "3/2",
        10,
        Nearest,
        "2.0489e9",
        "0x7.a2E+7",
        Less,
    );
    test(
        "5.7915682168245e-323228558",
        "0x6.3ff7127d17E-268435507",
        "3/2",
        10,
        Nearest,
        "1.428e-323228557",
        "0xf.68E-268435507",
        Less,
    );
    test(
        "1.73611966233642e341039443",
        "0xf.965c71421bE+283227126",
        "3/2",
        10,
        Nearest,
        "1.938e9",
        "0x7.38E+7",
        Greater,
    );
    test(
        "4.458891655161038e-323228558",
        "0x4.cfcd91e8207E-268435507",
        "3/2",
        10,
        Nearest,
        "1.100e-323228557",
        "0xb.eE-268435507",
        Greater,
    );
    test(
        "1.05667285444620e335130796",
        "0x6.d6ea776b1dcE+278320101",
        "3/2",
        10,
        Nearest,
        "1.9021e9",
        "0x7.16E+7",
        Less,
    );
    test(
        "5.67415236405047e-323228558",
        "0x6.1f87315124aE-268435507",
        "3/2",
        10,
        Nearest,
        "1.399e-323228557",
        "0xf.18E-268435507",
        Less,
    );
    test(
        "8.124619513877133e366889750",
        "0x9.ab7e893dea08E+304695342",
        "3/2",
        10,
        Nearest,
        "2.085e9",
        "0x7.c4E+7",
        Greater,
    );
    test(
        "4.517599880807867e-323228558",
        "0x4.e00587e91ebE-268435507",
        "3/2",
        10,
        Nearest,
        "1.115e-323228557",
        "0xc.08E-268435507",
        Greater,
    );
    test(
        "2.74361364155439e336607958",
        "0x4.48b122ea098E+279546858",
        "3/2",
        10,
        Nearest,
        "1.9105e9",
        "0x7.1eE+7",
        Less,
    );
    test(
        "4.969653391628068e-323228558",
        "0x5.5ce7a47a9154E-268435507",
        "3/2",
        10,
        Nearest,
        "1.225e-323228557",
        "0xd.38E-268435507",
        Less,
    );
    test(
        "1.4972249000678893e372059812",
        "0x8.3ed6e6d496e58E+308988985",
        "3/2",
        10,
        Nearest,
        "2.11e9",
        "0x7.eE+7",
        Greater,
    );
    test(
        "4.1888336703414932e-323228558",
        "0x4.85328a07c288cE-268435507",
        "3/2",
        10,
        Nearest,
        "1.034e-323228557",
        "0xb.28E-268435507",
        Greater,
    );
    test(
        "7.6480186933454917e344363051",
        "0x1.db705130e53aeE+285987324",
        "3/2",
        10,
        Nearest,
        "1.955e9",
        "0x7.48E+7",
        Less,
    );
    test(
        "3.5136887892300958e-323228558",
        "0x3.caaef7cdf9816E-268435507",
        "3/2",
        10,
        Nearest,
        "8.659e-323228558",
        "0x9.58E-268435507",
        Less,
    );
    test(
        "3.5465182053740e357842184",
        "0x5.3f920d8d1fE+297181501",
        "3/2",
        10,
        Floor,
        "2.0321e9",
        "0x7.92E+7",
        Less,
    );
    test(
        "4.385506831666e-323228558",
        "0x4.bb87a6b44E-268435507",
        "3/2",
        10,
        Floor,
        "1.0816e-323228557",
        "0xb.acE-268435507",
        Less,
    );
    test(
        "3.5465182053740e357842184",
        "0x5.3f920d8d1fE+297181501",
        "3/2",
        10,
        Ceiling,
        "2.034e9",
        "0x7.94E+7",
        Greater,
    );
    test(
        "4.385506831666e-323228558",
        "0x4.bb87a6b44E-268435507",
        "3/2",
        10,
        Ceiling,
        "1.083e-323228557",
        "0xb.bE-268435507",
        Greater,
    );
    test(
        "5.093313229390e363750740",
        "0x2.21363204a8E+302088451",
        "3/2",
        10,
        Floor,
        "2.064e9",
        "0x7.bE+7",
        Less,
    );
    test(
        "5.4422541682680e-323228558",
        "0x5.df76e5a5afE-268435507",
        "3/2",
        10,
        Floor,
        "1.341e-323228557",
        "0xe.78E-268435507",
        Less,
    );
    test(
        "5.093313229390e363750740",
        "0x2.21363204a8E+302088451",
        "3/2",
        10,
        Ceiling,
        "2.0657e9",
        "0x7.b2E+7",
        Greater,
    );
    test(
        "5.4422541682680e-323228558",
        "0x5.df76e5a5afE-268435507",
        "3/2",
        10,
        Ceiling,
        "1.3422e-323228557",
        "0xe.7cE-268435507",
        Greater,
    );
    test(
        "1.675626414224056e362273626",
        "0x1.fcc062ba972cE+300861734",
        "3/2",
        10,
        Floor,
        "2.0573e9",
        "0x7.aaE+7",
        Less,
    );
    test(
        "3.31701617533378e-323228558",
        "0x3.9459e50ab8fE-268435507",
        "3/2",
        10,
        Floor,
        "8.1808e-323228558",
        "0x8.d4E-268435507",
        Less,
    );
    test(
        "1.675626414224056e362273626",
        "0x1.fcc062ba972cE+300861734",
        "3/2",
        10,
        Ceiling,
        "2.059e9",
        "0x7.acE+7",
        Greater,
    );
    test(
        "3.31701617533378e-323228558",
        "0x3.9459e50ab8fE-268435507",
        "3/2",
        10,
        Ceiling,
        "8.195e-323228558",
        "0x8.d8E-268435507",
        Greater,
    );
    test(
        "2.287275056453459e363750781",
        "0x1.19102483e4e4E+302088485",
        "3/2",
        10,
        Floor,
        "2.064e9",
        "0x7.bE+7",
        Less,
    );
    test(
        "5.641862826541624e-323228558",
        "0x6.169b9cf8e03E-268435507",
        "3/2",
        10,
        Floor,
        "1.39e-323228557",
        "0xf.0E-268435507",
        Less,
    );
    test(
        "2.287275056453459e363750781",
        "0x1.19102483e4e4E+302088485",
        "3/2",
        10,
        Ceiling,
        "2.0657e9",
        "0x7.b2E+7",
        Greater,
    );
    test(
        "5.641862826541624e-323228558",
        "0x6.169b9cf8e03E-268435507",
        "3/2",
        10,
        Ceiling,
        "1.3915e-323228557",
        "0xf.04E-268435507",
        Greater,
    );
    test(
        "8.625722959519258e369659426",
        "0xe.b3d17e5166d8E+306995508",
        "3/2",
        10,
        Floor,
        "2.0992e9",
        "0x7.d2E+7",
        Less,
    );
    test(
        "5.935404131899622e-323228558",
        "0x6.67b36e32c804E-268435507",
        "3/2",
        10,
        Floor,
        "1.4639e-323228557",
        "0xf.ccE-268435507",
        Less,
    );
    test(
        "8.625722959519258e369659426",
        "0xe.b3d17e5166d8E+306995508",
        "3/2",
        10,
        Ceiling,
        "2.101e9",
        "0x7.d4E+7",
        Greater,
    );
    test(
        "5.935404131899622e-323228558",
        "0x6.67b36e32c804E-268435507",
        "3/2",
        10,
        Ceiling,
        "1.465e-323228557",
        "0xf.dE-268435507",
        Greater,
    );
    test(
        "1.199711944383088e343809116",
        "0x3.f2efa9ff0278E+285527290",
        "3/2",
        10,
        Floor,
        "1.950e9",
        "0x7.44E+7",
        Less,
    );
    test(
        "5.043038704558195e-323228558",
        "0x5.712d988ae4d4E-268435507",
        "3/2",
        10,
        Floor,
        "1.242e-323228557",
        "0xd.68E-268435507",
        Less,
    );
    test(
        "1.199711944383088e343809116",
        "0x3.f2efa9ff0278E+285527290",
        "3/2",
        10,
        Ceiling,
        "1.9524e9",
        "0x7.46E+7",
        Greater,
    );
    test(
        "5.043038704558195e-323228558",
        "0x5.712d988ae4d4E-268435507",
        "3/2",
        10,
        Ceiling,
        "1.2438e-323228557",
        "0xd.6cE-268435507",
        Greater,
    );
    test(
        "6.9842527418660668e354887820",
        "0x6.0b30b7453083cE+294727955",
        "3/2",
        10,
        Floor,
        "2.0154e9",
        "0x7.82E+7",
        Less,
    );
    test(
        "4.7025308618690065e-323228558",
        "0x5.131c367ec1468E-268435507",
        "3/2",
        10,
        Floor,
        "1.1598e-323228557",
        "0xc.84E-268435507",
        Less,
    );
    test(
        "6.9842527418660668e354887820",
        "0x6.0b30b7453083cE+294727955",
        "3/2",
        10,
        Ceiling,
        "2.017e9",
        "0x7.84E+7",
        Greater,
    );
    test(
        "4.7025308618690065e-323228558",
        "0x5.131c367ec1468E-268435507",
        "3/2",
        10,
        Ceiling,
        "1.161e-323228557",
        "0xc.88E-268435507",
        Greater,
    );
    test(
        "8.4251982157911015e377045228",
        "0x8.0fd960d541ebE+313129284",
        "3/2",
        10,
        Floor,
        "2.139e9",
        "0x7.f8E+7",
        Less,
    );
    test(
        "3.4813992514844556e-323228558",
        "0x3.c1c363749c1acE-268435507",
        "3/2",
        10,
        Floor,
        "8.57e-323228558",
        "0x9.4E-268435507",
        Less,
    );
    test(
        "8.4251982157911015e377045228",
        "0x8.0fd960d541ebE+313129284",
        "3/2",
        10,
        Ceiling,
        "2.1412e9",
        "0x7.faE+7",
        Greater,
    );
    test(
        "3.4813992514844556e-323228558",
        "0x3.c1c363749c1acE-268435507",
        "3/2",
        10,
        Ceiling,
        "8.5862e-323228558",
        "0x9.44E-268435507",
        Greater,
    );
    test(
        "1.82716682451200413614603839862560160884048e339105684",
        "0xc.dd0a36518ac887a6996715ae8d75a6b75E+281621174",
        "3/2",
        100,
        Nearest,
        "1925738313.646509279687740671217",
        "0x72c86f49.a581a1d4d1c1caa5b",
        Greater,
    );
    test(
        "3.30018688348637604889432105225940069008933e-323228558",
        "0x3.8fb3b1b34d7179f15390eba566b640d5ecE-268435507",
        "3/2",
        100,
        Nearest,
        "8.139262337231734791913912081729e-323228558",
        "0x8.c8889bcb05d5932b4e97f0e6E-268435507",
        Greater,
    );
    test(
        "2.87837382928428281484661238899862431266472e348224893",
        "0x1.e4c036c3f1c4d9ba54be98804dca83d194E+289194514",
        "3/2",
        100,
        Nearest,
        "1977525149.894215593854629356572",
        "0x75dea39d.e4eb502b2dc93902a",
        Less,
    );
    test(
        "3.63323538031774406156699255740046189212848e-323228558",
        "0x3.ebb5899d7d24f35dec66a72a7a182a71dcE-268435507",
        "3/2",
        100,
        Nearest,
        "8.9606609981062037589492136427526e-323228558",
        "0x9.ab7388bd4f045fc17f5d18a3E-268435507",
        Less,
    );
    test(
        "1.7683784025018106051796817916100522576297151e375104585",
        "0x1.317e0044ad41fb2f507f88f707c5a31aa02E+311517615",
        "3/2",
        100,
        Nearest,
        "2130171521.6254124572164465177525",
        "0x7ef7d681.a01b07e241716cf328",
        Greater,
    );
    test(
        "3.5929308811198676247891575666758577820815706e-323228558",
        "0x3.e0931fcfd06e82e2e832677a7e1b1d65b44E-268435507",
        "3/2",
        100,
        Nearest,
        "8.861257872185132989289053994210e-323228558",
        "0x9.8ffd8cca3db948fb0cbf48eeE-268435507",
        Greater,
    );
    test(
        "5.678007121713359312120036055054658814207284e332322239",
        "0x1.bc4a337ed5c7974cbd3639cb5732c44d4ccE+275987646",
        "3/2",
        100,
        Nearest,
        "1887215989.801705049549243804127",
        "0x707ca175.cd3c8ac8da20b6dae",
        Less,
    );
    test(
        "3.4307675618003775353987480479135024086148920e-323228558",
        "0x3.b3c69db39877d9381e302b6a00bfc2ac3acE-268435507",
        "3/2",
        100,
        Nearest,
        "8.461313916277019686050722778505e-323228558",
        "0x9.2180ba76a2f0fec1d0c035eaE-268435507",
        Less,
    );
    test(
        "1.8552467812433583124583036080231538689057756e347269515",
        "0x9.a21b457c98d3fb8a75575a12111b3f1f1ccE+288401089",
        "3/2",
        100,
        Nearest,
        "1972099677.9209394329364892453840",
        "0x758bda5d.ebc2afca0f1b0db398",
        Greater,
    );
    test(
        "3.2804515707162404906363860930380166486178600e-323228558",
        "0x3.8a3ff95daff2c345e23d7d197bef23b5044E-268435507",
        "3/2",
        100,
        Nearest,
        "8.0905890670156676573131088365108e-323228558",
        "0x8.bb1656e03c4b7b480c54a6c5E-268435507",
        Greater,
    );
    test(
        "1.6855790930361163898651936263240787930568149e376901452",
        "0x8.e9afc949374ade11e5d0f8242cba6ab2c1E+313009880",
        "3/2",
        100,
        Nearest,
        "2140375702.0532765813259182799138",
        "0x7f938a96.0da388b6700037dba8",
        Less,
    );
    test(
        "5.3882543438845209971450181501493677030683865e-323228558",
        "0x5.d08bec69929af60daac152717c9d672d2b4E-268435507",
        "3/2",
        100,
        Nearest,
        "1.3289070344487242330778345868924e-323228557",
        "0xe.5735239ba8676c1da19d19b3E-268435507",
        Less,
    );
    test(
        "5.30336969728314415656580878858003507497469520e370461532",
        "0x3.19d9a53f1fc0a554a9f0fe2b44fd28de059cE+307661643",
        "3/2",
        100,
        Nearest,
        "2103804213.2881193784853815506408",
        "0x7d658135.49c2310bf045740f08",
        Greater,
    );
    test(
        "6.0040581567819351602415406414587535243655676e-323228558",
        "0x6.7aaac721c3fb6aa6682d2276c149e6eb882E-268435507",
        "3/2",
        100,
        Nearest,
        "1.480782942038074319961020513820e-323228557",
        "0xf.fac6c313fb0ad19cb1aba3d8E-268435507",
        Greater,
    );
    test(
        "9.8860958621134184535206299456365862147112541e373586576",
        "0x3.fabe1378685c157ee68f3024a05b892303cE+310256936",
        "3/2",
        100,
        Nearest,
        "2121550944.654749873837489595223",
        "0x7e744c60.a79db00f3130568af",
        Less,
    );
    test(
        "3.46820979949122787861940043969732941565517186e-323228558",
        "0x3.be1e9ac4ba1e951334eef906d999bf665879E-268435507",
        "3/2",
        100,
        Nearest,
        "8.5536578367330852179215817143971e-323228558",
        "0x9.3b0378c0458bdcbd75ec061fE-268435507",
        Less,
    );
    test(
        "3.34982613842697026637139663837385664581618e331259894",
        "0x5.090d8e3d323860965ecf607d3202141c5E+275105387",
        "3/2",
        100,
        Floor,
        "1881183065.5391910017583050951963",
        "0x70209359.8a086be6d972347bf8",
        Less,
    );
    test(
        "5.66600674705868018722616435471666120078076e-323228558",
        "0x6.1d471e5e0605150169e4e6958bf4e1c248E-268435507",
        "3/2",
        100,
        Floor,
        "1.3974092058119045735634190350269e-323228557",
        "0xf.147338221eaf9cf03a6e6ad9E-268435507",
        Less,
    );
    test(
        "3.34982613842697026637139663837385664581618e331259894",
        "0x5.090d8e3d323860965ecf607d3202141c5E+275105387",
        "3/2",
        100,
        Ceiling,
        "1881183065.5391910017583050952",
        "0x70209359.8a086be6d972347c",
        Greater,
    );
    test(
        "5.66600674705868018722616435471666120078076e-323228558",
        "0x6.1d471e5e0605150169e4e6958bf4e1c248E-268435507",
        "3/2",
        100,
        Ceiling,
        "1.397409205811904573563419035028e-323228557",
        "0xf.147338221eaf9cf03a6e6adaE-268435507",
        Greater,
    );
    test(
        "4.14541492611627230681793081575799673027890e360647729",
        "0x2.977e8a37bd3876dee43c828eb59b62e634E+299511456",
        "3/2",
        100,
        Floor,
        "2048072866.033224089513280197535",
        "0x7a131ca2.08815fb9e61f4b391",
        Less,
    );
    test(
        "4.42007972103839799911808660623986607789072e-323228558",
        "0x4.c514b6baa4153cccc5b7b57bf700baac78E-268435507",
        "3/2",
        100,
        Floor,
        "1.090125791997685328183866690665e-323228557",
        "0xb.c38e5b87827847c2127023b4E-268435507",
        Less,
    );
    test(
        "4.14541492611627230681793081575799673027890e360647729",
        "0x2.977e8a37bd3876dee43c828eb59b62e634E+299511456",
        "3/2",
        100,
        Ceiling,
        "2048072866.0332240895132801975362",
        "0x7a131ca2.08815fb9e61f4b3918",
        Greater,
    );
    test(
        "4.42007972103839799911808660623986607789072e-323228558",
        "0x4.c514b6baa4153cccc5b7b57bf700baac78E-268435507",
        "3/2",
        100,
        Ceiling,
        "1.0901257919976853281838666906660e-323228557",
        "0xb.c38e5b87827847c2127023b5E-268435507",
        Greater,
    );
    test(
        "9.6597149363711507340076378259479604149836461e351778945",
        "0xe.7e4aa213ec1f6edf370dbb493ce1cc1e2fE+292146090",
        "3/2",
        100,
        Floor,
        "1997708164.9108400002503305629729",
        "0x77129b84.e92ccf6cf6bbb34ee8",
        Less,
    );
    test(
        "5.7891358785928495415296027779506238660512289e-323228558",
        "0x6.3f4b0d6fccda48a986d1850245eccbfe0a8E-268435507",
        "3/2",
        100,
        Floor,
        "1.4277765861541170692027162228509e-323228557",
        "0xf.6857a0703300260015686259E-268435507",
        Less,
    );
    test(
        "9.6597149363711507340076378259479604149836461e351778945",
        "0xe.7e4aa213ec1f6edf370dbb493ce1cc1e2fE+292146090",
        "3/2",
        100,
        Ceiling,
        "1997708164.910840000250330562975",
        "0x77129b84.e92ccf6cf6bbb34ef",
        Greater,
    );
    test(
        "5.7891358785928495415296027779506238660512289e-323228558",
        "0x6.3f4b0d6fccda48a986d1850245eccbfe0a8E-268435507",
        "3/2",
        100,
        Ceiling,
        "1.427776586154117069202716222852e-323228557",
        "0xf.6857a070330026001568625aE-268435507",
        Greater,
    );
    test(
        "3.7894676553657125767187004475392317584593705e351676698",
        "0x2.818c4b9f54ed4f3329363b7b907eaf38c44E+292061176",
        "3/2",
        100,
        Floor,
        "1997127514.815347330326883384115",
        "0x7709bf5a.d0ba9a46a2870b8a5",
        Less,
    );
    test(
        "4.7005525095100570050619618016942234761365724e-323228558",
        "0x5.12904cc6e05563fa9605789c44faad16388E-268435507",
        "3/2",
        100,
        Floor,
        "1.159298892928687842172525140068e-323228557",
        "0xc.82a6eea96ebaa59d9e2c46eaE-268435507",
        Less,
    );
    test(
        "3.7894676553657125767187004475392317584593705e351676698",
        "0x2.818c4b9f54ed4f3329363b7b907eaf38c44E+292061176",
        "3/2",
        100,
        Ceiling,
        "1997127514.8153473303268833841163",
        "0x7709bf5a.d0ba9a46a2870b8a58",
        Greater,
    );
    test(
        "4.7005525095100570050619618016942234761365724e-323228558",
        "0x5.12904cc6e05563fa9605789c44faad16388E-268435507",
        "3/2",
        100,
        Ceiling,
        "1.1592988929286878421725251400696e-323228557",
        "0xc.82a6eea96ebaa59d9e2c46ebE-268435507",
        Greater,
    );
    test(
        "2.7440211526139244807414518642359343226926854e337547486",
        "0x5.e3dd1509dbbf5b401ecff3d82276031a356E+280327119",
        "3/2",
        100,
        Floor,
        "1916889505.1835177479034270378280",
        "0x724169a1.2efb04e57b152a1908",
        Less,
    );
    test(
        "3.851023004242611699476442791089945778882943e-323228558",
        "0x4.27dfe7e2e32e10dec3ec57cbab87228dd6E-268435507",
        "3/2",
        100,
        Floor,
        "9.4977913690548410046827477739103e-323228558",
        "0xa.3fd672c20f2792971fa42a71E-268435507",
        Less,
    );
    test(
        "2.7440211526139244807414518642359343226926854e337547486",
        "0x5.e3dd1509dbbf5b401ecff3d82276031a356E+280327119",
        "3/2",
        100,
        Ceiling,
        "1916889505.183517747903427037830",
        "0x724169a1.2efb04e57b152a191",
        Greater,
    );
    test(
        "3.851023004242611699476442791089945778882943e-323228558",
        "0x4.27dfe7e2e32e10dec3ec57cbab87228dd6E-268435507",
        "3/2",
        100,
        Ceiling,
        "9.497791369054841004682747773922e-323228558",
        "0xa.3fd672c20f2792971fa42a72E-268435507",
        Greater,
    );
    test(
        "6.4733712955530143117478909754814119152474780e352714064",
        "0x2.90361284986598c4dfa39bbfc975d3d11afE+292922690",
        "3/2",
        100,
        Floor,
        "2003018586.513711932246852758798",
        "0x7763a35a.8382a00c90b0f928f",
        Less,
    );
    test(
        "5.5601281780499454919922058177488474773550107e-323228558",
        "0x6.00072dab9021d62650837e1571f02b3d302E-268435507",
        "3/2",
        100,
        Floor,
        "1.371296337678134140029510065378e-323228557",
        "0xe.cc4fafb6192e517f7259f374E-268435507",
        Less,
    );
    test(
        "6.4733712955530143117478909754814119152474780e352714064",
        "0x2.90361284986598c4dfa39bbfc975d3d11afE+292922690",
        "3/2",
        100,
        Ceiling,
        "2003018586.5137119322468527587993",
        "0x7763a35a.8382a00c90b0f928f8",
        Greater,
    );
    test(
        "5.5601281780499454919922058177488474773550107e-323228558",
        "0x6.00072dab9021d62650837e1571f02b3d302E-268435507",
        "3/2",
        100,
        Ceiling,
        "1.3712963376781341400295100653795e-323228557",
        "0xe.cc4fafb6192e517f7259f375E-268435507",
        Greater,
    );
    test(
        "4.19859821611494699587265256555658820245319491e365334349",
        "0x1.0744ccf9b8cee9957daca67f6f6312a974238E+303403610",
        "3/2",
        100,
        Floor,
        "2074687588.5962240468317697938900",
        "0x7ba93864.98a2239e3b31799978",
        Less,
    );
    test(
        "3.98027850273323183914057535106224657438174430e-323228558",
        "0x4.4b951bbdf0f0fd9b6cf0b97290eb67eea824E-268435507",
        "3/2",
        100,
        Floor,
        "9.8165746525134490948859384380021e-323228558",
        "0xa.97e76d84589a7aacf0ab5d37E-268435507",
        Less,
    );
    test(
        "4.19859821611494699587265256555658820245319491e365334349",
        "0x1.0744ccf9b8cee9957daca67f6f6312a974238E+303403610",
        "3/2",
        100,
        Ceiling,
        "2074687588.59622404683176979389",
        "0x7ba93864.98a2239e3b3179998",
        Greater,
    );
    test(
        "3.98027850273323183914057535106224657438174430e-323228558",
        "0x4.4b951bbdf0f0fd9b6cf0b97290eb67eea824E-268435507",
        "3/2",
        100,
        Ceiling,
        "9.816574652513449094885938438014e-323228558",
        "0xa.97e76d84589a7aacf0ab5d38E-268435507",
        Greater,
    );
    test(
        "3.46026071573307993736482047463859861687893600e370641675",
        "0x8.a17b13c5e301b698469608b45cf55ca7c684E+307811248",
        "3/2",
        100,
        Floor,
        "2104827221.559642708785820333214",
        "0x7d751d55.8f44be9bae11375e9",
        Less,
    );
    test(
        "3.56454291475349509032740392292969403339860078e-323228558",
        "0x3.d8bb78b84db769b1165afed89db54ad4d2acE-268435507",
        "3/2",
        100,
        Floor,
        "8.791244532445922717122687218421e-323228558",
        "0x9.7ca611b3c8dccfde3d8d30daE-268435507",
        Less,
    );
    test(
        "3.46026071573307993736482047463859861687893600e370641675",
        "0x8.a17b13c5e301b698469608b45cf55ca7c684E+307811248",
        "3/2",
        100,
        Ceiling,
        "2104827221.5596427087858203332158",
        "0x7d751d55.8f44be9bae11375e98",
        Greater,
    );
    test(
        "3.56454291475349509032740392292969403339860078e-323228558",
        "0x3.d8bb78b84db769b1165afed89db54ad4d2acE-268435507",
        "3/2",
        100,
        Ceiling,
        "8.7912445324459227171226872184324e-323228558",
        "0x9.7ca611b3c8dccfde3d8d30dbE-268435507",
        Greater,
    );
}

#[test]
#[should_panic]
fn log_base_rational_base_1_plus_x_prec_round_fail_1() {
    BigExponentFloat::from(5).log_base_rational_base_1_plus_x_prec_round(
        &Rational::from_unsigneds(3u8, 2),
        0,
        Floor,
    );
}

#[test]
#[should_panic]
fn log_base_rational_base_1_plus_x_prec_round_fail_2() {
    BigExponentFloat::from(core::f64::consts::PI).log_base_rational_base_1_plus_x_prec_round(
        &Rational::from_unsigneds(3u8, 2),
        10,
        Exact,
    );
}

#[test]
#[should_panic]
fn log_base_rational_base_1_plus_x_prec_round_fail_3() {
    BigExponentFloat::from(5).log_base_rational_base_1_plus_x_prec_round(&Rational::ONE, 10, Floor);
}

fn log_base_rational_base_1_plus_x_prec_round_properties_helper(
    x: &BigExponentFloat,
    b: &Rational,
    prec: u64,
    rm: RoundingMode,
) {
    let r = log_base_rational_base_1_plus_x_prec_round_all(x, b, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else if !r.0.is_nan() {
        // The result lies between the results rounded down and up.
        let floor = log_base_rational_base_1_plus_x_prec_round_all(x, b, prec, Floor);
        let ceiling = log_base_rational_base_1_plus_x_prec_round_all(x, b, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0);
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(
                &log_base_rational_base_1_plus_x_prec_round_all(x, b, prec, Exact),
                &r
            ));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = log_base_rational_base_1_plus_x_prec_round_all(x, b, prec + 16, rm);
            let (z, oz) = y.round_to_prec_round(prec, rm);
            assert!(same(&z, &r.0));
            assert_eq!(if o == Equal { oz } else { o }, r.1);
        }
    }
    // log_b(1 + x) is log_b of the exact sum 1 + x.
    if x.is_finite() && exponent_is_moderate(x) {
        let z = x + BigExponentFloat::ONE;
        assert!(same_pair(
            &z.log_base_rational_base_prec_round_ref(b, prec, rm),
            &r
        ));
    }
}

#[test]
fn log_base_rational_base_1_plus_x_prec_round_properties() {
    big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_14().test_properties(
        |(x, b, prec, rm)| {
            log_base_rational_base_1_plus_x_prec_round_properties_helper(&x, &b, prec, rm);
        },
    );
}

#[test]
fn log_base_rational_base_1_plus_x_prec_round_float_agreement_properties() {
    float_rational_unsigned_rounding_mode_quadruple_gen_var_13().test_properties(
        |(x, b, prec, rm)| {
            if rm == Exact {
                return;
            }
            let (f, fo) = x.log_base_rational_base_1_plus_x_prec_round_ref(&b, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = log_base_rational_base_1_plus_x_prec_round_all(
                    &float_to_big_exponent_float(&x),
                    &b,
                    prec,
                    rm,
                );
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
}
