// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_float::test_util::common::{rug_round_try_from_rounding_mode, to_hex_string};
use malachite_float::test_util::generators::*;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{natural_gen, natural_unsigned_pair_gen_var_7};
use malachite_q::Rational;
use malachite_q::conversion::primitive_float_from_rational::FloatConversionError;
use std::cmp::Ordering::*;
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_from_natural_prec() {
    let test = |s, prec, out, out_hex, out_o| {
        let u = Natural::from_str(s).unwrap();

        let (x, o) = Float::from_natural_prec(u.clone(), prec);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let (x, o) = Float::from_natural_prec_ref(&u, prec);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let rug_x = rug::Float::with_val(u32::exact_from(prec), rug::Integer::from(&u));
        let x = Float::exact_from(&rug_x);
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
    };
    test("0", 1, "0.0", "0x0.0", Equal);
    test("0", 10, "0.0", "0x0.0", Equal);
    test("0", 20, "0.0", "0x0.0", Equal);

    test("1", 1, "1.0", "0x1.0#1", Equal);
    test("1", 10, "1.0", "0x1.000#10", Equal);
    test("1", 20, "1.0", "0x1.00000#20", Equal);

    test("123", 1, "1.0e2", "0x8.0E+1#1", Greater);
    test("123", 10, "123.0", "0x7b.0#10", Equal);
    test("123", 20, "123.0", "0x7b.0000#20", Equal);

    test("1000000000000", 1, "1.0e12", "0x1.0E+10#1", Greater);
    test("1000000000000", 10, "9.997e11", "0xe.8cE+9#10", Less);
    test("1000000000000", 20, "9.999997e11", "0xe.8d4aE+9#20", Less);
    test(
        "289905948138435080392",
        64,
        "2.8990594813843508038e20",
        "0xf.b740d3d8283d70cE+16#64",
        Less,
    );

    let test_big = |u: Natural, prec, out, out_hex, out_o| {
        let (x, o) = Float::from_natural_prec(u.clone(), prec);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let (x, o) = Float::from_natural_prec_ref(&u, prec);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let rug_x = rug::Float::with_val(u32::exact_from(prec), rug::Integer::from(&u));
        let x = Float::exact_from(&rug_x);
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
    };
    test_big(
        Natural::power_of_2(1000),
        10,
        "1.072e301",
        "0x1.000E+250#10",
        Equal,
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT)),
        10,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 1),
        10,
        "too_big",
        "0x4.00E+268435455#10",
        Equal,
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 2) * Natural::from(3u8),
        1,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 12) * Natural::from(3073u16),
        1,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 12) * Natural::from(3071u16),
        1,
        "too_big",
        "0x4.0E+268435455#1",
        Less,
    );
}

#[test]
fn from_natural_prec_fail() {
    assert_panic!(Float::from_natural_prec(Natural::ZERO, 0));
    assert_panic!(Float::from_natural_prec(Natural::ONE, 0));
}

#[test]
fn from_natural_prec_ref_fail() {
    assert_panic!(Float::from_natural_prec_ref(&Natural::ZERO, 0));
    assert_panic!(Float::from_natural_prec_ref(&Natural::ONE, 0));
}

