use malachite_base::num::arithmetic::traits::{Ceiling, Floor};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeOne, OneHalf};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::from::{SignedFromFloatError, UnsignedFromFloatError};
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, RoundingFrom};
use malachite_base::rounding_modes::RoundingMode;
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
use std::cmp::Ordering;
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
        test_helper::<T>(
            "-Infinity",
            "-Infinity",
            RoundingMode::Ceiling,
            "0",
            Ordering::Greater,
        );
        test_helper::<T>(
            "-Infinity",
            "-Infinity",
            RoundingMode::Down,
            "0",
            Ordering::Greater,
        );
        test_helper::<T>(
            "-Infinity",
            "-Infinity",
            RoundingMode::Nearest,
            "0",
            Ordering::Greater,
        );

        test_helper::<T>("0.0", "0x0.0", RoundingMode::Floor, "0", Ordering::Equal);
        test_helper::<T>("0.0", "0x0.0", RoundingMode::Ceiling, "0", Ordering::Equal);
        test_helper::<T>("0.0", "0x0.0", RoundingMode::Down, "0", Ordering::Equal);
        test_helper::<T>("0.0", "0x0.0", RoundingMode::Up, "0", Ordering::Equal);
        test_helper::<T>("0.0", "0x0.0", RoundingMode::Nearest, "0", Ordering::Equal);
        test_helper::<T>("0.0", "0x0.0", RoundingMode::Exact, "0", Ordering::Equal);

        test_helper::<T>("-0.0", "-0x0.0", RoundingMode::Floor, "0", Ordering::Equal);
        test_helper::<T>(
            "-0.0",
            "-0x0.0",
            RoundingMode::Ceiling,
            "0",
            Ordering::Equal,
        );
        test_helper::<T>("-0.0", "-0x0.0", RoundingMode::Down, "0", Ordering::Equal);
        test_helper::<T>("-0.0", "-0x0.0", RoundingMode::Up, "0", Ordering::Equal);
        test_helper::<T>(
            "-0.0",
            "-0x0.0",
            RoundingMode::Nearest,
            "0",
            Ordering::Equal,
        );
        test_helper::<T>("-0.0", "-0x0.0", RoundingMode::Exact, "0", Ordering::Equal);

        test_helper::<T>("1.0", "0x1.0#1", RoundingMode::Floor, "1", Ordering::Equal);
        test_helper::<T>(
            "1.0",
            "0x1.0#1",
            RoundingMode::Ceiling,
            "1",
            Ordering::Equal,
        );
        test_helper::<T>("1.0", "0x1.0#1", RoundingMode::Down, "1", Ordering::Equal);
        test_helper::<T>("1.0", "0x1.0#1", RoundingMode::Up, "1", Ordering::Equal);
        test_helper::<T>(
            "1.0",
            "0x1.0#1",
            RoundingMode::Nearest,
            "1",
            Ordering::Equal,
        );
        test_helper::<T>("1.0", "0x1.0#1", RoundingMode::Exact, "1", Ordering::Equal);

        test_helper::<T>("2.0", "0x2.0#1", RoundingMode::Floor, "2", Ordering::Equal);
        test_helper::<T>(
            "2.0",
            "0x2.0#1",
            RoundingMode::Ceiling,
            "2",
            Ordering::Equal,
        );
        test_helper::<T>("2.0", "0x2.0#1", RoundingMode::Down, "2", Ordering::Equal);
        test_helper::<T>("2.0", "0x2.0#1", RoundingMode::Up, "2", Ordering::Equal);
        test_helper::<T>(
            "2.0",
            "0x2.0#1",
            RoundingMode::Nearest,
            "2",
            Ordering::Equal,
        );
        test_helper::<T>("2.0", "0x2.0#1", RoundingMode::Exact, "2", Ordering::Equal);

        test_helper::<T>("0.5", "0x0.8#1", RoundingMode::Floor, "0", Ordering::Less);
        test_helper::<T>(
            "0.5",
            "0x0.8#1",
            RoundingMode::Ceiling,
            "1",
            Ordering::Greater,
        );
        test_helper::<T>("0.5", "0x0.8#1", RoundingMode::Down, "0", Ordering::Less);
        test_helper::<T>("0.5", "0x0.8#1", RoundingMode::Up, "1", Ordering::Greater);
        test_helper::<T>("0.5", "0x0.8#1", RoundingMode::Nearest, "0", Ordering::Less);

        test_helper::<T>(
            "0.33333333333333331",
            "0x0.55555555555554#53",
            RoundingMode::Floor,
            "0",
            Ordering::Less,
        );
        test_helper::<T>(
            "0.33333333333333331",
            "0x0.55555555555554#53",
            RoundingMode::Ceiling,
            "1",
            Ordering::Greater,
        );
        test_helper::<T>(
            "0.33333333333333331",
            "0x0.55555555555554#53",
            RoundingMode::Down,
            "0",
            Ordering::Less,
        );
        test_helper::<T>(
            "0.33333333333333331",
            "0x0.55555555555554#53",
            RoundingMode::Up,
            "1",
            Ordering::Greater,
        );
        test_helper::<T>(
            "0.33333333333333331",
            "0x0.55555555555554#53",
            RoundingMode::Nearest,
            "0",
            Ordering::Less,
        );

        test_helper::<T>(
            "0.6666666666666666",
            "0x0.aaaaaaaaaaaaa8#53",
            RoundingMode::Floor,
            "0",
            Ordering::Less,
        );
        test_helper::<T>(
            "0.6666666666666666",
            "0x0.aaaaaaaaaaaaa8#53",
            RoundingMode::Ceiling,
            "1",
            Ordering::Greater,
        );
        test_helper::<T>(
            "0.6666666666666666",
            "0x0.aaaaaaaaaaaaa8#53",
            RoundingMode::Down,
            "0",
            Ordering::Less,
        );
        test_helper::<T>(
            "0.6666666666666666",
            "0x0.aaaaaaaaaaaaa8#53",
            RoundingMode::Up,
            "1",
            Ordering::Greater,
        );
        test_helper::<T>(
            "0.6666666666666666",
            "0x0.aaaaaaaaaaaaa8#53",
            RoundingMode::Nearest,
            "1",
            Ordering::Greater,
        );

        test_helper::<T>("1.5", "0x1.8#2", RoundingMode::Floor, "1", Ordering::Less);
        test_helper::<T>(
            "1.5",
            "0x1.8#2",
            RoundingMode::Ceiling,
            "2",
            Ordering::Greater,
        );
        test_helper::<T>("1.5", "0x1.8#2", RoundingMode::Down, "1", Ordering::Less);
        test_helper::<T>("1.5", "0x1.8#2", RoundingMode::Up, "2", Ordering::Greater);
        test_helper::<T>(
            "1.5",
            "0x1.8#2",
            RoundingMode::Nearest,
            "2",
            Ordering::Greater,
        );

        test_helper::<T>("2.5", "0x2.8#3", RoundingMode::Floor, "2", Ordering::Less);
        test_helper::<T>(
            "2.5",
            "0x2.8#3",
            RoundingMode::Ceiling,
            "3",
            Ordering::Greater,
        );
        test_helper::<T>("2.5", "0x2.8#3", RoundingMode::Down, "2", Ordering::Less);
        test_helper::<T>("2.5", "0x2.8#3", RoundingMode::Up, "3", Ordering::Greater);
        test_helper::<T>("2.5", "0x2.8#3", RoundingMode::Nearest, "2", Ordering::Less);

        test_helper::<T>(
            "123.0",
            "0x7b.0#7",
            RoundingMode::Floor,
            "123",
            Ordering::Equal,
        );
        test_helper::<T>(
            "123.0",
            "0x7b.0#7",
            RoundingMode::Ceiling,
            "123",
            Ordering::Equal,
        );
        test_helper::<T>(
            "123.0",
            "0x7b.0#7",
            RoundingMode::Down,
            "123",
            Ordering::Equal,
        );
        test_helper::<T>(
            "123.0",
            "0x7b.0#7",
            RoundingMode::Up,
            "123",
            Ordering::Equal,
        );
        test_helper::<T>(
            "123.0",
            "0x7b.0#7",
            RoundingMode::Nearest,
            "123",
            Ordering::Equal,
        );
        test_helper::<T>(
            "123.0",
            "0x7b.0#7",
            RoundingMode::Exact,
            "123",
            Ordering::Equal,
        );

        test_helper::<T>(
            "-1.0",
            "-0x1.0#1",
            RoundingMode::Ceiling,
            "0",
            Ordering::Greater,
        );
        test_helper::<T>(
            "-1.0",
            "-0x1.0#1",
            RoundingMode::Down,
            "0",
            Ordering::Greater,
        );
        test_helper::<T>(
            "-1.0",
            "-0x1.0#1",
            RoundingMode::Nearest,
            "0",
            Ordering::Greater,
        );

        test_helper::<T>(
            "-2.0",
            "-0x2.0#1",
            RoundingMode::Ceiling,
            "0",
            Ordering::Greater,
        );
        test_helper::<T>(
            "-2.0",
            "-0x2.0#1",
            RoundingMode::Down,
            "0",
            Ordering::Greater,
        );
        test_helper::<T>(
            "-2.0",
            "-0x2.0#1",
            RoundingMode::Nearest,
            "0",
            Ordering::Greater,
        );

        test_helper::<T>(
            "-0.5",
            "-0x0.8#1",
            RoundingMode::Ceiling,
            "0",
            Ordering::Greater,
        );
        test_helper::<T>(
            "-0.5",
            "-0x0.8#1",
            RoundingMode::Down,
            "0",
            Ordering::Greater,
        );
        test_helper::<T>(
            "-0.5",
            "-0x0.8#1",
            RoundingMode::Nearest,
            "0",
            Ordering::Greater,
        );

        test_helper::<T>(
            "-0.33333333333333331",
            "-0x0.55555555555554#53",
            RoundingMode::Ceiling,
            "0",
            Ordering::Greater,
        );
        test_helper::<T>(
            "-0.33333333333333331",
            "-0x0.55555555555554#53",
            RoundingMode::Down,
            "0",
            Ordering::Greater,
        );
        test_helper::<T>(
            "-0.33333333333333331",
            "-0x0.55555555555554#53",
            RoundingMode::Nearest,
            "0",
            Ordering::Greater,
        );

        test_helper::<T>(
            "-0.6666666666666666",
            "-0x0.aaaaaaaaaaaaa8#53",
            RoundingMode::Ceiling,
            "0",
            Ordering::Greater,
        );
        test_helper::<T>(
            "-0.6666666666666666",
            "-0x0.aaaaaaaaaaaaa8#53",
            RoundingMode::Down,
            "0",
            Ordering::Greater,
        );
        test_helper::<T>(
            "-0.6666666666666666",
            "-0x0.aaaaaaaaaaaaa8#53",
            RoundingMode::Nearest,
            "0",
            Ordering::Greater,
        );

        test_helper::<T>(
            "-1.5",
            "-0x1.8#2",
            RoundingMode::Ceiling,
            "0",
            Ordering::Greater,
        );
        test_helper::<T>(
            "-1.5",
            "-0x1.8#2",
            RoundingMode::Down,
            "0",
            Ordering::Greater,
        );
        test_helper::<T>(
            "-1.5",
            "-0x1.8#2",
            RoundingMode::Nearest,
            "0",
            Ordering::Greater,
        );

        test_helper::<T>(
            "-2.5",
            "-0x2.8#3",
            RoundingMode::Ceiling,
            "0",
            Ordering::Greater,
        );
        test_helper::<T>(
            "-2.5",
            "-0x2.8#3",
            RoundingMode::Down,
            "0",
            Ordering::Greater,
        );
        test_helper::<T>(
            "-2.5",
            "-0x2.8#3",
            RoundingMode::Nearest,
            "0",
            Ordering::Greater,
        );

        test_helper::<T>(
            "-123.0",
            "-0x7b.0#7",
            RoundingMode::Ceiling,
            "0",
            Ordering::Greater,
        );
        test_helper::<T>(
            "-123.0",
            "-0x7b.0#7",
            RoundingMode::Down,
            "0",
            Ordering::Greater,
        );
        test_helper::<T>(
            "-123.0",
            "-0x7b.0#7",
            RoundingMode::Nearest,
            "0",
            Ordering::Greater,
        );
    }
    apply_fn_to_unsigneds!(test_helper_u);

    #[allow(clippy::type_repetition_in_bounds)]
    fn test_helper_i<T: PrimitiveSigned + RoundingFrom<Float>>()
    where
        for<'a> T: RoundingFrom<&'a Float>,
    {
        test_helper::<T>("0.0", "0x0.0", RoundingMode::Floor, "0", Ordering::Equal);
        test_helper::<T>("0.0", "0x0.0", RoundingMode::Ceiling, "0", Ordering::Equal);
        test_helper::<T>("0.0", "0x0.0", RoundingMode::Down, "0", Ordering::Equal);
        test_helper::<T>("0.0", "0x0.0", RoundingMode::Up, "0", Ordering::Equal);
        test_helper::<T>("0.0", "0x0.0", RoundingMode::Nearest, "0", Ordering::Equal);
        test_helper::<T>("0.0", "0x0.0", RoundingMode::Exact, "0", Ordering::Equal);

        test_helper::<T>("-0.0", "-0x0.0", RoundingMode::Floor, "0", Ordering::Equal);
        test_helper::<T>(
            "-0.0",
            "-0x0.0",
            RoundingMode::Ceiling,
            "0",
            Ordering::Equal,
        );
        test_helper::<T>("-0.0", "-0x0.0", RoundingMode::Down, "0", Ordering::Equal);
        test_helper::<T>("-0.0", "-0x0.0", RoundingMode::Up, "0", Ordering::Equal);
        test_helper::<T>(
            "-0.0",
            "-0x0.0",
            RoundingMode::Nearest,
            "0",
            Ordering::Equal,
        );
        test_helper::<T>("-0.0", "-0x0.0", RoundingMode::Exact, "0", Ordering::Equal);

        test_helper::<T>("1.0", "0x1.0#1", RoundingMode::Floor, "1", Ordering::Equal);
        test_helper::<T>(
            "1.0",
            "0x1.0#1",
            RoundingMode::Ceiling,
            "1",
            Ordering::Equal,
        );
        test_helper::<T>("1.0", "0x1.0#1", RoundingMode::Down, "1", Ordering::Equal);
        test_helper::<T>("1.0", "0x1.0#1", RoundingMode::Up, "1", Ordering::Equal);
        test_helper::<T>(
            "1.0",
            "0x1.0#1",
            RoundingMode::Nearest,
            "1",
            Ordering::Equal,
        );
        test_helper::<T>("1.0", "0x1.0#1", RoundingMode::Exact, "1", Ordering::Equal);

        test_helper::<T>("2.0", "0x2.0#1", RoundingMode::Floor, "2", Ordering::Equal);
        test_helper::<T>(
            "2.0",
            "0x2.0#1",
            RoundingMode::Ceiling,
            "2",
            Ordering::Equal,
        );
        test_helper::<T>("2.0", "0x2.0#1", RoundingMode::Down, "2", Ordering::Equal);
        test_helper::<T>("2.0", "0x2.0#1", RoundingMode::Up, "2", Ordering::Equal);
        test_helper::<T>(
            "2.0",
            "0x2.0#1",
            RoundingMode::Nearest,
            "2",
            Ordering::Equal,
        );
        test_helper::<T>("2.0", "0x2.0#1", RoundingMode::Exact, "2", Ordering::Equal);

        test_helper::<T>("0.5", "0x0.8#1", RoundingMode::Floor, "0", Ordering::Less);
        test_helper::<T>(
            "0.5",
            "0x0.8#1",
            RoundingMode::Ceiling,
            "1",
            Ordering::Greater,
        );
        test_helper::<T>("0.5", "0x0.8#1", RoundingMode::Down, "0", Ordering::Less);
        test_helper::<T>("0.5", "0x0.8#1", RoundingMode::Up, "1", Ordering::Greater);
        test_helper::<T>("0.5", "0x0.8#1", RoundingMode::Nearest, "0", Ordering::Less);

        test_helper::<T>(
            "0.33333333333333331",
            "0x0.55555555555554#53",
            RoundingMode::Floor,
            "0",
            Ordering::Less,
        );
        test_helper::<T>(
            "0.33333333333333331",
            "0x0.55555555555554#53",
            RoundingMode::Ceiling,
            "1",
            Ordering::Greater,
        );
        test_helper::<T>(
            "0.33333333333333331",
            "0x0.55555555555554#53",
            RoundingMode::Down,
            "0",
            Ordering::Less,
        );
        test_helper::<T>(
            "0.33333333333333331",
            "0x0.55555555555554#53",
            RoundingMode::Up,
            "1",
            Ordering::Greater,
        );
        test_helper::<T>(
            "0.33333333333333331",
            "0x0.55555555555554#53",
            RoundingMode::Nearest,
            "0",
            Ordering::Less,
        );

        test_helper::<T>(
            "0.6666666666666666",
            "0x0.aaaaaaaaaaaaa8#53",
            RoundingMode::Floor,
            "0",
            Ordering::Less,
        );
        test_helper::<T>(
            "0.6666666666666666",
            "0x0.aaaaaaaaaaaaa8#53",
            RoundingMode::Ceiling,
            "1",
            Ordering::Greater,
        );
        test_helper::<T>(
            "0.6666666666666666",
            "0x0.aaaaaaaaaaaaa8#53",
            RoundingMode::Down,
            "0",
            Ordering::Less,
        );
        test_helper::<T>(
            "0.6666666666666666",
            "0x0.aaaaaaaaaaaaa8#53",
            RoundingMode::Up,
            "1",
            Ordering::Greater,
        );
        test_helper::<T>(
            "0.6666666666666666",
            "0x0.aaaaaaaaaaaaa8#53",
            RoundingMode::Nearest,
            "1",
            Ordering::Greater,
        );

        test_helper::<T>("1.5", "0x1.8#2", RoundingMode::Floor, "1", Ordering::Less);
        test_helper::<T>(
            "1.5",
            "0x1.8#2",
            RoundingMode::Ceiling,
            "2",
            Ordering::Greater,
        );
        test_helper::<T>("1.5", "0x1.8#2", RoundingMode::Down, "1", Ordering::Less);
        test_helper::<T>("1.5", "0x1.8#2", RoundingMode::Up, "2", Ordering::Greater);
        test_helper::<T>(
            "1.5",
            "0x1.8#2",
            RoundingMode::Nearest,
            "2",
            Ordering::Greater,
        );

        test_helper::<T>("2.5", "0x2.8#3", RoundingMode::Floor, "2", Ordering::Less);
        test_helper::<T>(
            "2.5",
            "0x2.8#3",
            RoundingMode::Ceiling,
            "3",
            Ordering::Greater,
        );
        test_helper::<T>("2.5", "0x2.8#3", RoundingMode::Down, "2", Ordering::Less);
        test_helper::<T>("2.5", "0x2.8#3", RoundingMode::Up, "3", Ordering::Greater);
        test_helper::<T>("2.5", "0x2.8#3", RoundingMode::Nearest, "2", Ordering::Less);

        test_helper::<T>(
            "123.0",
            "0x7b.0#7",
            RoundingMode::Floor,
            "123",
            Ordering::Equal,
        );
        test_helper::<T>(
            "123.0",
            "0x7b.0#7",
            RoundingMode::Ceiling,
            "123",
            Ordering::Equal,
        );
        test_helper::<T>(
            "123.0",
            "0x7b.0#7",
            RoundingMode::Down,
            "123",
            Ordering::Equal,
        );
        test_helper::<T>(
            "123.0",
            "0x7b.0#7",
            RoundingMode::Up,
            "123",
            Ordering::Equal,
        );
        test_helper::<T>(
            "123.0",
            "0x7b.0#7",
            RoundingMode::Nearest,
            "123",
            Ordering::Equal,
        );
        test_helper::<T>(
            "123.0",
            "0x7b.0#7",
            RoundingMode::Exact,
            "123",
            Ordering::Equal,
        );

        test_helper::<T>(
            "-1.0",
            "-0x1.0#1",
            RoundingMode::Floor,
            "-1",
            Ordering::Equal,
        );
        test_helper::<T>(
            "-1.0",
            "-0x1.0#1",
            RoundingMode::Ceiling,
            "-1",
            Ordering::Equal,
        );
        test_helper::<T>(
            "-1.0",
            "-0x1.0#1",
            RoundingMode::Down,
            "-1",
            Ordering::Equal,
        );
        test_helper::<T>("-1.0", "-0x1.0#1", RoundingMode::Up, "-1", Ordering::Equal);
        test_helper::<T>(
            "-1.0",
            "-0x1.0#1",
            RoundingMode::Nearest,
            "-1",
            Ordering::Equal,
        );
        test_helper::<T>(
            "-1.0",
            "-0x1.0#1",
            RoundingMode::Exact,
            "-1",
            Ordering::Equal,
        );

        test_helper::<T>(
            "-2.0",
            "-0x2.0#1",
            RoundingMode::Floor,
            "-2",
            Ordering::Equal,
        );
        test_helper::<T>(
            "-2.0",
            "-0x2.0#1",
            RoundingMode::Ceiling,
            "-2",
            Ordering::Equal,
        );
        test_helper::<T>(
            "-2.0",
            "-0x2.0#1",
            RoundingMode::Down,
            "-2",
            Ordering::Equal,
        );
        test_helper::<T>("-2.0", "-0x2.0#1", RoundingMode::Up, "-2", Ordering::Equal);
        test_helper::<T>(
            "-2.0",
            "-0x2.0#1",
            RoundingMode::Nearest,
            "-2",
            Ordering::Equal,
        );
        test_helper::<T>(
            "-2.0",
            "-0x2.0#1",
            RoundingMode::Exact,
            "-2",
            Ordering::Equal,
        );

        test_helper::<T>(
            "-0.5",
            "-0x0.8#1",
            RoundingMode::Floor,
            "-1",
            Ordering::Less,
        );
        test_helper::<T>(
            "-0.5",
            "-0x0.8#1",
            RoundingMode::Ceiling,
            "0",
            Ordering::Greater,
        );
        test_helper::<T>(
            "-0.5",
            "-0x0.8#1",
            RoundingMode::Down,
            "0",
            Ordering::Greater,
        );
        test_helper::<T>("-0.5", "-0x0.8#1", RoundingMode::Up, "-1", Ordering::Less);
        test_helper::<T>(
            "-0.5",
            "-0x0.8#1",
            RoundingMode::Nearest,
            "0",
            Ordering::Greater,
        );

        test_helper::<T>(
            "-0.33333333333333331",
            "-0x0.55555555555554#53",
            RoundingMode::Floor,
            "-1",
            Ordering::Less,
        );
        test_helper::<T>(
            "-0.33333333333333331",
            "-0x0.55555555555554#53",
            RoundingMode::Ceiling,
            "0",
            Ordering::Greater,
        );
        test_helper::<T>(
            "-0.33333333333333331",
            "-0x0.55555555555554#53",
            RoundingMode::Down,
            "0",
            Ordering::Greater,
        );
        test_helper::<T>(
            "-0.33333333333333331",
            "-0x0.55555555555554#53",
            RoundingMode::Up,
            "-1",
            Ordering::Less,
        );
        test_helper::<T>(
            "-0.33333333333333331",
            "-0x0.55555555555554#53",
            RoundingMode::Nearest,
            "0",
            Ordering::Greater,
        );

        test_helper::<T>(
            "-0.6666666666666666",
            "-0x0.aaaaaaaaaaaaa8#53",
            RoundingMode::Floor,
            "-1",
            Ordering::Less,
        );
        test_helper::<T>(
            "-0.6666666666666666",
            "-0x0.aaaaaaaaaaaaa8#53",
            RoundingMode::Ceiling,
            "0",
            Ordering::Greater,
        );
        test_helper::<T>(
            "-0.6666666666666666",
            "-0x0.aaaaaaaaaaaaa8#53",
            RoundingMode::Down,
            "0",
            Ordering::Greater,
        );
        test_helper::<T>(
            "-0.6666666666666666",
            "-0x0.aaaaaaaaaaaaa8#53",
            RoundingMode::Up,
            "-1",
            Ordering::Less,
        );
        test_helper::<T>(
            "-0.6666666666666666",
            "-0x0.aaaaaaaaaaaaa8#53",
            RoundingMode::Nearest,
            "-1",
            Ordering::Less,
        );

        test_helper::<T>(
            "-1.5",
            "-0x1.8#2",
            RoundingMode::Floor,
            "-2",
            Ordering::Less,
        );
        test_helper::<T>(
            "-1.5",
            "-0x1.8#2",
            RoundingMode::Ceiling,
            "-1",
            Ordering::Greater,
        );
        test_helper::<T>(
            "-1.5",
            "-0x1.8#2",
            RoundingMode::Down,
            "-1",
            Ordering::Greater,
        );
        test_helper::<T>("-1.5", "-0x1.8#2", RoundingMode::Up, "-2", Ordering::Less);
        test_helper::<T>(
            "-1.5",
            "-0x1.8#2",
            RoundingMode::Nearest,
            "-2",
            Ordering::Less,
        );

        test_helper::<T>(
            "-2.5",
            "-0x2.8#3",
            RoundingMode::Floor,
            "-3",
            Ordering::Less,
        );
        test_helper::<T>(
            "-2.5",
            "-0x2.8#3",
            RoundingMode::Ceiling,
            "-2",
            Ordering::Greater,
        );
        test_helper::<T>(
            "-2.5",
            "-0x2.8#3",
            RoundingMode::Down,
            "-2",
            Ordering::Greater,
        );
        test_helper::<T>("-2.5", "-0x2.8#3", RoundingMode::Up, "-3", Ordering::Less);
        test_helper::<T>(
            "-2.5",
            "-0x2.8#3",
            RoundingMode::Nearest,
            "-2",
            Ordering::Greater,
        );

        test_helper::<T>(
            "-123.0",
            "-0x7b.0#7",
            RoundingMode::Floor,
            "-123",
            Ordering::Equal,
        );
        test_helper::<T>(
            "-123.0",
            "-0x7b.0#7",
            RoundingMode::Ceiling,
            "-123",
            Ordering::Equal,
        );
        test_helper::<T>(
            "-123.0",
            "-0x7b.0#7",
            RoundingMode::Down,
            "-123",
            Ordering::Equal,
        );
        test_helper::<T>(
            "-123.0",
            "-0x7b.0#7",
            RoundingMode::Up,
            "-123",
            Ordering::Equal,
        );
        test_helper::<T>(
            "-123.0",
            "-0x7b.0#7",
            RoundingMode::Nearest,
            "-123",
            Ordering::Equal,
        );
        test_helper::<T>(
            "-123.0",
            "-0x7b.0#7",
            RoundingMode::Exact,
            "-123",
            Ordering::Equal,
        );
    }
    apply_fn_to_signeds!(test_helper_i);
}

