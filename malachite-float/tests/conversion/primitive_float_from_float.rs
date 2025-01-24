// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::strings::ToDebugString;
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_float::conversion::primitive_float_from_float::FloatFromFloatError;
use malachite_float::test_util::common::{parse_hex_string, rug_round_try_from_rounding_mode};
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_12, float_gen_var_4, float_rounding_mode_pair_gen_var_20,
    float_rounding_mode_pair_gen_var_6,
};
use malachite_float::Float;
use malachite_nz::integer::Integer;
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

#[allow(clippy::type_repetition_in_bounds)]
#[test]
fn test_try_from_float() {
    fn test_helper<T: PrimitiveFloat + TryFrom<Float, Error = FloatFromFloatError>>(
        s: &str,
        s_hex: &str,
        out: &str,
    ) where
        for<'a> T: TryFrom<&'a Float, Error = FloatFromFloatError>,
    {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let on = match T::try_from(x.clone()) {
            Ok(n) => format!("Ok({})", NiceFloat(n)),
            e => e.to_debug_string(),
        };
        assert_eq!(on, out);

        let on = match T::try_from(&x) {
            Ok(n) => format!("Ok({})", NiceFloat(n)),
            e => e.to_debug_string(),
        };
        assert_eq!(on, out);
    }
    fn test_helper_2<T: PrimitiveFloat + TryFrom<Float, Error = FloatFromFloatError>>()
    where
        for<'a> T: TryFrom<&'a Float, Error = FloatFromFloatError>,
    {
        test_helper::<T>("NaN", "NaN", "Ok(NaN)");
        test_helper::<T>("Infinity", "Infinity", "Ok(Infinity)");
        test_helper::<T>("-Infinity", "-Infinity", "Ok(-Infinity)");
        test_helper::<T>("0.0", "0x0.0", "Ok(0.0)");
        test_helper::<T>("-0.0", "-0x0.0", "Ok(-0.0)");

        test_helper::<T>("1.0", "0x1.0#1", "Ok(1.0)");
        test_helper::<T>("2.0", "0x2.0#1", "Ok(2.0)");
        test_helper::<T>("0.5", "0x0.8#1", "Ok(0.5)");
        test_helper::<T>("123.0", "0x7b.0#7", "Ok(123.0)");
        test_helper::<T>(
            "0.333333333333333332",
            "0x0.555555555555554#57",
            "Err(Inexact)",
        );
        test_helper::<T>("2.0e2408", "0x1.0E+2000#1", "Err(Overflow)");
        test_helper::<T>("6.0e-2409", "0x1.0E-2000#1", "Err(Underflow)");
        test_helper::<T>("too_big", "0x4.0E+268435455#1", "Err(Overflow)");
        test_helper::<T>("too_small", "0x1.0E-268435456#1", "Err(Underflow)");

        test_helper::<T>("-1.0", "-0x1.0#1", "Ok(-1.0)");
        test_helper::<T>("-2.0", "-0x2.0#1", "Ok(-2.0)");
        test_helper::<T>("-0.5", "-0x0.8#1", "Ok(-0.5)");
        test_helper::<T>("-123.0", "-0x7b.0#7", "Ok(-123.0)");
        test_helper::<T>(
            "-0.333333333333333332",
            "-0x0.555555555555554#57",
            "Err(Inexact)",
        );
        test_helper::<T>("-2.0e2408", "-0x1.0E+2000#1", "Err(Overflow)");
        test_helper::<T>("-6.0e-2409", "-0x1.0E-2000#1", "Err(Underflow)");
        test_helper::<T>("-too_big", "-0x4.0E+268435455#1", "Err(Overflow)");
        test_helper::<T>("-too_small", "-0x1.0E-268435456#1", "Err(Underflow)");
    }
    apply_fn_to_primitive_floats!(test_helper_2);
    test_helper::<f32>("0.33333334", "0x0.5555558#24", "Ok(0.33333334)");
    test_helper::<f64>("0.33333334", "0x0.5555558#24", "Ok(0.3333333432674408)");
    test_helper::<f32>(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "Err(Inexact)",
    );
    test_helper::<f64>(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "Ok(0.3333333333333333)",
    );
    test_helper::<f32>("7.0e240", "0x1.0E+200#1", "Err(Overflow)");
    test_helper::<f64>("7.0e240", "0x1.0E+200#1", "Ok(6.668014432879854e240)");
    test_helper::<f32>("1.0e-241", "0x1.0E-200#1", "Err(Underflow)");
    test_helper::<f64>("1.0e-241", "0x1.0E-200#1", "Ok(1.499696813895631e-241)");

    test_helper::<f32>("-0.33333334", "-0x0.5555558#24", "Ok(-0.33333334)");
    test_helper::<f64>("-0.33333334", "-0x0.5555558#24", "Ok(-0.3333333432674408)");
    test_helper::<f32>(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        "Err(Inexact)",
    );
    test_helper::<f64>(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        "Ok(-0.3333333333333333)",
    );
    test_helper::<f32>("-7.0e240", "-0x1.0E+200#1", "Err(Overflow)");
    test_helper::<f64>("-7.0e240", "-0x1.0E+200#1", "Ok(-6.668014432879854e240)");
    test_helper::<f32>("-1.0e-241", "-0x1.0E-200#1", "Err(Underflow)");
    test_helper::<f64>("-1.0e-241", "-0x1.0E-200#1", "Ok(-1.499696813895631e-241)");
}

