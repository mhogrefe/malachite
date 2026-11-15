// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::conversion::traits::FromStringBase;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::float_to_big_exponent_float;
use malachite_float::test_util::generators::{
    big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_4,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_5,
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_1,
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

fn log_base_prec_round_all(
    x: &BigExponentFloat,
    b: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().log_base_prec_round(b, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(&x.log_base_prec_round_ref(b, prec, rm), &r));
    let mut z = x.clone();
    let o = z.log_base_prec_round_assign(b, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().log_base_prec(b, prec), &r));
        assert!(same_pair(&x.log_base_prec_ref(b, prec), &r));
        let mut z = x.clone();
        let o = z.log_base_prec_assign(b, prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

fn log_base_rational_prec_round_all(
    x: &Rational,
    b: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = BigExponentFloat::log_base_rational_prec_round(x.clone(), b, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &BigExponentFloat::log_base_rational_prec_round_ref(x, b, prec, rm),
        &r
    ));
    if rm == Nearest {
        assert!(same_pair(
            &BigExponentFloat::log_base_rational_prec(x.clone(), b, prec),
            &r
        ));
        assert!(same_pair(
            &BigExponentFloat::log_base_rational_prec_ref(x, b, prec),
            &r
        ));
    }
    r
}

#[test]
fn test_log_base_prec_round() {
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
        let r = log_base_prec_round_all(&hex(x_hex), b.parse::<u64>().unwrap(), prec, rm);
        assert_eq!(format!("{:?}", r.0), out_hex, "{x} {b} {prec} {rm}");
        assert_eq!(r.0.to_string(), out);
        assert_eq!(r.1, o, "{x} {b} {prec} {rm}");
    };
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3",
        1,
        Nearest,
        "1.0",
        "0x1.0",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3",
        10,
        Floor,
        "1.0410",
        "0x1.0a8",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3",
        10,
        Ceiling,
        "1.043",
        "0x1.0b",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3",
        10,
        Down,
        "1.0410",
        "0x1.0a8",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3",
        10,
        Up,
        "1.043",
        "0x1.0b",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3",
        10,
        Nearest,
        "1.0410",
        "0x1.0a8",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3",
        100,
        Nearest,
        "1.041978045992185829632054054954",
        "0x1.0abf12beafb95a5373307e18c",
        Less,
    );
    test("0.50", "0x0.8", "3", 1, Nearest, "-0.50", "-0x0.8", Greater);
    test(
        "0.50", "0x0.8", "3", 10, Floor, "-0.63184", "-0x0.a1c", Less,
    );
    test(
        "0.50", "0x0.8", "3", 10, Ceiling, "-0.6309", "-0x0.a18", Greater,
    );
    test(
        "0.50", "0x0.8", "3", 10, Down, "-0.6309", "-0x0.a18", Greater,
    );
    test("0.50", "0x0.8", "3", 10, Up, "-0.63184", "-0x0.a1c", Less);
    test(
        "0.50", "0x0.8", "3", 10, Nearest, "-0.6309", "-0x0.a18", Greater,
    );
    test(
        "0.50",
        "0x0.8",
        "3",
        100,
        Nearest,
        "-0.63092975357145743709952711434266",
        "-0x0.a1849cc1a9a9e94e043eaf779",
        Greater,
    );
    test("3.0", "0x3", "3", 1, Nearest, "1.0", "0x1.0", Equal);
    test("3.0", "0x3", "3", 10, Floor, "1.0", "0x1.0", Equal);
    test("3.0", "0x3", "3", 10, Ceiling, "1.0", "0x1.0", Equal);
    test("3.0", "0x3", "3", 10, Down, "1.0", "0x1.0", Equal);
    test("3.0", "0x3", "3", 10, Up, "1.0", "0x1.0", Equal);
    test("3.0", "0x3", "3", 10, Nearest, "1.0", "0x1.0", Equal);
    test("3.0", "0x3", "3", 100, Nearest, "1.0", "0x1.0", Equal);
    test("3.0", "0x3", "3", 300, Exact, "1.0", "0x1.0", Equal);
    test("9.00", "0x9", "3", 1, Nearest, "2.0", "0x2.0", Equal);
    test("9.00", "0x9", "3", 10, Floor, "2.0", "0x2.0", Equal);
    test("9.00", "0x9", "3", 10, Ceiling, "2.0", "0x2.0", Equal);
    test("9.00", "0x9", "3", 10, Down, "2.0", "0x2.0", Equal);
    test("9.00", "0x9", "3", 10, Up, "2.0", "0x2.0", Equal);
    test("9.00", "0x9", "3", 10, Nearest, "2.0", "0x2.0", Equal);
    test("9.00", "0x9", "3", 100, Nearest, "2.0", "0x2.0", Equal);
    test("9.00", "0x9", "3", 300, Exact, "2.0", "0x2.0", Equal);
    test("27.0", "0x1b", "3", 1, Nearest, "4.0", "0x4.0", Greater);
    test("27.0", "0x1b", "3", 10, Floor, "3.0", "0x3.0", Equal);
    test("27.0", "0x1b", "3", 10, Ceiling, "3.0", "0x3.0", Equal);
    test("27.0", "0x1b", "3", 10, Down, "3.0", "0x3.0", Equal);
    test("27.0", "0x1b", "3", 10, Up, "3.0", "0x3.0", Equal);
    test("27.0", "0x1b", "3", 10, Nearest, "3.0", "0x3.0", Equal);
    test("27.0", "0x1b", "3", 100, Nearest, "3.0", "0x3.0", Equal);
    test("27.0", "0x1b", "3", 300, Exact, "3.0", "0x3.0", Equal);
    test(
        "2.4e24",
        "0x2.0E+20",
        "3",
        1,
        Nearest,
        "64.0",
        "0x4.0E+1",
        Greater,
    );
    test(
        "2.4e24",
        "0x2.0E+20",
        "3",
        10,
        Floor,
        "51.062",
        "0x33.1",
        Less,
    );
    test(
        "2.4e24",
        "0x2.0E+20",
        "3",
        10,
        Ceiling,
        "51.12",
        "0x33.2",
        Greater,
    );
    test(
        "2.4e24",
        "0x2.0E+20",
        "3",
        10,
        Down,
        "51.062",
        "0x33.1",
        Less,
    );
    test(
        "2.4e24",
        "0x2.0E+20",
        "3",
        10,
        Up,
        "51.12",
        "0x33.2",
        Greater,
    );
    test(
        "2.4e24",
        "0x2.0E+20",
        "3",
        10,
        Nearest,
        "51.12",
        "0x33.2",
        Greater,
    );
    test(
        "2.4e24",
        "0x2.0E+20",
        "3",
        100,
        Nearest,
        "51.105310039288052405061696261749",
        "0x33.1af59946aec2d1af57d584d4",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "3",
        1,
        Nearest,
        "5.4e8",
        "0x2.0E+7",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "3",
        10,
        Floor,
        "7.571e8",
        "0x2.d2E+7",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "3",
        10,
        Ceiling,
        "7.5812e8",
        "0x2.d3E+7",
        Greater,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "3",
        10,
        Down,
        "7.571e8",
        "0x2.d2E+7",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "3",
        10,
        Up,
        "7.5812e8",
        "0x2.d3E+7",
        Greater,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "3",
        10,
        Nearest,
        "7.571e8",
        "0x2.d2E+7",
        Less,
    );
    test(
        "6.3e361235994",
        "0x1.0E+300000000",
        "3",
        100,
        Nearest,
        "757115704.28574892451943253721173",
        "0x2d20ab38.4926d76dad9a45cc4c",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3",
        1,
        Nearest,
        "-5.4e8",
        "-0x2.0E+7",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3",
        10,
        Floor,
        "-7.5812e8",
        "-0x2.d3E+7",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3",
        10,
        Ceiling,
        "-7.571e8",
        "-0x2.d2E+7",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3",
        10,
        Down,
        "-7.571e8",
        "-0x2.d2E+7",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3",
        10,
        Up,
        "-7.5812e8",
        "-0x2.d3E+7",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3",
        10,
        Nearest,
        "-7.571e8",
        "-0x2.d2E+7",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3",
        100,
        Nearest,
        "-757115703.9166786780908899743112",
        "-0x2d20ab37.eaab742f57442f1a5",
        Less,
    );
    test("-3.0", "-0x3", "3", 1, Nearest, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", "3", 10, Floor, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", "3", 10, Ceiling, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", "3", 10, Down, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", "3", 10, Up, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", "3", 10, Nearest, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", "3", 100, Nearest, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", "3", 300, Exact, "NaN", "NaN", Equal);
    test("1.0", "0x1.0", "3", 1, Nearest, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0", "3", 10, Floor, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0", "3", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0", "3", 10, Down, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0", "3", 10, Up, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0", "3", 10, Nearest, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0", "3", 100, Nearest, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0", "3", 300, Exact, "0.0", "0x0.0", Equal);
    test(
        "0.99999999977",
        "0x0.ffffffff",
        "3",
        1,
        Nearest,
        "-2.3e-10",
        "-0x1.0E-8",
        Less,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        "3",
        10,
        Floor,
        "-2.1214e-10",
        "-0xe.94E-9",
        Less,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        "3",
        10,
        Ceiling,
        "-2.119e-10",
        "-0xe.9E-9",
        Greater,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        "3",
        10,
        Down,
        "-2.119e-10",
        "-0xe.9E-9",
        Greater,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        "3",
        10,
        Up,
        "-2.1214e-10",
        "-0xe.94E-9",
        Less,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        "3",
        10,
        Nearest,
        "-2.119e-10",
        "-0xe.9E-9",
        Greater,
    );
    test(
        "0.99999999977",
        "0x0.ffffffff",
        "3",
        100,
        Nearest,
        "-2.1193158503919914042063796657436e-10",
        "-0xe.905701e5a8d4d9ad73367453E-9",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "3",
        1,
        Nearest,
        "16.0",
        "0x1.0E+1",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "3",
        10,
        Floor,
        "20.94",
        "0x14.f",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "3",
        10,
        Ceiling,
        "20.969",
        "0x14.f8",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "3",
        10,
        Down,
        "20.94",
        "0x14.f",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "3",
        10,
        Up,
        "20.969",
        "0x14.f8",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "3",
        10,
        Nearest,
        "20.969",
        "0x14.f8",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2540be400.0",
        "3",
        100,
        Nearest,
        "20.95903274289384604296567522023",
        "0x14.f5832b7a85b1dbc1c4d0f94c",
        Greater,
    );
    test(
        "3.5026945402797e464526652",
        "0x3.c2e44f14928E+385781034",
        "3",
        10,
        Nearest,
        "9.7413e8",
        "0x3.a1E+7",
        Greater,
    );
    test(
        "2.8549449245439e-464526653",
        "0x4.40fc7d4464E-385781035",
        "3",
        10,
        Nearest,
        "-9.7413e8",
        "-0x3.a1E+7",
        Less,
    );
    test(
        "4.044708000033e462025052",
        "0x6.f2ba546e9E+383703500",
        "3",
        10,
        Nearest,
        "9.6784e8",
        "0x3.9bE+7",
        Less,
    );
    test(
        "2.4723663611608e-462025053",
        "0x2.4d824b004f8E-383703501",
        "3",
        10,
        Nearest,
        "-9.6784e8",
        "-0x3.9bE+7",
        Greater,
    );
    test(
        "5.75412285683784e494044176",
        "0x7.6f8b7486146E+410294807",
        "3",
        10,
        Nearest,
        "1.036e9",
        "0x3.dcE+7",
        Greater,
    );
    test(
        "1.7378843394205e-494044177",
        "0x2.26dab9fb138E-410294808",
        "3",
        10,
        Nearest,
        "-1.036e9",
        "-0x3.dcE+7",
        Less,
    );
    test(
        "8.23533630697565e449517656",
        "0x3.26d7404860dE+373316333",
        "3",
        10,
        Nearest,
        "9.416e8",
        "0x3.82E+7",
        Less,
    );
    test(
        "1.214279493544133e-449517657",
        "0x5.139b65afbe3E-373316334",
        "3",
        10,
        Nearest,
        "-9.416e8",
        "-0x3.82E+7",
        Greater,
    );
    test(
        "3.599748236155563e481536726",
        "0x1.9692eb744c01E+399907595",
        "3",
        10,
        Nearest,
        "1.0098e9",
        "0x3.c3E+7",
        Greater,
    );
    test(
        "2.777972053589986e-481536727",
        "0xa.130dc9d1199E-399907596",
        "3",
        10,
        Nearest,
        "-1.0098e9",
        "-0x3.c3E+7",
        Less,
    );
    test(
        "2.432918049749782e454520638",
        "0x5.232776a7d89E+377471219",
        "3",
        10,
        Nearest,
        "9.521e8",
        "0x3.8cE+7",
        Less,
    );
    test(
        "4.110290521716694e-454520639",
        "0x3.1d4d7fd56046E-377471220",
        "3",
        10,
        Nearest,
        "-9.521e8",
        "-0x3.8cE+7",
        Greater,
    );
    test(
        "5.8869066114629985e457022128",
        "0x2.eb7e706e5180eE+379548662",
        "3",
        10,
        Nearest,
        "9.584e8",
        "0x3.92E+7",
        Greater,
    );
    test(
        "1.6986850072545700e-457022129",
        "0x5.7ac9e83255c5E-379548663",
        "3",
        10,
        Nearest,
        "-9.584e8",
        "-0x3.92E+7",
        Less,
    );
    test(
        "8.3843492687501551e509053109",
        "0xc.818e63d73381E+422759456",
        "3",
        10,
        Nearest,
        "1.0664e9",
        "0x3.f9E+7",
        Less,
    );
    test(
        "1.1926984050237077e-509053110",
        "0x1.47854e0063788E-422759457",
        "3",
        10,
        Nearest,
        "-1.0664e9",
        "-0x3.f9E+7",
        Greater,
    );
    test(
        "1.0887880213835e504800638",
        "0x3.82e1d3cf4aE+419227855",
        "3",
        10,
        Floor,
        "1.0580e9",
        "0x3.f1E+7",
        Less,
    );
    test(
        "9.1845242633110e-504800639",
        "0x4.8e888a3691E-419227856",
        "3",
        10,
        Floor,
        "-1.059e9",
        "-0x3.f2E+7",
        Less,
    );
    test(
        "1.0887880213835e504800638",
        "0x3.82e1d3cf4aE+419227855",
        "3",
        10,
        Ceiling,
        "1.059e9",
        "0x3.f2E+7",
        Greater,
    );
    test(
        "9.1845242633110e-504800639",
        "0x4.8e888a3691E-419227856",
        "3",
        10,
        Ceiling,
        "-1.0580e9",
        "-0x3.f1E+7",
        Greater,
    );
    test(
        "1.6597731008076e487790391",
        "0x2.038da1368a8E+405101151",
        "3",
        10,
        Floor,
        "1.021e9",
        "0x3.ceE+7",
        Less,
    );
    test(
        "6.0249199093146e-487790392",
        "0x7.f1e28edcf6E-405101152",
        "3",
        10,
        Floor,
        "-1.0224e9",
        "-0x3.cfE+7",
        Less,
    );
    test(
        "1.6597731008076e487790391",
        "0x2.038da1368a8E+405101151",
        "3",
        10,
        Ceiling,
        "1.0224e9",
        "0x3.cfE+7",
        Greater,
    );
    test(
        "6.0249199093146e-487790392",
        "0x7.f1e28edcf6E-405101152",
        "3",
        10,
        Ceiling,
        "-1.021e9",
        "-0x3.ceE+7",
        Greater,
    );
    test(
        "6.152668701207155e473782111",
        "0x1.0145d6810ef4E+393467527",
        "3",
        10,
        Floor,
        "9.9300e8",
        "0x3.b3E+7",
        Less,
    );
    test(
        "1.625310980589282e-473782112",
        "0xf.ebbc62c06f2E-393467528",
        "3",
        10,
        Floor,
        "-9.941e8",
        "-0x3.b4E+7",
        Less,
    );
    test(
        "6.152668701207155e473782111",
        "0x1.0145d6810ef4E+393467527",
        "3",
        10,
        Ceiling,
        "9.941e8",
        "0x3.b4E+7",
        Greater,
    );
    test(
        "1.625310980589282e-473782112",
        "0xf.ebbc62c06f2E-393467528",
        "3",
        10,
        Ceiling,
        "-9.9300e8",
        "-0x3.b3E+7",
        Greater,
    );
    test(
        "3.84298460738501e469779721",
        "0xb.a2b6bcecdacE+390143613",
        "3",
        10,
        Floor,
        "9.836e8",
        "0x3.aaE+7",
        Less,
    );
    test(
        "2.60214417221009e-469779722",
        "0x1.6005e84663dE-390143614",
        "3",
        10,
        Floor,
        "-9.8461e8",
        "-0x3.abE+7",
        Less,
    );
    test(
        "3.84298460738501e469779721",
        "0xb.a2b6bcecdacE+390143613",
        "3",
        10,
        Ceiling,
        "9.8461e8",
        "0x3.abE+7",
        Greater,
    );
    test(
        "2.60214417221009e-469779722",
        "0x1.6005e84663dE-390143614",
        "3",
        10,
        Ceiling,
        "-9.836e8",
        "-0x3.aaE+7",
        Greater,
    );
    test(
        "1.895754760406790e487790450",
        "0x2.4a4f88ae1be6E+405101200",
        "3",
        10,
        Floor,
        "1.0224e9",
        "0x3.cfE+7",
        Less,
    );
    test(
        "5.274943895092314e-487790451",
        "0x6.fc6e40459304E-405101201",
        "3",
        10,
        Floor,
        "-1.02e9",
        "-0x3.dE+7",
        Less,
    );
    test(
        "1.895754760406790e487790450",
        "0x2.4a4f88ae1be6E+405101200",
        "3",
        10,
        Ceiling,
        "1.02e9",
        "0x3.dE+7",
        Greater,
    );
    test(
        "5.274943895092314e-487790451",
        "0x6.fc6e40459304E-405101201",
        "3",
        10,
        Ceiling,
        "-1.0224e9",
        "-0x3.cfE+7",
        Greater,
    );
    test(
        "3.168928078041073e458773170",
        "0x1.3428b1ab8663E+381002871",
        "3",
        10,
        Floor,
        "9.605e8",
        "0x3.94E+7",
        Less,
    );
    test(
        "3.155641199083844e-458773171",
        "0xd.4ab61c3efefE-381002872",
        "3",
        10,
        Floor,
        "-9.6154e8",
        "-0x3.95E+7",
        Less,
    );
    test(
        "3.168928078041073e458773170",
        "0x1.3428b1ab8663E+381002871",
        "3",
        10,
        Ceiling,
        "9.6154e8",
        "0x3.95E+7",
        Greater,
    );
    test(
        "3.155641199083844e-458773171",
        "0xd.4ab61c3efefE-381002872",
        "3",
        10,
        Ceiling,
        "-9.605e8",
        "-0x3.94E+7",
        Greater,
    );
    test(
        "6.8960317856155259e485789257",
        "0xe.02e796dccbb68E+403439245",
        "3",
        10,
        Floor,
        "1.0182e9",
        "0x3.cbE+7",
        Less,
    );
    test(
        "1.4501093253164899e-485789258",
        "0x1.2455a23a46664E-403439246",
        "3",
        10,
        Floor,
        "-1.019e9",
        "-0x3.ccE+7",
        Less,
    );
    test(
        "6.8960317856155259e485789257",
        "0xe.02e796dccbb68E+403439245",
        "3",
        10,
        Ceiling,
        "1.019e9",
        "0x3.ccE+7",
        Greater,
    );
    test(
        "1.4501093253164899e-485789258",
        "0x1.2455a23a46664E-403439246",
        "3",
        10,
        Ceiling,
        "-1.0182e9",
        "-0x3.cbE+7",
        Greater,
    );
    test(
        "2.1613691791948307e453770192",
        "0x5.d0051b12e6f3E+376847987",
        "3",
        10,
        Floor,
        "9.500e8",
        "0x3.8aE+7",
        Less,
    );
    test(
        "4.6266968624607087e-453770193",
        "0x2.c0adc10bd95fE-376847988",
        "3",
        10,
        Floor,
        "-9.5106e8",
        "-0x3.8bE+7",
        Less,
    );
    test(
        "2.1613691791948307e453770192",
        "0x5.d0051b12e6f3E+376847987",
        "3",
        10,
        Ceiling,
        "9.5106e8",
        "0x3.8bE+7",
        Greater,
    );
    test(
        "4.6266968624607087e-453770193",
        "0x2.c0adc10bd95fE-376847988",
        "3",
        10,
        Ceiling,
        "-9.500e8",
        "-0x3.8aE+7",
        Greater,
    );
    test(
        "6.72122221649737220715585213401286399677676e482680192",
        "0x2.a330a884e786d5bf3827f767160d076bb4E+400857223",
        "3",
        100,
        Nearest,
        "1011650996.5816803297443382188322",
        "0x3c4c91b4.94e90088fa7a4d7d84",
        Greater,
    );
    test(
        "1.48782463633694526501044947645479344644159e-482680193",
        "0x6.110234d2421443db77d11b855703fc19f8E-400857224",
        "3",
        100,
        Nearest,
        "-1011650996.5816803297443382188322",
        "-0x3c4c91b4.94e90088fa7a4d7d84",
        Less,
    );
    test(
        "1.99073191473344888227153958310108719201417e493813051",
        "0x1.a945d7bfab488e9b6801173f6e310f28a2E+410102862",
        "3",
        100,
        Nearest,
        "1034984391.104432730851631117517",
        "0x3db09bc7.1abc1a7ba3c5133c6",
        Less,
    );
    test(
        "5.02327808480377951250423761992837368831811e-493813052",
        "0x9.a1a7a45875aace6d0b6a9755f7e052a84E-410102863",
        "3",
        100,
        Nearest,
        "-1034984391.104432730851631117517",
        "-0x3db09bc7.1abc1a7ba3c5133c6",
        Greater,
    );
    test(
        "6.4500319845347359898944001985996112436936594e508719764",
        "0x8.d91ffa2631b9e836c07bea3ed8627eaed5E+422482619",
        "3",
        100,
        Nearest,
        "1066227420.7600863739616777451007",
        "0x3f8d56dc.c29505464cf635dc84",
        Greater,
    );
    test(
        "1.550379908809294993528588947675962528609819e-508719765",
        "0x1.ceec03bd16e32427dab050c70da2168c224E-422482620",
        "3",
        100,
        Nearest,
        "-1066227420.7600863739616777451007",
        "-0x3f8d56dc.c29505464cf635dc84",
        Less,
    );
    test(
        "6.3444623973511885407418943850211902014408621e495827803",
        "0xc.cb71685d5dde295f31ba122c61f123d9a9E+411776077",
        "3",
        100,
        Nearest,
        "1039207117.4731539012827555900774",
        "0x3df10acd.79209d33fbf7a865a4",
        Less,
    );
    test(
        "1.576177676484456349098836667608463565042830e-495827804",
        "0x1.4021f067fd660ac433b3ff3dc03473d1838E-411776078",
        "3",
        100,
        Nearest,
        "-1039207117.4731539012827555900774",
        "-0x3df10acd.79209d33fbf7a865a4",
        Greater,
    );
    test(
        "7.6729291637895087398025991623249601847154695e487007519",
        "0x6.9f3bcc11148ea2895adc569194a36a1e122E+404450990",
        "3",
        100,
        Nearest,
        "1020720655.530442737458800812504",
        "0x3cd6f60f.87cb1861c94892df5",
        Greater,
    );
    test(
        "1.3032832424926488910964788374554048663943266e-487007520",
        "0x2.6a8b2de48c966a9258e38b1c4fa10dbbf36E-404450991",
        "3",
        100,
        Nearest,
        "-1020720655.530442737458800812504",
        "-0x3cd6f60f.87cb1861c94892df5",
        Less,
    );
    test(
        "1.663625692618951798246892294297064360171599e483803507",
        "0x8.3d78940562081bce483000ff8e0a8c6432E+401790115",
        "3",
        100,
        Nearest,
        "1014005354.8972983998411548095528",
        "0x3c707e6a.e5b559121225f20a3c",
        Less,
    );
    test(
        "6.0109675177338514231691573411732034771597408e-483803508",
        "0x1.f1147f927face5a309359d137511e82544b8E-401790116",
        "3",
        100,
        Nearest,
        "-1014005354.8972983998411548095528",
        "-0x3c707e6a.e5b559121225f20a3c",
        Greater,
    );
    test(
        "2.10498738660716215245727757273206451934494595e485633991",
        "0xc.33964e92b247c3d6bb35b3065c71d08d0bd4E+403310299",
        "3",
        100,
        Nearest,
        "1017841872.5206212152187865317498",
        "0x3cab08d0.85476e94f7ed5d3564",
        Greater,
    );
    test(
        "4.75062229048226797526247220387874684366404572e-485633992",
        "0x1.4fb2333231f7fc111e4c82c0510a6770c68cE-403310300",
        "3",
        100,
        Nearest,
        "-1017841872.5206212152187865317498",
        "-0x3cab08d0.85476e94f7ed5d3564",
        Less,
    );
    test(
        "1.32267860277297708648660009844319294314520082e484794898",
        "0x1.41494b8f6cd0ffb8c14ed32d3596c5fe40ecE+402613448",
        "3",
        100,
        Nearest,
        "1016083214.331544755322841947494",
        "0x3c90330e.54e01df9459d143d6",
        Less,
    );
    test(
        "7.56041564370599226122539847717340551479721167e-484794899",
        "0xc.bfae51571fa5157533fd892ca8435dd0929cE-402613449",
        "3",
        100,
        Nearest,
        "-1016083214.331544755322841947494",
        "-0x3c90330e.54e01df9459d143d6",
        Greater,
    );
    test(
        "5.13844737232890055225739560723734028056332e487514723",
        "0x2.93f906ed5db9e0e68e133533e2144ef484E+404872214",
        "3",
        100,
        Floor,
        "1021783705.6898172908543196850023",
        "0x3ce72e99.b097ddb06f4373113c",
        Less,
    );
    test(
        "1.94611314963564490319148038878878992414721e-487514724",
        "0x6.34d12c800e6576986922e6c362db57cd3E-404872215",
        "3",
        100,
        Floor,
        "-1021783705.68981729085431968500",
        "-0x3ce72e99.b097ddb06f4373114",
        Less,
    );
    test(
        "5.13844737232890055225739560723734028056332e487514723",
        "0x2.93f906ed5db9e0e68e133533e2144ef484E+404872214",
        "3",
        100,
        Ceiling,
        "1021783705.68981729085431968500",
        "0x3ce72e99.b097ddb06f4373114",
        Greater,
    );
    test(
        "1.94611314963564490319148038878878992414721e-487514724",
        "0x6.34d12c800e6576986922e6c362db57cd3E-404872215",
        "3",
        100,
        Ceiling,
        "-1021783705.6898172908543196850023",
        "-0x3ce72e99.b097ddb06f4373113c",
        Greater,
    );
    test(
        "1.60377648152401032814356473190254926431484e491706378",
        "0x1.250b843c5a3895be3f008838f578247624E+408353308",
        "3",
        100,
        Floor,
        "1030569008.069135474639570381450",
        "0x3d6d3c30.11b2dcca920538bd08",
        Less,
    );
    test(
        "6.23528285593598712001129534626845177147415e-491706379",
        "0xd.fa35527aaa11dfc56e359cf922f25b89eE-408353309",
        "3",
        100,
        Floor,
        "-1030569008.0691354746395703814508",
        "-0x3d6d3c30.11b2dcca920538bd0c",
        Less,
    );
    test(
        "1.60377648152401032814356473190254926431484e491706378",
        "0x1.250b843c5a3895be3f008838f578247624E+408353308",
        "3",
        100,
        Ceiling,
        "1030569008.0691354746395703814508",
        "0x3d6d3c30.11b2dcca920538bd0c",
        Greater,
    );
    test(
        "6.23528285593598712001129534626845177147415e-491706379",
        "0xd.fa35527aaa11dfc56e359cf922f25b89eE-408353309",
        "3",
        100,
        Ceiling,
        "-1030569008.069135474639570381450",
        "-0x3d6d3c30.11b2dcca920538bd08",
        Greater,
    );
    test(
        "1.6295936508461404803538130605241361334001055e459584565",
        "0x8.e7a7870cf9164bb6fd59a572420fe8d681E+381676719",
        "3",
        100,
        Floor,
        "963244795.04086025688714507479339",
        "0x3969f2fb.0a75d15b095494d3a4",
        Less,
    );
    test(
        "6.13649911731532562684204159064229219316224e-459584566",
        "0x1.cbf8b29df039b367c1249e9cbdda84845aE-381676720",
        "3",
        100,
        Floor,
        "-963244795.0408602568871450747942",
        "-0x3969f2fb.0a75d15b095494d3a8",
        Less,
    );
    test(
        "1.6295936508461404803538130605241361334001055e459584565",
        "0x8.e7a7870cf9164bb6fd59a572420fe8d681E+381676719",
        "3",
        100,
        Ceiling,
        "963244795.0408602568871450747942",
        "0x3969f2fb.0a75d15b095494d3a8",
        Greater,
    );
    test(
        "6.13649911731532562684204159064229219316224e-459584566",
        "0x1.cbf8b29df039b367c1249e9cbdda84845aE-381676720",
        "3",
        100,
        Ceiling,
        "-963244795.04086025688714507479339",
        "-0x3969f2fb.0a75d15b095494d3a4",
        Greater,
    );
    test(
        "2.2641813145276546008084136755121682130609205e512265168",
        "0x6.eb0b30e42d93a09390025ff3bbad17c9628E+425427013",
        "3",
        100,
        Floor,
        "1073658243.659461235285406543580",
        "0x3ffeb983.a8d2739687d43e5e6",
        Less,
    );
    test(
        "4.4166074226640123103969507497279357448941199e-512265169",
        "0x2.501112f6c9646d83ab39f31fc3e212c526cE-425427014",
        "3",
        100,
        Floor,
        "-1073658243.6594612352854065435813",
        "-0x3ffeb983.a8d2739687d43e5e64",
        Less,
    );
    test(
        "2.2641813145276546008084136755121682130609205e512265168",
        "0x6.eb0b30e42d93a09390025ff3bbad17c9628E+425427013",
        "3",
        100,
        Ceiling,
        "1073658243.6594612352854065435813",
        "0x3ffeb983.a8d2739687d43e5e64",
        Greater,
    );
    test(
        "4.4166074226640123103969507497279357448941199e-512265169",
        "0x2.501112f6c9646d83ab39f31fc3e212c526cE-425427014",
        "3",
        100,
        Ceiling,
        "-1073658243.659461235285406543580",
        "-0x3ffeb983.a8d2739687d43e5e6",
        Greater,
    );
    test(
        "7.6874160144947386291966945002723686531020758e452216330",
        "0x5.cd1c0284f903d3cc43a11ba2aaf047e6996E+375557533",
        "3",
        100,
        Floor,
        "947801688.59063887108092902821718",
        "0x387e4e58.97341beb09f8a65c0c",
        Less,
    );
    test(
        "1.3008272195943148452270247042960716871388902e-452216331",
        "0x2.c20f4c9b482f926a729abeb28eee8e230baE-375557534",
        "3",
        100,
        Floor,
        "-947801688.5906388710809290282180",
        "-0x387e4e58.97341beb09f8a65c1",
        Less,
    );
    test(
        "7.6874160144947386291966945002723686531020758e452216330",
        "0x5.cd1c0284f903d3cc43a11ba2aaf047e6996E+375557533",
        "3",
        100,
        Ceiling,
        "947801688.5906388710809290282180",
        "0x387e4e58.97341beb09f8a65c1",
        Greater,
    );
    test(
        "1.3008272195943148452270247042960716871388902e-452216331",
        "0x2.c20f4c9b482f926a729abeb28eee8e230baE-375557534",
        "3",
        100,
        Ceiling,
        "-947801688.59063887108092902821718",
        "-0x387e4e58.97341beb09f8a65c0c",
        Greater,
    );
    test(
        "5.9112610565158055581723324805541459670272581e495024540",
        "0x3.2263b338e20e7e6ce72eaa0e03b140dc427E+411108982",
        "3",
        100,
        Floor,
        "1037523555.856963371362318049114",
        "0x3dd75a63.db61f395defdb8c908",
        Less,
    );
    test(
        "1.6916864108001625612226509673073909586730310e-495024541",
        "0x5.1ad118572536310d0e0041d8d69832a78acE-411108983",
        "3",
        100,
        Floor,
        "-1037523555.8569633713623180491145",
        "-0x3dd75a63.db61f395defdb8c90c",
        Less,
    );
    test(
        "5.9112610565158055581723324805541459670272581e495024540",
        "0x3.2263b338e20e7e6ce72eaa0e03b140dc427E+411108982",
        "3",
        100,
        Ceiling,
        "1037523555.8569633713623180491145",
        "0x3dd75a63.db61f395defdb8c90c",
        Greater,
    );
    test(
        "1.6916864108001625612226509673073909586730310e-495024541",
        "0x5.1ad118572536310d0e0041d8d69832a78acE-411108983",
        "3",
        100,
        Ceiling,
        "-1037523555.856963371362318049114",
        "-0x3dd75a63.db61f395defdb8c908",
        Greater,
    );
    test(
        "1.23409639701956236750021712021343345253834348e467720233",
        "0x7.3a23b464857c90009040f30127a7b03cbcc6E+388433245",
        "3",
        100,
        Floor,
        "980296367.98755272160362173775164",
        "0x3a6e22af.fcd041525d046087b4",
        Less,
    );
    test(
        "8.10309472108562057747994036615152810479597196e-467720234",
        "0x2.36c14ee7557f88ceda6ddcd88b18fb0b465cE-388433246",
        "3",
        100,
        Floor,
        "-980296367.9875527216036217377525",
        "-0x3a6e22af.fcd041525d046087b8",
        Less,
    );
    test(
        "1.23409639701956236750021712021343345253834348e467720233",
        "0x7.3a23b464857c90009040f30127a7b03cbcc6E+388433245",
        "3",
        100,
        Ceiling,
        "980296367.9875527216036217377525",
        "0x3a6e22af.fcd041525d046087b8",
        Greater,
    );
    test(
        "8.10309472108562057747994036615152810479597196e-467720234",
        "0x2.36c14ee7557f88ceda6ddcd88b18fb0b465cE-388433246",
        "3",
        100,
        Ceiling,
        "-980296367.98755272160362173775164",
        "-0x3a6e22af.fcd041525d046087b4",
        Greater,
    );
    test(
        "4.24003072515066026040303325839490390843174742e469490169",
        "0x1.b11c39504871018b3a74adf5783f2146962dE+389903146",
        "3",
        100,
        Floor,
        "984005983.7686812818099181405752",
        "0x3aa6bd5f.c4c84be66bc3e901f8",
        Less,
    );
    test(
        "2.35847347536489178909142894443750868846129637e-469490170",
        "0x9.75098207edbde76efee3e8ae080cf4669ce8E-389903147",
        "3",
        100,
        Floor,
        "-984005983.76868128180991814057609",
        "-0x3aa6bd5f.c4c84be66bc3e901fc",
        Less,
    );
    test(
        "4.24003072515066026040303325839490390843174742e469490169",
        "0x1.b11c39504871018b3a74adf5783f2146962dE+389903146",
        "3",
        100,
        Ceiling,
        "984005983.76868128180991814057609",
        "0x3aa6bd5f.c4c84be66bc3e901fc",
        Greater,
    );
    test(
        "2.35847347536489178909142894443750868846129637e-469490170",
        "0x9.75098207edbde76efee3e8ae080cf4669ce8E-389903147",
        "3",
        100,
        Ceiling,
        "-984005983.7686812818099181405752",
        "-0x3aa6bd5f.c4c84be66bc3e901f8",
        Greater,
    );
}

#[test]
#[should_panic]
fn log_base_prec_round_fail_1() {
    BigExponentFloat::from(5).log_base_prec_round(3, 0, Floor);
}

#[test]
#[should_panic]
fn log_base_prec_round_fail_2() {
    BigExponentFloat::from(core::f64::consts::PI).log_base_prec_round(3, 10, Exact);
}

#[test]
#[should_panic]
fn log_base_prec_round_fail_3() {
    BigExponentFloat::from(5).log_base_prec_round(1, 10, Floor);
}

fn log_base_prec_round_properties_helper(
    x: &BigExponentFloat,
    b: u64,
    prec: u64,
    rm: RoundingMode,
) {
    let r = log_base_prec_round_all(x, b, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else if !r.0.is_nan() {
        // The result lies between the results rounded down and up.
        let floor = log_base_prec_round_all(x, b, prec, Floor);
        let ceiling = log_base_prec_round_all(x, b, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0);
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(&log_base_prec_round_all(x, b, prec, Exact), &r));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = log_base_prec_round_all(x, b, prec + 16, rm);
            let (z, oz) = y.round_to_prec_round(prec, rm);
            assert!(same(&z, &r.0));
            assert_eq!(if o == Equal { oz } else { o }, r.1);
        }
    }
    // The base-2 logarithm.
    assert!(same_pair(
        &x.log_base_prec_round_ref(2, prec, rm),
        &x.log_base_2_prec_round_ref(prec, rm)
    ));
}

