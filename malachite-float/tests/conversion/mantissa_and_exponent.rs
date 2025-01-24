// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Abs, DivisibleByPowerOf2, Parity};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeZero, One, Zero,
};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{
    ExactFrom, IntegerMantissaAndExponent, RawMantissaAndExponent, SciMantissaAndExponent,
};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    primitive_float_gen_var_12, primitive_float_signed_pair_gen_var_3,
};
use malachite_float::test_util::common::{parse_hex_string, to_hex_string};
use malachite_float::test_util::generators::{
    float_gen_var_13, float_gen_var_3, float_rounding_mode_pair_gen,
    float_rounding_mode_pair_gen_var_21, float_signed_pair_gen_var_1,
};
use malachite_float::{ComparableFloatRef, Float};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_signed_pair_gen_var_2, natural_signed_pair_gen_var_4,
};
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_sci_mantissa_and_exponent_round() {
    fn test<T: PrimitiveFloat>(s: &str, s_hex: &str, rm: RoundingMode, out: &str) {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let meo: Option<(T, i32, Ordering)> = x.sci_mantissa_and_exponent_round(rm);
        if let Some((m, e, o)) = meo {
            assert_eq!(format!("Some({}, {}, {:?})", NiceFloat(m), e, o), out);
        } else {
            assert_eq!("None", out);
        }
    }
    test::<f32>("NaN", "NaN", Floor, "None");
    test::<f32>("NaN", "NaN", Ceiling, "None");
    test::<f32>("NaN", "NaN", Down, "None");
    test::<f32>("NaN", "NaN", Up, "None");
    test::<f32>("NaN", "NaN", Nearest, "None");
    test::<f32>("NaN", "NaN", Exact, "None");

    test::<f32>("Infinity", "Infinity", Floor, "None");
    test::<f32>("Infinity", "Infinity", Ceiling, "None");
    test::<f32>("Infinity", "Infinity", Down, "None");
    test::<f32>("Infinity", "Infinity", Up, "None");
    test::<f32>("Infinity", "Infinity", Nearest, "None");
    test::<f32>("Infinity", "Infinity", Exact, "None");

    test::<f32>("-Infinity", "-Infinity", Floor, "None");
    test::<f32>("-Infinity", "-Infinity", Ceiling, "None");
    test::<f32>("-Infinity", "-Infinity", Down, "None");
    test::<f32>("-Infinity", "-Infinity", Up, "None");
    test::<f32>("-Infinity", "-Infinity", Nearest, "None");
    test::<f32>("-Infinity", "-Infinity", Exact, "None");

    test::<f32>("0.0", "0x0.0", Floor, "None");
    test::<f32>("0.0", "0x0.0", Ceiling, "None");
    test::<f32>("0.0", "0x0.0", Down, "None");
    test::<f32>("0.0", "0x0.0", Up, "None");
    test::<f32>("0.0", "0x0.0", Nearest, "None");
    test::<f32>("0.0", "0x0.0", Exact, "None");

    test::<f32>("-0.0", "-0x0.0", Floor, "None");
    test::<f32>("-0.0", "-0x0.0", Ceiling, "None");
    test::<f32>("-0.0", "-0x0.0", Down, "None");
    test::<f32>("-0.0", "-0x0.0", Up, "None");
    test::<f32>("-0.0", "-0x0.0", Nearest, "None");
    test::<f32>("-0.0", "-0x0.0", Exact, "None");

    test::<f32>("1.0", "0x1.0#1", Floor, "Some(1.0, 0, Equal)");
    test::<f32>("1.0", "0x1.0#1", Ceiling, "Some(1.0, 0, Equal)");
    test::<f32>("1.0", "0x1.0#1", Down, "Some(1.0, 0, Equal)");
    test::<f32>("1.0", "0x1.0#1", Up, "Some(1.0, 0, Equal)");
    test::<f32>("1.0", "0x1.0#1", Nearest, "Some(1.0, 0, Equal)");
    test::<f32>("1.0", "0x1.0#1", Exact, "Some(1.0, 0, Equal)");

    test::<f32>("123.0", "0x7b.0#7", Floor, "Some(1.921875, 6, Equal)");
    test::<f32>("123.0", "0x7b.0#7", Ceiling, "Some(1.921875, 6, Equal)");
    test::<f32>("123.0", "0x7b.0#7", Down, "Some(1.921875, 6, Equal)");
    test::<f32>("123.0", "0x7b.0#7", Up, "Some(1.921875, 6, Equal)");
    test::<f32>("123.0", "0x7b.0#7", Nearest, "Some(1.921875, 6, Equal)");
    test::<f32>("123.0", "0x7b.0#7", Exact, "Some(1.921875, 6, Equal)");

    test::<f32>(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Floor,
        "Some(1.3333333, -2, Less)",
    );
    test::<f32>(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Ceiling,
        "Some(1.3333334, -2, Greater)",
    );
    test::<f32>(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Down,
        "Some(1.3333333, -2, Less)",
    );
    test::<f32>(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Up,
        "Some(1.3333334, -2, Greater)",
    );
    test::<f32>(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Nearest,
        "Some(1.3333334, -2, Greater)",
    );
    test::<f32>(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Exact,
        "None",
    );

    test::<f32>(
        "3.141592653589793",
        "0x3.243f6a8885a3#50",
        Floor,
        "Some(1.5707963, 1, Less)",
    );
    test::<f32>(
        "3.141592653589793",
        "0x3.243f6a8885a3#50",
        Ceiling,
        "Some(1.5707964, 1, Greater)",
    );
    test::<f32>(
        "3.141592653589793",
        "0x3.243f6a8885a3#50",
        Down,
        "Some(1.5707963, 1, Less)",
    );
    test::<f32>(
        "3.141592653589793",
        "0x3.243f6a8885a3#50",
        Up,
        "Some(1.5707964, 1, Greater)",
    );
    test::<f32>(
        "3.141592653589793",
        "0x3.243f6a8885a3#50",
        Nearest,
        "Some(1.5707964, 1, Greater)",
    );
    test::<f32>("3.141592653589793", "0x3.243f6a8885a3#50", Exact, "None");

    test::<f32>(
        "1000000000000.0",
        "0xe8d4a51000.0#40",
        Floor,
        "Some(1.8189894, 39, Less)",
    );
    test::<f32>(
        "1000000000000.0",
        "0xe8d4a51000.0#40",
        Ceiling,
        "Some(1.8189895, 39, Greater)",
    );
    test::<f32>(
        "1000000000000.0",
        "0xe8d4a51000.0#40",
        Down,
        "Some(1.8189894, 39, Less)",
    );
    test::<f32>(
        "1000000000000.0",
        "0xe8d4a51000.0#40",
        Up,
        "Some(1.8189895, 39, Greater)",
    );
    test::<f32>(
        "1000000000000.0",
        "0xe8d4a51000.0#40",
        Nearest,
        "Some(1.8189894, 39, Less)",
    );
    test::<f32>("1000000000000.0", "0xe8d4a51000.0#40", Exact, "None");

    test::<f32>(
        "1000000000000000000000000.0",
        "0xd3c21bcecceda1000000.0#80",
        Floor,
        "Some(1.6543611, 79, Less)",
    );
    test::<f32>(
        "1000000000000000000000000.0",
        "0xd3c21bcecceda1000000.0#80",
        Ceiling,
        "Some(1.6543612, 79, Greater)",
    );
    test::<f32>(
        "1000000000000000000000000.0",
        "0xd3c21bcecceda1000000.0#80",
        Down,
        "Some(1.6543611, 79, Less)",
    );
    test::<f32>(
        "1000000000000000000000000.0",
        "0xd3c21bcecceda1000000.0#80",
        Up,
        "Some(1.6543612, 79, Greater)",
    );
    test::<f32>(
        "1000000000000000000000000.0",
        "0xd3c21bcecceda1000000.0#80",
        Nearest,
        "Some(1.6543612, 79, Greater)",
    );
    test::<f32>(
        "1000000000000000000000000.0",
        "0xd3c21bcecceda1000000.0#80",
        Exact,
        "None",
    );

    test::<f32>(
        "0.999999999999999999999999",
        "0x0.ffffffffffffffffffff#80",
        Floor,
        "Some(1.9999999, -1, Less)",
    );
    test::<f32>(
        "0.999999999999999999999999",
        "0x0.ffffffffffffffffffff#80",
        Ceiling,
        "Some(1.0, 0, Greater)",
    );
    test::<f32>(
        "0.999999999999999999999999",
        "0x0.ffffffffffffffffffff#80",
        Down,
        "Some(1.9999999, -1, Less)",
    );
    test::<f32>(
        "0.999999999999999999999999",
        "0x0.ffffffffffffffffffff#80",
        Up,
        "Some(1.0, 0, Greater)",
    );
    test::<f32>(
        "0.999999999999999999999999",
        "0x0.ffffffffffffffffffff#80",
        Nearest,
        "Some(1.0, 0, Greater)",
    );
    test::<f32>(
        "0.999999999999999999999999",
        "0x0.ffffffffffffffffffff#80",
        Exact,
        "None",
    );

    test::<f32>(
        "too_big",
        "0x4.0E+268435455#1",
        Floor,
        "Some(1.0, 1073741822, Equal)",
    );
    test::<f32>(
        "too_big",
        "0x4.0E+268435455#1",
        Ceiling,
        "Some(1.0, 1073741822, Equal)",
    );
    test::<f32>(
        "too_big",
        "0x4.0E+268435455#1",
        Down,
        "Some(1.0, 1073741822, Equal)",
    );
    test::<f32>(
        "too_big",
        "0x4.0E+268435455#1",
        Up,
        "Some(1.0, 1073741822, Equal)",
    );
    test::<f32>(
        "too_big",
        "0x4.0E+268435455#1",
        Nearest,
        "Some(1.0, 1073741822, Equal)",
    );
    test::<f32>(
        "too_big",
        "0x4.0E+268435455#1",
        Exact,
        "Some(1.0, 1073741822, Equal)",
    );

    test::<f32>(
        "too_big",
        "0x4.000001E+268435455#27",
        Floor,
        "Some(1.0, 1073741822, Less)",
    );
    test::<f32>(
        "too_big",
        "0x4.000001E+268435455#27",
        Ceiling,
        "Some(1.0000001, 1073741822, Greater)",
    );
    test::<f32>(
        "too_big",
        "0x4.000001E+268435455#27",
        Down,
        "Some(1.0, 1073741822, Less)",
    );
    test::<f32>(
        "too_big",
        "0x4.000001E+268435455#27",
        Up,
        "Some(1.0000001, 1073741822, Greater)",
    );
    test::<f32>(
        "too_big",
        "0x4.000001E+268435455#27",
        Nearest,
        "Some(1.0, 1073741822, Less)",
    );
    test::<f32>("too_big", "0x4.000001E+268435455#27", Exact, "None");

    test::<f32>(
        "too_small",
        "0x1.0E-268435456#1",
        Floor,
        "Some(1.0, -1073741824, Equal)",
    );
    test::<f32>(
        "too_small",
        "0x1.0E-268435456#1",
        Ceiling,
        "Some(1.0, -1073741824, Equal)",
    );
    test::<f32>(
        "too_small",
        "0x1.0E-268435456#1",
        Down,
        "Some(1.0, -1073741824, Equal)",
    );
    test::<f32>(
        "too_small",
        "0x1.0E-268435456#1",
        Up,
        "Some(1.0, -1073741824, Equal)",
    );
    test::<f32>(
        "too_small",
        "0x1.0E-268435456#1",
        Nearest,
        "Some(1.0, -1073741824, Equal)",
    );
    test::<f32>(
        "too_small",
        "0x1.0E-268435456#1",
        Exact,
        "Some(1.0, -1073741824, Equal)",
    );

    test::<f32>("-1.0", "-0x1.0#1", Floor, "Some(1.0, 0, Equal)");
    test::<f32>("-1.0", "-0x1.0#1", Ceiling, "Some(1.0, 0, Equal)");
    test::<f32>("-1.0", "-0x1.0#1", Down, "Some(1.0, 0, Equal)");
    test::<f32>("-1.0", "-0x1.0#1", Up, "Some(1.0, 0, Equal)");
    test::<f32>("-1.0", "-0x1.0#1", Nearest, "Some(1.0, 0, Equal)");
    test::<f32>("-1.0", "-0x1.0#1", Exact, "Some(1.0, 0, Equal)");

    test::<f32>("-123.0", "-0x7b.0#7", Floor, "Some(1.921875, 6, Equal)");
    test::<f32>("-123.0", "-0x7b.0#7", Ceiling, "Some(1.921875, 6, Equal)");
    test::<f32>("-123.0", "-0x7b.0#7", Down, "Some(1.921875, 6, Equal)");
    test::<f32>("-123.0", "-0x7b.0#7", Up, "Some(1.921875, 6, Equal)");
    test::<f32>("-123.0", "-0x7b.0#7", Nearest, "Some(1.921875, 6, Equal)");
    test::<f32>("-123.0", "-0x7b.0#7", Exact, "Some(1.921875, 6, Equal)");

    test::<f32>(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        Floor,
        "Some(1.3333333, -2, Less)",
    );
    test::<f32>(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        Ceiling,
        "Some(1.3333334, -2, Greater)",
    );
    test::<f32>(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        Down,
        "Some(1.3333333, -2, Less)",
    );
    test::<f32>(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        Up,
        "Some(1.3333334, -2, Greater)",
    );
    test::<f32>(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        Nearest,
        "Some(1.3333334, -2, Greater)",
    );
    test::<f32>(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        Exact,
        "None",
    );

    test::<f32>(
        "-3.141592653589793",
        "-0x3.243f6a8885a3#50",
        Floor,
        "Some(1.5707963, 1, Less)",
    );
    test::<f32>(
        "-3.141592653589793",
        "-0x3.243f6a8885a3#50",
        Ceiling,
        "Some(1.5707964, 1, Greater)",
    );
    test::<f32>(
        "-3.141592653589793",
        "-0x3.243f6a8885a3#50",
        Down,
        "Some(1.5707963, 1, Less)",
    );
    test::<f32>(
        "-3.141592653589793",
        "-0x3.243f6a8885a3#50",
        Up,
        "Some(1.5707964, 1, Greater)",
    );
    test::<f32>(
        "-3.141592653589793",
        "-0x3.243f6a8885a3#50",
        Nearest,
        "Some(1.5707964, 1, Greater)",
    );
    test::<f32>("-3.141592653589793", "-0x3.243f6a8885a3#50", Exact, "None");

    test::<f32>(
        "-1000000000000.0",
        "-0xe8d4a51000.0#40",
        Floor,
        "Some(1.8189894, 39, Less)",
    );
    test::<f32>(
        "-1000000000000.0",
        "-0xe8d4a51000.0#40",
        Ceiling,
        "Some(1.8189895, 39, Greater)",
    );
    test::<f32>(
        "-1000000000000.0",
        "-0xe8d4a51000.0#40",
        Down,
        "Some(1.8189894, 39, Less)",
    );
    test::<f32>(
        "-1000000000000.0",
        "-0xe8d4a51000.0#40",
        Up,
        "Some(1.8189895, 39, Greater)",
    );
    test::<f32>(
        "-1000000000000.0",
        "-0xe8d4a51000.0#40",
        Nearest,
        "Some(1.8189894, 39, Less)",
    );
    test::<f32>("-1000000000000.0", "-0xe8d4a51000.0#40", Exact, "None");

    test::<f32>(
        "-1000000000000000000000000.0",
        "-0xd3c21bcecceda1000000.0#80",
        Floor,
        "Some(1.6543611, 79, Less)",
    );
    test::<f32>(
        "-1000000000000000000000000.0",
        "-0xd3c21bcecceda1000000.0#80",
        Ceiling,
        "Some(1.6543612, 79, Greater)",
    );
    test::<f32>(
        "-1000000000000000000000000.0",
        "-0xd3c21bcecceda1000000.0#80",
        Down,
        "Some(1.6543611, 79, Less)",
    );
    test::<f32>(
        "-1000000000000000000000000.0",
        "-0xd3c21bcecceda1000000.0#80",
        Up,
        "Some(1.6543612, 79, Greater)",
    );
    test::<f32>(
        "-1000000000000000000000000.0",
        "-0xd3c21bcecceda1000000.0#80",
        Nearest,
        "Some(1.6543612, 79, Greater)",
    );
    test::<f32>(
        "-1000000000000000000000000.0",
        "-0xd3c21bcecceda1000000.0#80",
        Exact,
        "None",
    );

    test::<f32>(
        "-0.999999999999999999999999",
        "-0x0.ffffffffffffffffffff#80",
        Floor,
        "Some(1.9999999, -1, Less)",
    );
    test::<f32>(
        "-0.999999999999999999999999",
        "-0x0.ffffffffffffffffffff#80",
        Ceiling,
        "Some(1.0, 0, Greater)",
    );
    test::<f32>(
        "-0.999999999999999999999999",
        "-0x0.ffffffffffffffffffff#80",
        Down,
        "Some(1.9999999, -1, Less)",
    );
    test::<f32>(
        "-0.999999999999999999999999",
        "-0x0.ffffffffffffffffffff#80",
        Up,
        "Some(1.0, 0, Greater)",
    );
    test::<f32>(
        "-0.999999999999999999999999",
        "-0x0.ffffffffffffffffffff#80",
        Nearest,
        "Some(1.0, 0, Greater)",
    );
    test::<f32>(
        "-0.999999999999999999999999",
        "-0x0.ffffffffffffffffffff#80",
        Exact,
        "None",
    );

    test::<f32>(
        "-too_big",
        "-0x4.0E+268435455#1",
        Floor,
        "Some(1.0, 1073741822, Equal)",
    );
    test::<f32>(
        "-too_big",
        "-0x4.0E+268435455#1",
        Ceiling,
        "Some(1.0, 1073741822, Equal)",
    );
    test::<f32>(
        "-too_big",
        "-0x4.0E+268435455#1",
        Down,
        "Some(1.0, 1073741822, Equal)",
    );
    test::<f32>(
        "-too_big",
        "-0x4.0E+268435455#1",
        Up,
        "Some(1.0, 1073741822, Equal)",
    );
    test::<f32>(
        "-too_big",
        "-0x4.0E+268435455#1",
        Nearest,
        "Some(1.0, 1073741822, Equal)",
    );
    test::<f32>(
        "-too_big",
        "-0x4.0E+268435455#1",
        Exact,
        "Some(1.0, 1073741822, Equal)",
    );

    test::<f32>(
        "-too_big",
        "-0x4.000001E+268435455#27",
        Floor,
        "Some(1.0, 1073741822, Less)",
    );
    test::<f32>(
        "-too_big",
        "-0x4.000001E+268435455#27",
        Ceiling,
        "Some(1.0000001, 1073741822, Greater)",
    );
    test::<f32>(
        "-too_big",
        "-0x4.000001E+268435455#27",
        Down,
        "Some(1.0, 1073741822, Less)",
    );
    test::<f32>(
        "-too_big",
        "-0x4.000001E+268435455#27",
        Up,
        "Some(1.0000001, 1073741822, Greater)",
    );
    test::<f32>(
        "-too_big",
        "-0x4.000001E+268435455#27",
        Nearest,
        "Some(1.0, 1073741822, Less)",
    );
    test::<f32>("-too_big", "-0x4.000001E+268435455#27", Exact, "None");

    test::<f32>(
        "-too_small",
        "-0x1.0E-268435456#1",
        Floor,
        "Some(1.0, -1073741824, Equal)",
    );
    test::<f32>(
        "-too_small",
        "-0x1.0E-268435456#1",
        Ceiling,
        "Some(1.0, -1073741824, Equal)",
    );
    test::<f32>(
        "-too_small",
        "-0x1.0E-268435456#1",
        Down,
        "Some(1.0, -1073741824, Equal)",
    );
    test::<f32>(
        "-too_small",
        "-0x1.0E-268435456#1",
        Up,
        "Some(1.0, -1073741824, Equal)",
    );
    test::<f32>(
        "-too_small",
        "-0x1.0E-268435456#1",
        Nearest,
        "Some(1.0, -1073741824, Equal)",
    );
    test::<f32>(
        "-too_small",
        "-0x1.0E-268435456#1",
        Exact,
        "Some(1.0, -1073741824, Equal)",
    );

    test::<f64>("NaN", "NaN", Floor, "None");
    test::<f64>("NaN", "NaN", Ceiling, "None");
    test::<f64>("NaN", "NaN", Down, "None");
    test::<f64>("NaN", "NaN", Up, "None");
    test::<f64>("NaN", "NaN", Nearest, "None");
    test::<f64>("NaN", "NaN", Exact, "None");

    test::<f64>("Infinity", "Infinity", Floor, "None");
    test::<f64>("Infinity", "Infinity", Ceiling, "None");
    test::<f64>("Infinity", "Infinity", Down, "None");
    test::<f64>("Infinity", "Infinity", Up, "None");
    test::<f64>("Infinity", "Infinity", Nearest, "None");
    test::<f64>("Infinity", "Infinity", Exact, "None");

    test::<f64>("-Infinity", "-Infinity", Floor, "None");
    test::<f64>("-Infinity", "-Infinity", Ceiling, "None");
    test::<f64>("-Infinity", "-Infinity", Down, "None");
    test::<f64>("-Infinity", "-Infinity", Up, "None");
    test::<f64>("-Infinity", "-Infinity", Nearest, "None");
    test::<f64>("-Infinity", "-Infinity", Exact, "None");

    test::<f64>("0.0", "0x0.0", Floor, "None");
    test::<f64>("0.0", "0x0.0", Ceiling, "None");
    test::<f64>("0.0", "0x0.0", Down, "None");
    test::<f64>("0.0", "0x0.0", Up, "None");
    test::<f64>("0.0", "0x0.0", Nearest, "None");
    test::<f64>("0.0", "0x0.0", Exact, "None");

    test::<f64>("-0.0", "-0x0.0", Floor, "None");
    test::<f64>("-0.0", "-0x0.0", Ceiling, "None");
    test::<f64>("-0.0", "-0x0.0", Down, "None");
    test::<f64>("-0.0", "-0x0.0", Up, "None");
    test::<f64>("-0.0", "-0x0.0", Nearest, "None");
    test::<f64>("-0.0", "-0x0.0", Exact, "None");

    test::<f64>("1.0", "0x1.0#1", Floor, "Some(1.0, 0, Equal)");
    test::<f64>("1.0", "0x1.0#1", Ceiling, "Some(1.0, 0, Equal)");
    test::<f64>("1.0", "0x1.0#1", Down, "Some(1.0, 0, Equal)");
    test::<f64>("1.0", "0x1.0#1", Up, "Some(1.0, 0, Equal)");
    test::<f64>("1.0", "0x1.0#1", Nearest, "Some(1.0, 0, Equal)");
    test::<f64>("1.0", "0x1.0#1", Exact, "Some(1.0, 0, Equal)");

    test::<f64>("123.0", "0x7b.0#7", Floor, "Some(1.921875, 6, Equal)");
    test::<f64>("123.0", "0x7b.0#7", Ceiling, "Some(1.921875, 6, Equal)");
    test::<f64>("123.0", "0x7b.0#7", Down, "Some(1.921875, 6, Equal)");
    test::<f64>("123.0", "0x7b.0#7", Up, "Some(1.921875, 6, Equal)");
    test::<f64>("123.0", "0x7b.0#7", Nearest, "Some(1.921875, 6, Equal)");
    test::<f64>("123.0", "0x7b.0#7", Exact, "Some(1.921875, 6, Equal)");

    test::<f64>(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Floor,
        "Some(1.3333333333333333, -2, Equal)",
    );
    test::<f64>(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Ceiling,
        "Some(1.3333333333333333, -2, Equal)",
    );
    test::<f64>(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Down,
        "Some(1.3333333333333333, -2, Equal)",
    );
    test::<f64>(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Up,
        "Some(1.3333333333333333, -2, Equal)",
    );
    test::<f64>(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Nearest,
        "Some(1.3333333333333333, -2, Equal)",
    );
    test::<f64>(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Exact,
        "Some(1.3333333333333333, -2, Equal)",
    );

    test::<f64>(
        "3.141592653589793",
        "0x3.243f6a8885a3#50",
        Floor,
        "Some(1.5707963267948966, 1, Equal)",
    );
    test::<f64>(
        "3.141592653589793",
        "0x3.243f6a8885a3#50",
        Ceiling,
        "Some(1.5707963267948966, 1, Equal)",
    );
    test::<f64>(
        "3.141592653589793",
        "0x3.243f6a8885a3#50",
        Down,
        "Some(1.5707963267948966, 1, Equal)",
    );
    test::<f64>(
        "3.141592653589793",
        "0x3.243f6a8885a3#50",
        Up,
        "Some(1.5707963267948966, 1, Equal)",
    );
    test::<f64>(
        "3.141592653589793",
        "0x3.243f6a8885a3#50",
        Nearest,
        "Some(1.5707963267948966, 1, Equal)",
    );
    test::<f64>(
        "3.141592653589793",
        "0x3.243f6a8885a3#50",
        Exact,
        "Some(1.5707963267948966, 1, Equal)",
    );

    test::<f64>(
        "1000000000000.0",
        "0xe8d4a51000.0#40",
        Floor,
        "Some(1.8189894035458565, 39, Equal)",
    );
    test::<f64>(
        "1000000000000.0",
        "0xe8d4a51000.0#40",
        Ceiling,
        "Some(1.8189894035458565, 39, Equal)",
    );
    test::<f64>(
        "1000000000000.0",
        "0xe8d4a51000.0#40",
        Down,
        "Some(1.8189894035458565, 39, Equal)",
    );
    test::<f64>(
        "1000000000000.0",
        "0xe8d4a51000.0#40",
        Up,
        "Some(1.8189894035458565, 39, Equal)",
    );
    test::<f64>(
        "1000000000000.0",
        "0xe8d4a51000.0#40",
        Nearest,
        "Some(1.8189894035458565, 39, Equal)",
    );
    test::<f64>(
        "1000000000000.0",
        "0xe8d4a51000.0#40",
        Exact,
        "Some(1.8189894035458565, 39, Equal)",
    );

    test::<f64>(
        "1000000000000000000000000.0",
        "0xd3c21bcecceda1000000.0#80",
        Floor,
        "Some(1.6543612251060553, 79, Less)",
    );
    test::<f64>(
        "1000000000000000000000000.0",
        "0xd3c21bcecceda1000000.0#80",
        Ceiling,
        "Some(1.6543612251060555, 79, Greater)",
    );
    test::<f64>(
        "1000000000000000000000000.0",
        "0xd3c21bcecceda1000000.0#80",
        Down,
        "Some(1.6543612251060553, 79, Less)",
    );
    test::<f64>(
        "1000000000000000000000000.0",
        "0xd3c21bcecceda1000000.0#80",
        Up,
        "Some(1.6543612251060555, 79, Greater)",
    );
    test::<f64>(
        "1000000000000000000000000.0",
        "0xd3c21bcecceda1000000.0#80",
        Nearest,
        "Some(1.6543612251060553, 79, Less)",
    );
    test::<f64>(
        "1000000000000000000000000.0",
        "0xd3c21bcecceda1000000.0#80",
        Exact,
        "None",
    );

    test::<f64>(
        "0.999999999999999999999999",
        "0x0.ffffffffffffffffffff#80",
        Floor,
        "Some(1.9999999999999998, -1, Less)",
    );
    test::<f64>(
        "0.999999999999999999999999",
        "0x0.ffffffffffffffffffff#80",
        Ceiling,
        "Some(1.0, 0, Greater)",
    );
    test::<f64>(
        "0.999999999999999999999999",
        "0x0.ffffffffffffffffffff#80",
        Down,
        "Some(1.9999999999999998, -1, Less)",
    );
    test::<f64>(
        "0.999999999999999999999999",
        "0x0.ffffffffffffffffffff#80",
        Up,
        "Some(1.0, 0, Greater)",
    );
    test::<f64>(
        "0.999999999999999999999999",
        "0x0.ffffffffffffffffffff#80",
        Nearest,
        "Some(1.0, 0, Greater)",
    );
    test::<f64>(
        "0.999999999999999999999999",
        "0x0.ffffffffffffffffffff#80",
        Exact,
        "None",
    );

    test::<f64>(
        "too_big",
        "0x4.0E+268435455#1",
        Floor,
        "Some(1.0, 1073741822, Equal)",
    );
    test::<f64>(
        "too_big",
        "0x4.0E+268435455#1",
        Ceiling,
        "Some(1.0, 1073741822, Equal)",
    );
    test::<f64>(
        "too_big",
        "0x4.0E+268435455#1",
        Down,
        "Some(1.0, 1073741822, Equal)",
    );
    test::<f64>(
        "too_big",
        "0x4.0E+268435455#1",
        Up,
        "Some(1.0, 1073741822, Equal)",
    );
    test::<f64>(
        "too_big",
        "0x4.0E+268435455#1",
        Nearest,
        "Some(1.0, 1073741822, Equal)",
    );
    test::<f64>(
        "too_big",
        "0x4.0E+268435455#1",
        Exact,
        "Some(1.0, 1073741822, Equal)",
    );

    test::<f64>(
        "too_big",
        "0x4.0000000000001E+268435455#55",
        Floor,
        "Some(1.0, 1073741822, Less)",
    );
    test::<f64>(
        "too_big",
        "0x4.0000000000001E+268435455#55",
        Ceiling,
        "Some(1.0000000000000002, 1073741822, Greater)",
    );
    test::<f64>(
        "too_big",
        "0x4.0000000000001E+268435455#55",
        Down,
        "Some(1.0, 1073741822, Less)",
    );
    test::<f64>(
        "too_big",
        "0x4.0000000000001E+268435455#55",
        Up,
        "Some(1.0000000000000002, 1073741822, Greater)",
    );
    test::<f64>(
        "too_big",
        "0x4.0000000000001E+268435455#55",
        Nearest,
        "Some(1.0, 1073741822, Less)",
    );
    test::<f64>("too_big", "0x4.0000000000001E+268435455#55", Exact, "None");

    test::<f64>(
        "too_small",
        "0x1.0E-268435456#1",
        Floor,
        "Some(1.0, -1073741824, Equal)",
    );
    test::<f64>(
        "too_small",
        "0x1.0E-268435456#1",
        Ceiling,
        "Some(1.0, -1073741824, Equal)",
    );
    test::<f64>(
        "too_small",
        "0x1.0E-268435456#1",
        Down,
        "Some(1.0, -1073741824, Equal)",
    );
    test::<f64>(
        "too_small",
        "0x1.0E-268435456#1",
        Up,
        "Some(1.0, -1073741824, Equal)",
    );
    test::<f64>(
        "too_small",
        "0x1.0E-268435456#1",
        Nearest,
        "Some(1.0, -1073741824, Equal)",
    );
    test::<f64>(
        "too_small",
        "0x1.0E-268435456#1",
        Exact,
        "Some(1.0, -1073741824, Equal)",
    );

    test::<f64>("-1.0", "-0x1.0#1", Floor, "Some(1.0, 0, Equal)");
    test::<f64>("-1.0", "-0x1.0#1", Ceiling, "Some(1.0, 0, Equal)");
    test::<f64>("-1.0", "-0x1.0#1", Down, "Some(1.0, 0, Equal)");
    test::<f64>("-1.0", "-0x1.0#1", Up, "Some(1.0, 0, Equal)");
    test::<f64>("-1.0", "-0x1.0#1", Nearest, "Some(1.0, 0, Equal)");
    test::<f64>("-1.0", "-0x1.0#1", Exact, "Some(1.0, 0, Equal)");

    test::<f64>("-123.0", "-0x7b.0#7", Floor, "Some(1.921875, 6, Equal)");
    test::<f64>("-123.0", "-0x7b.0#7", Ceiling, "Some(1.921875, 6, Equal)");
    test::<f64>("-123.0", "-0x7b.0#7", Down, "Some(1.921875, 6, Equal)");
    test::<f64>("-123.0", "-0x7b.0#7", Up, "Some(1.921875, 6, Equal)");
    test::<f64>("-123.0", "-0x7b.0#7", Nearest, "Some(1.921875, 6, Equal)");
    test::<f64>("-123.0", "-0x7b.0#7", Exact, "Some(1.921875, 6, Equal)");

    test::<f64>(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        Floor,
        "Some(1.3333333333333333, -2, Equal)",
    );
    test::<f64>(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        Ceiling,
        "Some(1.3333333333333333, -2, Equal)",
    );
    test::<f64>(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        Down,
        "Some(1.3333333333333333, -2, Equal)",
    );
    test::<f64>(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        Up,
        "Some(1.3333333333333333, -2, Equal)",
    );
    test::<f64>(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        Nearest,
        "Some(1.3333333333333333, -2, Equal)",
    );
    test::<f64>(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        Exact,
        "Some(1.3333333333333333, -2, Equal)",
    );

    test::<f64>(
        "-3.141592653589793",
        "-0x3.243f6a8885a3#50",
        Floor,
        "Some(1.5707963267948966, 1, Equal)",
    );
    test::<f64>(
        "-3.141592653589793",
        "-0x3.243f6a8885a3#50",
        Ceiling,
        "Some(1.5707963267948966, 1, Equal)",
    );
    test::<f64>(
        "-3.141592653589793",
        "-0x3.243f6a8885a3#50",
        Down,
        "Some(1.5707963267948966, 1, Equal)",
    );
    test::<f64>(
        "-3.141592653589793",
        "-0x3.243f6a8885a3#50",
        Up,
        "Some(1.5707963267948966, 1, Equal)",
    );
    test::<f64>(
        "-3.141592653589793",
        "-0x3.243f6a8885a3#50",
        Nearest,
        "Some(1.5707963267948966, 1, Equal)",
    );
    test::<f64>(
        "-3.141592653589793",
        "-0x3.243f6a8885a3#50",
        Exact,
        "Some(1.5707963267948966, 1, Equal)",
    );

    test::<f64>(
        "-1000000000000.0",
        "-0xe8d4a51000.0#40",
        Floor,
        "Some(1.8189894035458565, 39, Equal)",
    );
    test::<f64>(
        "-1000000000000.0",
        "-0xe8d4a51000.0#40",
        Ceiling,
        "Some(1.8189894035458565, 39, Equal)",
    );
    test::<f64>(
        "-1000000000000.0",
        "-0xe8d4a51000.0#40",
        Down,
        "Some(1.8189894035458565, 39, Equal)",
    );
    test::<f64>(
        "-1000000000000.0",
        "-0xe8d4a51000.0#40",
        Up,
        "Some(1.8189894035458565, 39, Equal)",
    );
    test::<f64>(
        "-1000000000000.0",
        "-0xe8d4a51000.0#40",
        Nearest,
        "Some(1.8189894035458565, 39, Equal)",
    );
    test::<f64>(
        "-1000000000000.0",
        "-0xe8d4a51000.0#40",
        Exact,
        "Some(1.8189894035458565, 39, Equal)",
    );

    test::<f64>(
        "-1000000000000000000000000.0",
        "-0xd3c21bcecceda1000000.0#80",
        Floor,
        "Some(1.6543612251060553, 79, Less)",
    );
    test::<f64>(
        "-1000000000000000000000000.0",
        "-0xd3c21bcecceda1000000.0#80",
        Ceiling,
        "Some(1.6543612251060555, 79, Greater)",
    );
    test::<f64>(
        "-1000000000000000000000000.0",
        "-0xd3c21bcecceda1000000.0#80",
        Down,
        "Some(1.6543612251060553, 79, Less)",
    );
    test::<f64>(
        "-1000000000000000000000000.0",
        "-0xd3c21bcecceda1000000.0#80",
        Up,
        "Some(1.6543612251060555, 79, Greater)",
    );
    test::<f64>(
        "-1000000000000000000000000.0",
        "-0xd3c21bcecceda1000000.0#80",
        Nearest,
        "Some(1.6543612251060553, 79, Less)",
    );
    test::<f64>(
        "-1000000000000000000000000.0",
        "-0xd3c21bcecceda1000000.0#80",
        Exact,
        "None",
    );

    test::<f64>(
        "-0.999999999999999999999999",
        "-0x0.ffffffffffffffffffff#80",
        Floor,
        "Some(1.9999999999999998, -1, Less)",
    );
    test::<f64>(
        "-0.999999999999999999999999",
        "-0x0.ffffffffffffffffffff#80",
        Ceiling,
        "Some(1.0, 0, Greater)",
    );
    test::<f64>(
        "-0.999999999999999999999999",
        "-0x0.ffffffffffffffffffff#80",
        Down,
        "Some(1.9999999999999998, -1, Less)",
    );
    test::<f64>(
        "-0.999999999999999999999999",
        "-0x0.ffffffffffffffffffff#80",
        Up,
        "Some(1.0, 0, Greater)",
    );
    test::<f64>(
        "-0.999999999999999999999999",
        "-0x0.ffffffffffffffffffff#80",
        Nearest,
        "Some(1.0, 0, Greater)",
    );
    test::<f64>(
        "-0.999999999999999999999999",
        "-0x0.ffffffffffffffffffff#80",
        Exact,
        "None",
    );

    test::<f64>(
        "-too_big",
        "-0x4.0E+268435455#1",
        Floor,
        "Some(1.0, 1073741822, Equal)",
    );
    test::<f64>(
        "-too_big",
        "-0x4.0E+268435455#1",
        Ceiling,
        "Some(1.0, 1073741822, Equal)",
    );
    test::<f64>(
        "-too_big",
        "-0x4.0E+268435455#1",
        Down,
        "Some(1.0, 1073741822, Equal)",
    );
    test::<f64>(
        "-too_big",
        "-0x4.0E+268435455#1",
        Up,
        "Some(1.0, 1073741822, Equal)",
    );
    test::<f64>(
        "-too_big",
        "-0x4.0E+268435455#1",
        Nearest,
        "Some(1.0, 1073741822, Equal)",
    );
    test::<f64>(
        "-too_big",
        "-0x4.0E+268435455#1",
        Exact,
        "Some(1.0, 1073741822, Equal)",
    );

    test::<f64>(
        "-too_big",
        "-0x4.0000000000001E+268435455#55",
        Floor,
        "Some(1.0, 1073741822, Less)",
    );
    test::<f64>(
        "-too_big",
        "-0x4.0000000000001E+268435455#55",
        Ceiling,
        "Some(1.0000000000000002, 1073741822, Greater)",
    );
    test::<f64>(
        "-too_big",
        "-0x4.0000000000001E+268435455#55",
        Down,
        "Some(1.0, 1073741822, Less)",
    );
    test::<f64>(
        "-too_big",
        "-0x4.0000000000001E+268435455#55",
        Up,
        "Some(1.0000000000000002, 1073741822, Greater)",
    );
    test::<f64>(
        "-too_big",
        "-0x4.0000000000001E+268435455#55",
        Nearest,
        "Some(1.0, 1073741822, Less)",
    );
    test::<f64>(
        "-too_big",
        "-0x4.0000000000001E+268435455#55",
        Exact,
        "None",
    );

    test::<f64>(
        "-too_small",
        "-0x1.0E-268435456#1",
        Floor,
        "Some(1.0, -1073741824, Equal)",
    );
    test::<f64>(
        "-too_small",
        "-0x1.0E-268435456#1",
        Ceiling,
        "Some(1.0, -1073741824, Equal)",
    );
    test::<f64>(
        "-too_small",
        "-0x1.0E-268435456#1",
        Down,
        "Some(1.0, -1073741824, Equal)",
    );
    test::<f64>(
        "-too_small",
        "-0x1.0E-268435456#1",
        Up,
        "Some(1.0, -1073741824, Equal)",
    );
    test::<f64>(
        "-too_small",
        "-0x1.0E-268435456#1",
        Nearest,
        "Some(1.0, -1073741824, Equal)",
    );
    test::<f64>(
        "-too_small",
        "-0x1.0E-268435456#1",
        Exact,
        "Some(1.0, -1073741824, Equal)",
    );
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_raw_mantissa_and_exponent() {
    let test = |s, s_hex, m_out, e_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (m, e) = x.clone().raw_mantissa_and_exponent();
        assert_eq!(m.to_string(), m_out);
        assert_eq!(e, e_out);

        let (m_alt, e_alt) = (&x).raw_mantissa_and_exponent();
        assert_eq!(m_alt, m);
        assert_eq!(e_alt, e);

        assert_eq!(x.clone().raw_mantissa(), m);
        assert_eq!((&x).raw_mantissa(), m);
        assert_eq!(x.clone().raw_exponent(), e);
        assert_eq!((&x).raw_exponent(), e);
    };
    test("1.0", "0x1.0#1", "9223372036854775808", 1);
    test("2.0", "0x2.0#1", "9223372036854775808", 2);
    test("0.5", "0x0.8#1", "9223372036854775808", 0);
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "12297829382473033728",
        -1,
    );
    test("123.0", "0x7b.0#7", "17726168133330272256", 7);
    test(
        "1000000000000.0",
        "0xe8d4a51000.0#40",
        "16777216000000000000",
        40,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "13043817825332783104",
        1,
    );
    test(
        "3.141592653589793",
        "0x3.243f6a8885a3#50",
        "14488038916154245120",
        2,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        "12535862302449813504",
        2,
    );
    test(
        "1000000000000000000000000.0",
        "0xd3c21bcecceda1000000.0#80",
        "281474976710656000000000000000000000000",
        80,
    );
    test(
        "too_big",
        "0x4.0E+268435455#1",
        "9223372036854775808",
        1073741823,
    );
    test(
        "too_small",
        "0x1.0E-268435456#1",
        "9223372036854775808",
        -1073741823,
    );

    test("-1.0", "-0x1.0#1", "9223372036854775808", 1);
    test("-2.0", "-0x2.0#1", "9223372036854775808", 2);
    test("-0.5", "-0x0.8#1", "9223372036854775808", 0);
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        "12297829382473033728",
        -1,
    );
    test("-123.0", "-0x7b.0#7", "17726168133330272256", 7);
    test(
        "-1000000000000.0",
        "-0xe8d4a51000.0#40",
        "16777216000000000000",
        40,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "13043817825332783104",
        1,
    );
    test(
        "-3.141592653589793",
        "-0x3.243f6a8885a3#50",
        "14488038916154245120",
        2,
    );
    test(
        "-2.7182818284590451",
        "-0x2.b7e151628aed2#53",
        "12535862302449813504",
        2,
    );
    test(
        "-1000000000000000000000000.0",
        "-0xd3c21bcecceda1000000.0#80",
        "281474976710656000000000000000000000000",
        80,
    );
    test(
        "-too_big",
        "-0x4.0E+268435455#1",
        "9223372036854775808",
        1073741823,
    );
    test(
        "-too_small",
        "-0x1.0E-268435456#1",
        "9223372036854775808",
        -1073741823,
    );
}

