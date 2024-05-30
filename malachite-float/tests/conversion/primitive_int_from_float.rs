// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Ceiling, Floor};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeOne, OneHalf};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::from::{SignedFromFloatError, UnsignedFromFloatError};
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, RoundingFrom};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::strings::ToDebugString;
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};
use malachite_float::test_util::common::parse_hex_string;
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_4, float_gen_var_5, float_rounding_mode_pair_gen_var_4,
    float_rounding_mode_pair_gen_var_5,
};
use malachite_float::Float;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

#[allow(clippy::type_repetition_in_bounds)]
#[test]
fn test_try_from_float() {
    fn test_helper_u<T: PrimitiveUnsigned + TryFrom<Float, Error = UnsignedFromFloatError>>(
        s: &str,
        s_hex: &str,
        out: &str,
    ) where
        for<'a> T: TryFrom<&'a Float, Error = UnsignedFromFloatError>,
    {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let on = T::try_from(x.clone());
        assert_eq!(on.to_debug_string(), out);

        let on = T::try_from(&x);
        assert_eq!(on.to_debug_string(), out);
    }
    fn test_helper_u2<T: PrimitiveUnsigned + TryFrom<Float, Error = UnsignedFromFloatError>>()
    where
        for<'a> T: TryFrom<&'a Float, Error = UnsignedFromFloatError>,
    {
        test_helper_u::<T>("NaN", "NaN", "Err(FloatInfiniteOrNan)");
        test_helper_u::<T>("Infinity", "Infinity", "Err(FloatInfiniteOrNan)");
        test_helper_u::<T>("-Infinity", "-Infinity", "Err(FloatInfiniteOrNan)");
        test_helper_u::<T>("0.0", "0x0.0", "Ok(0)");
        test_helper_u::<T>("-0.0", "-0x0.0", "Ok(0)");

        test_helper_u::<T>("1.0", "0x1.0#1", "Ok(1)");
        test_helper_u::<T>("2.0", "0x2.0#1", "Ok(2)");
        test_helper_u::<T>("0.5", "0x0.8#1", "Err(FloatNonIntegerOrOutOfRange)");
        test_helper_u::<T>(
            "0.33333333333333331",
            "0x0.55555555555554#53",
            "Err(FloatNonIntegerOrOutOfRange)",
        );
        test_helper_u::<T>("123.0", "0x7b.0#7", "Ok(123)");

        test_helper_u::<T>("-1.0", "-0x1.0#1", "Err(FloatNegative)");
        test_helper_u::<T>("-2.0", "-0x2.0#1", "Err(FloatNegative)");
        test_helper_u::<T>("-0.5", "-0x0.8#1", "Err(FloatNegative)");
        test_helper_u::<T>(
            "-0.33333333333333331",
            "-0x0.55555555555554#53",
            "Err(FloatNegative)",
        );
        test_helper_u::<T>("-123.0", "-0x7b.0#7", "Err(FloatNegative)");
        test_helper_u::<T>(
            "-1000000000000.0",
            "-0xe8d4a51000.0#40",
            "Err(FloatNegative)",
        );
    }
    apply_fn_to_unsigneds!(test_helper_u2);
    test_helper_u::<u64>("1000000000000.0", "0xe8d4a51000.0#40", "Ok(1000000000000)");

    fn test_helper_i<T: PrimitiveSigned + TryFrom<Float, Error = SignedFromFloatError>>(
        s: &str,
        s_hex: &str,
        out: &str,
    ) where
        for<'a> T: TryFrom<&'a Float, Error = SignedFromFloatError>,
    {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let on = T::try_from(x.clone());
        assert_eq!(on.to_debug_string(), out);

        let on = T::try_from(&x);
        assert_eq!(on.to_debug_string(), out);
    }
    fn test_helper_i2<T: PrimitiveSigned + TryFrom<Float, Error = SignedFromFloatError>>()
    where
        for<'a> T: TryFrom<&'a Float, Error = SignedFromFloatError>,
    {
        test_helper_i::<T>("NaN", "NaN", "Err(FloatInfiniteOrNan)");
        test_helper_i::<T>("Infinity", "Infinity", "Err(FloatInfiniteOrNan)");
        test_helper_i::<T>("-Infinity", "-Infinity", "Err(FloatInfiniteOrNan)");
        test_helper_i::<T>("0.0", "0x0.0", "Ok(0)");
        test_helper_i::<T>("-0.0", "-0x0.0", "Ok(0)");

        test_helper_i::<T>("1.0", "0x1.0#1", "Ok(1)");
        test_helper_i::<T>("2.0", "0x2.0#1", "Ok(2)");
        test_helper_i::<T>("0.5", "0x0.8#1", "Err(FloatNonIntegerOrOutOfRange)");
        test_helper_i::<T>(
            "0.33333333333333331",
            "0x0.55555555555554#53",
            "Err(FloatNonIntegerOrOutOfRange)",
        );
        test_helper_i::<T>("123.0", "0x7b.0#7", "Ok(123)");

        test_helper_i::<T>("-1.0", "-0x1.0#1", "Ok(-1)");
        test_helper_i::<T>("-2.0", "-0x2.0#1", "Ok(-2)");
        test_helper_i::<T>("-0.5", "-0x0.8#1", "Err(FloatNonIntegerOrOutOfRange)");
        test_helper_i::<T>(
            "-0.33333333333333331",
            "-0x0.55555555555554#53",
            "Err(FloatNonIntegerOrOutOfRange)",
        );
        test_helper_i::<T>("-123.0", "-0x7b.0#7", "Ok(-123)");
    }
    apply_fn_to_signeds!(test_helper_i2);
    test_helper_i::<i64>("1000000000000.0", "0xe8d4a51000.0#40", "Ok(1000000000000)");
    test_helper_i::<i64>(
        "-1000000000000.0",
        "-0xe8d4a51000.0#40",
        "Ok(-1000000000000)",
    );
}