#[test]
fn test_from_natural_prec_round() {
    let test = |s, prec, rm, out, out_hex, out_o| {
        let u = Natural::from_str(s).unwrap();

        let (x, o) = Float::from_natural_prec_round(u.clone(), prec, rm);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let (x, o) = Float::from_natural_prec_round_ref(&u, prec, rm);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_x, rug_o) =
                rug::Float::with_val_round(u32::exact_from(prec), rug::Integer::from(&u), rm);
            let x = Float::exact_from(&rug_x);
            assert_eq!(x.to_string(), out);
            assert_eq!(to_hex_string(&x), out_hex);
            assert_eq!(rug_o, out_o);
        }
    };
    test("0", 1, Floor, "0.0", "0x0.0", Equal);
    test("0", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 1, Down, "0.0", "0x0.0", Equal);
    test("0", 1, Up, "0.0", "0x0.0", Equal);
    test("0", 1, Nearest, "0.0", "0x0.0", Equal);
    test("0", 1, Exact, "0.0", "0x0.0", Equal);

    test("0", 10, Floor, "0.0", "0x0.0", Equal);
    test("0", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 10, Down, "0.0", "0x0.0", Equal);
    test("0", 10, Up, "0.0", "0x0.0", Equal);
    test("0", 10, Nearest, "0.0", "0x0.0", Equal);
    test("0", 10, Exact, "0.0", "0x0.0", Equal);

    test("0", 20, Floor, "0.0", "0x0.0", Equal);
    test("0", 20, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 20, Down, "0.0", "0x0.0", Equal);
    test("0", 20, Up, "0.0", "0x0.0", Equal);
    test("0", 20, Nearest, "0.0", "0x0.0", Equal);
    test("0", 20, Exact, "0.0", "0x0.0", Equal);

    test("1", 1, Floor, "1.0", "0x1.0#1", Equal);
    test("1", 1, Ceiling, "1.0", "0x1.0#1", Equal);
    test("1", 1, Down, "1.0", "0x1.0#1", Equal);
    test("1", 1, Up, "1.0", "0x1.0#1", Equal);
    test("1", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test("1", 1, Exact, "1.0", "0x1.0#1", Equal);

    test("1", 10, Floor, "1.0", "0x1.000#10", Equal);
    test("1", 10, Ceiling, "1.0", "0x1.000#10", Equal);
    test("1", 10, Down, "1.0", "0x1.000#10", Equal);
    test("1", 10, Up, "1.0", "0x1.000#10", Equal);
    test("1", 10, Nearest, "1.0", "0x1.000#10", Equal);
    test("1", 10, Exact, "1.0", "0x1.000#10", Equal);

    test("1", 20, Floor, "1.0", "0x1.00000#20", Equal);
    test("1", 20, Ceiling, "1.0", "0x1.00000#20", Equal);
    test("1", 20, Down, "1.0", "0x1.00000#20", Equal);
    test("1", 20, Up, "1.0", "0x1.00000#20", Equal);
    test("1", 20, Nearest, "1.0", "0x1.00000#20", Equal);
    test("1", 20, Exact, "1.0", "0x1.00000#20", Equal);

    test("123", 1, Floor, "6.0e1", "0x4.0E+1#1", Less);
    test("123", 1, Ceiling, "1.0e2", "0x8.0E+1#1", Greater);
    test("123", 1, Down, "6.0e1", "0x4.0E+1#1", Less);
    test("123", 1, Up, "1.0e2", "0x8.0E+1#1", Greater);
    test("123", 1, Nearest, "1.0e2", "0x8.0E+1#1", Greater);

    test("123", 10, Floor, "123.0", "0x7b.0#10", Equal);
    test("123", 10, Ceiling, "123.0", "0x7b.0#10", Equal);
    test("123", 10, Down, "123.0", "0x7b.0#10", Equal);
    test("123", 10, Up, "123.0", "0x7b.0#10", Equal);
    test("123", 10, Nearest, "123.0", "0x7b.0#10", Equal);
    test("123", 10, Exact, "123.0", "0x7b.0#10", Equal);

    test("123", 20, Floor, "123.0", "0x7b.0000#20", Equal);
    test("123", 20, Ceiling, "123.0", "0x7b.0000#20", Equal);
    test("123", 20, Down, "123.0", "0x7b.0000#20", Equal);
    test("123", 20, Up, "123.0", "0x7b.0000#20", Equal);
    test("123", 20, Nearest, "123.0", "0x7b.0000#20", Equal);
    test("123", 20, Exact, "123.0", "0x7b.0000#20", Equal);

    test("1000000000000", 1, Floor, "5.0e11", "0x8.0E+9#1", Less);
    test(
        "1000000000000",
        1,
        Ceiling,
        "1.0e12",
        "0x1.0E+10#1",
        Greater,
    );
    test("1000000000000", 1, Down, "5.0e11", "0x8.0E+9#1", Less);
    test("1000000000000", 1, Up, "1.0e12", "0x1.0E+10#1", Greater);
    test(
        "1000000000000",
        1,
        Nearest,
        "1.0e12",
        "0x1.0E+10#1",
        Greater,
    );

    test("1000000000000", 10, Floor, "9.997e11", "0xe.8cE+9#10", Less);
    test(
        "1000000000000",
        10,
        Ceiling,
        "1.001e12",
        "0xe.90E+9#10",
        Greater,
    );
    test("1000000000000", 10, Down, "9.997e11", "0xe.8cE+9#10", Less);
    test("1000000000000", 10, Up, "1.001e12", "0xe.90E+9#10", Greater);
    test(
        "1000000000000",
        10,
        Nearest,
        "9.997e11",
        "0xe.8cE+9#10",
        Less,
    );

    test(
        "1000000000000",
        20,
        Floor,
        "9.999997e11",
        "0xe.8d4aE+9#20",
        Less,
    );
    test(
        "1000000000000",
        20,
        Ceiling,
        "1.000001e12",
        "0xe.8d4bE+9#20",
        Greater,
    );
    test(
        "1000000000000",
        20,
        Down,
        "9.999997e11",
        "0xe.8d4aE+9#20",
        Less,
    );
    test(
        "1000000000000",
        20,
        Up,
        "1.000001e12",
        "0xe.8d4bE+9#20",
        Greater,
    );
    test(
        "1000000000000",
        20,
        Nearest,
        "9.999997e11",
        "0xe.8d4aE+9#20",
        Less,
    );

    test(
        "20928269765806917927943182622889",
        64,
        Nearest,
        "2.0928269765806917929e31",
        "0x1.0826e3012a87296eE+26#64",
        Greater,
    );

    // - in from_natural_prec_round
    // - x == 0 in from_natural_prec_round
    test("0", 1, Down, "0.0", "0x0.0", Equal);
    // - x != 0 in from_natural_prec_round
    // - bits <= prec in from_natural_prec_round
    test("1", 1, Down, "1.0", "0x1.0#1", Equal);
    // - bits > prec in from_natural_prec_round
    // - needed_bits == 0 in from_natural_prec_round
    // - mask_width < Limb::WIDTH in from_natural_prec_round
    // - rm == Floor || rm == Down in from_natural_prec_round
    // - (rm == Floor || rm == Down) && !inexact in from_natural_prec_round
    test("2", 1, Down, "2.0", "0x2.0#1", Equal);
    // - rm == Ceiling || rm == Up in from_natural_prec_round
    // - (rm == Ceiling || rm == Up) && !inexact in from_natural_prec_round
    test("2", 1, Up, "2.0", "0x2.0#1", Equal);
    // - rm == Nearest in from_natural_prec_round
    // - rm == Nearest && (!half_bit || !inexact_after_half && !x.get_bit(bits - prec) && !inexact
    //   in from_natural_prec_round
    test("2", 1, Nearest, "2.0", "0x2.0#1", Equal);
    // - rm == Exact in from_natural_prec_round
    test("2", 1, Exact, "2.0", "0x2.0#1", Equal);
    // - (rm == Floor || rm == Down) && inexact in from_natural_prec_round
    test("3", 1, Down, "2.0", "0x2.0#1", Less);
    // - (rm == Ceiling || rm == Up) && inexact in from_natural_prec_round
    // - (rm == Ceiling || rm == Up) && significand.limb_count() > original_limb_count in
    //   from_natural_prec_round
    test("3", 1, Up, "4.0", "0x4.0#1", Greater);
    // - rm == Nearest && half_bit && (inexact_after_half || x.get_bit(bits - prec)) in
    //   from_natural_prec_round
    // - rm == Nearest && half_bit && (inexact_after_half || x.get_bit(bits - prec)) &&
    //   significand.limb_count() > original_limb_count in from_natural_prec_round
    test("3", 1, Nearest, "4.0", "0x4.0#1", Greater);
    // - rm == Nearest && (!half_bit || !inexact_after_half && !x.get_bit(bits - prec) && inexact in
    //   from_natural_prec_round
    test("5", 1, Nearest, "4.0", "0x4.0#1", Less);
    // - (rm == Ceiling || rm == Up) && significand.limb_count() <= original_limb_count in
    //   from_natural_prec_round
    test("5", 2, Up, "6.0", "0x6.0#2", Greater);
    // - rm == Nearest && half_bit && (inexact_after_half || x.get_bit(bits - prec)) &&
    //   significand.limb_count() <= original_limb_count in from_natural_prec_round
    test("11", 2, Nearest, "1.0e1", "0xc.0#2", Greater);
    // - needed_bits != 0 in from_natural_prec_round
    // - mask_width >= Limb::WIDTH in from_natural_prec_round
    test(
        "10524811972430560515843",
        15,
        Floor,
        "1.05244e22",
        "0x2.3a88E+18#15",
        Less,
    );
    let test_big = |u: Natural, prec, rm, out, out_hex, out_o| {
        let (x, o) = Float::from_natural_prec_round(u.clone(), prec, rm);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let (x, o) = Float::from_natural_prec_round_ref(&u, prec, rm);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_x, o) =
                rug::Float::with_val_round(u32::exact_from(prec), rug::Integer::from(&u), rm);
            let x = Float::exact_from(&rug_x);
            assert_eq!(x.to_string(), out);
            assert_eq!(to_hex_string(&x), out_hex);
            assert_eq!(o, out_o);
        }
    };
    test_big(
        Natural::power_of_2(1000),
        10,
        Floor,
        "1.072e301",
        "0x1.000E+250#10",
        Equal,
    );
    test_big(
        Natural::power_of_2(1000),
        10,
        Ceiling,
        "1.072e301",
        "0x1.000E+250#10",
        Equal,
    );
    test_big(
        Natural::power_of_2(1000),
        10,
        Down,
        "1.072e301",
        "0x1.000E+250#10",
        Equal,
    );
    test_big(
        Natural::power_of_2(1000),
        10,
        Up,
        "1.072e301",
        "0x1.000E+250#10",
        Equal,
    );
    test_big(
        Natural::power_of_2(1000),
        10,
        Nearest,
        "1.072e301",
        "0x1.000E+250#10",
        Equal,
    );
    test_big(
        Natural::power_of_2(1000),
        10,
        Exact,
        "1.072e301",
        "0x1.000E+250#10",
        Equal,
    );

    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT)),
        10,
        Floor,
        "too_big",
        "0x7.feE+268435455#10",
        Less,
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT)),
        10,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT)),
        10,
        Down,
        "too_big",
        "0x7.feE+268435455#10",
        Less,
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT)),
        10,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT)),
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );

    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 1),
        10,
        Floor,
        "too_big",
        "0x4.00E+268435455#10",
        Equal,
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 1),
        10,
        Ceiling,
        "too_big",
        "0x4.00E+268435455#10",
        Equal,
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 1),
        10,
        Down,
        "too_big",
        "0x4.00E+268435455#10",
        Equal,
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 1),
        10,
        Up,
        "too_big",
        "0x4.00E+268435455#10",
        Equal,
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 1),
        10,
        Nearest,
        "too_big",
        "0x4.00E+268435455#10",
        Equal,
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 1),
        10,
        Exact,
        "too_big",
        "0x4.00E+268435455#10",
        Equal,
    );

    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 2) * Natural::from(3u8),
        1,
        Floor,
        "too_big",
        "0x4.0E+268435455#1",
        Less,
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 2) * Natural::from(3u8),
        1,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 2) * Natural::from(3u8),
        1,
        Down,
        "too_big",
        "0x4.0E+268435455#1",
        Less,
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 2) * Natural::from(3u8),
        1,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 2) * Natural::from(3u8),
        1,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );

    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 12) * Natural::from(3073u16),
        1,
        Floor,
        "too_big",
        "0x4.0E+268435455#1",
        Less,
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 12) * Natural::from(3073u16),
        1,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 12) * Natural::from(3073u16),
        1,
        Down,
        "too_big",
        "0x4.0E+268435455#1",
        Less,
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 12) * Natural::from(3073u16),
        1,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 12) * Natural::from(3073u16),
        1,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );

    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 12) * Natural::from(3071u16),
        1,
        Floor,
        "too_big",
        "0x4.0E+268435455#1",
        Less,
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 12) * Natural::from(3071u16),
        1,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 12) * Natural::from(3071u16),
        1,
        Down,
        "too_big",
        "0x4.0E+268435455#1",
        Less,
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 12) * Natural::from(3071u16),
        1,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 12) * Natural::from(3071u16),
        1,
        Nearest,
        "too_big",
        "0x4.0E+268435455#1",
        Less,
    );
}