#[test]
fn raw_mantissa_and_exponent_fail() {
    assert_panic!(Float::NAN.raw_mantissa_and_exponent());
    assert_panic!(Float::INFINITY.raw_mantissa_and_exponent());
    assert_panic!(Float::NEGATIVE_INFINITY.raw_mantissa_and_exponent());
    assert_panic!(Float::ZERO.raw_mantissa_and_exponent());
    assert_panic!(Float::NEGATIVE_ZERO.raw_mantissa_and_exponent());

    assert_panic!(Float::NAN.raw_mantissa());
    assert_panic!(Float::INFINITY.raw_mantissa());
    assert_panic!(Float::NEGATIVE_INFINITY.raw_mantissa());
    assert_panic!(Float::ZERO.raw_mantissa());
    assert_panic!(Float::NEGATIVE_ZERO.raw_mantissa());

    assert_panic!(Float::NAN.raw_exponent());
    assert_panic!(Float::INFINITY.raw_exponent());
    assert_panic!(Float::NEGATIVE_INFINITY.raw_exponent());
    assert_panic!(Float::ZERO.raw_exponent());
    assert_panic!(Float::NEGATIVE_ZERO.raw_exponent());
}

#[test]
fn raw_mantissa_and_exponent_ref_fail() {
    assert_panic!((&Float::NAN).raw_mantissa_and_exponent());
    assert_panic!((&Float::INFINITY).raw_mantissa_and_exponent());
    assert_panic!((&Float::NEGATIVE_INFINITY).raw_mantissa_and_exponent());
    assert_panic!((&Float::ZERO).raw_mantissa_and_exponent());
    assert_panic!((&Float::NEGATIVE_ZERO).raw_mantissa_and_exponent());

    assert_panic!((&Float::NAN).raw_mantissa());
    assert_panic!((&Float::INFINITY).raw_mantissa());
    assert_panic!((&Float::NEGATIVE_INFINITY).raw_mantissa());
    assert_panic!((&Float::ZERO).raw_mantissa());
    assert_panic!((&Float::NEGATIVE_ZERO).raw_mantissa());

    assert_panic!((&Float::NAN).raw_exponent());
    assert_panic!((&Float::INFINITY).raw_exponent());
    assert_panic!((&Float::NEGATIVE_INFINITY).raw_exponent());
    assert_panic!((&Float::ZERO).raw_exponent());
    assert_panic!((&Float::NEGATIVE_ZERO).raw_exponent());
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_from_raw_mantissa_and_exponent() {
    let test = |m, e, out, out_hex| {
        let m = Natural::from_str(m).unwrap();
        let x = Float::from_raw_mantissa_and_exponent(m.clone(), e);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);

        let x_alt =
            <&Float as RawMantissaAndExponent<_, _, _>>::from_raw_mantissa_and_exponent(m, e);
        assert!(x_alt.is_valid());
        assert_eq!(x_alt, x);
    };
    test("9223372036854775808", 1, "1.0", "0x1.0#1");
    test(
        "9223372036854775809",
        1,
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
    );
    test(
        "18446744073709551615",
        1,
        "1.9999999999999999999",
        "0x1.fffffffffffffffe#64",
    );
    test(
        "170141183460469231731687303715884105728",
        1,
        "1.0",
        "0x1.0000000000000000#65",
    );

    test("9223372036854775808", 2, "2.0", "0x2.0#1");
    test("9223372036854775808", 0, "0.5", "0x0.8#1");
    test(
        "12297829382473033728",
        -1,
        "0.33333333333333331",
        "0x0.55555555555554#53",
    );
    test("17726168133330272256", 7, "123.0", "0x7b.0#7");
    test("16777216000000000000", 40, "1.0e12", "0xe.8d4a51E+9#28");
    test(
        "13043817825332783104",
        1,
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
    );
    test(
        "14488038916154245120",
        2,
        "3.141592653589793",
        "0x3.243f6a8885a3#50",
    );
    test(
        "12535862302449813504",
        2,
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
    );
    test(
        "281474976710656000000000000000000000000",
        80,
        "1.0e24",
        "0xd.3c21bcecceda1000E+19#65",
    );
    test(
        "9223372036854775808",
        1073741823,
        "too_big",
        "0x4.0E+268435455#1",
    );
    test(
        "9223372036854775808",
        -1073741823,
        "too_small",
        "0x1.0E-268435456#1",
    );
}