fn unsigned_rounding_from_float_fail_helper<T: PrimitiveUnsigned + RoundingFrom<Float>>() {
    assert_panic!(T::rounding_from(Float::NAN, RoundingMode::Floor));
    assert_panic!(T::rounding_from(Float::NAN, RoundingMode::Ceiling));
    assert_panic!(T::rounding_from(Float::NAN, RoundingMode::Down));
    assert_panic!(T::rounding_from(Float::NAN, RoundingMode::Up));
    assert_panic!(T::rounding_from(Float::NAN, RoundingMode::Nearest));
    assert_panic!(T::rounding_from(Float::NAN, RoundingMode::Exact));

    assert_panic!(T::rounding_from(Float::INFINITY, RoundingMode::Ceiling));
    assert_panic!(T::rounding_from(Float::INFINITY, RoundingMode::Up));
    assert_panic!(T::rounding_from(Float::INFINITY, RoundingMode::Exact));

    assert_panic!(T::rounding_from(
        Float::NEGATIVE_INFINITY,
        RoundingMode::Floor
    ));
    assert_panic!(T::rounding_from(Float::NEGATIVE_INFINITY, RoundingMode::Up));
    assert_panic!(T::rounding_from(
        Float::NEGATIVE_INFINITY,
        RoundingMode::Exact
    ));

    assert_panic!(T::rounding_from(Float::NEGATIVE_ONE, RoundingMode::Floor));
    assert_panic!(T::rounding_from(Float::NEGATIVE_ONE, RoundingMode::Up));
    assert_panic!(T::rounding_from(Float::NEGATIVE_ONE, RoundingMode::Exact));

    assert_panic!(T::rounding_from(
        Float::from_unsigned_times_power_of_2(3u8, -1),
        RoundingMode::Exact
    ));
}