#[test]
fn from_natural_prec_round_fail() {
    assert_panic!(Float::from_natural_prec_round(Natural::ZERO, 0, Floor));
    assert_panic!(Float::from_natural_prec_round(Natural::ONE, 0, Floor));
    assert_panic!(Float::from_natural_prec_round(
        Natural::from(123u32),
        1,
        Exact
    ));
}

#[test]
fn from_natural_prec_round_ref_fail() {
    assert_panic!(Float::from_natural_prec_round_ref(&Natural::ZERO, 0, Floor));
    assert_panic!(Float::from_natural_prec_round_ref(&Natural::ONE, 0, Floor));
    assert_panic!(Float::from_natural_prec_round_ref(
        &Natural::from(123u32),
        1,
        Exact
    ));
}

#[allow(clippy::needless_borrow)]
#[test]
fn test_try_from_natural() {
    let test = |s, out, out_hex| {
        let x = Natural::from_str(s).unwrap();

        let of = Float::try_from(x.clone());
        assert!(of.as_ref().map_or(true, Float::is_valid));
        let ofs = of.as_ref().map(ToString::to_string);
        assert_eq!(ofs.as_ref().map(String::as_str), out);
        let ofs = of.map(|f| to_hex_string(&f));
        assert_eq!(ofs.as_ref().map(String::as_str), out_hex);

        let of = Float::try_from(&x);
        assert!(of.as_ref().map_or(true, Float::is_valid));
        let ofs = of.as_ref().map(ToString::to_string);
        assert_eq!(ofs.as_ref().map(String::as_str), out);
        let ofs = of.map(|f| to_hex_string(&f));
        assert_eq!(ofs.as_ref().map(String::as_str), out_hex);
    };
    test("0", Ok("0.0"), Ok("0x0.0"));
    test("1", Ok("1.0"), Ok("0x1.0#1"));
    test("123", Ok("123.0"), Ok("0x7b.0#7"));
    test("1000000000000", Ok("1.0e12"), Ok("0xe.8d4a51E+9#28"));

    let test_big = |x: Natural, out, out_hex| {
        let of = Float::try_from(x.clone());
        assert!(of.as_ref().map_or(true, Float::is_valid));
        let ofs = of.as_ref().map(ToString::to_string);
        assert_eq!(ofs.as_ref().map(String::as_str), out);
        let ofs = of.map(|f| to_hex_string(&f));
        assert_eq!(ofs.as_ref().map(String::as_str), out_hex);

        let of = Float::try_from(&x);
        assert!(of.as_ref().map_or(true, Float::is_valid));
        let ofs = of.as_ref().map(ToString::to_string);
        assert_eq!(ofs.as_ref().map(String::as_str), out);
        let ofs = of.map(|f| to_hex_string(&f));
        assert_eq!(ofs.as_ref().map(String::as_str), out_hex);
    };
    test_big(Natural::power_of_2(1000), Ok("1.0e301"), Ok("0x1.0E+250#1"));
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT)),
        Err(&&FloatConversionError::Overflow),
        Err(&&FloatConversionError::Overflow),
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 1),
        Ok("too_big"),
        Ok("0x4.0E+268435455#1"),
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 2) * Natural::from(3u8),
        Ok("too_big"),
        Ok("0x6.0E+268435455#2"),
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 12) * Natural::from(3073u16),
        Ok("too_big"),
        Ok("0x6.008E+268435455#12"),
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 12) * Natural::from(3071u16),
        Ok("too_big"),
        Ok("0x5.ff8E+268435455#12"),
    );
}