#[test]
fn test_convertible_from_float() {
    #[allow(clippy::type_repetition_in_bounds)]
    fn test_helper<T: PrimitiveFloat>(s: &str, s_hex: &str, out: bool)
    where
        for<'a> T: ConvertibleFrom<&'a Float>,
    {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        assert_eq!(T::convertible_from(&x), out);
    }
    #[allow(clippy::type_repetition_in_bounds)]
    fn test_helper_2<T: PrimitiveFloat>()
    where
        for<'a> T: ConvertibleFrom<&'a Float>,
    {
        test_helper::<T>("NaN", "NaN", true);
        test_helper::<T>("Infinity", "Infinity", true);
        test_helper::<T>("-Infinity", "-Infinity", true);
        test_helper::<T>("0.0", "0x0.0", true);
        test_helper::<T>("-0.0", "-0x0.0", true);

        test_helper::<T>("1.0", "0x1.0#1", true);
        test_helper::<T>("2.0", "0x2.0#1", true);
        test_helper::<T>("0.5", "0x0.8#1", true);
        test_helper::<T>("0.33333334", "0x0.5555558#24", true);
        test_helper::<T>("123.0", "0x7b.0#7", true);
        test_helper::<T>("0.333333333333333332", "0x0.555555555555554#57", false);
        test_helper::<T>("2.0e2408", "0x1.0E+2000#1", false);
        test_helper::<T>("6.0e-2409", "0x1.0E-2000#1", false);
        test_helper::<T>("too_big", "0x4.0E+268435455#1", false);
        test_helper::<T>("too_small", "0x1.0E-268435456#1", false);

        test_helper::<T>("-1.0", "-0x1.0#1", true);
        test_helper::<T>("-2.0", "-0x2.0#1", true);
        test_helper::<T>("-0.5", "-0x0.8#1", true);
        test_helper::<T>("-0.33333334", "-0x0.5555558#24", true);
        test_helper::<T>("-123.0", "-0x7b.0#7", true);
        test_helper::<T>("-0.333333333333333332", "-0x0.555555555555554#57", false);
        test_helper::<T>("-2.0e2408", "-0x1.0E+2000#1", false);
        test_helper::<T>("-6.0e-2409", "-0x1.0E-2000#1", false);
        test_helper::<T>("-too_big", "-0x4.0E+268435455#1", false);
        test_helper::<T>("-too_small", "-0x1.0E-268435456#1", false);
    }
    apply_fn_to_primitive_floats!(test_helper_2);
    test_helper::<f32>("0.33333333333333331", "0x0.55555555555554#53", false);
    test_helper::<f64>("0.33333333333333331", "0x0.55555555555554#53", true);
    test_helper::<f32>("7.0e240", "0x1.0E+200#1", false);
    test_helper::<f64>("7.0e240", "0x1.0E+200#1", true);
    test_helper::<f32>("1.0e-241", "0x1.0E-200#1", false);
    test_helper::<f64>("1.0e-241", "0x1.0E-200#1", true);

    test_helper::<f32>("-0.33333333333333331", "-0x0.55555555555554#53", false);
    test_helper::<f64>("-0.33333333333333331", "-0x0.55555555555554#53", true);
    test_helper::<f32>("-7.0e240", "-0x1.0E+200#1", false);
    test_helper::<f64>("-7.0e240", "-0x1.0E+200#1", true);
    test_helper::<f32>("-1.0e-241", "-0x1.0E-200#1", false);
    test_helper::<f64>("-1.0e-241", "-0x1.0E-200#1", true);
}