fn signed_rounding_from_float_fail_helper<T: PrimitiveSigned + RoundingFrom<Float>>() {
    assert_panic!(T::rounding_from(Float::NAN, RoundingMode::Floor));
    assert_panic!(T::rounding_from(Float::NAN, RoundingMode::Ceiling));
    assert_panic!(T::rounding_from(Float::NAN, RoundingMode::Down));
    assert_panic!(T::rounding_from(Float::NAN, RoundingMode::Up));
    assert_panic!(T::rounding_from(Float::NAN, RoundingMode::Nearest));
    assert_panic!(T::rounding_from(Float::NAN, RoundingMode::Exact));

    assert_panic!(T::rounding_from(Float::INFINITY, RoundingMode::Ceiling));
    assert_panic!(T::rounding_from(Float::INFINITY, RoundingMode::Up));
    assert_panic!(T::rounding_from(Float::INFINITY, RoundingMode::Exact));

    assert_panic!(T::rounding_from(
        Float::NEGATIVE_INFINITY,
        RoundingMode::Floor
    ));
    assert_panic!(T::rounding_from(Float::NEGATIVE_INFINITY, RoundingMode::Up));
    assert_panic!(T::rounding_from(
        Float::NEGATIVE_INFINITY,
        RoundingMode::Exact
    ));

    assert_panic!(T::rounding_from(
        Float::from_unsigned_times_power_of_2(3u8, -1),
        RoundingMode::Exact
    ));
    assert_panic!(T::rounding_from(
        Float::from_signed_times_power_of_2(-3i8, -1),
        RoundingMode::Exact
    ));
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
    assert_panic!(T::rounding_from(&Float::NAN, RoundingMode::Floor));
    assert_panic!(T::rounding_from(&Float::NAN, RoundingMode::Ceiling));
    assert_panic!(T::rounding_from(&Float::NAN, RoundingMode::Down));
    assert_panic!(T::rounding_from(&Float::NAN, RoundingMode::Up));
    assert_panic!(T::rounding_from(&Float::NAN, RoundingMode::Nearest));
    assert_panic!(T::rounding_from(&Float::NAN, RoundingMode::Exact));

    assert_panic!(T::rounding_from(&Float::INFINITY, RoundingMode::Ceiling));
    assert_panic!(T::rounding_from(&Float::INFINITY, RoundingMode::Up));
    assert_panic!(T::rounding_from(&Float::INFINITY, RoundingMode::Exact));

    assert_panic!(T::rounding_from(
        &Float::NEGATIVE_INFINITY,
        RoundingMode::Floor
    ));
    assert_panic!(T::rounding_from(
        &Float::NEGATIVE_INFINITY,
        RoundingMode::Up
    ));
    assert_panic!(T::rounding_from(
        &Float::NEGATIVE_INFINITY,
        RoundingMode::Exact
    ));

    assert_panic!(T::rounding_from(&Float::NEGATIVE_ONE, RoundingMode::Floor));
    assert_panic!(T::rounding_from(&Float::NEGATIVE_ONE, RoundingMode::Up));
    assert_panic!(T::rounding_from(&Float::NEGATIVE_ONE, RoundingMode::Exact));

    assert_panic!(T::rounding_from(
        &Float::from_unsigned_times_power_of_2(3u8, -1),
        RoundingMode::Exact
    ));
}