#[test]
fn from_raw_mantissa_and_exponent_float_fail() {
    assert_panic!(Float::from_raw_mantissa_and_exponent(Natural::ZERO, 0));
    assert_panic!(Float::from_raw_mantissa_and_exponent(Natural::ONE, 0));
    assert_panic!(Float::from_raw_mantissa_and_exponent(
        Natural::from_str("9223372036854775808").unwrap(),
        1073741824
    ));
    assert_panic!(Float::from_raw_mantissa_and_exponent(
        Natural::from_str("9223372036854775808").unwrap(),
        -1073741824
    ));

    assert_panic!(
        <&Float as RawMantissaAndExponent<_, _, _>>::from_raw_mantissa_and_exponent(
            Natural::ZERO,
            0
        )
    );
    assert_panic!(
        <&Float as RawMantissaAndExponent<_, _, _>>::from_raw_mantissa_and_exponent(
            Natural::ONE,
            0
        )
    );
    assert_panic!(
        <&Float as RawMantissaAndExponent<_, _, _>>::from_raw_mantissa_and_exponent(
            Natural::from_str("9223372036854775808").unwrap(),
            1073741824
        )
    );
    assert_panic!(
        <&Float as RawMantissaAndExponent<_, _, _>>::from_raw_mantissa_and_exponent(
            Natural::from_str("9223372036854775808").unwrap(),
            -1073741824
        )
    );
}