#[test]
fn test_rounding_from_float() {
    #[allow(clippy::type_repetition_in_bounds)]
    fn test_helper<T: PrimitiveFloat + RoundingFrom<Float>>(
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
        assert_eq!(NiceFloat(n).to_string(), out);
        assert_eq!(o, o_out);

        let (n, o) = T::rounding_from(&x, rm);
        assert_eq!(NiceFloat(n).to_string(), out);
        assert_eq!(o, o_out);
    }
    #[allow(clippy::type_repetition_in_bounds)]
    fn test_helper_2<T: PrimitiveFloat + RoundingFrom<Float>>()
    where
        for<'a> T: RoundingFrom<&'a Float>,
    {
        test_helper::<T>("NaN", "NaN", Floor, "NaN", Equal);
        test_helper::<T>("NaN", "NaN", Ceiling, "NaN", Equal);
        test_helper::<T>("NaN", "NaN", Down, "NaN", Equal);
        test_helper::<T>("NaN", "NaN", Up, "NaN", Equal);
        test_helper::<T>("NaN", "NaN", Nearest, "NaN", Equal);
        test_helper::<T>("NaN", "NaN", Exact, "NaN", Equal);

        test_helper::<T>("Infinity", "Infinity", Floor, "Infinity", Equal);
        test_helper::<T>("Infinity", "Infinity", Ceiling, "Infinity", Equal);
        test_helper::<T>("Infinity", "Infinity", Down, "Infinity", Equal);
        test_helper::<T>("Infinity", "Infinity", Up, "Infinity", Equal);
        test_helper::<T>("Infinity", "Infinity", Nearest, "Infinity", Equal);
        test_helper::<T>("Infinity", "Infinity", Exact, "Infinity", Equal);

        test_helper::<T>("-Infinity", "-Infinity", Floor, "-Infinity", Equal);
        test_helper::<T>("-Infinity", "-Infinity", Ceiling, "-Infinity", Equal);
        test_helper::<T>("-Infinity", "-Infinity", Down, "-Infinity", Equal);
        test_helper::<T>("-Infinity", "-Infinity", Up, "-Infinity", Equal);
        test_helper::<T>("-Infinity", "-Infinity", Nearest, "-Infinity", Equal);
        test_helper::<T>("-Infinity", "-Infinity", Exact, "-Infinity", Equal);

        test_helper::<T>("0.0", "0x0.0", Floor, "0.0", Equal);
        test_helper::<T>("0.0", "0x0.0", Ceiling, "0.0", Equal);
        test_helper::<T>("0.0", "0x0.0", Down, "0.0", Equal);
        test_helper::<T>("0.0", "0x0.0", Up, "0.0", Equal);
        test_helper::<T>("0.0", "0x0.0", Nearest, "0.0", Equal);
        test_helper::<T>("0.0", "0x0.0", Exact, "0.0", Equal);

        test_helper::<T>("-0.0", "-0x0.0", Floor, "-0.0", Equal);
        test_helper::<T>("-0.0", "-0x0.0", Ceiling, "-0.0", Equal);
        test_helper::<T>("-0.0", "-0x0.0", Down, "-0.0", Equal);
        test_helper::<T>("-0.0", "-0x0.0", Up, "-0.0", Equal);
        test_helper::<T>("-0.0", "-0x0.0", Nearest, "-0.0", Equal);
        test_helper::<T>("-0.0", "-0x0.0", Exact, "-0.0", Equal);

        test_helper::<T>("1.0", "0x1.0#1", Floor, "1.0", Equal);
        test_helper::<T>("1.0", "0x1.0#1", Ceiling, "1.0", Equal);
        test_helper::<T>("1.0", "0x1.0#1", Down, "1.0", Equal);
        test_helper::<T>("1.0", "0x1.0#1", Up, "1.0", Equal);
        test_helper::<T>("1.0", "0x1.0#1", Nearest, "1.0", Equal);
        test_helper::<T>("1.0", "0x1.0#1", Exact, "1.0", Equal);

        test_helper::<T>("-1.0", "-0x1.0#1", Floor, "-1.0", Equal);
        test_helper::<T>("-1.0", "-0x1.0#1", Ceiling, "-1.0", Equal);
        test_helper::<T>("-1.0", "-0x1.0#1", Down, "-1.0", Equal);
        test_helper::<T>("-1.0", "-0x1.0#1", Up, "-1.0", Equal);
        test_helper::<T>("-1.0", "-0x1.0#1", Nearest, "-1.0", Equal);
        test_helper::<T>("-1.0", "-0x1.0#1", Exact, "-1.0", Equal);
    }
    apply_fn_to_primitive_floats!(test_helper_2);

    test_helper::<f32>("0.33333334", "0x0.5555558#24", Floor, "0.33333334", Equal);
    test_helper::<f32>("0.33333334", "0x0.5555558#24", Ceiling, "0.33333334", Equal);
    test_helper::<f32>("0.33333334", "0x0.5555558#24", Down, "0.33333334", Equal);
    test_helper::<f32>("0.33333334", "0x0.5555558#24", Up, "0.33333334", Equal);
    test_helper::<f32>("0.33333334", "0x0.5555558#24", Nearest, "0.33333334", Equal);
    test_helper::<f32>("0.33333334", "0x0.5555558#24", Exact, "0.33333334", Equal);

    test_helper::<f64>(
        "0.33333334",
        "0x0.5555558#24",
        Floor,
        "0.3333333432674408",
        Equal,
    );
    test_helper::<f64>(
        "0.33333334",
        "0x0.5555558#24",
        Ceiling,
        "0.3333333432674408",
        Equal,
    );
    test_helper::<f64>(
        "0.33333334",
        "0x0.5555558#24",
        Down,
        "0.3333333432674408",
        Equal,
    );
    test_helper::<f64>(
        "0.33333334",
        "0x0.5555558#24",
        Up,
        "0.3333333432674408",
        Equal,
    );
    test_helper::<f64>(
        "0.33333334",
        "0x0.5555558#24",
        Nearest,
        "0.3333333432674408",
        Equal,
    );
    test_helper::<f64>(
        "0.33333334",
        "0x0.5555558#24",
        Exact,
        "0.3333333432674408",
        Equal,
    );

    test_helper::<f32>(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Floor,
        "0.3333333",
        Less,
    );
    test_helper::<f32>(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Ceiling,
        "0.33333334",
        Greater,
    );
    test_helper::<f32>(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Down,
        "0.3333333",
        Less,
    );
    test_helper::<f32>(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Up,
        "0.33333334",
        Greater,
    );
    test_helper::<f32>(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Nearest,
        "0.33333334",
        Greater,
    );

    test_helper::<f64>(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Floor,
        "0.3333333333333333",
        Equal,
    );
    test_helper::<f64>(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Ceiling,
        "0.3333333333333333",
        Equal,
    );
    test_helper::<f64>(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Down,
        "0.3333333333333333",
        Equal,
    );
    test_helper::<f64>(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Up,
        "0.3333333333333333",
        Equal,
    );
    test_helper::<f64>(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Nearest,
        "0.3333333333333333",
        Equal,
    );
    test_helper::<f64>(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Exact,
        "0.3333333333333333",
        Equal,
    );

    test_helper::<f32>(
        "0.333333333333333332",
        "0x0.555555555555554#57",
        Floor,
        "0.3333333",
        Less,
    );
    test_helper::<f32>(
        "0.333333333333333332",
        "0x0.555555555555554#57",
        Ceiling,
        "0.33333334",
        Greater,
    );
    test_helper::<f32>(
        "0.333333333333333332",
        "0x0.555555555555554#57",
        Down,
        "0.3333333",
        Less,
    );
    test_helper::<f32>(
        "0.333333333333333332",
        "0x0.555555555555554#57",
        Up,
        "0.33333334",
        Greater,
    );
    test_helper::<f32>(
        "0.333333333333333332",
        "0x0.555555555555554#57",
        Nearest,
        "0.33333334",
        Greater,
    );

    test_helper::<f64>(
        "0.333333333333333332",
        "0x0.555555555555554#57",
        Floor,
        "0.3333333333333333",
        Less,
    );
    test_helper::<f64>(
        "0.333333333333333332",
        "0x0.555555555555554#57",
        Ceiling,
        "0.33333333333333337",
        Greater,
    );
    test_helper::<f64>(
        "0.333333333333333332",
        "0x0.555555555555554#57",
        Down,
        "0.3333333333333333",
        Less,
    );
    test_helper::<f64>(
        "0.333333333333333332",
        "0x0.555555555555554#57",
        Up,
        "0.33333333333333337",
        Greater,
    );
    test_helper::<f64>(
        "0.333333333333333332",
        "0x0.555555555555554#57",
        Nearest,
        "0.3333333333333333",
        Less,
    );

    test_helper::<f32>("7.0e240", "0x1.0E+200#1", Floor, "3.4028235e38", Less);
    test_helper::<f32>("7.0e240", "0x1.0E+200#1", Ceiling, "Infinity", Greater);
    test_helper::<f32>("7.0e240", "0x1.0E+200#1", Down, "3.4028235e38", Less);
    test_helper::<f32>("7.0e240", "0x1.0E+200#1", Up, "Infinity", Greater);
    test_helper::<f32>("7.0e240", "0x1.0E+200#1", Nearest, "3.4028235e38", Less);

    test_helper::<f64>(
        "7.0e240",
        "0x1.0E+200#1",
        Floor,
        "6.668014432879854e240",
        Equal,
    );
    test_helper::<f64>(
        "7.0e240",
        "0x1.0E+200#1",
        Ceiling,
        "6.668014432879854e240",
        Equal,
    );
    test_helper::<f64>(
        "7.0e240",
        "0x1.0E+200#1",
        Down,
        "6.668014432879854e240",
        Equal,
    );
    test_helper::<f64>(
        "7.0e240",
        "0x1.0E+200#1",
        Up,
        "6.668014432879854e240",
        Equal,
    );
    test_helper::<f64>(
        "7.0e240",
        "0x1.0E+200#1",
        Nearest,
        "6.668014432879854e240",
        Equal,
    );
    test_helper::<f64>(
        "7.0e240",
        "0x1.0E+200#1",
        Exact,
        "6.668014432879854e240",
        Equal,
    );

    test_helper::<f32>("2.0e2408", "0x1.0E+2000#1", Floor, "3.4028235e38", Less);
    test_helper::<f32>("2.0e2408", "0x1.0E+2000#1", Ceiling, "Infinity", Greater);
    test_helper::<f32>("2.0e2408", "0x1.0E+2000#1", Down, "3.4028235e38", Less);
    test_helper::<f32>("2.0e2408", "0x1.0E+2000#1", Up, "Infinity", Greater);
    test_helper::<f32>("2.0e2408", "0x1.0E+2000#1", Nearest, "3.4028235e38", Less);

    test_helper::<f64>(
        "2.0e2408",
        "0x1.0E+2000#1",
        Floor,
        "1.7976931348623157e308",
        Less,
    );
    test_helper::<f64>("2.0e2408", "0x1.0E+2000#1", Ceiling, "Infinity", Greater);
    test_helper::<f64>(
        "2.0e2408",
        "0x1.0E+2000#1",
        Down,
        "1.7976931348623157e308",
        Less,
    );
    test_helper::<f64>("2.0e2408", "0x1.0E+2000#1", Up, "Infinity", Greater);
    test_helper::<f64>(
        "2.0e2408",
        "0x1.0E+2000#1",
        Nearest,
        "1.7976931348623157e308",
        Less,
    );
    test_helper::<f32>("1.0e-241", "0x1.0E-200#1", Floor, "0.0", Less);
    test_helper::<f32>("1.0e-241", "0x1.0E-200#1", Ceiling, "1.0e-45", Greater);
    test_helper::<f32>("1.0e-241", "0x1.0E-200#1", Down, "0.0", Less);
    test_helper::<f32>("1.0e-241", "0x1.0E-200#1", Up, "1.0e-45", Greater);
    test_helper::<f32>("1.0e-241", "0x1.0E-200#1", Nearest, "0.0", Less);

    test_helper::<f64>(
        "1.0e-241",
        "0x1.0E-200#1",
        Floor,
        "1.499696813895631e-241",
        Equal,
    );
    test_helper::<f64>(
        "1.0e-241",
        "0x1.0E-200#1",
        Ceiling,
        "1.499696813895631e-241",
        Equal,
    );
    test_helper::<f64>(
        "1.0e-241",
        "0x1.0E-200#1",
        Down,
        "1.499696813895631e-241",
        Equal,
    );
    test_helper::<f64>(
        "1.0e-241",
        "0x1.0E-200#1",
        Up,
        "1.499696813895631e-241",
        Equal,
    );
    test_helper::<f64>(
        "1.0e-241",
        "0x1.0E-200#1",
        Nearest,
        "1.499696813895631e-241",
        Equal,
    );
    test_helper::<f64>(
        "1.0e-241",
        "0x1.0E-200#1",
        Exact,
        "1.499696813895631e-241",
        Equal,
    );

    test_helper::<f32>("6.0e-2409", "0x1.0E-2000#1", Floor, "0.0", Less);
    test_helper::<f32>("6.0e-2409", "0x1.0E-2000#1", Ceiling, "1.0e-45", Greater);
    test_helper::<f32>("6.0e-2409", "0x1.0E-2000#1", Down, "0.0", Less);
    test_helper::<f32>("6.0e-2409", "0x1.0E-2000#1", Up, "1.0e-45", Greater);
    test_helper::<f32>("6.0e-2409", "0x1.0E-2000#1", Nearest, "0.0", Less);

    test_helper::<f64>("6.0e-2409", "0x1.0E-2000#1", Floor, "0.0", Less);
    test_helper::<f64>("6.0e-2409", "0x1.0E-2000#1", Ceiling, "5.0e-324", Greater);
    test_helper::<f64>("6.0e-2409", "0x1.0E-2000#1", Down, "0.0", Less);
    test_helper::<f64>("6.0e-2409", "0x1.0E-2000#1", Up, "5.0e-324", Greater);
    test_helper::<f64>("6.0e-2409", "0x1.0E-2000#1", Nearest, "0.0", Less);

    test_helper::<f32>("too_big", "0x4.0E+268435455#1", Floor, "3.4028235e38", Less);
    test_helper::<f32>(
        "too_big",
        "0x4.0E+268435455#1",
        Ceiling,
        "Infinity",
        Greater,
    );
    test_helper::<f32>("too_big", "0x4.0E+268435455#1", Down, "3.4028235e38", Less);
    test_helper::<f32>("too_big", "0x4.0E+268435455#1", Up, "Infinity", Greater);
    test_helper::<f32>(
        "too_big",
        "0x4.0E+268435455#1",
        Nearest,
        "3.4028235e38",
        Less,
    );

    test_helper::<f64>(
        "too_big",
        "0x4.0E+268435455#1",
        Floor,
        "1.7976931348623157e308",
        Less,
    );
    test_helper::<f64>(
        "too_big",
        "0x4.0E+268435455#1",
        Ceiling,
        "Infinity",
        Greater,
    );
    test_helper::<f64>(
        "too_big",
        "0x4.0E+268435455#1",
        Down,
        "1.7976931348623157e308",
        Less,
    );
    test_helper::<f64>("too_big", "0x4.0E+268435455#1", Up, "Infinity", Greater);
    test_helper::<f64>(
        "too_big",
        "0x4.0E+268435455#1",
        Nearest,
        "1.7976931348623157e308",
        Less,
    );

    test_helper::<f32>("too_small", "0x1.0E-268435456#1", Floor, "0.0", Less);
    test_helper::<f32>(
        "too_small",
        "0x1.0E-268435456#1",
        Ceiling,
        "1.0e-45",
        Greater,
    );
    test_helper::<f32>("too_small", "0x1.0E-268435456#1", Down, "0.0", Less);
    test_helper::<f32>("too_small", "0x1.0E-268435456#1", Up, "1.0e-45", Greater);
    test_helper::<f32>("too_small", "0x1.0E-268435456#1", Nearest, "0.0", Less);

    test_helper::<f64>("too_small", "0x1.0E-268435456#1", Floor, "0.0", Less);
    test_helper::<f64>(
        "too_small",
        "0x1.0E-268435456#1",
        Ceiling,
        "5.0e-324",
        Greater,
    );
    test_helper::<f64>("too_small", "0x1.0E-268435456#1", Down, "0.0", Less);
    test_helper::<f64>("too_small", "0x1.0E-268435456#1", Up, "5.0e-324", Greater);
    test_helper::<f64>("too_small", "0x1.0E-268435456#1", Nearest, "0.0", Less);

    test_helper::<f32>(
        "-0.33333334",
        "-0x0.5555558#24",
        Floor,
        "-0.33333334",
        Equal,
    );
    test_helper::<f32>(
        "-0.33333334",
        "-0x0.5555558#24",
        Ceiling,
        "-0.33333334",
        Equal,
    );
    test_helper::<f32>("-0.33333334", "-0x0.5555558#24", Down, "-0.33333334", Equal);
    test_helper::<f32>("-0.33333334", "-0x0.5555558#24", Up, "-0.33333334", Equal);
    test_helper::<f32>(
        "-0.33333334",
        "-0x0.5555558#24",
        Nearest,
        "-0.33333334",
        Equal,
    );
    test_helper::<f32>(
        "-0.33333334",
        "-0x0.5555558#24",
        Exact,
        "-0.33333334",
        Equal,
    );

    test_helper::<f64>(
        "-0.33333334",
        "-0x0.5555558#24",
        Floor,
        "-0.3333333432674408",
        Equal,
    );
    test_helper::<f64>(
        "-0.33333334",
        "-0x0.5555558#24",
        Ceiling,
        "-0.3333333432674408",
        Equal,
    );
    test_helper::<f64>(
        "-0.33333334",
        "-0x0.5555558#24",
        Down,
        "-0.3333333432674408",
        Equal,
    );
    test_helper::<f64>(
        "-0.33333334",
        "-0x0.5555558#24",
        Up,
        "-0.3333333432674408",
        Equal,
    );
    test_helper::<f64>(
        "-0.33333334",
        "-0x0.5555558#24",
        Nearest,
        "-0.3333333432674408",
        Equal,
    );
    test_helper::<f64>(
        "-0.33333334",
        "-0x0.5555558#24",
        Exact,
        "-0.3333333432674408",
        Equal,
    );

    test_helper::<f32>(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        Floor,
        "-0.33333334",
        Less,
    );
    test_helper::<f32>(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        Ceiling,
        "-0.3333333",
        Greater,
    );
    test_helper::<f32>(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        Down,
        "-0.3333333",
        Greater,
    );
    test_helper::<f32>(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        Up,
        "-0.33333334",
        Less,
    );
    test_helper::<f32>(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        Nearest,
        "-0.33333334",
        Less,
    );

    test_helper::<f64>(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        Floor,
        "-0.3333333333333333",
        Equal,
    );
    test_helper::<f64>(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        Ceiling,
        "-0.3333333333333333",
        Equal,
    );
    test_helper::<f64>(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        Down,
        "-0.3333333333333333",
        Equal,
    );
    test_helper::<f64>(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        Up,
        "-0.3333333333333333",
        Equal,
    );
    test_helper::<f64>(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        Nearest,
        "-0.3333333333333333",
        Equal,
    );
    test_helper::<f64>(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        Exact,
        "-0.3333333333333333",
        Equal,
    );

    test_helper::<f32>(
        "-0.333333333333333332",
        "-0x0.555555555555554#57",
        Floor,
        "-0.33333334",
        Less,
    );
    test_helper::<f32>(
        "-0.333333333333333332",
        "-0x0.555555555555554#57",
        Ceiling,
        "-0.3333333",
        Greater,
    );
    test_helper::<f32>(
        "-0.333333333333333332",
        "-0x0.555555555555554#57",
        Down,
        "-0.3333333",
        Greater,
    );
    test_helper::<f32>(
        "-0.333333333333333332",
        "-0x0.555555555555554#57",
        Up,
        "-0.33333334",
        Less,
    );
    test_helper::<f32>(
        "-0.333333333333333332",
        "-0x0.555555555555554#57",
        Nearest,
        "-0.33333334",
        Less,
    );

    test_helper::<f64>(
        "-0.333333333333333332",
        "-0x0.555555555555554#57",
        Floor,
        "-0.33333333333333337",
        Less,
    );
    test_helper::<f64>(
        "-0.333333333333333332",
        "-0x0.555555555555554#57",
        Ceiling,
        "-0.3333333333333333",
        Greater,
    );
    test_helper::<f64>(
        "-0.333333333333333332",
        "-0x0.555555555555554#57",
        Down,
        "-0.3333333333333333",
        Greater,
    );
    test_helper::<f64>(
        "-0.333333333333333332",
        "-0x0.555555555555554#57",
        Up,
        "-0.33333333333333337",
        Less,
    );
    test_helper::<f64>(
        "-0.333333333333333332",
        "-0x0.555555555555554#57",
        Nearest,
        "-0.3333333333333333",
        Greater,
    );

    test_helper::<f32>("-7.0e240", "-0x1.0E+200#1", Floor, "-Infinity", Less);
    test_helper::<f32>(
        "-7.0e240",
        "-0x1.0E+200#1",
        Ceiling,
        "-3.4028235e38",
        Greater,
    );
    test_helper::<f32>("-7.0e240", "-0x1.0E+200#1", Down, "-3.4028235e38", Greater);
    test_helper::<f32>("-7.0e240", "-0x1.0E+200#1", Up, "-Infinity", Less);
    test_helper::<f32>(
        "-7.0e240",
        "-0x1.0E+200#1",
        Nearest,
        "-3.4028235e38",
        Greater,
    );

    test_helper::<f64>(
        "-7.0e240",
        "-0x1.0E+200#1",
        Floor,
        "-6.668014432879854e240",
        Equal,
    );
    test_helper::<f64>(
        "-7.0e240",
        "-0x1.0E+200#1",
        Ceiling,
        "-6.668014432879854e240",
        Equal,
    );
    test_helper::<f64>(
        "-7.0e240",
        "-0x1.0E+200#1",
        Down,
        "-6.668014432879854e240",
        Equal,
    );
    test_helper::<f64>(
        "-7.0e240",
        "-0x1.0E+200#1",
        Up,
        "-6.668014432879854e240",
        Equal,
    );
    test_helper::<f64>(
        "-7.0e240",
        "-0x1.0E+200#1",
        Nearest,
        "-6.668014432879854e240",
        Equal,
    );
    test_helper::<f64>(
        "-7.0e240",
        "-0x1.0E+200#1",
        Exact,
        "-6.668014432879854e240",
        Equal,
    );

    test_helper::<f32>("-2.0e2408", "-0x1.0E+2000#1", Floor, "-Infinity", Less);
    test_helper::<f32>(
        "-2.0e2408",
        "-0x1.0E+2000#1",
        Ceiling,
        "-3.4028235e38",
        Greater,
    );
    test_helper::<f32>(
        "-2.0e2408",
        "-0x1.0E+2000#1",
        Down,
        "-3.4028235e38",
        Greater,
    );
    test_helper::<f32>("-2.0e2408", "-0x1.0E+2000#1", Up, "-Infinity", Less);
    test_helper::<f32>(
        "-2.0e2408",
        "-0x1.0E+2000#1",
        Nearest,
        "-3.4028235e38",
        Greater,
    );

    test_helper::<f64>("-2.0e2408", "-0x1.0E+2000#1", Floor, "-Infinity", Less);
    test_helper::<f64>(
        "-2.0e2408",
        "-0x1.0E+2000#1",
        Ceiling,
        "-1.7976931348623157e308",
        Greater,
    );
    test_helper::<f64>(
        "-2.0e2408",
        "-0x1.0E+2000#1",
        Down,
        "-1.7976931348623157e308",
        Greater,
    );
    test_helper::<f64>("-2.0e2408", "-0x1.0E+2000#1", Up, "-Infinity", Less);
    test_helper::<f64>(
        "-2.0e2408",
        "-0x1.0E+2000#1",
        Nearest,
        "-1.7976931348623157e308",
        Greater,
    );

    test_helper::<f32>("-1.0e-241", "-0x1.0E-200#1", Floor, "-1.0e-45", Less);
    test_helper::<f32>("-1.0e-241", "-0x1.0E-200#1", Ceiling, "-0.0", Greater);
    test_helper::<f32>("-1.0e-241", "-0x1.0E-200#1", Down, "-0.0", Greater);
    test_helper::<f32>("-1.0e-241", "-0x1.0E-200#1", Up, "-1.0e-45", Less);
    test_helper::<f32>("-1.0e-241", "-0x1.0E-200#1", Nearest, "-0.0", Greater);

    test_helper::<f64>(
        "-1.0e-241",
        "-0x1.0E-200#1",
        Floor,
        "-1.499696813895631e-241",
        Equal,
    );
    test_helper::<f64>(
        "-1.0e-241",
        "-0x1.0E-200#1",
        Ceiling,
        "-1.499696813895631e-241",
        Equal,
    );
    test_helper::<f64>(
        "-1.0e-241",
        "-0x1.0E-200#1",
        Down,
        "-1.499696813895631e-241",
        Equal,
    );
    test_helper::<f64>(
        "-1.0e-241",
        "-0x1.0E-200#1",
        Up,
        "-1.499696813895631e-241",
        Equal,
    );
    test_helper::<f64>(
        "-1.0e-241",
        "-0x1.0E-200#1",
        Nearest,
        "-1.499696813895631e-241",
        Equal,
    );
    test_helper::<f64>(
        "-1.0e-241",
        "-0x1.0E-200#1",
        Exact,
        "-1.499696813895631e-241",
        Equal,
    );

    test_helper::<f32>("-6.0e-2409", "-0x1.0E-2000#1", Floor, "-1.0e-45", Less);
    test_helper::<f32>("-6.0e-2409", "-0x1.0E-2000#1", Ceiling, "-0.0", Greater);
    test_helper::<f32>("-6.0e-2409", "-0x1.0E-2000#1", Down, "-0.0", Greater);
    test_helper::<f32>("-6.0e-2409", "-0x1.0E-2000#1", Up, "-1.0e-45", Less);
    test_helper::<f32>("-6.0e-2409", "-0x1.0E-2000#1", Nearest, "-0.0", Greater);

    test_helper::<f64>("-6.0e-2409", "-0x1.0E-2000#1", Floor, "-5.0e-324", Less);
    test_helper::<f64>("-6.0e-2409", "-0x1.0E-2000#1", Ceiling, "-0.0", Greater);
    test_helper::<f64>("-6.0e-2409", "-0x1.0E-2000#1", Down, "-0.0", Greater);
    test_helper::<f64>("-6.0e-2409", "-0x1.0E-2000#1", Up, "-5.0e-324", Less);
    test_helper::<f64>("-6.0e-2409", "-0x1.0E-2000#1", Nearest, "-0.0", Greater);

    test_helper::<f32>("-too_big", "-0x4.0E+268435455#1", Floor, "-Infinity", Less);
    test_helper::<f32>(
        "-too_big",
        "-0x4.0E+268435455#1",
        Ceiling,
        "-3.4028235e38",
        Greater,
    );
    test_helper::<f32>(
        "-too_big",
        "-0x4.0E+268435455#1",
        Down,
        "-3.4028235e38",
        Greater,
    );
    test_helper::<f32>("-too_big", "-0x4.0E+268435455#1", Up, "-Infinity", Less);
    test_helper::<f32>(
        "-too_big",
        "-0x4.0E+268435455#1",
        Nearest,
        "-3.4028235e38",
        Greater,
    );

    test_helper::<f64>("-too_big", "-0x4.0E+268435455#1", Floor, "-Infinity", Less);
    test_helper::<f64>(
        "-too_big",
        "-0x4.0E+268435455#1",
        Ceiling,
        "-1.7976931348623157e308",
        Greater,
    );
    test_helper::<f64>(
        "-too_big",
        "-0x4.0E+268435455#1",
        Down,
        "-1.7976931348623157e308",
        Greater,
    );
    test_helper::<f64>("-too_big", "-0x4.0E+268435455#1", Up, "-Infinity", Less);
    test_helper::<f64>(
        "-too_big",
        "-0x4.0E+268435455#1",
        Nearest,
        "-1.7976931348623157e308",
        Greater,
    );

    test_helper::<f32>("-too_small", "-0x1.0E-268435456#1", Floor, "-1.0e-45", Less);
    test_helper::<f32>(
        "-too_small",
        "-0x1.0E-268435456#1",
        Ceiling,
        "-0.0",
        Greater,
    );
    test_helper::<f32>("-too_small", "-0x1.0E-268435456#1", Down, "-0.0", Greater);
    test_helper::<f32>("-too_small", "-0x1.0E-268435456#1", Up, "-1.0e-45", Less);
    test_helper::<f32>(
        "-too_small",
        "-0x1.0E-268435456#1",
        Nearest,
        "-0.0",
        Greater,
    );

    test_helper::<f64>(
        "-too_small",
        "-0x1.0E-268435456#1",
        Floor,
        "-5.0e-324",
        Less,
    );
    test_helper::<f64>(
        "-too_small",
        "-0x1.0E-268435456#1",
        Ceiling,
        "-0.0",
        Greater,
    );
    test_helper::<f64>("-too_small", "-0x1.0E-268435456#1", Down, "-0.0", Greater);
    test_helper::<f64>("-too_small", "-0x1.0E-268435456#1", Up, "-5.0e-324", Less);
    test_helper::<f64>(
        "-too_small",
        "-0x1.0E-268435456#1",
        Nearest,
        "-0.0",
        Greater,
    );
}