#[test]
fn test_convertible_from_float() {
    #[allow(clippy::type_repetition_in_bounds)]
    fn test_helper<T: PrimitiveInt>(s: &str, s_hex: &str, out: bool)
    where
        for<'a> T: ConvertibleFrom<&'a Float>,
    {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        assert_eq!(T::convertible_from(&x), out);
    }
    #[allow(clippy::type_repetition_in_bounds)]
    fn test_helper_u<T: PrimitiveUnsigned>()
    where
        for<'a> T: ConvertibleFrom<&'a Float>,
    {
        test_helper::<T>("NaN", "NaN", false);
        test_helper::<T>("Infinity", "Infinity", false);
        test_helper::<T>("-Infinity", "-Infinity", false);
        test_helper::<T>("0.0", "0x0.0", true);
        test_helper::<T>("-0.0", "-0x0.0", true);

        test_helper::<T>("1.0", "0x1.0#1", true);
        test_helper::<T>("2.0", "0x2.0#1", true);
        test_helper::<T>("0.5", "0x0.8#1", false);
        test_helper::<T>("0.33333333333333331", "0x0.55555555555554#53", false);
        test_helper::<T>("123.0", "0x7b.0#7", true);

        test_helper::<T>("-1.0", "-0x1.0#1", false);
        test_helper::<T>("-2.0", "-0x2.0#1", false);
        test_helper::<T>("-0.5", "-0x0.8#1", false);
        test_helper::<T>("-0.33333333333333331", "-0x0.55555555555554#53", false);
        test_helper::<T>("-123.0", "-0x7b.0#7", false);
        test_helper::<T>("-1000000000000.0", "-0xe8d4a51000.0#40", false);
    }
    apply_fn_to_unsigneds!(test_helper_u);
    test_helper::<u64>("1000000000000.0", "0xe8d4a51000.0#40", true);

    #[allow(clippy::type_repetition_in_bounds)]
    fn test_helper_i<T: PrimitiveSigned>()
    where
        for<'a> T: ConvertibleFrom<&'a Float>,
    {
        test_helper::<T>("NaN", "NaN", false);
        test_helper::<T>("Infinity", "Infinity", false);
        test_helper::<T>("-Infinity", "-Infinity", false);
        test_helper::<T>("0.0", "0x0.0", true);
        test_helper::<T>("-0.0", "-0x0.0", true);

        test_helper::<T>("1.0", "0x1.0#1", true);
        test_helper::<T>("2.0", "0x2.0#1", true);
        test_helper::<T>("0.5", "0x0.8#1", false);
        test_helper::<T>("0.33333333333333331", "0x0.55555555555554#53", false);
        test_helper::<T>("123.0", "0x7b.0#7", true);

        test_helper::<T>("-1.0", "-0x1.0#1", true);
        test_helper::<T>("-2.0", "-0x2.0#1", true);
        test_helper::<T>("-0.5", "-0x0.8#1", false);
        test_helper::<T>("-0.33333333333333331", "-0x0.55555555555554#53", false);
        test_helper::<T>("-123.0", "-0x7b.0#7", true);
    }
    apply_fn_to_signeds!(test_helper_i);
    test_helper::<i64>("1000000000000.0", "0xe8d4a51000.0#40", true);
    test_helper::<i64>("-1000000000000.0", "-0xe8d4a51000.0#40", true);
}