#[test]
fn test_convertible_from_natural() {
    let test = |s, out| {
        let x = Natural::from_str(s).unwrap();
        assert_eq!(Float::convertible_from(&x), out);
    };
    test("0", true);
    test("1", true);
    test("123", true);

    let test_big = |x: Natural, out| {
        assert_eq!(Float::convertible_from(&x), out);
    };
    test_big(Natural::power_of_2(1000), true);
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT)),
        false,
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT)) - Natural::ONE,
        true,
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 1),
        true,
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 2) * Natural::from(3u8),
        true,
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 12) * Natural::from(3073u16),
        true,
    );
    test_big(
        Natural::power_of_2(u64::exact_from(Float::MAX_EXPONENT) - 12) * Natural::from(3071u16),
        true,
    );
}

#[test]
fn from_natural_prec_properties() {
    natural_unsigned_pair_gen_var_7().test_properties(|(n, prec)| {
        let (float_n, o) = Float::from_natural_prec(n.clone(), prec);
        assert!(float_n.is_valid());

        let (float_n_alt, o_alt) = Float::from_natural_prec_ref(&n, prec);
        assert!(float_n_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&float_n_alt),
            ComparableFloatRef(&float_n)
        );
        assert_eq!(o, o_alt);
        assert_eq!(float_n.partial_cmp(&n), Some(o));

        let rug_n = rug::Float::with_val(u32::exact_from(prec), rug::Integer::from(&n));
        assert_eq!(
            ComparableFloatRef(&float_n),
            ComparableFloatRef(&Float::from(&rug_n))
        );

        let (float_n_alt, o_alt) = Float::from_integer_prec(Integer::from(&n), prec);
        assert_eq!(
            ComparableFloatRef(&float_n_alt),
            ComparableFloatRef(&float_n)
        );
        assert_eq!(o_alt, o);

        assert_eq!(
            float_n.get_prec(),
            if n == 0u32 { None } else { Some(prec) }
        );
    });
}