#[test]
fn test_integer_mantissa_and_exponent() {
    let test = |s, s_hex, m_out, e_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (m, e) = x.clone().integer_mantissa_and_exponent();
        assert_eq!(m.to_string(), m_out);
        assert_eq!(e, e_out);

        let (m_alt, e_alt) = (&x).integer_mantissa_and_exponent();
        assert_eq!(m_alt, m);
        assert_eq!(e_alt, e);

        assert_eq!(x.clone().integer_mantissa(), m);
        assert_eq!((&x).integer_mantissa(), m);
        assert_eq!(x.clone().integer_exponent(), e);
        assert_eq!((&x).integer_exponent(), e);
    };
    test("1.0", "0x1.0#1", "1", 0);
    test("2.0", "0x2.0#1", "1", 1);
    test("0.5", "0x0.8#1", "1", -1);
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "6004799503160661",
        -54,
    );
    test("123.0", "0x7b.0#7", "123", 0);
    test("1000000000000.0", "0xe8d4a51000.0#40", "244140625", 12);
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "6369051672525773",
        -52,
    );
    test(
        "3.141592653589793",
        "0x3.243f6a8885a3#50",
        "884279719003555",
        -48,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        "6121026514868073",
        -51,
    );
    test(
        "1000000000000000000000000.0",
        "0xd3c21bcecceda1000000.0#80",
        "59604644775390625",
        24,
    );
    test("too_big", "0x4.0E+268435455#1", "1", 1073741822);
    test("too_small", "0x1.0E-268435456#1", "1", -1073741824);
    test(
        "too_small",
        "0x1.0000001E-268435456#29",
        "268435457",
        -1073741852,
    );

    test("-1.0", "-0x1.0#1", "1", 0);
    test("-2.0", "-0x2.0#1", "1", 1);
    test("-0.5", "-0x0.8#1", "1", -1);
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        "6004799503160661",
        -54,
    );
    test("-123.0", "-0x7b.0#7", "123", 0);
    test("-1000000000000.0", "-0xe8d4a51000.0#40", "244140625", 12);
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "6369051672525773",
        -52,
    );
    test(
        "-3.141592653589793",
        "-0x3.243f6a8885a3#50",
        "884279719003555",
        -48,
    );
    test(
        "-2.7182818284590451",
        "-0x2.b7e151628aed2#53",
        "6121026514868073",
        -51,
    );
    test(
        "-1000000000000000000000000.0",
        "-0xd3c21bcecceda1000000.0#80",
        "59604644775390625",
        24,
    );
    test("-too_big", "-0x4.0E+268435455#1", "1", 1073741822);
    test("-too_small", "-0x1.0E-268435456#1", "1", -1073741824);
    test(
        "-too_small",
        "-0x1.0000001E-268435456#29",
        "268435457",
        -1073741852,
    );
}