#[test]
fn test_rounding_from_float() {
    #[allow(clippy::type_repetition_in_bounds)]
    fn test_helper<T: PrimitiveInt + RoundingFrom<Float>>(
        s: &str,
        s_hex: &str,
        rm: RoundingMode,
        out: &str,
        o_out: Ordering,
    ) where
        for<'a> T: RoundingFrom<&'a Float>,
    {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (n, o) = T::rounding_from(x.clone(), rm);
        assert_eq!(n.to_string(), out);
        assert_eq!(o, o_out);

        let (n, o) = T::rounding_from(&x, rm);
        assert_eq!(n.to_string(), out);
        assert_eq!(o, o_out);
    }
    #[allow(clippy::type_repetition_in_bounds)]
    fn test_helper_u<T: PrimitiveUnsigned + RoundingFrom<Float>>()
    where
        for<'a> T: RoundingFrom<&'a Float>,
    {
        test_helper::<T>("-Infinity", "-Infinity", Ceiling, "0", Greater);
        test_helper::<T>("-Infinity", "-Infinity", Down, "0", Greater);
        test_helper::<T>("-Infinity", "-Infinity", Nearest, "0", Greater);

        test_helper::<T>("0.0", "0x0.0", Floor, "0", Equal);
        test_helper::<T>("0.0", "0x0.0", Ceiling, "0", Equal);
        test_helper::<T>("0.0", "0x0.0", Down, "0", Equal);
        test_helper::<T>("0.0", "0x0.0", Up, "0", Equal);
        test_helper::<T>("0.0", "0x0.0", Nearest, "0", Equal);
        test_helper::<T>("0.0", "0x0.0", Exact, "0", Equal);

        test_helper::<T>("-0.0", "-0x0.0", Floor, "0", Equal);
        test_helper::<T>("-0.0", "-0x0.0", Ceiling, "0", Equal);
        test_helper::<T>("-0.0", "-0x0.0", Down, "0", Equal);
        test_helper::<T>("-0.0", "-0x0.0", Up, "0", Equal);
        test_helper::<T>("-0.0", "-0x0.0", Nearest, "0", Equal);
        test_helper::<T>("-0.0", "-0x0.0", Exact, "0", Equal);

        test_helper::<T>("1.0", "0x1.0#1", Floor, "1", Equal);
        test_helper::<T>("1.0", "0x1.0#1", Ceiling, "1", Equal);
        test_helper::<T>("1.0", "0x1.0#1", Down, "1", Equal);
        test_helper::<T>("1.0", "0x1.0#1", Up, "1", Equal);
        test_helper::<T>("1.0", "0x1.0#1", Nearest, "1", Equal);
        test_helper::<T>("1.0", "0x1.0#1", Exact, "1", Equal);

        test_helper::<T>("2.0", "0x2.0#1", Floor, "2", Equal);
        test_helper::<T>("2.0", "0x2.0#1", Ceiling, "2", Equal);
        test_helper::<T>("2.0", "0x2.0#1", Down, "2", Equal);
        test_helper::<T>("2.0", "0x2.0#1", Up, "2", Equal);
        test_helper::<T>("2.0", "0x2.0#1", Nearest, "2", Equal);
        test_helper::<T>("2.0", "0x2.0#1", Exact, "2", Equal);

        test_helper::<T>("0.5", "0x0.8#1", Floor, "0", Less);
        test_helper::<T>("0.5", "0x0.8#1", Ceiling, "1", Greater);
        test_helper::<T>("0.5", "0x0.8#1", Down, "0", Less);
        test_helper::<T>("0.5", "0x0.8#1", Up, "1", Greater);
        test_helper::<T>("0.5", "0x0.8#1", Nearest, "0", Less);

        test_helper::<T>(
            "0.33333333333333331",
            "0x0.55555555555554#53",
            Floor,
            "0",
            Less,
        );
        test_helper::<T>(
            "0.33333333333333331",
            "0x0.55555555555554#53",
            Ceiling,
            "1",
            Greater,
        );
        test_helper::<T>(
            "0.33333333333333331",
            "0x0.55555555555554#53",
            Down,
            "0",
            Less,
        );
        test_helper::<T>(
            "0.33333333333333331",
            "0x0.55555555555554#53",
            Up,
            "1",
            Greater,
        );
        test_helper::<T>(
            "0.33333333333333331",
            "0x0.55555555555554#53",
            Nearest,
            "0",
            Less,
        );

        test_helper::<T>(
            "0.6666666666666666",
            "0x0.aaaaaaaaaaaaa8#53",
            Floor,
            "0",
            Less,
        );
        test_helper::<T>(
            "0.6666666666666666",
            "0x0.aaaaaaaaaaaaa8#53",
            Ceiling,
            "1",
            Greater,
        );
        test_helper::<T>(
            "0.6666666666666666",
            "0x0.aaaaaaaaaaaaa8#53",
            Down,
            "0",
            Less,
        );
        test_helper::<T>(
            "0.6666666666666666",
            "0x0.aaaaaaaaaaaaa8#53",
            Up,
            "1",
            Greater,
        );
        test_helper::<T>(
            "0.6666666666666666",
            "0x0.aaaaaaaaaaaaa8#53",
            Nearest,
            "1",
            Greater,
        );

        test_helper::<T>("1.5", "0x1.8#2", Floor, "1", Less);
        test_helper::<T>("1.5", "0x1.8#2", Ceiling, "2", Greater);
        test_helper::<T>("1.5", "0x1.8#2", Down, "1", Less);
        test_helper::<T>("1.5", "0x1.8#2", Up, "2", Greater);
        test_helper::<T>("1.5", "0x1.8#2", Nearest, "2", Greater);

        test_helper::<T>("2.5", "0x2.8#3", Floor, "2", Less);
        test_helper::<T>("2.5", "0x2.8#3", Ceiling, "3", Greater);
        test_helper::<T>("2.5", "0x2.8#3", Down, "2", Less);
        test_helper::<T>("2.5", "0x2.8#3", Up, "3", Greater);
        test_helper::<T>("2.5", "0x2.8#3", Nearest, "2", Less);

        test_helper::<T>("123.0", "0x7b.0#7", Floor, "123", Equal);
        test_helper::<T>("123.0", "0x7b.0#7", Ceiling, "123", Equal);
        test_helper::<T>("123.0", "0x7b.0#7", Down, "123", Equal);
        test_helper::<T>("123.0", "0x7b.0#7", Up, "123", Equal);
        test_helper::<T>("123.0", "0x7b.0#7", Nearest, "123", Equal);
        test_helper::<T>("123.0", "0x7b.0#7", Exact, "123", Equal);

        test_helper::<T>("-1.0", "-0x1.0#1", Ceiling, "0", Greater);
        test_helper::<T>("-1.0", "-0x1.0#1", Down, "0", Greater);
        test_helper::<T>("-1.0", "-0x1.0#1", Nearest, "0", Greater);

        test_helper::<T>("-2.0", "-0x2.0#1", Ceiling, "0", Greater);
        test_helper::<T>("-2.0", "-0x2.0#1", Down, "0", Greater);
        test_helper::<T>("-2.0", "-0x2.0#1", Nearest, "0", Greater);

        test_helper::<T>("-0.5", "-0x0.8#1", Ceiling, "0", Greater);
        test_helper::<T>("-0.5", "-0x0.8#1", Down, "0", Greater);
        test_helper::<T>("-0.5", "-0x0.8#1", Nearest, "0", Greater);

        test_helper::<T>(
            "-0.33333333333333331",
            "-0x0.55555555555554#53",
            Ceiling,
            "0",
            Greater,
        );
        test_helper::<T>(
            "-0.33333333333333331",
            "-0x0.55555555555554#53",
            Down,
            "0",
            Greater,
        );
        test_helper::<T>(
            "-0.33333333333333331",
            "-0x0.55555555555554#53",
            Nearest,
            "0",
            Greater,
        );

        test_helper::<T>(
            "-0.6666666666666666",
            "-0x0.aaaaaaaaaaaaa8#53",
            Ceiling,
            "0",
            Greater,
        );
        test_helper::<T>(
            "-0.6666666666666666",
            "-0x0.aaaaaaaaaaaaa8#53",
            Down,
            "0",
            Greater,
        );
        test_helper::<T>(
            "-0.6666666666666666",
            "-0x0.aaaaaaaaaaaaa8#53",
            Nearest,
            "0",
            Greater,
        );

        test_helper::<T>("-1.5", "-0x1.8#2", Ceiling, "0", Greater);
        test_helper::<T>("-1.5", "-0x1.8#2", Down, "0", Greater);
        test_helper::<T>("-1.5", "-0x1.8#2", Nearest, "0", Greater);

        test_helper::<T>("-2.5", "-0x2.8#3", Ceiling, "0", Greater);
        test_helper::<T>("-2.5", "-0x2.8#3", Down, "0", Greater);
        test_helper::<T>("-2.5", "-0x2.8#3", Nearest, "0", Greater);

        test_helper::<T>("-123.0", "-0x7b.0#7", Ceiling, "0", Greater);
        test_helper::<T>("-123.0", "-0x7b.0#7", Down, "0", Greater);
        test_helper::<T>("-123.0", "-0x7b.0#7", Nearest, "0", Greater);
    }
    apply_fn_to_unsigneds!(test_helper_u);

    #[allow(clippy::type_repetition_in_bounds)]
    fn test_helper_i<T: PrimitiveSigned + RoundingFrom<Float>>()
    where
        for<'a> T: RoundingFrom<&'a Float>,
    {
        test_helper::<T>("0.0", "0x0.0", Floor, "0", Equal);
        test_helper::<T>("0.0", "0x0.0", Ceiling, "0", Equal);
        test_helper::<T>("0.0", "0x0.0", Down, "0", Equal);
        test_helper::<T>("0.0", "0x0.0", Up, "0", Equal);
        test_helper::<T>("0.0", "0x0.0", Nearest, "0", Equal);
        test_helper::<T>("0.0", "0x0.0", Exact, "0", Equal);

        test_helper::<T>("-0.0", "-0x0.0", Floor, "0", Equal);
        test_helper::<T>("-0.0", "-0x0.0", Ceiling, "0", Equal);
        test_helper::<T>("-0.0", "-0x0.0", Down, "0", Equal);
        test_helper::<T>("-0.0", "-0x0.0", Up, "0", Equal);
        test_helper::<T>("-0.0", "-0x0.0", Nearest, "0", Equal);
        test_helper::<T>("-0.0", "-0x0.0", Exact, "0", Equal);

        test_helper::<T>("1.0", "0x1.0#1", Floor, "1", Equal);
        test_helper::<T>("1.0", "0x1.0#1", Ceiling, "1", Equal);
        test_helper::<T>("1.0", "0x1.0#1", Down, "1", Equal);
        test_helper::<T>("1.0", "0x1.0#1", Up, "1", Equal);
        test_helper::<T>("1.0", "0x1.0#1", Nearest, "1", Equal);
        test_helper::<T>("1.0", "0x1.0#1", Exact, "1", Equal);

        test_helper::<T>("2.0", "0x2.0#1", Floor, "2", Equal);
        test_helper::<T>("2.0", "0x2.0#1", Ceiling, "2", Equal);
        test_helper::<T>("2.0", "0x2.0#1", Down, "2", Equal);
        test_helper::<T>("2.0", "0x2.0#1", Up, "2", Equal);
        test_helper::<T>("2.0", "0x2.0#1", Nearest, "2", Equal);
        test_helper::<T>("2.0", "0x2.0#1", Exact, "2", Equal);

        test_helper::<T>("0.5", "0x0.8#1", Floor, "0", Less);
        test_helper::<T>("0.5", "0x0.8#1", Ceiling, "1", Greater);
        test_helper::<T>("0.5", "0x0.8#1", Down, "0", Less);
        test_helper::<T>("0.5", "0x0.8#1", Up, "1", Greater);
        test_helper::<T>("0.5", "0x0.8#1", Nearest, "0", Less);

        test_helper::<T>(
            "0.33333333333333331",
            "0x0.55555555555554#53",
            Floor,
            "0",
            Less,
        );
        test_helper::<T>(
            "0.33333333333333331",
            "0x0.55555555555554#53",
            Ceiling,
            "1",
            Greater,
        );
        test_helper::<T>(
            "0.33333333333333331",
            "0x0.55555555555554#53",
            Down,
            "0",
            Less,
        );
        test_helper::<T>(
            "0.33333333333333331",
            "0x0.55555555555554#53",
            Up,
            "1",
            Greater,
        );
        test_helper::<T>(
            "0.33333333333333331",
            "0x0.55555555555554#53",
            Nearest,
            "0",
            Less,
        );

        test_helper::<T>(
            "0.6666666666666666",
            "0x0.aaaaaaaaaaaaa8#53",
            Floor,
            "0",
            Less,
        );
        test_helper::<T>(
            "0.6666666666666666",
            "0x0.aaaaaaaaaaaaa8#53",
            Ceiling,
            "1",
            Greater,
        );
        test_helper::<T>(
            "0.6666666666666666",
            "0x0.aaaaaaaaaaaaa8#53",
            Down,
            "0",
            Less,
        );
        test_helper::<T>(
            "0.6666666666666666",
            "0x0.aaaaaaaaaaaaa8#53",
            Up,
            "1",
            Greater,
        );
        test_helper::<T>(
            "0.6666666666666666",
            "0x0.aaaaaaaaaaaaa8#53",
            Nearest,
            "1",
            Greater,
        );

        test_helper::<T>("1.5", "0x1.8#2", Floor, "1", Less);
        test_helper::<T>("1.5", "0x1.8#2", Ceiling, "2", Greater);
        test_helper::<T>("1.5", "0x1.8#2", Down, "1", Less);
        test_helper::<T>("1.5", "0x1.8#2", Up, "2", Greater);
        test_helper::<T>("1.5", "0x1.8#2", Nearest, "2", Greater);

        test_helper::<T>("2.5", "0x2.8#3", Floor, "2", Less);
        test_helper::<T>("2.5", "0x2.8#3", Ceiling, "3", Greater);
        test_helper::<T>("2.5", "0x2.8#3", Down, "2", Less);
        test_helper::<T>("2.5", "0x2.8#3", Up, "3", Greater);
        test_helper::<T>("2.5", "0x2.8#3", Nearest, "2", Less);

        test_helper::<T>("123.0", "0x7b.0#7", Floor, "123", Equal);
        test_helper::<T>("123.0", "0x7b.0#7", Ceiling, "123", Equal);
        test_helper::<T>("123.0", "0x7b.0#7", Down, "123", Equal);
        test_helper::<T>("123.0", "0x7b.0#7", Up, "123", Equal);
        test_helper::<T>("123.0", "0x7b.0#7", Nearest, "123", Equal);
        test_helper::<T>("123.0", "0x7b.0#7", Exact, "123", Equal);

        test_helper::<T>("-1.0", "-0x1.0#1", Floor, "-1", Equal);
        test_helper::<T>("-1.0", "-0x1.0#1", Ceiling, "-1", Equal);
        test_helper::<T>("-1.0", "-0x1.0#1", Down, "-1", Equal);
        test_helper::<T>("-1.0", "-0x1.0#1", Up, "-1", Equal);
        test_helper::<T>("-1.0", "-0x1.0#1", Nearest, "-1", Equal);
        test_helper::<T>("-1.0", "-0x1.0#1", Exact, "-1", Equal);

        test_helper::<T>("-2.0", "-0x2.0#1", Floor, "-2", Equal);
        test_helper::<T>("-2.0", "-0x2.0#1", Ceiling, "-2", Equal);
        test_helper::<T>("-2.0", "-0x2.0#1", Down, "-2", Equal);
        test_helper::<T>("-2.0", "-0x2.0#1", Up, "-2", Equal);
        test_helper::<T>("-2.0", "-0x2.0#1", Nearest, "-2", Equal);
        test_helper::<T>("-2.0", "-0x2.0#1", Exact, "-2", Equal);

        test_helper::<T>("-0.5", "-0x0.8#1", Floor, "-1", Less);
        test_helper::<T>("-0.5", "-0x0.8#1", Ceiling, "0", Greater);
        test_helper::<T>("-0.5", "-0x0.8#1", Down, "0", Greater);
        test_helper::<T>("-0.5", "-0x0.8#1", Up, "-1", Less);
        test_helper::<T>("-0.5", "-0x0.8#1", Nearest, "0", Greater);

        test_helper::<T>(
            "-0.33333333333333331",
            "-0x0.55555555555554#53",
            Floor,
            "-1",
            Less,
        );
        test_helper::<T>(
            "-0.33333333333333331",
            "-0x0.55555555555554#53",
            Ceiling,
            "0",
            Greater,
        );
        test_helper::<T>(
            "-0.33333333333333331",
            "-0x0.55555555555554#53",
            Down,
            "0",
            Greater,
        );
        test_helper::<T>(
            "-0.33333333333333331",
            "-0x0.55555555555554#53",
            Up,
            "-1",
            Less,
        );
        test_helper::<T>(
            "-0.33333333333333331",
            "-0x0.55555555555554#53",
            Nearest,
            "0",
            Greater,
        );

        test_helper::<T>(
            "-0.6666666666666666",
            "-0x0.aaaaaaaaaaaaa8#53",
            Floor,
            "-1",
            Less,
        );
        test_helper::<T>(
            "-0.6666666666666666",
            "-0x0.aaaaaaaaaaaaa8#53",
            Ceiling,
            "0",
            Greater,
        );
        test_helper::<T>(
            "-0.6666666666666666",
            "-0x0.aaaaaaaaaaaaa8#53",
            Down,
            "0",
            Greater,
        );
        test_helper::<T>(
            "-0.6666666666666666",
            "-0x0.aaaaaaaaaaaaa8#53",
            Up,
            "-1",
            Less,
        );
        test_helper::<T>(
            "-0.6666666666666666",
            "-0x0.aaaaaaaaaaaaa8#53",
            Nearest,
            "-1",
            Less,
        );

        test_helper::<T>("-1.5", "-0x1.8#2", Floor, "-2", Less);
        test_helper::<T>("-1.5", "-0x1.8#2", Ceiling, "-1", Greater);
        test_helper::<T>("-1.5", "-0x1.8#2", Down, "-1", Greater);
        test_helper::<T>("-1.5", "-0x1.8#2", Up, "-2", Less);
        test_helper::<T>("-1.5", "-0x1.8#2", Nearest, "-2", Less);

        test_helper::<T>("-2.5", "-0x2.8#3", Floor, "-3", Less);
        test_helper::<T>("-2.5", "-0x2.8#3", Ceiling, "-2", Greater);
        test_helper::<T>("-2.5", "-0x2.8#3", Down, "-2", Greater);
        test_helper::<T>("-2.5", "-0x2.8#3", Up, "-3", Less);
        test_helper::<T>("-2.5", "-0x2.8#3", Nearest, "-2", Greater);

        test_helper::<T>("-123.0", "-0x7b.0#7", Floor, "-123", Equal);
        test_helper::<T>("-123.0", "-0x7b.0#7", Ceiling, "-123", Equal);
        test_helper::<T>("-123.0", "-0x7b.0#7", Down, "-123", Equal);
        test_helper::<T>("-123.0", "-0x7b.0#7", Up, "-123", Equal);
        test_helper::<T>("-123.0", "-0x7b.0#7", Nearest, "-123", Equal);
        test_helper::<T>("-123.0", "-0x7b.0#7", Exact, "-123", Equal);
    }
    apply_fn_to_signeds!(test_helper_i);
}