fn rounding_from_float_fail_helper<T: PrimitiveFloat + RoundingFrom<Float>>() {
    assert_panic!(T::rounding_from(
        Float::from_rational_prec(Rational::from_unsigneds(1u8, 3), 100).0,
        Exact
    ));
    assert_panic!(T::rounding_from(
        Float::exact_from(Rational::power_of_2(10000i64)),
        Exact
    ));
    assert_panic!(T::rounding_from(
        Float::exact_from(Rational::power_of_2(-10000i64)),
        Exact
    ));
    assert_panic!(T::rounding_from(
        Float::from_rational_prec(Rational::from_signeds(-1, 3), 100).0,
        Exact
    ));
    assert_panic!(T::rounding_from(
        Float::exact_from(-Rational::power_of_2(10000i64)),
        Exact
    ));
    assert_panic!(T::rounding_from(
        Float::exact_from(-Rational::power_of_2(-10000i64)),
        Exact
    ));
}

#[test]
fn rounding_from_float_fail() {
    apply_fn_to_primitive_floats!(rounding_from_float_fail_helper);
}

#[allow(
    clippy::type_repetition_in_bounds,
    clippy::op_ref,
    clippy::needless_pass_by_value
)]
fn try_from_float_properties_helper_helper<
    T: PrimitiveFloat + PartialEq<Float> + TryFrom<Float, Error = FloatFromFloatError>,