#[test]
fn integer_mantissa_and_exponent_fail() {
    assert_panic!(Float::NAN.integer_mantissa_and_exponent());
    assert_panic!(Float::INFINITY.integer_mantissa_and_exponent());
    assert_panic!(Float::NEGATIVE_INFINITY.integer_mantissa_and_exponent());
    assert_panic!(Float::ZERO.integer_mantissa_and_exponent());
    assert_panic!(Float::NEGATIVE_ZERO.integer_mantissa_and_exponent());

    assert_panic!(Float::NAN.integer_mantissa());
    assert_panic!(Float::INFINITY.integer_mantissa());
    assert_panic!(Float::NEGATIVE_INFINITY.integer_mantissa());
    assert_panic!(Float::ZERO.integer_mantissa());
    assert_panic!(Float::NEGATIVE_ZERO.integer_mantissa());

    assert_panic!(Float::NAN.integer_exponent());
    assert_panic!(Float::INFINITY.integer_exponent());
    assert_panic!(Float::NEGATIVE_INFINITY.integer_exponent());
    assert_panic!(Float::ZERO.integer_exponent());
    assert_panic!(Float::NEGATIVE_ZERO.integer_exponent());
}

#[test]
fn integer_mantissa_and_exponent_ref_fail() {
    assert_panic!((&Float::NAN).integer_mantissa_and_exponent());
    assert_panic!((&Float::INFINITY).integer_mantissa_and_exponent());
    assert_panic!((&Float::NEGATIVE_INFINITY).integer_mantissa_and_exponent());
    assert_panic!((&Float::ZERO).integer_mantissa_and_exponent());
    assert_panic!((&Float::NEGATIVE_ZERO).integer_mantissa_and_exponent());

    assert_panic!((&Float::NAN).integer_mantissa());
    assert_panic!((&Float::INFINITY).integer_mantissa());
    assert_panic!((&Float::NEGATIVE_INFINITY).integer_mantissa());
    assert_panic!((&Float::ZERO).integer_mantissa());
    assert_panic!((&Float::NEGATIVE_ZERO).integer_mantissa());

    assert_panic!((&Float::NAN).integer_exponent());
    assert_panic!((&Float::INFINITY).integer_exponent());
    assert_panic!((&Float::NEGATIVE_INFINITY).integer_exponent());
    assert_panic!((&Float::ZERO).integer_exponent());
    assert_panic!((&Float::NEGATIVE_ZERO).integer_exponent());
}

#[test]
fn test_from_integer_mantissa_and_exponent() {
    let test = |m, e, out, out_hex| {
        let m = Natural::from_str(m).unwrap();
        let ox = Float::from_integer_mantissa_and_exponent(m.clone(), e);
        assert!(ox.as_ref().map_or(true, Float::is_valid));
        let os = ox.as_ref().map(ToString::to_string);
        assert_eq!(os.as_deref(), out);
        let os = ox.as_ref().map(to_hex_string);
        assert_eq!(os.as_deref(), out_hex);

        let ox_alt =
            <&Float as IntegerMantissaAndExponent<_, _, _>>::from_integer_mantissa_and_exponent(
                m, e,
            );
        assert!(ox_alt.as_ref().map_or(true, Float::is_valid));
        assert_eq!(ox_alt, ox);
    };
    test("0", 0, Some("0.0"), Some("0x0.0"));
    test("0", 10, Some("0.0"), Some("0x0.0"));
    test("0", -10, Some("0.0"), Some("0x0.0"));
    test("1", 0, Some("1.0"), Some("0x1.0#1"));
    test("1", 1, Some("2.0"), Some("0x2.0#1"));
    test("1", -1, Some("0.5"), Some("0x0.8#1"));
    test("2", -1, Some("1.0"), Some("0x1.0#1"));
    test("2", 0, Some("2.0"), Some("0x2.0#1"));
    test("2", -2, Some("0.5"), Some("0x0.8#1"));
    test(
        "6004799503160661",
        -54,
        Some("0.33333333333333331"),
        Some("0x0.55555555555554#53"),
    );
    test("123", 0, Some("123.0"), Some("0x7b.0#7"));
    test("244140625", 12, Some("1.0e12"), Some("0xe.8d4a51E+9#28"));
    test(
        "6369051672525773",
        -52,
        Some("1.4142135623730951"),
        Some("0x1.6a09e667f3bcd#53"),
    );
    test(
        "884279719003555",
        -48,
        Some("3.141592653589793"),
        Some("0x3.243f6a8885a3#50"),
    );
    test(
        "6121026514868073",
        -51,
        Some("2.7182818284590451"),
        Some("0x2.b7e151628aed2#53"),
    );
    test(
        "59604644775390625",
        24,
        Some("1.0e24"),
        Some("0xd.3c21bcecceda1E+19#56"),
    );
    test("1", 1073741822, Some("too_big"), Some("0x4.0E+268435455#1"));
    test(
        "1",
        -1073741824,
        Some("too_small"),
        Some("0x1.0E-268435456#1"),
    );
    test(
        "268435457",
        -1073741852,
        Some("too_small"),
        Some("0x1.0000001E-268435456#29"),
    );
    test("1", 1073741823, None, None);
    test("1", -1073741852, None, None);
}

#[test]
fn test_sci_mantissa_and_exponent_float() {
    let test = |s, s_hex, m_out, m_out_hex, e_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (m, e) = x.clone().sci_mantissa_and_exponent();
        assert!(m.is_valid());
        assert_eq!(m.to_string(), m_out);
        assert_eq!(to_hex_string(&m), m_out_hex);
        assert_eq!(e, e_out);

        let (m_alt, e_alt): (Float, i32) = (&x).sci_mantissa_and_exponent();
        assert!(m_alt.is_valid());
        assert_eq!(ComparableFloatRef(&m_alt), ComparableFloatRef(&m));
        assert_eq!(e_alt, e);

        let mantissa_alt: Float = x.clone().sci_mantissa();
        assert!(mantissa_alt.is_valid());
        assert_eq!(ComparableFloatRef(&mantissa_alt), ComparableFloatRef(&m));
        let mantissa_alt: Float = (&x).sci_mantissa();
        assert!(mantissa_alt.is_valid());
        assert_eq!(ComparableFloatRef(&mantissa_alt), ComparableFloatRef(&m));
        assert_eq!(x.clone().sci_exponent(), e);
        assert_eq!(
            <&Float as SciMantissaAndExponent<Float, _, _>>::sci_exponent(&x),
            e
        );
    };
    test("1.0", "0x1.0#1", "1.0", "0x1.0#1", 0);
    test("2.0", "0x2.0#1", "1.0", "0x1.0#1", 1);
    test("0.5", "0x0.8#1", "1.0", "0x1.0#1", -1);
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "1.3333333333333333",
        "0x1.5555555555555#53",
        -2,
    );
    test("123.0", "0x7b.0#7", "1.92", "0x1.ec#7", 6);
    test(
        "1000000000000.0",
        "0xe8d4a51000.0#40",
        "1.818989403546",
        "0x1.d1a94a2000#40",
        39,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        0,
    );
    test(
        "3.141592653589793",
        "0x3.243f6a8885a3#50",
        "1.570796326794897",
        "0x1.921fb54442d18#50",
        1,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        "1.3591409142295225",
        "0x1.5bf0a8b145769#53",
        1,
    );
    test(
        "1000000000000000000000000.0",
        "0xd3c21bcecceda1000000.0#80",
        "1.654361225106055349742817",
        "0x1.a784379d99db42000000#80",
        79,
    );
    test(
        "too_big",
        "0x4.0E+268435455#1",
        "1.0",
        "0x1.0#1",
        1073741822,
    );
    test(
        "too_small",
        "0x1.0E-268435456#1",
        "1.0",
        "0x1.0#1",
        -1073741824,
    );

    test("-1.0", "-0x1.0#1", "1.0", "0x1.0#1", 0);
    test("-2.0", "-0x2.0#1", "1.0", "0x1.0#1", 1);
    test("-0.5", "-0x0.8#1", "1.0", "0x1.0#1", -1);
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        "1.3333333333333333",
        "0x1.5555555555555#53",
        -2,
    );
    test("-123.0", "-0x7b.0#7", "1.92", "0x1.ec#7", 6);
    test(
        "-1000000000000.0",
        "-0xe8d4a51000.0#40",
        "1.818989403546",
        "0x1.d1a94a2000#40",
        39,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        0,
    );
    test(
        "-3.141592653589793",
        "-0x3.243f6a8885a3#50",
        "1.570796326794897",
        "0x1.921fb54442d18#50",
        1,
    );
    test(
        "-2.7182818284590451",
        "-0x2.b7e151628aed2#53",
        "1.3591409142295225",
        "0x1.5bf0a8b145769#53",
        1,
    );
    test(
        "-1000000000000000000000000.0",
        "-0xd3c21bcecceda1000000.0#80",
        "1.654361225106055349742817",
        "0x1.a784379d99db42000000#80",
        79,
    );
    test(
        "-too_big",
        "-0x4.0E+268435455#1",
        "1.0",
        "0x1.0#1",
        1073741822,
    );
    test(
        "-too_small",
        "-0x1.0E-268435456#1",
        "1.0",
        "0x1.0#1",
        -1073741824,
    );
}