fn unsigned_rounding_from_float_fail_helper<T: PrimitiveUnsigned + RoundingFrom<Float>>() {
    assert_panic!(T::rounding_from(Float::NAN, Floor));
    assert_panic!(T::rounding_from(Float::NAN, Ceiling));
    assert_panic!(T::rounding_from(Float::NAN, Down));
    assert_panic!(T::rounding_from(Float::NAN, Up));
    assert_panic!(T::rounding_from(Float::NAN, Nearest));
    assert_panic!(T::rounding_from(Float::NAN, Exact));

    assert_panic!(T::rounding_from(Float::INFINITY, Ceiling));
    assert_panic!(T::rounding_from(Float::INFINITY, Up));
    assert_panic!(T::rounding_from(Float::INFINITY, Exact));

    assert_panic!(T::rounding_from(Float::NEGATIVE_INFINITY, Floor));
    assert_panic!(T::rounding_from(Float::NEGATIVE_INFINITY, Up));
    assert_panic!(T::rounding_from(Float::NEGATIVE_INFINITY, Exact));

    assert_panic!(T::rounding_from(Float::NEGATIVE_ONE, Floor));
    assert_panic!(T::rounding_from(Float::NEGATIVE_ONE, Up));
    assert_panic!(T::rounding_from(Float::NEGATIVE_ONE, Exact));

    assert_panic!(T::rounding_from(Float::from(3u8) >> 1, Exact));
}