>(
    x: Float,
) where
    for<'a> T: ConvertibleFrom<&'a Float> + TryFrom<&'a Float, Error = FloatFromFloatError>,
    Float: From<T> + PartialEq<T>,
{
    let t_x = T::try_from(x.clone());

    let t_x_alt = T::try_from(&x);
    assert_eq!(t_x.map(NiceFloat), t_x_alt.map(NiceFloat));

    assert_eq!(t_x.is_ok(), T::convertible_from(&x));
    if let Ok(n) = t_x {
        assert_eq!(NiceFloat(T::exact_from(&x)), NiceFloat(n));
        assert!(n.is_nan() && x.is_nan() || n == x);
        let n_alt = Float::from(n);
        assert!(n_alt.is_nan() && x.is_nan() || &n_alt == &x);
    }
}

#[allow(clippy::type_repetition_in_bounds, clippy::op_ref)]
fn try_from_float_properties_helper<
    T: PrimitiveFloat + PartialEq<Float> + TryFrom<Float, Error = FloatFromFloatError>,
>()
where
    for<'a> T: ConvertibleFrom<&'a Float> + TryFrom<&'a Float, Error = FloatFromFloatError>,
    Float: From<T> + PartialEq<T>,
{
    float_gen().test_properties(|x| {
        try_from_float_properties_helper_helper(x);
    });

    float_gen_var_12().test_properties(|x| {
        try_from_float_properties_helper_helper(x);
    });
}