#[test]
fn sci_mantissa_and_exponent_float_fail() {
    assert_panic!(Float::NAN.sci_mantissa_and_exponent());
    assert_panic!(Float::INFINITY.sci_mantissa_and_exponent());
    assert_panic!(Float::NEGATIVE_INFINITY.sci_mantissa_and_exponent());
    assert_panic!(Float::ZERO.sci_mantissa_and_exponent());
    assert_panic!(Float::NEGATIVE_ZERO.sci_mantissa_and_exponent());

    assert_panic!(Float::NAN.sci_mantissa());
    assert_panic!(Float::INFINITY.sci_mantissa());
    assert_panic!(Float::NEGATIVE_INFINITY.sci_mantissa());
    assert_panic!(Float::ZERO.sci_mantissa());
    assert_panic!(Float::NEGATIVE_ZERO.sci_mantissa());

    assert_panic!(Float::NAN.sci_exponent());
    assert_panic!(Float::INFINITY.sci_exponent());
    assert_panic!(Float::NEGATIVE_INFINITY.sci_exponent());
    assert_panic!(Float::ZERO.sci_exponent());
    assert_panic!(Float::NEGATIVE_ZERO.sci_exponent());
}

#[test]
fn sci_mantissa_and_exponent_float_ref_fail() {
    assert_panic!(
        <&Float as SciMantissaAndExponent<Float, _, _>>::sci_mantissa_and_exponent(&Float::NAN)
    );
    assert_panic!(
        <&Float as SciMantissaAndExponent<Float, _, _>>::sci_mantissa_and_exponent(
            &Float::INFINITY
        )
    );
    assert_panic!(
        <&Float as SciMantissaAndExponent<Float, _, _>>::sci_mantissa_and_exponent(
            &Float::NEGATIVE_INFINITY
        )
    );
    assert_panic!(
        <&Float as SciMantissaAndExponent<Float, _, _>>::sci_mantissa_and_exponent(&Float::ZERO)
    );
    assert_panic!(
        <&Float as SciMantissaAndExponent<Float, _, _>>::sci_mantissa_and_exponent(
            &Float::NEGATIVE_ZERO
        )
    );

    assert_panic!(<&Float as SciMantissaAndExponent<Float, _, _>>::sci_mantissa(&Float::NAN));
    assert_panic!(<&Float as SciMantissaAndExponent<Float, _, _>>::sci_mantissa(&Float::INFINITY));
    assert_panic!(
        <&Float as SciMantissaAndExponent<Float, _, _>>::sci_mantissa(&Float::NEGATIVE_INFINITY)
    );
    assert_panic!(<&Float as SciMantissaAndExponent<Float, _, _>>::sci_mantissa(&Float::ZERO));
    assert_panic!(
        <&Float as SciMantissaAndExponent<Float, _, _>>::sci_mantissa(&Float::NEGATIVE_ZERO)
    );

    assert_panic!(<&Float as SciMantissaAndExponent<Float, _, _>>::sci_exponent(&Float::NAN));
    assert_panic!(<&Float as SciMantissaAndExponent<Float, _, _>>::sci_exponent(&Float::INFINITY));
    assert_panic!(
        <&Float as SciMantissaAndExponent<Float, _, _>>::sci_exponent(&Float::NEGATIVE_INFINITY)
    );
    assert_panic!(<&Float as SciMantissaAndExponent<Float, _, _>>::sci_exponent(&Float::ZERO));
    assert_panic!(
        <&Float as SciMantissaAndExponent<Float, _, _>>::sci_exponent(&Float::NEGATIVE_ZERO)
    );
}

#[test]
fn test_from_sci_mantissa_and_exponent_float() {
    let test = |ms, ms_hex, e, out, out_hex| {
        let m = parse_hex_string(ms_hex);
        assert_eq!(m.to_string(), ms);
        let ox = Float::from_sci_mantissa_and_exponent(m.clone(), e);
        assert!(ox.as_ref().map_or(true, Float::is_valid));
        let os = ox.as_ref().map(ToString::to_string);
        assert_eq!(os.as_deref(), out);
        let os = ox.as_ref().map(to_hex_string);
        assert_eq!(os.as_deref(), out_hex);

        let ox_alt =
            <&Float as SciMantissaAndExponent<_, _, _>>::from_sci_mantissa_and_exponent(m, e);
        assert!(ox_alt.as_ref().map_or(true, Float::is_valid));
        assert_eq!(ox_alt, ox);
    };
    test("1.0", "0x1.0#1", 0, Some("1.0"), Some("0x1.0#1"));
    test("1.0", "0x1.0#1", 1, Some("2.0"), Some("0x2.0#1"));
    test("1.0", "0x1.0#1", -1, Some("0.5"), Some("0x0.8#1"));
    test(
        "1.3333333333333333",
        "0x1.5555555555555#53",
        -2,
        Some("0.33333333333333331"),
        Some("0x0.55555555555554#53"),
    );
    test("1.92", "0x1.ec#7", 6, Some("123.0"), Some("0x7b.0#7"));
    test(
        "1.818989403546",
        "0x1.d1a94a2000#40",
        39,
        Some("1000000000000.0"),
        Some("0xe8d4a51000.0#40"),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        0,
        Some("1.4142135623730951"),
        Some("0x1.6a09e667f3bcd#53"),
    );
    test(
        "1.570796326794897",
        "0x1.921fb54442d18#50",
        1,
        Some("3.141592653589793"),
        Some("0x3.243f6a8885a3#50"),
    );
    test(
        "1.3591409142295225",
        "0x1.5bf0a8b145769#53",
        1,
        Some("2.7182818284590451"),
        Some("0x2.b7e151628aed2#53"),
    );
    test(
        "1.654361225106055349742817",
        "0x1.a784379d99db42000000#80",
        79,
        Some("1000000000000000000000000.0"),
        Some("0xd3c21bcecceda1000000.0#80"),
    );
    test(
        "1.0",
        "0x1.0#1",
        1073741822,
        Some("too_big"),
        Some("0x4.0E+268435455#1"),
    );
    test(
        "1.0",
        "0x1.0#1",
        -1073741824,
        Some("too_small"),
        Some("0x1.0E-268435456#1"),
    );
    test("1.0", "0x1.0#1", 1073741823, None, None);
    test("1.0", "0x1.0#1", -1073741825, None, None);

    test("-1.0", "-0x1.0#1", 0, None, None);
    test("2.0", "0x2.0#1", 0, None, None);
    test("0.5", "0x0.8#1", 0, None, None);
}

#[test]
fn from_sci_mantissa_and_exponent_float_fail() {
    assert_panic!(Float::from_sci_mantissa_and_exponent(Float::NAN, 0));
    assert_panic!(Float::from_sci_mantissa_and_exponent(Float::INFINITY, 0));
    assert_panic!(Float::from_sci_mantissa_and_exponent(
        Float::NEGATIVE_INFINITY,
        0
    ));
    assert_panic!(Float::from_sci_mantissa_and_exponent(Float::ZERO, 0));
    assert_panic!(Float::from_sci_mantissa_and_exponent(
        Float::NEGATIVE_ZERO,
        0
    ));
}

#[test]
fn test_sci_mantissa_and_exponent_primitive_float() {
    fn test<T: PrimitiveFloat>(s: &str, s_hex: &str, m_out: &str, e_out: i32)
    where
        for<'a> &'a Float: SciMantissaAndExponent<T, i32, Float>,
    {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (m, e): (T, i32) = (&x).sci_mantissa_and_exponent();
        assert_eq!(NiceFloat(m).to_string(), m_out);
        assert_eq!(e, e_out);

        assert_eq!(NiceFloat((&x).sci_mantissa()), NiceFloat(m));
        assert_eq!(
            <&Float as SciMantissaAndExponent<T, _, _>>::sci_exponent(&x),
            e
        );
    }
    test::<f32>("1.0", "0x1.0#1", "1.0", 0);
    test::<f32>("2.0", "0x2.0#1", "1.0", 1);
    test::<f32>("0.5", "0x0.8#1", "1.0", -1);
    test::<f32>(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "1.3333334",
        -2,
    );
    test::<f32>("123.0", "0x7b.0#7", "1.921875", 6);
    test::<f32>("1000000000000.0", "0xe8d4a51000.0#40", "1.8189894", 39);
    test::<f32>("1.4142135623730951", "0x1.6a09e667f3bcd#53", "1.4142135", 0);
    test::<f32>("3.141592653589793", "0x3.243f6a8885a3#50", "1.5707964", 1);
    test::<f32>("2.7182818284590451", "0x2.b7e151628aed2#53", "1.3591409", 1);
    test::<f32>(
        "1000000000000000000000000.0",
        "0xd3c21bcecceda1000000.0#80",
        "1.6543612",
        79,
    );
    test::<f32>("too_big", "0x4.0E+268435455#1", "1.0", 1073741822);
    test::<f32>("too_small", "0x1.0E-268435456#1", "1.0", -1073741824);

    test::<f32>("-1.0", "-0x1.0#1", "1.0", 0);
    test::<f32>("-2.0", "-0x2.0#1", "1.0", 1);
    test::<f32>("-0.5", "-0x0.8#1", "1.0", -1);
    test::<f32>(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        "1.3333334",
        -2,
    );
    test::<f32>("-123.0", "-0x7b.0#7", "1.921875", 6);
    test::<f32>("-1000000000000.0", "-0xe8d4a51000.0#40", "1.8189894", 39);
    test::<f32>(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "1.4142135",
        0,
    );
    test::<f32>("-3.141592653589793", "-0x3.243f6a8885a3#50", "1.5707964", 1);
    test::<f32>(
        "-2.7182818284590451",
        "-0x2.b7e151628aed2#53",
        "1.3591409",
        1,
    );
    test::<f32>(
        "-1000000000000000000000000.0",
        "-0xd3c21bcecceda1000000.0#80",
        "1.6543612",
        79,
    );

    test::<f64>("1.0", "0x1.0#1", "1.0", 0);
    test::<f64>("2.0", "0x2.0#1", "1.0", 1);
    test::<f64>("0.5", "0x0.8#1", "1.0", -1);
    test::<f64>(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "1.3333333333333333",
        -2,
    );
    test::<f64>("123.0", "0x7b.0#7", "1.921875", 6);
    test::<f64>(
        "1000000000000.0",
        "0xe8d4a51000.0#40",
        "1.8189894035458565",
        39,
    );
    test::<f64>(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "1.4142135623730951",
        0,
    );
    test::<f64>(
        "3.141592653589793",
        "0x3.243f6a8885a3#50",
        "1.5707963267948966",
        1,
    );
    test::<f64>(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        "1.3591409142295225",
        1,
    );
    test::<f64>(
        "1000000000000000000000000.0",
        "0xd3c21bcecceda1000000.0#80",
        "1.6543612251060553",
        79,
    );

    test::<f64>("-1.0", "-0x1.0#1", "1.0", 0);
    test::<f64>("-2.0", "-0x2.0#1", "1.0", 1);
    test::<f64>("-0.5", "-0x0.8#1", "1.0", -1);
    test::<f64>(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        "1.3333333333333333",
        -2,
    );
    test::<f64>("-123.0", "-0x7b.0#7", "1.921875", 6);
    test::<f64>(
        "-1000000000000.0",
        "-0xe8d4a51000.0#40",
        "1.8189894035458565",
        39,
    );
    test::<f64>(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "1.4142135623730951",
        0,
    );
    test::<f64>(
        "-3.141592653589793",
        "-0x3.243f6a8885a3#50",
        "1.5707963267948966",
        1,
    );
    test::<f64>(
        "-2.7182818284590451",
        "-0x2.b7e151628aed2#53",
        "1.3591409142295225",
        1,
    );
    test::<f64>(
        "-1000000000000000000000000.0",
        "-0xd3c21bcecceda1000000.0#80",
        "1.6543612251060553",
        79,
    );
    test::<f32>("-too_big", "-0x4.0E+268435455#1", "1.0", 1073741822);
    test::<f32>("-too_small", "-0x1.0E-268435456#1", "1.0", -1073741824);
}