fn signed_rounding_from_float_fail_helper<T: PrimitiveSigned + RoundingFrom<Float>>() {
    assert_panic!(T::rounding_from(Float::NAN, Floor));
    assert_panic!(T::rounding_from(Float::NAN, Ceiling));
    assert_panic!(T::rounding_from(Float::NAN, Down));
    assert_panic!(T::rounding_from(Float::NAN, Up));
    assert_panic!(T::rounding_from(Float::NAN, Nearest));
    assert_panic!(T::rounding_from(Float::NAN, Exact));

    assert_panic!(T::rounding_from(Float::INFINITY, Ceiling));
    assert_panic!(T::rounding_from(Float::INFINITY, Up));
    assert_panic!(T::rounding_from(Float::INFINITY, Exact));

    assert_panic!(T::rounding_from(Float::NEGATIVE_INFINITY, Floor));
    assert_panic!(T::rounding_from(Float::NEGATIVE_INFINITY, Up));
    assert_panic!(T::rounding_from(Float::NEGATIVE_INFINITY, Exact));

    assert_panic!(T::rounding_from(Float::from(3u8) >> 1, Exact));
    assert_panic!(T::rounding_from(Float::from(-3i8) >> 1, Exact));
}

#[test]
fn rounding_from_float_fail() {
    apply_fn_to_unsigneds!(unsigned_rounding_from_float_fail_helper);
    apply_fn_to_signeds!(signed_rounding_from_float_fail_helper);
}