#[allow(clippy::type_repetition_in_bounds)]
fn signed_rounding_from_float_ref_fail_helper<T: PrimitiveSigned>()
where
    for<'a> T: RoundingFrom<&'a Float>,
{
    assert_panic!(T::rounding_from(&Float::NAN, RoundingMode::Floor));
    assert_panic!(T::rounding_from(&Float::NAN, RoundingMode::Ceiling));
    assert_panic!(T::rounding_from(&Float::NAN, RoundingMode::Down));
    assert_panic!(T::rounding_from(&Float::NAN, RoundingMode::Up));
    assert_panic!(T::rounding_from(&Float::NAN, RoundingMode::Nearest));
    assert_panic!(T::rounding_from(&Float::NAN, RoundingMode::Exact));

    assert_panic!(T::rounding_from(&Float::INFINITY, RoundingMode::Ceiling));
    assert_panic!(T::rounding_from(&Float::INFINITY, RoundingMode::Up));
    assert_panic!(T::rounding_from(&Float::INFINITY, RoundingMode::Exact));

    assert_panic!(T::rounding_from(
        &Float::NEGATIVE_INFINITY,
        RoundingMode::Floor
    ));
    assert_panic!(T::rounding_from(
        &Float::NEGATIVE_INFINITY,
        RoundingMode::Up
    ));
    assert_panic!(T::rounding_from(
        &Float::NEGATIVE_INFINITY,
        RoundingMode::Exact
    ));

    assert_panic!(T::rounding_from(
        &Float::from_unsigned_times_power_of_2(3u8, -1),
        RoundingMode::Exact
    ));
    assert_panic!(T::rounding_from(
        &Float::from_signed_times_power_of_2(-3i8, -1),
        RoundingMode::Exact
    ));
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
            (_, RoundingMode::Floor) | (true, RoundingMode::Down) | (false, RoundingMode::Up) => {
                assert_ne!(o, Ordering::Greater)
            }
            (_, RoundingMode::Ceiling) | (true, RoundingMode::Up) | (false, RoundingMode::Down) => {
                assert_ne!(o, Ordering::Less)
            }
            (_, RoundingMode::Exact) => assert_eq!(o, Ordering::Equal),
            _ => {}
        }
    });

    float_gen_var_5().test_properties(|x| {
        let floor = T::rounding_from(&x, RoundingMode::Floor);
        if x <= T::MAX {
            assert_eq!(floor.0, Rational::exact_from(&x).floor());
        }
        assert!(floor.0 <= x);
        if floor.0 != T::MAX {
            assert!(floor.0 + T::ONE > x);
        }
        assert_eq!(T::rounding_from(&x, RoundingMode::Down), floor);

        let nearest = T::rounding_from(&x, RoundingMode::Nearest);
        if x <= T::MAX {
            let ceiling = T::rounding_from(&x, RoundingMode::Ceiling);
            assert_eq!(ceiling.0, Rational::exact_from(&x).ceiling());
            assert!(ceiling.0 >= x);
            if x > T::ZERO {
                assert!(ceiling.0 - T::ONE < x);
            }
            assert_eq!(T::rounding_from(&x, RoundingMode::Up), ceiling);
            assert!(nearest == floor || nearest == ceiling);
            assert!(
                (Rational::from(nearest.0) - Rational::exact_from(x)).le_abs(&Rational::ONE_HALF)
            );
        } else {
            assert!(nearest == (T::MAX, Ordering::Less));
        }
    });

    unsigned_gen::<T>().test_properties(|n| {
        let x = Float::from(n);
        let no = (n, Ordering::Equal);
        assert_eq!(T::rounding_from(&x, RoundingMode::Floor), no);
        assert_eq!(T::rounding_from(&x, RoundingMode::Down), no);
        assert_eq!(T::rounding_from(&x, RoundingMode::Ceiling), no);
        assert_eq!(T::rounding_from(&x, RoundingMode::Up), no);
        assert_eq!(T::rounding_from(&x, RoundingMode::Nearest), no);
        assert_eq!(T::rounding_from(&x, RoundingMode::Exact), no);

        let x = Float::from_unsigned_times_power_of_2((no.0 << 1) | T::ONE, -1);
        assert!(T::rounding_from(x, RoundingMode::Nearest).0.even());
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
            (_, RoundingMode::Floor) | (true, RoundingMode::Down) | (false, RoundingMode::Up) => {
                assert_ne!(o, Ordering::Greater)
            }
            (_, RoundingMode::Ceiling) | (true, RoundingMode::Up) | (false, RoundingMode::Down) => {
                assert_ne!(o, Ordering::Less)
            }
            (_, RoundingMode::Exact) => assert_eq!(o, Ordering::Equal),
            _ => {}
        }
    });

    float_gen_var_4().test_properties(|x| {
        let mut o_floor = None;
        if x >= T::MIN {
            let floor = T::rounding_from(&x, RoundingMode::Floor);
            assert!(floor.0 <= x);
            if x <= T::MAX {
                assert_eq!(floor.0, Rational::exact_from(&x).floor());
            }
            if floor.0 != T::MAX {
                assert!(floor.0 + T::ONE > x);
            }
            assert_eq!(
                T::rounding_from(
                    &x,
                    if x >= T::ZERO {
                        RoundingMode::Down
                    } else {
                        RoundingMode::Up
                    }
                ),
                floor
            );
            o_floor = Some(floor);
        }
        let mut o_ceiling = None;
        if x <= T::MAX {
            let ceiling = T::rounding_from(&x, RoundingMode::Ceiling);
            assert!(ceiling.0 >= x);
            if x >= T::MIN {
                assert_eq!(ceiling.0, Rational::exact_from(&x).ceiling());
            }
            if ceiling.0 != T::MIN {
                assert!(ceiling.0 - T::ONE < x);
            }
            assert_eq!(
                T::rounding_from(
                    &x,
                    if x >= T::ZERO {
                        RoundingMode::Up
                    } else {
                        RoundingMode::Down
                    }
                ),
                ceiling
            );
            o_ceiling = Some(ceiling);
        }
        let nearest = T::rounding_from(&x, RoundingMode::Nearest);
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
        let no = (n, Ordering::Equal);
        assert_eq!(T::rounding_from(&x, RoundingMode::Floor), no);
        assert_eq!(T::rounding_from(&x, RoundingMode::Down), no);
        assert_eq!(T::rounding_from(&x, RoundingMode::Ceiling), no);
        assert_eq!(T::rounding_from(&x, RoundingMode::Up), no);
        assert_eq!(T::rounding_from(&x, RoundingMode::Nearest), no);
        assert_eq!(T::rounding_from(&x, RoundingMode::Exact), no);

        let x = Float::from_signed_times_power_of_2((no.0 << 1) | T::ONE, -1);
        assert!(T::rounding_from(x, RoundingMode::Nearest).0.even());
    });
}

#[test]
fn rounding_from_float_properties() {
    apply_fn_to_unsigneds!(rounding_from_float_properties_helper_unsigned);
    apply_fn_to_signeds!(rounding_from_float_properties_helper_signed);
}