fn sci_mantissa_and_exponent_primitive_float_fail_helper<T: PrimitiveFloat>()
where
    for<'a> &'a Float: SciMantissaAndExponent<T, i32, Float>,
{
    assert_panic!(
        <&Float as SciMantissaAndExponent<T, _, _>>::sci_mantissa_and_exponent(&Float::NAN)
    );
    assert_panic!(
        <&Float as SciMantissaAndExponent<T, _, _>>::sci_mantissa_and_exponent(&Float::INFINITY)
    );
    assert_panic!(
        <&Float as SciMantissaAndExponent<T, _, _>>::sci_mantissa_and_exponent(
            &Float::NEGATIVE_INFINITY
        )
    );
    assert_panic!(
        <&Float as SciMantissaAndExponent<T, _, _>>::sci_mantissa_and_exponent(&Float::ZERO)
    );
    assert_panic!(
        <&Float as SciMantissaAndExponent<T, _, _>>::sci_mantissa_and_exponent(
            &Float::NEGATIVE_ZERO
        )
    );

    assert_panic!(<&Float as SciMantissaAndExponent<T, _, _>>::sci_mantissa(
        &Float::NAN
    ));
    assert_panic!(<&Float as SciMantissaAndExponent<T, _, _>>::sci_mantissa(
        &Float::INFINITY
    ));
    assert_panic!(<&Float as SciMantissaAndExponent<T, _, _>>::sci_mantissa(
        &Float::NEGATIVE_INFINITY
    ));
    assert_panic!(<&Float as SciMantissaAndExponent<T, _, _>>::sci_mantissa(
        &Float::ZERO
    ));
    assert_panic!(<&Float as SciMantissaAndExponent<T, _, _>>::sci_mantissa(
        &Float::NEGATIVE_ZERO
    ));

    assert_panic!(<&Float as SciMantissaAndExponent<T, _, _>>::sci_exponent(
        &Float::NAN
    ));
    assert_panic!(<&Float as SciMantissaAndExponent<T, _, _>>::sci_exponent(
        &Float::INFINITY
    ));
    assert_panic!(<&Float as SciMantissaAndExponent<T, _, _>>::sci_exponent(
        &Float::NEGATIVE_INFINITY
    ));
    assert_panic!(<&Float as SciMantissaAndExponent<T, _, _>>::sci_exponent(
        &Float::ZERO
    ));
    assert_panic!(<&Float as SciMantissaAndExponent<T, _, _>>::sci_exponent(
        &Float::NEGATIVE_ZERO
    ));
}

#[test]
fn sci_mantissa_and_exponent_primitive_float_fail() {
    apply_fn_to_primitive_floats!(sci_mantissa_and_exponent_primitive_float_fail_helper);
}

#[test]
fn test_from_sci_mantissa_and_exponent_primitive_float() {
    fn test<T: PrimitiveFloat>(m: T, e: i32, out: Option<&str>, out_hex: Option<&str>)
    where
        for<'a> &'a Float: SciMantissaAndExponent<T, i32, Float>,
    {
        let ox = <&Float as SciMantissaAndExponent<_, _, _>>::from_sci_mantissa_and_exponent(m, e);
        assert!(ox.as_ref().map_or(true, Float::is_valid));
        let os = ox.as_ref().map(ToString::to_string);
        assert_eq!(os.as_deref(), out);
        let os = ox.as_ref().map(to_hex_string);
        assert_eq!(os.as_deref(), out_hex);
    }
    test::<f32>(1.0, 0, Some("1.0"), Some("0x1.0#1"));
    test::<f32>(1.0, 1, Some("2.0"), Some("0x2.0#1"));
    test::<f32>(1.0, -1, Some("0.5"), Some("0x0.8#1"));
    test::<f32>(1.3333334, -2, Some("0.33333334"), Some("0x0.5555558#24"));
    test::<f32>(1.921875, 6, Some("123.0"), Some("0x7b.0#7"));
    test::<f32>(1.8189894, 39, Some("1.0e12"), Some("0xe.8d4a5E+9#24"));
    test::<f32>(
        std::f32::consts::SQRT_2,
        0,
        Some("1.4142135"),
        Some("0x1.6a09e6#24"),
    );
    test::<f32>(1.5707964, 1, Some("3.1415927"), Some("0x3.243f6c#24"));
    test::<f32>(1.3591409, 1, Some("2.718282"), Some("0x2.b7e15#22"));
    test::<f32>(1.6543612, 79, Some("1.0e24"), Some("0xd.3c21cE+19#22"));
    test(1.0, 1073741822, Some("too_big"), Some("0x4.0E+268435455#1"));
    test(1.0, 1073741823, None, None);
    test(
        1.0,
        -1073741824,
        Some("too_small"),
        Some("0x1.0E-268435456#1"),
    );
    test(1.0, -1073741825, None, None);

    test::<f32>(-1.0, 0, None, None);
    test::<f32>(2.0, 0, None, None);
    test::<f32>(0.5, 0, None, None);

    test::<f64>(1.0, 0, Some("1.0"), Some("0x1.0#1"));
    test::<f64>(1.0, 1, Some("2.0"), Some("0x2.0#1"));
    test::<f64>(1.0, -1, Some("0.5"), Some("0x0.8#1"));
    test::<f64>(
        1.3333333333333333,
        -2,
        Some("0.33333333333333331"),
        Some("0x0.55555555555554#53"),
    );
    test::<f64>(1.921875, 6, Some("123.0"), Some("0x7b.0#7"));
    test::<f64>(
        1.8189894035458565,
        39,
        Some("1.0e12"),
        Some("0xe.8d4a51E+9#28"),
    );
    test::<f64>(
        std::f64::consts::SQRT_2,
        0,
        Some("1.4142135623730951"),
        Some("0x1.6a09e667f3bcd#53"),
    );
    test::<f64>(
        std::f64::consts::FRAC_PI_2,
        1,
        Some("3.141592653589793"),
        Some("0x3.243f6a8885a3#50"),
    );
    test::<f64>(
        1.3591409142295225,
        1,
        Some("2.7182818284590451"),
        Some("0x2.b7e151628aed2#53"),
    );
    test::<f64>(
        1.6543612251060553,
        79,
        Some("1.0e24"),
        Some("0xd.3c21bceccedaE+19#51"),
    );

    test::<f64>(-1.0, 0, None, None);
    test::<f64>(2.0, 0, None, None);
    test::<f64>(0.5, 0, None, None);
}

fn from_sci_mantissa_and_exponent_primitive_float_fail_helper<T: PrimitiveFloat>()
where
    for<'a> &'a Float: SciMantissaAndExponent<T, i32, Float>,
{
    assert_panic!(
        <&Float as SciMantissaAndExponent<_, _, _>>::from_sci_mantissa_and_exponent(T::NAN, 0)
    );
    assert_panic!(
        <&Float as SciMantissaAndExponent<_, _, _>>::from_sci_mantissa_and_exponent(T::INFINITY, 0)
    );
    assert_panic!(
        <&Float as SciMantissaAndExponent<_, _, _>>::from_sci_mantissa_and_exponent(
            T::NEGATIVE_INFINITY,
            0
        )
    );
    assert_panic!(
        <&Float as SciMantissaAndExponent<_, _, _>>::from_sci_mantissa_and_exponent(T::ZERO, 0)
    );
    assert_panic!(
        <&Float as SciMantissaAndExponent<_, _, _>>::from_sci_mantissa_and_exponent(
            T::NEGATIVE_ZERO,
            0
        )
    );
}

#[test]
fn from_sci_mantissa_and_exponent_primitive_float_fail() {
    apply_fn_to_primitive_floats!(from_sci_mantissa_and_exponent_primitive_float_fail_helper);
}

fn raw_mantissa_and_exponent_properties_helper(x: Float) {
    let (mantissa, exponent) = x.clone().raw_mantissa_and_exponent();
    let (mantissa_alt, exponent_alt) = (&x).raw_mantissa_and_exponent();
    assert_eq!(mantissa_alt, mantissa);
    assert_eq!(exponent_alt, exponent);
    let (mantissa_alt, exponent_alt) = (-&x).raw_mantissa_and_exponent();
    assert_eq!(mantissa_alt, mantissa);
    assert_eq!(exponent_alt, exponent);

    assert_eq!(x.clone().raw_mantissa(), mantissa);
    assert_eq!((&x).raw_mantissa(), mantissa);
    assert_eq!(x.clone().raw_exponent(), exponent);
    assert_eq!((&x).raw_exponent(), exponent);

    assert_eq!(x.to_significand().unwrap(), mantissa);
    assert_eq!(x.get_exponent().unwrap(), exponent);

    assert_eq!(
        Float::from_raw_mantissa_and_exponent(mantissa.clone(), exponent),
        (&x).abs()
    );
    assert_eq!(
        <&Float as RawMantissaAndExponent<_, _, _>>::from_raw_mantissa_and_exponent(
            mantissa.clone(),
            exponent
        ),
        x.abs()
    );

    let bits = mantissa.significant_bits();
    assert_ne!(bits, 0);
    assert!(bits.divisible_by_power_of_2(Limb::LOG_WIDTH));
    assert!(exponent <= Float::MAX_EXPONENT);
    assert!(exponent >= Float::MIN_EXPONENT);
}

#[test]
fn raw_mantissa_and_exponent_properties() {
    float_gen_var_3().test_properties(|x| {
        raw_mantissa_and_exponent_properties_helper(x);
    });

    float_gen_var_13().test_properties(|x| {
        raw_mantissa_and_exponent_properties_helper(x);
    });
}

#[test]
fn from_raw_mantissa_and_exponent_properties() {
    natural_signed_pair_gen_var_4::<i32>().test_properties(|(mantissa, exponent)| {
        let x = Float::from_raw_mantissa_and_exponent(mantissa.clone(), exponent);
        assert!(x.is_valid());
        assert!(x.is_finite());
        assert!(x > 0u32);
        assert_eq!((&x).raw_mantissa(), mantissa);
        assert_eq!((&x).raw_exponent(), exponent);

        let x_alt = <&Float as RawMantissaAndExponent<_, _, _>>::from_raw_mantissa_and_exponent(
            mantissa, exponent,
        );
        assert!(x_alt.is_valid());
        assert_eq!(x_alt, x);
    });
}

fn integer_mantissa_and_exponent_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T>,
{
    primitive_float_gen_var_12::<T>().test_properties(|x| {
        let (mantissa, exponent) = x.integer_mantissa_and_exponent();
        let (mantissa_alt, exponent_alt) = Float::from(x).integer_mantissa_and_exponent();
        assert_eq!(mantissa_alt, mantissa);
        assert_eq!(exponent_alt, exponent);
    });
}

fn integer_mantissa_and_exponent_properties_helper_2(x: Float) {
    let (mantissa, exponent) = x.clone().integer_mantissa_and_exponent();
    let (mantissa_alt, exponent_alt) = (&x).integer_mantissa_and_exponent();
    assert_eq!(mantissa_alt, mantissa);
    assert_eq!(exponent_alt, exponent);
    let (mantissa_alt, exponent_alt) = (-&x).integer_mantissa_and_exponent();
    assert_eq!(mantissa_alt, mantissa);
    assert_eq!(exponent_alt, exponent);

    assert_eq!(x.clone().integer_mantissa(), mantissa);
    assert_eq!((&x).integer_mantissa(), mantissa);
    assert_eq!(x.clone().integer_exponent(), exponent);
    assert_eq!((&x).integer_exponent(), exponent);

    assert_eq!(
        Float::from_integer_mantissa_and_exponent(mantissa.clone(), exponent),
        Some(x.clone().abs())
    );
    assert_eq!(
        <&Float as IntegerMantissaAndExponent<_, _, _>>::from_integer_mantissa_and_exponent(
            mantissa.clone(),
            exponent
        ),
        Some(x.abs())
    );

    assert!(mantissa.odd());
    assert!(exponent < i64::from(Float::MAX_EXPONENT));
}

#[test]
fn integer_mantissa_and_exponent_properties() {
    float_gen_var_3().test_properties(|x| {
        integer_mantissa_and_exponent_properties_helper_2(x);
    });

    float_gen_var_13().test_properties(|x| {
        integer_mantissa_and_exponent_properties_helper_2(x);
    });

    apply_fn_to_primitive_floats!(integer_mantissa_and_exponent_properties_helper);
}