#[allow(clippy::type_repetition_in_bounds)]
fn unsigned_rounding_from_float_ref_fail_helper<T: PrimitiveUnsigned>()
where
    for<'a> T: RoundingFrom<&'a Float>,
{
    assert_panic!(T::rounding_from(&Float::NAN, Floor));
    assert_panic!(T::rounding_from(&Float::NAN, Ceiling));
    assert_panic!(T::rounding_from(&Float::NAN, Down));
    assert_panic!(T::rounding_from(&Float::NAN, Up));
    assert_panic!(T::rounding_from(&Float::NAN, Nearest));
    assert_panic!(T::rounding_from(&Float::NAN, Exact));

    assert_panic!(T::rounding_from(&Float::INFINITY, Ceiling));
    assert_panic!(T::rounding_from(&Float::INFINITY, Up));
    assert_panic!(T::rounding_from(&Float::INFINITY, Exact));

    assert_panic!(T::rounding_from(&Float::NEGATIVE_INFINITY, Floor));
    assert_panic!(T::rounding_from(&Float::NEGATIVE_INFINITY, Up));
    assert_panic!(T::rounding_from(&Float::NEGATIVE_INFINITY, Exact));

    assert_panic!(T::rounding_from(&Float::NEGATIVE_ONE, Floor));
    assert_panic!(T::rounding_from(&Float::NEGATIVE_ONE, Up));
    assert_panic!(T::rounding_from(&Float::NEGATIVE_ONE, Exact));

    assert_panic!(T::rounding_from(&(Float::from(3u8) >> 1), Exact));
}