#[test]
fn log_base_prec_round_properties() {
    big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_4().test_properties(
        |(x, b, prec, rm)| {
            log_base_prec_round_properties_helper(&x, b, prec, rm);
        },
    );
}

#[test]
fn log_base_prec_round_float_agreement_properties() {
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_5().test_properties(
        |(x, b, prec, rm)| {
            if rm == Exact {
                return;
            }
            let (f, fo) = x.log_base_prec_round_ref(b, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = log_base_prec_round_all(&float_to_big_exponent_float(&x), b, prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
}

#[test]
fn test_log_base_rational_prec_round() {
    // The expected values agree with FLINT's arb (see the `arf_logarithm_general` oracle mode).
    let test =
        |x: &str, b: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
            let r = log_base_rational_prec_round_all(
                &Rational::from_str(x).unwrap(),
                b.parse::<u64>().unwrap(),
                prec,
                rm,
            );
            assert_eq!(format!("{:?}", r.0), out_hex, "{x} {b} {prec} {rm}");
            assert_eq!(r.0.to_string(), out);
            assert_eq!(r.1, o, "{x} {b} {prec} {rm}");
        };
    test("3/5", "3", 1, Nearest, "-0.50", "-0x0.8", Less);
    test("3/5", "3", 10, Floor, "-0.46533", "-0x0.772", Less);
    test("3/5", "3", 10, Ceiling, "-0.4648", "-0x0.77", Greater);
    test("3/5", "3", 10, Down, "-0.4648", "-0x0.77", Greater);
    test("3/5", "3", 10, Up, "-0.46533", "-0x0.772", Less);
    test("3/5", "3", 10, Nearest, "-0.4648", "-0x0.77", Greater);
    test(
        "3/5",
        "3",
        100,
        Nearest,
        "-0.46497352071792716719704040767853",
        "-0x0.77088130fd4e46455c3c9ca988",
        Greater,
    );
    test("9", "3", 1, Nearest, "2.0", "0x2.0", Equal);
    test("9", "3", 10, Floor, "2.0", "0x2.0", Equal);
    test("9", "3", 10, Ceiling, "2.0", "0x2.0", Equal);
    test("9", "3", 10, Down, "2.0", "0x2.0", Equal);
    test("9", "3", 10, Up, "2.0", "0x2.0", Equal);
    test("9", "3", 10, Nearest, "2.0", "0x2.0", Equal);
    test("9", "3", 100, Nearest, "2.0", "0x2.0", Equal);
    test("9", "3", 300, Exact, "2.0", "0x2.0", Equal);
    test("1/9", "3", 1, Nearest, "-2.0", "-0x2.0", Equal);
    test("1/9", "3", 10, Floor, "-2.0", "-0x2.0", Equal);
    test("1/9", "3", 10, Ceiling, "-2.0", "-0x2.0", Equal);
    test("1/9", "3", 10, Down, "-2.0", "-0x2.0", Equal);
    test("1/9", "3", 10, Up, "-2.0", "-0x2.0", Equal);
    test("1/9", "3", 10, Nearest, "-2.0", "-0x2.0", Equal);
    test("1/9", "3", 100, Nearest, "-2.0", "-0x2.0", Equal);
    test("1/9", "3", 300, Exact, "-2.0", "-0x2.0", Equal);
    test("1/3", "3", 1, Nearest, "-1.0", "-0x1.0", Equal);
    test("1/3", "3", 10, Floor, "-1.0", "-0x1.0", Equal);
    test("1/3", "3", 10, Ceiling, "-1.0", "-0x1.0", Equal);
    test("1/3", "3", 10, Down, "-1.0", "-0x1.0", Equal);
    test("1/3", "3", 10, Up, "-1.0", "-0x1.0", Equal);
    test("1/3", "3", 10, Nearest, "-1.0", "-0x1.0", Equal);
    test("1/3", "3", 100, Nearest, "-1.0", "-0x1.0", Equal);
    test("1/3", "3", 300, Exact, "-1.0", "-0x1.0", Equal);
    test("10000000000/3", "3", 1, Nearest, "16.0", "0x1.0E+1", Less);
    test("10000000000/3", "3", 10, Floor, "19.94", "0x13.f", Less);
    test(
        "10000000000/3",
        "3",
        10,
        Ceiling,
        "19.969",
        "0x13.f8",
        Greater,
    );
    test("10000000000/3", "3", 10, Down, "19.94", "0x13.f", Less);
    test("10000000000/3", "3", 10, Up, "19.969", "0x13.f8", Greater);
    test(
        "10000000000/3",
        "3",
        10,
        Nearest,
        "19.969",
        "0x13.f8",
        Greater,
    );
    test(
        "10000000000/3",
        "3",
        100,
        Nearest,
        "19.95903274289384604296567522023",
        "0x13.f5832b7a85b1dbc1c4d0f94c",
        Greater,
    );
    test("1", "3", 1, Nearest, "0.0", "0x0.0", Equal);
    test("1", "3", 10, Floor, "0.0", "0x0.0", Equal);
    test("1", "3", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("1", "3", 10, Down, "0.0", "0x0.0", Equal);
    test("1", "3", 10, Up, "0.0", "0x0.0", Equal);
    test("1", "3", 10, Nearest, "0.0", "0x0.0", Equal);
    test("1", "3", 100, Nearest, "0.0", "0x0.0", Equal);
    test("1", "3", 300, Exact, "0.0", "0x0.0", Equal);
    test("0", "3", 1, Nearest, "-Infinity", "-Infinity", Equal);
    test("0", "3", 10, Floor, "-Infinity", "-Infinity", Equal);
    test("0", "3", 10, Ceiling, "-Infinity", "-Infinity", Equal);
    test("0", "3", 10, Down, "-Infinity", "-Infinity", Equal);
    test("0", "3", 10, Up, "-Infinity", "-Infinity", Equal);
    test("0", "3", 10, Nearest, "-Infinity", "-Infinity", Equal);
    test("0", "3", 100, Nearest, "-Infinity", "-Infinity", Equal);
    test("0", "3", 300, Exact, "-Infinity", "-Infinity", Equal);
    test("-1", "3", 1, Nearest, "NaN", "NaN", Equal);
    test("-1", "3", 10, Floor, "NaN", "NaN", Equal);
    test("-1", "3", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1", "3", 10, Down, "NaN", "NaN", Equal);
    test("-1", "3", 10, Up, "NaN", "NaN", Equal);
    test("-1", "3", 10, Nearest, "NaN", "NaN", Equal);
    test("-1", "3", 100, Nearest, "NaN", "NaN", Equal);
    test("-1", "3", 300, Exact, "NaN", "NaN", Equal);
}

#[test]
#[should_panic]
fn log_base_rational_prec_round_fail_1() {
    BigExponentFloat::log_base_rational_prec_round(Rational::from_unsigneds(1u8, 3), 3, 0, Floor);
}

#[test]
#[should_panic]
fn log_base_rational_prec_round_fail_2() {
    BigExponentFloat::log_base_rational_prec_round(Rational::from_unsigneds(1u8, 5), 3, 10, Exact);
}

#[test]
#[should_panic]
fn log_base_rational_prec_round_fail_3() {
    BigExponentFloat::log_base_rational_prec_round(Rational::from_unsigneds(1u8, 3), 1, 10, Floor);
}

fn log_base_rational_prec_round_properties_helper(
    x: &Rational,
    b: u64,
    prec: u64,
    rm: RoundingMode,
) {
    let r = log_base_rational_prec_round_all(x, b, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else if !r.0.is_nan() {
        // The result lies between the results rounded down and up.
        let floor = log_base_rational_prec_round_all(x, b, prec, Floor);
        let ceiling = log_base_rational_prec_round_all(x, b, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0);
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(
                &log_base_rational_prec_round_all(x, b, prec, Exact),
                &r
            ));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = log_base_rational_prec_round_all(x, b, prec + 16, rm);
            let (z, oz) = y.round_to_prec_round(prec, rm);
            assert!(same(&z, &r.0));
            assert_eq!(if o == Equal { oz } else { o }, r.1);
        }
    }
    // A dyadic `Rational` gives the same result as the equal `BigExponentFloat`.
    if let Ok(y) = BigExponentFloat::try_from(x) {
        assert!(same_pair(&y.log_base_prec_round_ref(b, prec, rm), &r));
    }
}

#[test]
fn log_base_rational_prec_round_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_1().test_properties(
        |(x, b, prec, rm)| {
            log_base_rational_prec_round_properties_helper(&x, b, prec, rm);
        },
    );
}

#[test]
fn log_base_rational_prec_round_float_agreement_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_1().test_properties(
        |(x, b, prec, rm)| {
            if rm == Exact {
                return;
            }
            let (f, fo) = Float::log_base_rational_prec_round_ref(&x, b, prec, rm);
            if float_result_comparable(&f, fo) {
                let r = log_base_rational_prec_round_all(&x.clone(), b, prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
}