#[test]
fn from_natural_prec_round_properties() {
    natural_unsigned_rounding_mode_triple_gen_var_2().test_properties(|(n, prec, rm)| {
        let (float_n, o) = Float::from_natural_prec_round(n.clone(), prec, rm);
        assert!(float_n.is_valid());

        let (float_n_alt, o_alt) = Float::from_natural_prec_round_ref(&n, prec, rm);
        assert!(float_n_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&float_n_alt),
            ComparableFloatRef(&float_n)
        );
        assert_eq!(o, o_alt);
        assert_eq!(float_n.partial_cmp(&n), Some(o));
        match rm {
            Floor | Down => {
                assert_ne!(o, Greater);
            }
            Ceiling | Up => {
                assert_ne!(o, Less);
            }
            Exact => assert_eq!(o, Equal),
            _ => {}
        }

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_n, rug_o) =
                rug::Float::with_val_round(u32::exact_from(prec), rug::Integer::from(&n), rm);
            assert_eq!(
                ComparableFloatRef(&float_n),
                ComparableFloatRef(&Float::from(&rug_n))
            );
            assert_eq!(rug_o, o);
        }

        let (float_n_alt, o_alt) = Float::from_integer_prec_round(Integer::from(&n), prec, rm);
        assert_eq!(
            ComparableFloatRef(&float_n_alt),
            ComparableFloatRef(&float_n)
        );
        assert_eq!(o_alt, o);

        assert_eq!(
            float_n.get_prec(),
            if n == 0u32 { None } else { Some(prec) }
        );
    });

    natural_unsigned_pair_gen_var_7().test_properties(|(n, prec)| {
        let floor = Float::from_natural_prec_round_ref(&n, prec, Floor);
        let r_floor = Rational::exact_from(&floor.0);
        assert!(r_floor <= n);
        if r_floor != 0u32 {
            assert!(r_floor + Rational::exact_from(floor.0.ulp().unwrap()) > n);
        }
        let (floor_alt, floor_o_alt) = Float::from_natural_prec_round_ref(&n, prec, Down);
        assert_eq!(ComparableFloatRef(&floor_alt), ComparableFloatRef(&floor.0));
        assert_eq!(floor_o_alt, floor.1);

        let ceiling = Float::from_natural_prec_round_ref(&n, prec, Ceiling);
        let r_ceiling = Rational::exact_from(&ceiling.0);
        assert!(r_ceiling >= n);
        if r_ceiling != 0u32 {
            assert!(r_ceiling - Rational::exact_from(ceiling.0.ulp().unwrap()) < n);
        }
        let (ceiling_alt, ceiling_o_alt) = Float::from_natural_prec_round_ref(&n, prec, Up);
        assert_eq!(
            ComparableFloatRef(&ceiling_alt),
            ComparableFloatRef(&ceiling.0)
        );
        assert_eq!(ceiling_o_alt, ceiling.1);

        let nearest = Float::from_natural_prec_round_ref(&n, prec, Nearest);
        assert!(
            ComparableFloatRef(&nearest.0) == ComparableFloatRef(&floor.0) && nearest.1 == floor.1
                || ComparableFloatRef(&nearest.0) == ComparableFloatRef(&ceiling.0)
                    && nearest.1 == ceiling.1
        );
        let r_nearest = Rational::exact_from(&nearest.0);
        if r_nearest != 0u32 {
            assert!(
                (r_nearest - Rational::from(&n))
                    .le_abs(&(Rational::exact_from(nearest.0.ulp().unwrap()) >> 1))
            );
        }
    });
}

#[test]
fn float_try_from_natural_properties() {
    natural_gen().test_properties(|x| {
        let of = Float::try_from(&x);
        assert!(of.as_ref().map_or(true, Float::is_valid));
        assert_eq!(
            Float::try_from(x.clone()).map(ComparableFloat),
            of.clone().map(ComparableFloat)
        );
        if let Ok(f) = of {
            assert_eq!(-x, -f);
        }
    });
}

#[test]
fn float_convertible_from_natural_properties() {
    natural_gen().test_properties(|x| {
        assert_eq!(Float::convertible_from(&x), Float::try_from(&x).is_ok());
    });
}