#[allow(clippy::type_repetition_in_bounds)]
fn signed_rounding_from_float_ref_fail_helper<T: PrimitiveSigned>()
where
    for<'a> T: RoundingFrom<&'a Float>,
{
    assert_panic!(T::rounding_from(&Float::NAN, Floor));
    assert_panic!(T::rounding_from(&Float::NAN, Ceiling));
    assert_panic!(T::rounding_from(&Float::NAN, Down));
    assert_panic!(T::rounding_from(&Float::NAN, Up));
    assert_panic!(T::rounding_from(&Float::NAN, Nearest));
    assert_panic!(T::rounding_from(&Float::NAN, Exact));

    assert_panic!(T::rounding_from(&Float::INFINITY, Ceiling));
    assert_panic!(T::rounding_from(&Float::INFINITY, Up));
    assert_panic!(T::rounding_from(&Float::INFINITY, Exact));

    assert_panic!(T::rounding_from(&Float::NEGATIVE_INFINITY, Floor));
    assert_panic!(T::rounding_from(&Float::NEGATIVE_INFINITY, Up));
    assert_panic!(T::rounding_from(&Float::NEGATIVE_INFINITY, Exact));

    assert_panic!(T::rounding_from(&(Float::from(3u8) >> 1), Exact));
    assert_panic!(T::rounding_from(&(Float::from(-3i8) >> 1), Exact));
}

#[test]
fn rounding_from_float_ref_fail() {
    apply_fn_to_unsigneds!(unsigned_rounding_from_float_ref_fail_helper);
    apply_fn_to_signeds!(signed_rounding_from_float_ref_fail_helper);
}

#[allow(clippy::type_repetition_in_bounds)]
fn try_from_float_properties_helper_unsigned<
    T: PrimitiveUnsigned + PartialEq<Float> + TryFrom<Float, Error = UnsignedFromFloatError>,
>()
where
    for<'a> T: ConvertibleFrom<&'a Float> + TryFrom<&'a Float, Error = UnsignedFromFloatError>,
    Float: From<T> + PartialEq<T>,
{
    float_gen().test_properties(|x| {
        let t_x = T::try_from(x.clone());

        let t_x_alt = T::try_from(&x);
        assert_eq!(t_x, t_x_alt);

        assert_eq!(t_x.is_ok(), T::convertible_from(&x));
        if let Ok(n) = t_x {
            assert_eq!(T::exact_from(&x), n);
            assert_eq!(n, x);
            assert_eq!(&Float::from(n), &x);
        }
    });
}

#[allow(clippy::type_repetition_in_bounds)]
fn try_from_float_properties_helper_signed<
    T: PrimitiveSigned + PartialEq<Float> + TryFrom<Float, Error = SignedFromFloatError>,
>()
where
    for<'a> T: ConvertibleFrom<&'a Float> + TryFrom<&'a Float, Error = SignedFromFloatError>,
    Float: From<T> + PartialEq<T>,
{
    float_gen().test_properties(|x| {
        let t_x = T::try_from(x.clone());

        let t_x_alt = T::try_from(&x);
        assert_eq!(t_x, t_x_alt);

        assert_eq!(t_x.is_ok(), T::convertible_from(&x));
        if let Ok(n) = t_x {
            assert_eq!(T::exact_from(&x), n);
            assert_eq!(n, x);
            assert_eq!(&Float::from(n), &x);
        }
    });
}

#[test]
fn try_from_float_properties() {
    apply_fn_to_unsigneds!(try_from_float_properties_helper_unsigned);
    apply_fn_to_signeds!(try_from_float_properties_helper_signed);
}

fn convertible_from_float_properties_helper<T>()
where
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    float_gen().test_properties(|x| {
        T::convertible_from(&x);
    });
}

#[test]
fn convertible_from_float_properties() {
    apply_fn_to_primitive_ints!(convertible_from_float_properties_helper);
}

fn rounding_from_float_properties_helper_unsigned<
    T: PrimitiveUnsigned + PartialOrd<Integer> + PartialOrd<Float> + RoundingFrom<Float>,