#[test]
fn from_integer_mantissa_and_exponent_properties() {
    natural_signed_pair_gen_var_2::<i64>().test_properties(|(mantissa, exponent)| {
        let ox = Float::from_integer_mantissa_and_exponent(mantissa.clone(), exponent);
        if let Some(x) = ox.as_ref() {
            assert!(x.is_valid());
            assert!(x.is_finite());
            assert!(*x >= 0u32);
            if mantissa.odd() {
                assert_eq!(x.integer_mantissa(), mantissa);
                assert_eq!(x.integer_exponent(), exponent);
            }
        }

        let ox_alt =
            <&Float as IntegerMantissaAndExponent<_, _, _>>::from_integer_mantissa_and_exponent(
                mantissa, exponent,
            );
        assert!(ox_alt.as_ref().map_or(true, Float::is_valid));
        assert_eq!(ox_alt, ox);
    });
}

fn sci_mantissa_and_exponent_round_properties_helper_helper<T: PrimitiveFloat>(
    x: Float,
    rm: RoundingMode,
    extreme: bool,
) where
    Rational: TryFrom<T>,
{
    let meo = x.sci_mantissa_and_exponent_round::<T>(rm);
    assert_eq!((-&x).sci_mantissa_and_exponent_round(rm), meo);
    if let Some((mantissa, exponent, o)) = meo {
        assert!(mantissa >= T::ONE);
        assert!(mantissa < T::TWO);
        assert!(x.is_valid());
        // Although the maximum sci-exponent for a Float is Float::MAX_EXPONENT, this function may
        // return an exponent that is larger by 1, due to rounding.
        assert!(exponent <= Float::MAX_EXPONENT);
        assert!(exponent >= Float::MIN_EXPONENT - 1);
        match rm {
            Floor | Down => assert_ne!(o, Greater),
            Ceiling | Up => assert_ne!(o, Less),
            Exact => assert_eq!(o, Equal),
            _ => {}
        }
        if !extreme {
            let x_alt = Rational::exact_from(mantissa) << exponent;
            assert_eq!(x_alt.partial_cmp_abs(&x), Some(o));
            if rm == Exact {
                assert_eq!(x_alt.partial_cmp_abs(&x), Some(Equal));
            }

            let r_x: Rational = ExactFrom::exact_from(x);
            assert_eq!(
                r_x.sci_mantissa_and_exponent_round(rm).unwrap(),
                (mantissa, i64::from(exponent), o)
            );
        }
    } else {
        assert!(!x.is_finite() || x == 0u32 || rm == Exact);
    }
}

fn sci_mantissa_and_exponent_round_properties_helper<T: PrimitiveFloat>()
where
    Rational: TryFrom<T>,
{
    float_rounding_mode_pair_gen().test_properties(|(x, rm)| {
        sci_mantissa_and_exponent_round_properties_helper_helper(x, rm, false);
    });

    float_rounding_mode_pair_gen_var_21().test_properties(|(x, rm)| {
        sci_mantissa_and_exponent_round_properties_helper_helper(x, rm, true);
    });

    float_gen_var_3().test_properties(|n| {
        let (floor_mantissa, floor_exponent, floor_o) =
            n.sci_mantissa_and_exponent_round::<T>(Floor).unwrap();
        assert_eq!(
            n.sci_mantissa_and_exponent_round::<T>(Down).unwrap(),
            (floor_mantissa, floor_exponent, floor_o)
        );
        let (ceiling_mantissa, ceiling_exponent, ceiling_o) =
            n.sci_mantissa_and_exponent_round::<T>(Ceiling).unwrap();
        assert_eq!(
            n.sci_mantissa_and_exponent_round::<T>(Up).unwrap(),
            (ceiling_mantissa, ceiling_exponent, ceiling_o)
        );
        let (nearest_mantissa, nearest_exponent, nearest_o) =
            n.sci_mantissa_and_exponent_round::<T>(Nearest).unwrap();
        if let Some((mantissa, exponent, o)) = n.sci_mantissa_and_exponent_round::<T>(Exact) {
            assert_eq!(o, Equal);
            assert_eq!(floor_mantissa, mantissa);
            assert_eq!(ceiling_mantissa, mantissa);
            assert_eq!(nearest_mantissa, mantissa);
            assert_eq!(floor_exponent, exponent);
            assert_eq!(ceiling_exponent, exponent);
            assert_eq!(nearest_exponent, exponent);
        } else {
            assert_eq!(floor_o, Less);
            assert_eq!(ceiling_o, Greater);
            assert_ne!(
                (floor_mantissa, floor_exponent),
                (ceiling_mantissa, ceiling_exponent)
            );
            assert!(
                (nearest_mantissa, nearest_exponent, nearest_o)
                    == (floor_mantissa, floor_exponent, floor_o)
                    || (nearest_mantissa, nearest_exponent, nearest_o)
                        == (ceiling_mantissa, ceiling_exponent, ceiling_o)
            );
            if ceiling_mantissa == T::ONE {
                assert_eq!(floor_mantissa, T::TWO.next_lower());
                assert_eq!(floor_exponent, ceiling_exponent - 1);
            } else {
                assert_eq!(floor_mantissa, ceiling_mantissa.next_lower());
                assert_eq!(floor_exponent, ceiling_exponent);
            }
        }
    });
}

#[test]
fn sci_mantissa_and_exponent_round_properties() {
    apply_fn_to_primitive_floats!(sci_mantissa_and_exponent_round_properties_helper);
}

fn sci_mantissa_and_exponent_float_properties_helper(x: Float) {
    let (mantissa, exponent): (Float, i32) = x.clone().sci_mantissa_and_exponent();
    assert!(mantissa.is_valid());
    let (mantissa_alt, exponent_alt): (Float, i32) = (&x).sci_mantissa_and_exponent();
    assert!(mantissa_alt.is_valid());
    assert_eq!(mantissa_alt, mantissa);
    assert_eq!(exponent_alt, exponent);
    let (mantissa_alt, exponent_alt): (Float, i32) = (-&x).sci_mantissa_and_exponent();
    assert_eq!(mantissa_alt, mantissa);
    assert_eq!(exponent_alt, exponent);

    let mantissa_alt: Float = (&x).sci_mantissa();
    assert!(mantissa_alt.is_valid());
    assert_eq!(mantissa_alt, mantissa);
    let mantissa_alt: Float = x.clone().sci_mantissa();
    assert!(mantissa_alt.is_valid());
    assert_eq!(mantissa_alt, mantissa);
    assert_eq!(
        <Float as SciMantissaAndExponent<Float, _, _>>::sci_exponent(x.clone()),
        exponent
    );
    assert_eq!(
        <&Float as SciMantissaAndExponent<Float, _, _>>::sci_exponent(&x),
        exponent
    );

    assert_eq!(
        Float::from_sci_mantissa_and_exponent(mantissa.clone(), exponent),
        Some(x.clone().abs())
    );
    assert_eq!(
        <&Float as SciMantissaAndExponent<_, _, _>>::from_sci_mantissa_and_exponent(
            mantissa.clone(),
            exponent
        ),
        Some(x.abs())
    );
    assert!(mantissa >= 1u32);
    assert!(mantissa < 2u32);
    assert!(exponent < Float::MAX_EXPONENT);
    assert!(exponent >= Float::MIN_EXPONENT - 1);
}

#[test]
fn sci_mantissa_and_exponent_float_properties() {
    float_gen_var_3().test_properties(|x| {
        sci_mantissa_and_exponent_float_properties_helper(x);
    });

    float_gen_var_13().test_properties(|x| {
        sci_mantissa_and_exponent_float_properties_helper(x);
    });
}

#[test]
fn from_sci_mantissa_and_exponent_float_properties() {
    float_signed_pair_gen_var_1::<i32>().test_properties(|(mantissa, exponent)| {
        let ox = Float::from_sci_mantissa_and_exponent(mantissa.clone(), exponent);
        if let Some(x) = ox.as_ref() {
            assert!(x.is_valid());
            assert!(x.is_finite());
            assert!(*x > 0u32);
            assert_eq!(
                <&Float as SciMantissaAndExponent<Float, _, _>>::sci_mantissa(x),
                mantissa
            );
            assert_eq!(
                <&Float as SciMantissaAndExponent<Float, _, _>>::sci_exponent(x),
                exponent
            );
        }

        let ox_alt = <&Float as SciMantissaAndExponent<_, _, _>>::from_sci_mantissa_and_exponent(
            mantissa, exponent,
        );
        assert!(ox_alt.as_ref().map_or(true, Float::is_valid));
        assert_eq!(ox_alt, ox);
    });
}

fn sci_mantissa_and_exponent_primitive_float_properties_helper_helper<T: PrimitiveFloat>(
    x: Float,
    extreme: bool,
) where
    for<'a> &'a Float: SciMantissaAndExponent<T, i32, Float>,
    Float: From<T>,
{
    let (mantissa, exponent): (T, i32) = (&x).sci_mantissa_and_exponent();

    assert_eq!(
        <&Float as SciMantissaAndExponent<T, _, _>>::sci_mantissa(&x),
        mantissa
    );
    assert_eq!(
        <&Float as SciMantissaAndExponent<T, _, _>>::sci_exponent(&x),
        exponent
    );
    let (mantissa_alt, exponent_alt): (T, i32) = (&-&x).sci_mantissa_and_exponent();
    assert_eq!(mantissa_alt, mantissa);
    assert_eq!(exponent_alt, exponent);

    if !extreme {
        let x_alt = <&Float as SciMantissaAndExponent<_, _, _>>::from_sci_mantissa_and_exponent(
            mantissa, exponent,
        )
        .unwrap();
        assert!(
            Rational::exact_from(x) - Rational::exact_from(&x_alt)
                <= Rational::exact_from(x_alt.ulp().unwrap()) >> 1u32
        );
    }

    assert!(mantissa >= T::ONE);
    assert!(mantissa < T::TWO);
}

fn sci_mantissa_and_exponent_primitive_float_properties_helper<T: PrimitiveFloat>()
where
    for<'a> &'a Float: SciMantissaAndExponent<T, i32, Float>,
    Float: From<T>,
{
    float_gen_var_3().test_properties(|x| {
        sci_mantissa_and_exponent_primitive_float_properties_helper_helper(x, false);
    });

    float_gen_var_13().test_properties(|x| {
        sci_mantissa_and_exponent_primitive_float_properties_helper_helper(x, true);
    });

    primitive_float_gen_var_12::<T>().test_properties(|x| {
        let (mantissa, exponent) = x.sci_mantissa_and_exponent();
        let (mantissa_alt, exponent_alt): (T, i32) = (&Float::from(x)).sci_mantissa_and_exponent();
        assert_eq!(mantissa_alt, mantissa);
        assert_eq!(i64::from(exponent_alt), exponent);
    });
}

#[test]
fn sci_mantissa_and_exponent_primitive_float_properties() {
    apply_fn_to_primitive_floats!(sci_mantissa_and_exponent_primitive_float_properties_helper);
}

fn from_sci_mantissa_and_exponent_primitive_float_properties_helper<T: PrimitiveFloat>()
where
    for<'a> &'a Float: SciMantissaAndExponent<T, i32, Float>,
    Float: From<T>,
{
    primitive_float_signed_pair_gen_var_3::<T>().test_properties(|(mantissa, exponent)| {
        let ox = <&Float as SciMantissaAndExponent<_, _, _>>::from_sci_mantissa_and_exponent(
            mantissa,
            i32::exact_from(exponent),
        );
        if let Some(x) = ox.as_ref() {
            assert!(x.is_valid());
            assert!(x.is_finite());
            assert!(*x > 0u32);
            assert_eq!(
                <&Float as SciMantissaAndExponent<T, _, _>>::sci_mantissa(x),
                mantissa
            );
            assert_eq!(
                i64::from(<&Float as SciMantissaAndExponent<T, _, _>>::sci_exponent(x)),
                exponent
            );
        }

        let ox_alt =
            Float::from_sci_mantissa_and_exponent(Float::from(mantissa), i32::exact_from(exponent));
        assert!(ox_alt.as_ref().map_or(true, Float::is_valid));
        assert_eq!(ox_alt, ox);
    });
}

#[test]
fn from_sci_mantissa_and_exponent_primitive_float_properties() {
    apply_fn_to_primitive_floats!(from_sci_mantissa_and_exponent_primitive_float_properties_helper);
}
