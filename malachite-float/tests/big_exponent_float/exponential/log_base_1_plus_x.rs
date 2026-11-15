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
use malachite_float::test_util::big_exponent_float::float_to_big_exponent_float;
use malachite_float::test_util::generators::{
    big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_5,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_7,
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

fn log_base_1_plus_x_prec_round_all(
    x: &BigExponentFloat,
    b: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let r = x.clone().log_base_1_plus_x_prec_round(b, prec, rm);
    assert!(r.0.is_valid());
    assert!(same_pair(
        &x.log_base_1_plus_x_prec_round_ref(b, prec, rm),
        &r
    ));
    let mut z = x.clone();
    let o = z.log_base_1_plus_x_prec_round_assign(b, prec, rm);
    assert!(same_pair(&(z, o), &r));
    if rm == Nearest {
        assert!(same_pair(&x.clone().log_base_1_plus_x_prec(b, prec), &r));
        assert!(same_pair(&x.log_base_1_plus_x_prec_ref(b, prec), &r));
        let mut z = x.clone();
        let o = z.log_base_1_plus_x_prec_assign(b, prec);
        assert!(same_pair(&(z, o), &r));
    }
    r
}

#[test]
fn test_log_base_1_plus_x_prec_round() {
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
        let r = log_base_1_plus_x_prec_round_all(&hex(x_hex), b.parse::<u64>().unwrap(), prec, rm);
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
        "1.293",
        "0x1.4b",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3",
        10,
        Ceiling,
        "1.2949",
        "0x1.4b8",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3",
        10,
        Down,
        "1.293",
        "0x1.4b",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3",
        10,
        Up,
        "1.2949",
        "0x1.4b8",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3",
        10,
        Nearest,
        "1.293",
        "0x1.4b",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30",
        "3",
        100,
        Nearest,
        "1.2935231359164237386853033768279",
        "0x1.4b24550d616216685f5c9e1f6",
        Greater,
    );
    test("0.50", "0x0.8", "3", 1, Nearest, "0.25", "0x0.4", Less);
    test("0.50", "0x0.8", "3", 10, Floor, "0.36865", "0x0.5e6", Less);
    test(
        "0.50", "0x0.8", "3", 10, Ceiling, "0.3691", "0x0.5e8", Greater,
    );
    test("0.50", "0x0.8", "3", 10, Down, "0.36865", "0x0.5e6", Less);
    test("0.50", "0x0.8", "3", 10, Up, "0.3691", "0x0.5e8", Greater);
    test(
        "0.50", "0x0.8", "3", 10, Nearest, "0.3691", "0x0.5e8", Greater,
    );
    test(
        "0.50",
        "0x0.8",
        "3",
        100,
        Nearest,
        "0.3690702464285425629004728856573",
        "0x0.5e7b633e565616b1fbc150887",
        Greater,
    );
    test(
        "-0.50", "-0x0.8", "3", 1, Nearest, "-0.50", "-0x0.8", Greater,
    );
    test(
        "-0.50", "-0x0.8", "3", 10, Floor, "-0.63184", "-0x0.a1c", Less,
    );
    test(
        "-0.50", "-0x0.8", "3", 10, Ceiling, "-0.6309", "-0x0.a18", Greater,
    );
    test(
        "-0.50", "-0x0.8", "3", 10, Down, "-0.6309", "-0x0.a18", Greater,
    );
    test("-0.50", "-0x0.8", "3", 10, Up, "-0.63184", "-0x0.a1c", Less);
    test(
        "-0.50", "-0x0.8", "3", 10, Nearest, "-0.6309", "-0x0.a18", Greater,
    );
    test(
        "-0.50",
        "-0x0.8",
        "3",
        100,
        Nearest,
        "-0.63092975357145743709952711434266",
        "-0x0.a1849cc1a9a9e94e043eaf779",
        Greater,
    );
    test("2.0", "0x2", "3", 1, Nearest, "1.0", "0x1.0", Equal);
    test("2.0", "0x2", "3", 10, Floor, "1.0", "0x1.0", Equal);
    test("2.0", "0x2", "3", 10, Ceiling, "1.0", "0x1.0", Equal);
    test("2.0", "0x2", "3", 10, Down, "1.0", "0x1.0", Equal);
    test("2.0", "0x2", "3", 10, Up, "1.0", "0x1.0", Equal);
    test("2.0", "0x2", "3", 10, Nearest, "1.0", "0x1.0", Equal);
    test("2.0", "0x2", "3", 100, Nearest, "1.0", "0x1.0", Equal);
    test("2.0", "0x2", "3", 300, Exact, "1.0", "0x1.0", Equal);
    test("8.0", "0x8", "3", 1, Nearest, "2.0", "0x2.0", Equal);
    test("8.0", "0x8", "3", 10, Floor, "2.0", "0x2.0", Equal);
    test("8.0", "0x8", "3", 10, Ceiling, "2.0", "0x2.0", Equal);
    test("8.0", "0x8", "3", 10, Down, "2.0", "0x2.0", Equal);
    test("8.0", "0x8", "3", 10, Up, "2.0", "0x2.0", Equal);
    test("8.0", "0x8", "3", 10, Nearest, "2.0", "0x2.0", Equal);
    test("8.0", "0x8", "3", 100, Nearest, "2.0", "0x2.0", Equal);
    test("8.0", "0x8", "3", 300, Exact, "2.0", "0x2.0", Equal);
    test("26.0", "0x1a", "3", 1, Nearest, "4.0", "0x4.0", Greater);
    test("26.0", "0x1a", "3", 10, Floor, "3.0", "0x3.0", Equal);
    test("26.0", "0x1a", "3", 10, Ceiling, "3.0", "0x3.0", Equal);
    test("26.0", "0x1a", "3", 10, Down, "3.0", "0x3.0", Equal);
    test("26.0", "0x1a", "3", 10, Up, "3.0", "0x3.0", Equal);
    test("26.0", "0x1a", "3", 10, Nearest, "3.0", "0x3.0", Equal);
    test("26.0", "0x1a", "3", 100, Nearest, "3.0", "0x3.0", Equal);
    test("26.0", "0x1a", "3", 300, Exact, "3.0", "0x3.0", Equal);
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
        "1.6e-361235995",
        "0x1.0E-300000000",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3",
        10,
        Floor,
        "2.1799e-361235995",
        "0x1.5d8E-300000000",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3",
        10,
        Ceiling,
        "2.183e-361235995",
        "0x1.5eE-300000000",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3",
        10,
        Down,
        "2.1799e-361235995",
        "0x1.5d8E-300000000",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3",
        10,
        Up,
        "2.183e-361235995",
        "0x1.5eE-300000000",
        Greater,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3",
        10,
        Nearest,
        "2.1799e-361235995",
        "0x1.5d8E-300000000",
        Less,
    );
    test(
        "2.4e-361235995",
        "0x1.8E-300000000",
        "3",
        100,
        Nearest,
        "2.1800646469651550091401663766377e-361235995",
        "0x1.5d88282cd90fe05161c272afeE-300000000",
        Greater,
    );
    test("-3.0", "-0x3", "3", 1, Nearest, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", "3", 10, Floor, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", "3", 10, Ceiling, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", "3", 10, Down, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", "3", 10, Up, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", "3", 10, Nearest, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", "3", 100, Nearest, "NaN", "NaN", Equal);
    test("-3.0", "-0x3", "3", 300, Exact, "NaN", "NaN", Equal);
    test("0.0", "0x0", "3", 1, Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0", "3", 10, Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0", "3", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("0.0", "0x0", "3", 10, Down, "0.0", "0x0.0", Equal);
    test("0.0", "0x0", "3", 10, Up, "0.0", "0x0.0", Equal);
    test("0.0", "0x0", "3", 10, Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0", "3", 100, Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0", "3", 300, Exact, "0.0", "0x0.0", Equal);
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
        "20.959032742984869965623807763437",
        "0x14.f5832b7ae9c6d09fa0e6795e",
        Less,
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
        "1.0108942124649e-323228557",
        "0xa.e8ac3ab378E-268435507",
        "3",
        10,
        Nearest,
        "9.209e-323228558",
        "0x9.fE-268435507",
        Greater,
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
        "1.2924490193584e-323228557",
        "0xd.f27d50b18cE-268435507",
        "3",
        10,
        Nearest,
        "1.176e-323228557",
        "0xc.bE-268435507",
        Less,
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
        "1.208141728657172e-323228557",
        "0xd.09958b5e456E-268435507",
        "3",
        10,
        Nearest,
        "1.100e-323228557",
        "0xb.eE-268435507",
        Greater,
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
        "1.43720347014654e-323228557",
        "0xf.8262810d1acE-268435507",
        "3",
        10,
        Nearest,
        "1.3075e-323228557",
        "0xe.1cE-268435507",
        Less,
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
        "1.219276668651828e-323228557",
        "0xd.2858647d0428E-268435507",
        "3",
        10,
        Nearest,
        "1.1106e-323228557",
        "0xb.fcE-268435507",
        Greater,
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
        "1.171555463948821e-323228557",
        "0xc.a48305168bb8E-268435507",
        "3",
        10,
        Nearest,
        "1.07e-323228557",
        "0xb.8E-268435507",
        Less,
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
        "1.5024224547040857e-323228557",
        "0x1.0368ea66eefb5E-268435506",
        "3",
        10,
        Nearest,
        "1.3683e-323228557",
        "0xe.c4E-268435507",
        Greater,
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
        "1.1651926392954019e-323228557",
        "0xc.92ef1afab9558E-268435507",
        "3",
        10,
        Nearest,
        "1.060e-323228557",
        "0xb.7E-268435507",
        Less,
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
        "1.4173198163747e-323228557",
        "0xf.4b7464d4f2E-268435507",
        "3",
        10,
        Floor,
        "1.2901e-323228557",
        "0xd.ecE-268435507",
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
        "1.4173198163747e-323228557",
        "0xf.4b7464d4f2E-268435507",
        "3",
        10,
        Ceiling,
        "1.292e-323228557",
        "0xd.fE-268435507",
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
        "8.3512090481620e-323228558",
        "0x9.0315e55f08E-268435507",
        "3",
        10,
        Floor,
        "7.587e-323228558",
        "0x8.3E-268435507",
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
        "8.3512090481620e-323228558",
        "0x9.0315e55f08E-268435507",
        "3",
        10,
        Ceiling,
        "7.6016e-323228558",
        "0x8.34E-268435507",
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
        "1.156443761334774e-323228557",
        "0xc.7ac3ba27ad6E-268435507",
        "3",
        10,
        Floor,
        "1.0526e-323228557",
        "0xb.5cE-268435507",
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
        "1.156443761334774e-323228557",
        "0xc.7ac3ba27ad6E-268435507",
        "3",
        10,
        Ceiling,
        "1.054e-323228557",
        "0xb.6E-268435507",
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
        "1.31551441041637e-323228557",
        "0xe.32359cccf38E-268435507",
        "3",
        10,
        Floor,
        "1.196e-323228557",
        "0xc.e8E-268435507",
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
        "1.31551441041637e-323228557",
        "0xe.32359cccf38E-268435507",
        "3",
        10,
        Ceiling,
        "1.1974e-323228557",
        "0xc.ecE-268435507",
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
        "1.442770956904381e-323228557",
        "0xf.91c3f0a54c9E-268435507",
        "3",
        10,
        Floor,
        "1.3133e-323228557",
        "0xe.2cE-268435507",
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
        "1.442770956904381e-323228557",
        "0xf.91c3f0a54c9E-268435507",
        "3",
        10,
        Ceiling,
        "1.315e-323228557",
        "0xe.3E-268435507",
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
        "1.124629617123987e-323228557",
        "0xc.22e023d1dcd8E-268435507",
        "3",
        10,
        Floor,
        "1.022e-323228557",
        "0xb.08E-268435507",
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
        "1.124629617123987e-323228557",
        "0xc.22e023d1dcd8E-268435507",
        "3",
        10,
        Ceiling,
        "1.0237e-323228557",
        "0xb.0cE-268435507",
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
        "1.382324100728270e-323228557",
        "0xe.eac6bca2c838E-268435507",
        "3",
        10,
        Floor,
        "1.2582e-323228557",
        "0xd.94E-268435507",
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
        "1.382324100728270e-323228557",
        "0xe.eac6bca2c838E-268435507",
        "3",
        10,
        Ceiling,
        "1.260e-323228557",
        "0xd.98E-268435507",
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
        "9.9419167175375595e-323228558",
        "0xa.ba87dd5abbd78E-268435507",
        "3",
        10,
        Floor,
        "9.04e-323228558",
        "0x9.cE-268435507",
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
        "9.9419167175375595e-323228558",
        "0xa.ba87dd5abbd78E-268435507",
        "3",
        10,
        Ceiling,
        "9.0495e-323228558",
        "0x9.c4E-268435507",
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
        "9.95930616587827843086119361704856197827687e-323228558",
        "0xa.bf55ae3ad3795b34a222d4f0eb5f5f4a7E-268435507",
        "3",
        100,
        Nearest,
        "9.065351142168937288344958868299e-323228558",
        "0x9.c85f6d36c2e6c9ab7805c532E-268435507",
        Greater,
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
        "1.59806998511848590934800157967833368233808e-323228557",
        "0x1.13eca760741d1db041402030ae5533e47cE-268435506",
        "3",
        100,
        Nearest,
        "1.454625987349812156701650398573e-323228557",
        "0xf.b2840cdd18bf41f47573688eE-268435507",
        Less,
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
        "9.488381738189322241137944982707135519154546e-323228558",
        "0xa.3d3cfb0fc1e9643940255193f761390e04E-268435507",
        "3",
        100,
        Nearest,
        "8.6366972553096557966215368658005e-323228558",
        "0x9.51f42eaf0ffcb491ddec6e8fE-268435507",
        Greater,
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
        "1.1797919941760755710486036552932832194840613e-323228557",
        "0xc.bb440e43cb9fb6ecafc0a5319ae3c36679E-268435507",
        "3",
        100,
        Nearest,
        "1.073892952359365274145067023538e-323228557",
        "0xb.96b62cba696083c92db9fb3aE-268435507",
        Less,
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
        "1.2797469120350480871189612721450664042465486e-323228557",
        "0xd.cf6622b5e3dfdaa5541ffe26b0726f30eccE-268435507",
        "3",
        100,
        Nearest,
        "1.164875839488865474517865701190e-323228557",
        "0xc.920f0eefd09da21771d2ad38E-268435507",
        Greater,
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
        "9.4304461858979097279125930559898175727218036e-323228558",
        "0xa.2d3baa2beedd4ee55a0ccfc54d01a0525acE-268435507",
        "3",
        100,
        Nearest,
        "8.583962042997721773540627225117e-323228558",
        "0x9.4362a506ae87e7b48e72036aE-268435507",
        Less,
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
        "1.4833079799650999364092363314816898747411928e-323228557",
        "0x1.001c084b664b339378d1c06bcc2facff6c6E-268435506",
        "3",
        100,
        Nearest,
        "1.3501651085328489812183193121942e-323228557",
        "0xe.91ef442f68524dcbf65b064dE-268435507",
        Greater,
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
        "1.05543944075175916549905452780447980075905622e-323228557",
        "0xb.63bb7d357d3a21d6a93c236ca4ec9de4898cE-268435507",
        "3",
        100,
        Nearest,
        "9.6070238030134302910076926959703e-323228558",
        "0xa.5e039495b01cefda931755f3E-268435507",
        Less,
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
        "1.16287804450991789221340452971823116097058e-323228557",
        "0xc.8c8a2db3c1f90ca0f1ca6c0fcc7baeeb5E-268435507",
        "3",
        100,
        Floor,
        "1.0584972118960366540437020629454e-323228557",
        "0xb.6c2e013f267d3faf0f92499fE-268435507",
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
        "1.16287804450991789221340452971823116097058e-323228557",
        "0xc.8c8a2db3c1f90ca0f1ca6c0fcc7baeeb5E-268435507",
        "3",
        100,
        Ceiling,
        "1.05849721189603665404370206295e-323228557",
        "0xb.6c2e013f267d3faf0f9249aE-268435507",
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
        "1.00270154164847055903377449993854346659444e-323228557",
        "0xa.d20a3631591df8b1b376ae7e02893b829E-268435507",
        "3",
        100,
        Floor,
        "9.126982758076414266785979774694e-323228558",
        "0x9.d96622adf51872c09aa3a89aE-268435507",
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
        "1.00270154164847055903377449993854346659444e-323228557",
        "0xa.d20a3631591df8b1b376ae7e02893b829E-268435507",
        "3",
        100,
        Ceiling,
        "9.1269827580764142667859797747059e-323228558",
        "0x9.d96622adf51872c09aa3a89bE-268435507",
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
        "1.30159011857470231996977073596583356099619e-323228557",
        "0xe.0bbe147ae434b3e22f7ffeac49c41c382E-268435507",
        "3",
        100,
        Floor,
        "1.1847583829165706203911046662577e-323228557",
        "0xc.c8fc62215cb9f2d4c344d633E-268435507",
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
        "1.30159011857470231996977073596583356099619e-323228557",
        "0xe.0bbe147ae434b3e22f7ffeac49c41c382E-268435507",
        "3",
        100,
        Ceiling,
        "1.184758382916570620391104666259e-323228557",
        "0xc.c8fc62215cb9f2d4c344d634E-268435507",
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
        "1.033769278432262890141059761783479362244029e-323228557",
        "0xb.27ddecc9cd5d62f9f8171106f03089843eE-268435507",
        "3",
        100,
        Floor,
        "9.409773485107667066414746784482e-323228558",
        "0xa.2785a697edde944f7e4788f2E-268435507",
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
        "1.033769278432262890141059761783479362244029e-323228557",
        "0xb.27ddecc9cd5d62f9f8171106f03089843eE-268435507",
        "3",
        100,
        Ceiling,
        "9.4097734851076670664147467844933e-323228558",
        "0xa.2785a697edde944f7e4788f3E-268435507",
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
        "1.5917497568567748357677346940321892619033496e-323228557",
        "0x1.12d54abebb7d535aec15c08aa91996d0f2bE-268435506",
        "3",
        100,
        Floor,
        "1.4488730676647671882397011905720e-323228557",
        "0xf.a29f78f7832a1fabb1b4ab1fE-268435507",
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
        "1.5917497568567748357677346940321892619033496e-323228557",
        "0x1.12d54abebb7d535aec15c08aa91996d0f2bE-268435506",
        "3",
        100,
        Ceiling,
        "1.44887306766476718823970119057e-323228557",
        "0xf.a29f78f7832a1fabb1b4ab2E-268435507",
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
        "1.0708312617947442051827832605585426447907475e-323228557",
        "0xb.8e40e3161a18c0799f2f64f3e3e2cecf92cE-268435507",
        "3",
        100,
        Floor,
        "9.747126195838884133376273540142e-323228558",
        "0xa.84b7e589702b093dc42fafacE-268435507",
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
        "1.0708312617947442051827832605585426447907475e-323228557",
        "0xb.8e40e3161a18c0799f2f64f3e3e2cecf92cE-268435507",
        "3",
        100,
        Ceiling,
        "9.7471261958388841333762735401541e-323228558",
        "0xa.84b7e589702b093dc42fafadE-268435507",
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
        "1.38435974904734261665896089680133205279423521e-323228557",
        "0xe.f066632687401f1966d44e71522b588529bcE-268435507",
        "3",
        100,
        Floor,
        "1.2600985473461758376617671137662e-323228557",
        "0xd.991e6d29746e323d7eb39a31E-268435507",
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
        "1.38435974904734261665896089680133205279423521e-323228557",
        "0xe.f066632687401f1966d44e71522b588529bcE-268435507",
        "3",
        100,
        Ceiling,
        "1.260098547346175837661767113767e-323228557",
        "0xd.991e6d29746e323d7eb39a32E-268435507",
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
        "1.49648431406473622059768998160119883240408119e-323228557",
        "0x1.02627149c8e3c53fbe47e9aa39ccc7c26a21E-268435506",
        "3",
        100,
        Floor,
        "1.362158724693478738343939845084e-323228557",
        "0xe.b3116319d2e78ca2d31dcedcE-268435507",
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
        "1.49648431406473622059768998160119883240408119e-323228557",
        "0x1.02627149c8e3c53fbe47e9aa39ccc7c26a21E-268435506",
        "3",
        100,
        Ceiling,
        "1.3621587246934787383439398450854e-323228557",
        "0xe.b3116319d2e78ca2d31dceddE-268435507",
        Greater,
    );
}