#[test]
fn try_from_float_properties() {
    apply_fn_to_primitive_floats!(try_from_float_properties_helper);
}

#[allow(clippy::needless_pass_by_value)]
fn convertible_from_float_properties_helper_helper<T>(x: Float)
where
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    T::convertible_from(&x);
}

fn convertible_from_float_properties_helper<T>()
where
    for<'a> T: ConvertibleFrom<&'a Float>,
{
    float_gen().test_properties(|x| {
        convertible_from_float_properties_helper_helper::<T>(x);
    });

    float_gen_var_12().test_properties(|x| {
        convertible_from_float_properties_helper_helper::<T>(x);
    });
}

#[test]
fn convertible_from_float_properties() {
    apply_fn_to_primitive_floats!(convertible_from_float_properties_helper);
}

const fn wrap_nice_float<T: PrimitiveFloat>(p: (T, Ordering)) -> (NiceFloat<T>, Ordering) {
    (NiceFloat(p.0), p.1)
}

#[allow(clippy::type_repetition_in_bounds, clippy::needless_pass_by_value)]
fn rounding_from_float_properties_helper_helper<
    T: PrimitiveFloat + RoundingFrom<Float> + PartialOrd<Integer>,
>(
    x: Float,
    rm: RoundingMode,
    extreme: bool,
) where
    for<'a> T: ConvertibleFrom<&'a Float> + PartialOrd<Float> + RoundingFrom<&'a Float>,
    Float: From<T> + PartialOrd<T>,
    Rational: TryFrom<T>,
{
    let no = T::rounding_from(&x, rm);
    let no_alt = T::rounding_from(x.clone(), rm);
    assert_eq!(NiceFloat(no_alt.0), NiceFloat(no.0));
    assert_eq!(no_alt.1, no.1);
    let (n, o) = no;
    if !extreme && n > -T::MAX_FINITE && n < T::MAX_FINITE && n != T::ZERO {
        let r_x: Rational = ExactFrom::<&Float>::exact_from(&x);
        assert!((Rational::exact_from(n) - r_x).lt_abs(&Float::from(n).ulp().unwrap()));
    }

    assert_eq!(n.partial_cmp(&x), if x.is_nan() { None } else { Some(o) });
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
}