>()
where
    Float: From<T> + PartialOrd<T>,
    Natural: From<T>,
    for<'a> T: ConvertibleFrom<&'a Float> + RoundingFrom<&'a Float>,
    for<'a> Rational: From<T>,
{
    float_rounding_mode_pair_gen_var_4::<T>().test_properties(|(x, rm)| {
        let no = T::rounding_from(&x, rm);
        assert_eq!(T::rounding_from(x.clone(), rm), no);
        let (n, o) = no;
        if x >= T::ZERO && x <= T::MAX {
            assert!((Rational::from(n) - Rational::exact_from(&x)).lt_abs(&1));
        }

        assert_eq!(n.partial_cmp(&x), Some(o));
        match (x >= T::ZERO, rm) {
            (_, Floor) | (true, Down) | (false, Up) => {
                assert_ne!(o, Greater);
            }
            (_, Ceiling) | (true, Up) | (false, Down) => {
                assert_ne!(o, Less);
            }
            (_, Exact) => assert_eq!(o, Equal),
            _ => {}
        }
    });

    float_gen_var_5().test_properties(|x| {
        let floor = T::rounding_from(&x, Floor);
        if x <= T::MAX {
            assert_eq!(floor.0, Rational::exact_from(&x).floor());
        }
        assert!(floor.0 <= x);
        if floor.0 != T::MAX {
            assert!(floor.0 + T::ONE > x);
        }
        assert_eq!(T::rounding_from(&x, Down), floor);

        let nearest = T::rounding_from(&x, Nearest);
        if x <= T::MAX {
            let ceiling = T::rounding_from(&x, Ceiling);
            assert_eq!(ceiling.0, Rational::exact_from(&x).ceiling());
            assert!(ceiling.0 >= x);
            if x > T::ZERO {
                assert!(ceiling.0 - T::ONE < x);
            }
            assert_eq!(T::rounding_from(&x, Up), ceiling);
            assert!(nearest == floor || nearest == ceiling);
            assert!(
                (Rational::from(nearest.0) - Rational::exact_from(x)).le_abs(&Rational::ONE_HALF)
            );
        } else {
            assert!(nearest == (T::MAX, Less));
        }
    });

    unsigned_gen::<T>().test_properties(|n| {
        let x = Float::from(n);
        let no = (n, Equal);
        assert_eq!(T::rounding_from(&x, Floor), no);
        assert_eq!(T::rounding_from(&x, Down), no);
        assert_eq!(T::rounding_from(&x, Ceiling), no);
        assert_eq!(T::rounding_from(&x, Up), no);
        assert_eq!(T::rounding_from(&x, Nearest), no);
        assert_eq!(T::rounding_from(&x, Exact), no);

        let x = Float::from((no.0 << 1) | T::ONE) >> 1;
        assert!(T::rounding_from(x, Nearest).0.even());
    });
}

fn rounding_from_float_properties_helper_signed<
    T: PrimitiveSigned + RoundingFrom<Float> + PartialOrd<Float> + PartialOrd<Integer>,
>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ConvertibleFrom<&'a Float> + RoundingFrom<&'a Float>,
    Integer: From<T> + PartialEq<T>,
    Rational: From<T>,
{
    float_rounding_mode_pair_gen_var_5::<T>().test_properties(|(x, rm)| {
        let no = T::rounding_from(&x, rm);
        assert_eq!(T::rounding_from(x.clone(), rm), no);
        let (n, o) = no;
        if x >= T::MIN && x <= T::MAX {
            assert!((Rational::from(n) - Rational::exact_from(&x)).lt_abs(&1));
        }

        assert_eq!(n.partial_cmp(&x), Some(o));
        match (x >= T::ZERO, rm) {
            (_, Floor) | (true, Down) | (false, Up) => {
                assert_ne!(o, Greater);
            }
            (_, Ceiling) | (true, Up) | (false, Down) => {
                assert_ne!(o, Less);
            }
            (_, Exact) => assert_eq!(o, Equal),
            _ => {}
        }
    });

    float_gen_var_4().test_properties(|x| {
        let mut o_floor = None;
        if x >= T::MIN {
            let floor = T::rounding_from(&x, Floor);
            assert!(floor.0 <= x);
            if x <= T::MAX {
                assert_eq!(floor.0, Rational::exact_from(&x).floor());
            }
            if floor.0 != T::MAX {
                assert!(floor.0 + T::ONE > x);
            }
            assert_eq!(
                T::rounding_from(&x, if x >= T::ZERO { Down } else { Up }),
                floor
            );
            o_floor = Some(floor);
        }
        let mut o_ceiling = None;
        if x <= T::MAX {
            let ceiling = T::rounding_from(&x, Ceiling);
            assert!(ceiling.0 >= x);
            if x >= T::MIN {
                assert_eq!(ceiling.0, Rational::exact_from(&x).ceiling());
            }
            if ceiling.0 != T::MIN {
                assert!(ceiling.0 - T::ONE < x);
            }
            assert_eq!(
                T::rounding_from(&x, if x >= T::ZERO { Up } else { Down }),
                ceiling
            );
            o_ceiling = Some(ceiling);
        }
        let nearest = T::rounding_from(&x, Nearest);
        if let Some(floor) = o_floor {
            if let Some(ceiling) = o_ceiling {
                assert!(nearest == floor || nearest == ceiling);
            }
        }
        if x >= T::MIN && x <= T::MAX {
            assert!(
                (Rational::from(nearest.0) - Rational::exact_from(x)).le_abs(&Rational::ONE_HALF)
            );
        }
    });

    signed_gen::<T>().test_properties(|n| {
        let x = Float::from(n);
        let no = (n, Equal);
        assert_eq!(T::rounding_from(&x, Floor), no);
        assert_eq!(T::rounding_from(&x, Down), no);
        assert_eq!(T::rounding_from(&x, Ceiling), no);
        assert_eq!(T::rounding_from(&x, Up), no);
        assert_eq!(T::rounding_from(&x, Nearest), no);
        assert_eq!(T::rounding_from(&x, Exact), no);

        let x = Float::from((no.0 << 1) | T::ONE) >> 1;
        assert!(T::rounding_from(x, Nearest).0.even());
    });
}

#[test]
fn rounding_from_float_properties() {
    apply_fn_to_unsigneds!(rounding_from_float_properties_helper_unsigned);
    apply_fn_to_signeds!(rounding_from_float_properties_helper_signed);
}