#[test]
#[should_panic]
fn log_base_1_plus_x_prec_round_fail_1() {
    BigExponentFloat::from(5).log_base_1_plus_x_prec_round(3, 0, Floor);
}

#[test]
#[should_panic]
fn log_base_1_plus_x_prec_round_fail_2() {
    BigExponentFloat::from(core::f64::consts::PI).log_base_1_plus_x_prec_round(3, 10, Exact);
}

#[test]
#[should_panic]
fn log_base_1_plus_x_prec_round_fail_3() {
    BigExponentFloat::from(5).log_base_1_plus_x_prec_round(1, 10, Floor);
}

fn log_base_1_plus_x_prec_round_properties_helper(
    x: &BigExponentFloat,
    b: u64,
    prec: u64,
    rm: RoundingMode,
) {
    let r = log_base_1_plus_x_prec_round_all(x, b, prec, rm);
    if !r.0.is_finite() || r.0.is_zero() {
        assert_eq!(r.1, Equal);
    } else {
        assert!(r.0.get_prec().unwrap() <= prec);
    }
    if rm == Exact {
        assert_eq!(r.1, Equal);
    } else if !r.0.is_nan() {
        // The result lies between the results rounded down and up.
        let floor = log_base_1_plus_x_prec_round_all(x, b, prec, Floor);
        let ceiling = log_base_1_plus_x_prec_round_all(x, b, prec, Ceiling);
        assert!(floor.0 <= r.0 && r.0 <= ceiling.0);
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        if r.1 == Equal {
            assert!(same_pair(
                &log_base_1_plus_x_prec_round_all(x, b, prec, Exact),
                &r
            ));
        }
        // Rounding in one direction twice is rounding once.
        if rm != Nearest {
            let (y, o) = log_base_1_plus_x_prec_round_all(x, b, prec + 16, rm);
            let (z, oz) = y.round_to_prec_round(prec, rm);
            assert!(same(&z, &r.0));
            assert_eq!(if o == Equal { oz } else { o }, r.1);
        }
    }
    // log_b(1 + x) is log_b of the exact sum 1 + x.
    if x.is_finite() && exponent_is_moderate(x) {
        let z = x + BigExponentFloat::ONE;
        assert!(same_pair(&z.log_base_prec_round_ref(b, prec, rm), &r));
    }
}

#[test]
fn log_base_1_plus_x_prec_round_properties() {
    big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_5().test_properties(
        |(x, b, prec, rm)| {
            log_base_1_plus_x_prec_round_properties_helper(&x, b, prec, rm);
        },
    );
}

#[test]
fn log_base_1_plus_x_prec_round_float_agreement_properties() {
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_7().test_properties(
        |(x, b, prec, rm)| {
            if rm == Exact {
                return;
            }
            let (f, fo) = x.log_base_1_plus_x_prec_round_ref(b, prec, rm);
            if float_result_comparable(&f, fo) {
                let r =
                    log_base_1_plus_x_prec_round_all(&float_to_big_exponent_float(&x), b, prec, rm);
                assert!(same(&r.0, &float_to_big_exponent_float(&f)));
                assert_eq!(r.1, fo);
            }
        },
    );
}