#[allow(clippy::type_repetition_in_bounds)]
fn rounding_from_float_properties_helper<
    T: PrimitiveFloat + RoundingFrom<Float> + PartialOrd<Integer>,
>()
where
    for<'a> T: ConvertibleFrom<&'a Float> + PartialOrd<Float> + RoundingFrom<&'a Float>,
    Float: From<T> + PartialOrd<T>,
    Rational: TryFrom<T>,
{
    float_rounding_mode_pair_gen_var_6::<T>().test_properties(|(x, rm)| {
        rounding_from_float_properties_helper_helper(x, rm, false);
    });

    float_rounding_mode_pair_gen_var_20::<T>().test_properties(|(x, rm)| {
        rounding_from_float_properties_helper_helper(x, rm, true);
    });

    float_gen_var_4().test_properties(|x| {
        let floor = T::rounding_from(&x, Floor);
        assert!(floor.0 <= x);
        assert_eq!(
            T::rounding_from(&x, if x >= T::ZERO { Down } else { Up }),
            floor
        );
        let ceiling = T::rounding_from(&x, Ceiling);
        assert!(ceiling.0 >= x);
        assert_eq!(
            T::rounding_from(&x, if x >= T::ZERO { Up } else { Down }),
            ceiling
        );
        let nearest = T::rounding_from(&x, Nearest);
        assert!(nearest == floor || nearest == ceiling);
        if nearest.0 > -T::MAX_FINITE && nearest.0 < T::MAX_FINITE && nearest.0 != T::ZERO {
            let r_x: Rational = ExactFrom::<&Float>::exact_from(&x);
            let rulp: Rational = ExactFrom::exact_from(Float::from(nearest.0).ulp().unwrap());
            assert!((Rational::exact_from(nearest.0) - r_x).le_abs(&(rulp >> 1u32)));
        }
    });

    primitive_float_gen::<T>().test_properties(|n| {
        let x = Float::from(n);
        let no = (NiceFloat(n), Equal);
        assert_eq!(wrap_nice_float(T::rounding_from(&x, Floor)), no);
        assert_eq!(wrap_nice_float(T::rounding_from(&x, Down)), no);
        assert_eq!(wrap_nice_float(T::rounding_from(&x, Ceiling)), no);
        assert_eq!(wrap_nice_float(T::rounding_from(&x, Up)), no);
        assert_eq!(wrap_nice_float(T::rounding_from(&x, Nearest)), no);
        assert_eq!(wrap_nice_float(T::rounding_from(&x, Exact)), no);
    });
}

#[allow(clippy::manual_range_contains)]
#[test]
fn rounding_from_float_properties() {
    apply_fn_to_primitive_floats!(rounding_from_float_properties_helper);

    float_rounding_mode_pair_gen_var_6::<f32>().test_properties(|(x, rm)| {
        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            if x >= -f32::MAX_FINITE && x <= f32::MAX_FINITE {
                assert_eq!(
                    NiceFloat(f32::rounding_from(&x, rm).0),
                    NiceFloat(rug::Float::exact_from(&x).to_f32_round(rug_rm)),
                );
            }
        }
    });

    float_rounding_mode_pair_gen_var_6::<f64>().test_properties(|(x, rm)| {
        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            if x >= -f64::MAX_FINITE && x <= f64::MAX_FINITE {
                assert_eq!(
                    NiceFloat(f64::rounding_from(&x, rm).0),
                    NiceFloat(rug::Float::exact_from(&x).to_f64_round(rug_rm)),
                );
            }
        }
    });
}
