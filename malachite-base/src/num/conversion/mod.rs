// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Traits for working with the digits of numbers.
pub mod digits;
/// Traits for converting between different number types. The traits are
/// [`WrappingFrom`](traits::WrappingFrom), [`SaturatingFrom`](traits::SaturatingFrom),
/// [`OverflowingFrom`](traits::OverflowingFrom), [`ConvertibleFrom`](traits::ConvertibleFrom), and
/// [`RoundingFrom`](traits::RoundingFrom).
///
/// # try_from
/// ```
/// use malachite_base::num::conversion::from::{
///     PrimitiveFloatFromSignedError, PrimitiveFloatFromUnsignedError, SignedFromFloatError,
///     UnsignedFromFloatError,
/// };
/// use malachite_base::num::float::NiceFloat;
///
/// assert_eq!(NiceFloat::<f32>::try_from(100u8), Ok(NiceFloat(100.0)));
/// assert_eq!(
///     NiceFloat::<f32>::try_from(u32::MAX),
///     Err(PrimitiveFloatFromUnsignedError)
/// );
/// assert_eq!(NiceFloat::<f32>::try_from(100i8), Ok(NiceFloat(100.0)));
/// assert_eq!(
///     NiceFloat::<f32>::try_from(i32::MAX),
///     Err(PrimitiveFloatFromSignedError)
/// );
///
/// assert_eq!(u8::try_from(NiceFloat(100.0f32)), Ok(100));
/// assert_eq!(
///     u8::try_from(NiceFloat(100.1f32)),
///     Err(UnsignedFromFloatError::FloatNonIntegerOrOutOfRange)
/// );
/// assert_eq!(
///     u8::try_from(NiceFloat(300.0f32)),
///     Err(UnsignedFromFloatError::FloatNonIntegerOrOutOfRange)
/// );
/// assert_eq!(
///     u8::try_from(NiceFloat(-100.0f32)),
///     Err(UnsignedFromFloatError::FloatNegative)
/// );
/// assert_eq!(i8::try_from(NiceFloat(-100.0f32)), Ok(-100));
/// assert_eq!(
///     i8::try_from(NiceFloat(-200.0f32)),
///     Err(SignedFromFloatError::FloatNonIntegerOrOutOfRange)
/// );
/// ```
///
/// # wrapping_from
/// ```
/// use malachite_base::num::conversion::traits::WrappingFrom;
///
/// assert_eq!(u8::wrapping_from(123u8), 123);
/// assert_eq!(i32::wrapping_from(-5i32), -5);
///
/// assert_eq!(u16::wrapping_from(123u8), 123);
/// assert_eq!(i64::wrapping_from(-5i32), -5);
/// assert_eq!(u32::wrapping_from(5u64), 5);
///
/// assert_eq!(u8::wrapping_from(1000u16), 232);
/// assert_eq!(u32::wrapping_from(-5i32), 4294967291);
/// assert_eq!(i32::wrapping_from(3000000000u32), -1294967296);
/// assert_eq!(i8::wrapping_from(-1000i16), 24);
/// ```
///
/// # saturating_from
/// ```
/// use malachite_base::num::conversion::traits::SaturatingFrom;
///
/// assert_eq!(u8::saturating_from(123u8), 123);
/// assert_eq!(i32::saturating_from(-5i32), -5);
///
/// assert_eq!(u16::saturating_from(123u8), 123);
/// assert_eq!(i64::saturating_from(-5i32), -5);
/// assert_eq!(u32::saturating_from(5u64), 5);
///
/// assert_eq!(u8::saturating_from(1000u16), 255);
/// assert_eq!(u32::saturating_from(-5i32), 0);
/// assert_eq!(i32::saturating_from(3000000000u32), 2147483647);
/// assert_eq!(i8::saturating_from(-1000i16), -128);
/// ```
///
/// # overflowing_from
/// ```
/// use malachite_base::num::conversion::traits::OverflowingFrom;
///
/// assert_eq!(u8::overflowing_from(123u8), (123, false));
/// assert_eq!(i32::overflowing_from(-5i32), (-5, false));
///
/// assert_eq!(u16::overflowing_from(123u8), (123, false));
/// assert_eq!(i64::overflowing_from(-5i32), (-5, false));
/// assert_eq!(u32::overflowing_from(5u64), (5, false));
///
/// assert_eq!(u8::overflowing_from(1000u16), (232, true));
/// assert_eq!(u32::overflowing_from(-5i32), (4294967291, true));
/// assert_eq!(i32::overflowing_from(3000000000u32), (-1294967296, true));
/// assert_eq!(i8::overflowing_from(-1000i16), (24, true));
/// ```
///
/// # convertible_from
/// ```
/// use malachite_base::num::conversion::traits::ConvertibleFrom;
///
/// assert_eq!(u8::convertible_from(123u8), true);
/// assert_eq!(i32::convertible_from(-5i32), true);
///
/// assert_eq!(u16::convertible_from(123u8), true);
/// assert_eq!(i64::convertible_from(-5i32), true);
/// assert_eq!(u32::convertible_from(5u64), true);
///
/// assert_eq!(u8::convertible_from(1000u16), false);
/// assert_eq!(u32::convertible_from(-5i32), false);
/// assert_eq!(i32::convertible_from(3000000000u32), false);
/// assert_eq!(i8::convertible_from(-1000i16), false);
///
/// assert_eq!(f32::convertible_from(100u8), true);
/// assert_eq!(f32::convertible_from(u32::MAX), false);
///
/// assert_eq!(u8::convertible_from(100.0f32), true);
/// assert_eq!(u8::convertible_from(100.1f32), false);
/// assert_eq!(u8::convertible_from(300.0f32), false);
/// assert_eq!(u8::convertible_from(-100.0f32), false);
/// ```
///
/// # rounding_from
/// ```
/// use malachite_base::num::conversion::traits::RoundingFrom;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use std::cmp::Ordering::*;
///
/// assert_eq!(f32::rounding_from(100, Floor), (100.0, Equal));
/// assert_eq!(f32::rounding_from(100, Down), (100.0, Equal));
/// assert_eq!(f32::rounding_from(100, Ceiling), (100.0, Equal));
/// assert_eq!(f32::rounding_from(100, Up), (100.0, Equal));
/// assert_eq!(f32::rounding_from(100, Nearest), (100.0, Equal));
/// assert_eq!(f32::rounding_from(100, Exact), (100.0, Equal));
///
/// assert_eq!(f32::rounding_from(i32::MAX, Floor), (2147483500.0, Less));
/// assert_eq!(f32::rounding_from(i32::MAX, Down), (2147483500.0, Less));
/// assert_eq!(
///     f32::rounding_from(i32::MAX, Ceiling),
///     (2147483600.0, Greater)
/// );
/// assert_eq!(f32::rounding_from(i32::MAX, Up), (2147483600.0, Greater));
/// assert_eq!(
///     f32::rounding_from(i32::MAX, Nearest),
///     (2147483600.0, Greater)
/// );
///
/// assert_eq!(u32::rounding_from(100.0f32, Floor), (100, Equal));
/// assert_eq!(u32::rounding_from(100.0f32, Down), (100, Equal));
/// assert_eq!(u32::rounding_from(100.0f32, Ceiling), (100, Equal));
/// assert_eq!(u32::rounding_from(100.0f32, Up), (100, Equal));
/// assert_eq!(u32::rounding_from(100.0f32, Nearest), (100, Equal));
/// assert_eq!(u32::rounding_from(100.0f32, Exact), (100, Equal));
///
/// assert_eq!(u32::rounding_from(100.5f32, Floor), (100, Less));
/// assert_eq!(u32::rounding_from(100.5f32, Down), (100, Less));
/// assert_eq!(u32::rounding_from(100.5f32, Ceiling), (101, Greater));
/// assert_eq!(u32::rounding_from(100.5f32, Up), (101, Greater));
/// assert_eq!(u32::rounding_from(100.5f32, Nearest), (100, Less));
/// ```
pub mod from;
/// [`JoinHalves`](traits::JoinHalves) and [`SplitInHalf`](traits::SplitInHalf), traits for joining
/// the bits of two numbers or for splitting a number in half.
///
/// # join_halves
/// ```
/// use malachite_base::num::conversion::traits::JoinHalves;
///
/// assert_eq!(u16::join_halves(1, 2), 258);
/// assert_eq!(u32::join_halves(0xabcd, 0x1234), 0xabcd1234);
/// ```
///
/// # split_in_half
/// ```
/// use malachite_base::num::conversion::traits::SplitInHalf;
///
/// assert_eq!(258u16.split_in_half(), (1, 2));
/// assert_eq!(0xabcd1234u32.split_in_half(), (0xabcd, 0x1234));
/// ```
///
/// # lower_half
/// ```
/// use malachite_base::num::conversion::traits::SplitInHalf;
///
/// assert_eq!(258u16.lower_half(), 2);
/// assert_eq!(0xabcd1234u32.lower_half(), 0x1234);
/// ```
///
/// # upper_half
/// ```
/// use malachite_base::num::conversion::traits::SplitInHalf;
///
/// assert_eq!(258u16.upper_half(), 1);
/// assert_eq!(0xabcd1234u32.upper_half(), 0xabcd);
/// ```
pub mod half;
/// [`IsInteger`](traits::IsInteger), a trait for determining whether a value is an integer.
///
/// # is_integer
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::conversion::traits::IsInteger;
///
/// assert_eq!(0.is_integer(), true);
/// assert_eq!(1.is_integer(), true);
/// assert_eq!(100.is_integer(), true);
/// assert_eq!((-1).is_integer(), true);
/// assert_eq!((-100).is_integer(), true);
///
/// assert_eq!(0.0.is_integer(), true);
/// assert_eq!(1.0.is_integer(), true);
/// assert_eq!(100.0.is_integer(), true);
/// assert_eq!((-1.0).is_integer(), true);
/// assert_eq!((-100.0).is_integer(), true);
///
/// assert_eq!(0.1.is_integer(), false);
/// assert_eq!(100.1.is_integer(), false);
/// assert_eq!(f32::NAN.is_integer(), false);
/// assert_eq!(f32::INFINITY.is_integer(), false);
/// assert_eq!(f32::NEGATIVE_INFINITY.is_integer(), false);
/// ```
pub mod is_integer;
/// Traits for converting numbers to and from mantissa and exponent representations.
///
/// See [`PrimitiveFloat`](super::basic::floats::PrimitiveFloat) for a description of the different
/// types of mantissas and exponents. The traits are
/// [`RawMantissaAndExponent`](traits::RawMantissaAndExponent),
/// [`IntegerMantissaAndExponent`](traits::IntegerMantissaAndExponent), and
/// [`SciMantissaAndExponent`](traits::SciMantissaAndExponent).
///
/// # raw_mantissa_and_exponent
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::conversion::traits::RawMantissaAndExponent;
///
/// assert_eq!(0.0f32.raw_mantissa_and_exponent(), (0, 0));
/// assert_eq!((-0.0f32).raw_mantissa_and_exponent(), (0, 0));
/// assert_eq!(f32::NAN.raw_mantissa_and_exponent(), (0x400000, 255));
/// assert_eq!(f32::INFINITY.raw_mantissa_and_exponent(), (0, 255));
/// assert_eq!(f32::NEGATIVE_INFINITY.raw_mantissa_and_exponent(), (0, 255));
/// assert_eq!(1.0f32.raw_mantissa_and_exponent(), (0, 127));
/// assert_eq!(
///     core::f32::consts::PI.raw_mantissa_and_exponent(),
///     (4788187, 128)
/// );
/// assert_eq!(0.1f32.raw_mantissa_and_exponent(), (5033165, 123));
/// ```
///
/// # raw_mantissa
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::conversion::traits::RawMantissaAndExponent;
///
/// assert_eq!(0.0f32.raw_mantissa(), 0);
/// assert_eq!((-0.0f32).raw_mantissa(), 0);
/// assert_eq!(f32::NAN.raw_mantissa(), 0x400000);
/// assert_eq!(f32::INFINITY.raw_mantissa(), 0);
/// assert_eq!(f32::NEGATIVE_INFINITY.raw_mantissa(), 0);
/// assert_eq!(1.0f32.raw_mantissa(), 0);
/// assert_eq!(core::f32::consts::PI.raw_mantissa(), 4788187);
/// assert_eq!(0.1f32.raw_mantissa(), 5033165);
/// ```
///
/// # raw_exponent
///
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::conversion::traits::RawMantissaAndExponent;
///
/// assert_eq!(0.0f32.raw_exponent(), 0);
/// assert_eq!((-0.0f32).raw_exponent(), 0);
/// assert_eq!(f32::NAN.raw_exponent(), 255);
/// assert_eq!(f32::INFINITY.raw_exponent(), 255);
/// assert_eq!(f32::NEGATIVE_INFINITY.raw_exponent(), 255);
/// assert_eq!(1.0f32.raw_exponent(), 127);
/// assert_eq!(core::f32::consts::PI.raw_exponent(), 128);
/// assert_eq!(0.1f32.raw_exponent(), 123);
/// ```
///
/// # from_raw_mantissa_and_exponent
/// ```
/// use malachite_base::num::conversion::traits::RawMantissaAndExponent;
/// use malachite_base::num::float::NiceFloat;
///
/// assert_eq!(
///     NiceFloat(f32::from_raw_mantissa_and_exponent(0, 0)),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(f32::from_raw_mantissa_and_exponent(0x400000, 255)),
///     NiceFloat(f32::NAN)
/// );
/// assert_eq!(
///     NiceFloat(f32::from_raw_mantissa_and_exponent(0, 255)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(f32::from_raw_mantissa_and_exponent(0, 127)),
///     NiceFloat(1.0)
/// );
/// assert_eq!(
///     NiceFloat(f32::from_raw_mantissa_and_exponent(4788187, 128)),
///     NiceFloat(core::f32::consts::PI)
/// );
/// assert_eq!(
///     NiceFloat(f32::from_raw_mantissa_and_exponent(5033165, 123)),
///     NiceFloat(0.1)
/// );
/// assert_eq!(
///     NiceFloat(f32::from_raw_mantissa_and_exponent(2097152, 130)),
///     NiceFloat(10.0)
/// );
/// ```
///
/// # integer_mantissa_and_exponent
/// ```
/// use malachite_base::num::basic::floats::PrimitiveFloat;
/// use malachite_base::num::conversion::traits::IntegerMantissaAndExponent;
///
/// assert_eq!(1u8.integer_mantissa_and_exponent(), (1, 0));
/// assert_eq!(2u8.integer_mantissa_and_exponent(), (1, 1));
/// assert_eq!(3u8.integer_mantissa_and_exponent(), (3, 0));
/// assert_eq!(100u8.integer_mantissa_and_exponent(), (25, 2));
///
/// assert_eq!(
///     core::f32::consts::PI.integer_mantissa_and_exponent(),
///     (13176795, -22)
/// );
/// assert_eq!(0.1f32.integer_mantissa_and_exponent(), (13421773, -27));
/// assert_eq!(10.0f32.integer_mantissa_and_exponent(), (5, 1));
/// assert_eq!(
///     f32::MIN_POSITIVE_SUBNORMAL.integer_mantissa_and_exponent(),
///     (1, -149)
/// );
/// assert_eq!(
///     f32::MAX_SUBNORMAL.integer_mantissa_and_exponent(),
///     (0x7fffff, -149)
/// );
/// assert_eq!(
///     f32::MIN_POSITIVE_NORMAL.integer_mantissa_and_exponent(),
///     (1, -126)
/// );
/// assert_eq!(
///     f32::MAX_FINITE.integer_mantissa_and_exponent(),
///     (0xffffff, 104)
/// );
/// ```
///
/// # integer_mantissa
/// ```
/// use malachite_base::num::basic::floats::PrimitiveFloat;
/// use malachite_base::num::conversion::traits::IntegerMantissaAndExponent;
///
/// assert_eq!(1u8.integer_mantissa(), 1);
/// assert_eq!(2u8.integer_mantissa(), 1);
/// assert_eq!(3u8.integer_mantissa(), 3);
/// assert_eq!(100u8.integer_mantissa(), 25);
///
/// assert_eq!(1.0f32.integer_mantissa(), 1);
/// assert_eq!(core::f32::consts::PI.integer_mantissa(), 13176795);
/// assert_eq!(0.1f32.integer_mantissa(), 13421773);
/// assert_eq!(10.0f32.integer_mantissa(), 5);
/// assert_eq!(f32::MIN_POSITIVE_SUBNORMAL.integer_mantissa(), 1);
/// assert_eq!(f32::MAX_SUBNORMAL.integer_mantissa(), 0x7fffff);
/// assert_eq!(f32::MIN_POSITIVE_NORMAL.integer_mantissa(), 1);
/// assert_eq!(f32::MAX_FINITE.integer_mantissa(), 0xffffff);
/// ```
///
/// # integer_exponent
/// ```
/// use malachite_base::num::basic::floats::PrimitiveFloat;
/// use malachite_base::num::conversion::traits::IntegerMantissaAndExponent;
///
/// assert_eq!(1u8.integer_exponent(), 0);
/// assert_eq!(2u8.integer_exponent(), 1);
/// assert_eq!(3u8.integer_exponent(), 0);
/// assert_eq!(100u8.integer_exponent(), 2);
///
/// assert_eq!(1.0f32.integer_exponent(), 0);
/// assert_eq!(core::f32::consts::PI.integer_exponent(), -22);
/// assert_eq!(0.1f32.integer_exponent(), -27);
/// assert_eq!(10.0f32.integer_exponent(), 1);
/// assert_eq!(f32::MIN_POSITIVE_SUBNORMAL.integer_exponent(), -149);
/// assert_eq!(f32::MAX_SUBNORMAL.integer_exponent(), -149);
/// assert_eq!(f32::MIN_POSITIVE_NORMAL.integer_exponent(), -126);
/// assert_eq!(f32::MAX_FINITE.integer_exponent(), 104);
/// ```
///
/// # from_integer_mantissa_and_exponent;
/// ```
/// use malachite_base::num::conversion::traits::IntegerMantissaAndExponent;
/// use malachite_base::num::float::NiceFloat;
///
/// assert_eq!(u8::from_integer_mantissa_and_exponent(0, 1), Some(0));
/// assert_eq!(u8::from_integer_mantissa_and_exponent(1, 0), Some(1));
/// assert_eq!(u8::from_integer_mantissa_and_exponent(1, 1), Some(2));
/// assert_eq!(u8::from_integer_mantissa_and_exponent(3, 0), Some(3));
/// assert_eq!(u8::from_integer_mantissa_and_exponent(25, 2), Some(100));
///
/// assert_eq!(
///     f32::from_integer_mantissa_and_exponent(0, 5).map(NiceFloat),
///     Some(NiceFloat(0.0))
/// );
/// assert_eq!(
///     f32::from_integer_mantissa_and_exponent(1, 0).map(NiceFloat),
///     Some(NiceFloat(1.0))
/// );
/// assert_eq!(
///     f32::from_integer_mantissa_and_exponent(4, -2).map(NiceFloat),
///     Some(NiceFloat(1.0))
/// );
/// assert_eq!(
///     f32::from_integer_mantissa_and_exponent(13176795, -22).map(NiceFloat),
///     Some(NiceFloat(core::f32::consts::PI))
/// );
/// assert_eq!(
///     f32::from_integer_mantissa_and_exponent(13421773, -27).map(NiceFloat),
///     Some(NiceFloat(0.1))
/// );
/// assert_eq!(
///     f32::from_integer_mantissa_and_exponent(5, 1).map(NiceFloat),
///     Some(NiceFloat(10.0))
/// );
///
/// assert_eq!(f32::from_integer_mantissa_and_exponent(5, 10000), None);
/// assert_eq!(f32::from_integer_mantissa_and_exponent(5, -10000), None);
/// // In the next 3 examples, the precision is too high.
/// assert_eq!(f32::from_integer_mantissa_and_exponent(u64::MAX, -32), None);
/// assert_eq!(f32::from_integer_mantissa_and_exponent(3, -150), None);
/// assert_eq!(f32::from_integer_mantissa_and_exponent(1, 128), None);
/// ```
///
/// # sci_mantissa_and_exponent
/// ```
/// use malachite_base::num::basic::floats::PrimitiveFloat;
/// use malachite_base::num::conversion::traits::SciMantissaAndExponent;
/// use malachite_base::num::float::NiceFloat;
///
/// let test = |n: u32, mantissa: f32, exponent: u64| {
///     let (m, e) = n.sci_mantissa_and_exponent();
///     assert_eq!(NiceFloat(m), NiceFloat(mantissa));
///     assert_eq!(e, exponent);
/// };
/// test(3, 1.5, 1);
/// test(123, 1.921875, 6);
/// test(1000000000, 1.8626451, 29);
///
/// let test = |x: f32, mantissa: f32, exponent: i64| {
///     let (actual_mantissa, actual_exponent) = x.sci_mantissa_and_exponent();
///     assert_eq!(NiceFloat(actual_mantissa), NiceFloat(mantissa));
///     assert_eq!(actual_exponent, exponent);
/// };
/// test(1.0, 1.0, 0);
/// test(core::f32::consts::PI, 1.5707964, 1);
/// test(0.1, 1.6, -4);
/// test(10.0, 1.25, 3);
/// test(f32::MIN_POSITIVE_SUBNORMAL, 1.0, -149);
/// test(f32::MAX_SUBNORMAL, 1.9999998, -127);
/// test(f32::MIN_POSITIVE_NORMAL, 1.0, -126);
/// test(f32::MAX_FINITE, 1.9999999, 127);
/// ```
///
/// # sci_mantissa
/// ```
/// use malachite_base::num::basic::floats::PrimitiveFloat;
/// use malachite_base::num::conversion::traits::SciMantissaAndExponent;
/// use malachite_base::num::float::NiceFloat;
///
/// assert_eq!(NiceFloat(1.0f32.sci_mantissa()), NiceFloat(1.0));
/// assert_eq!(
///     NiceFloat(core::f32::consts::PI.sci_mantissa()),
///     NiceFloat(1.5707964)
/// );
/// assert_eq!(NiceFloat(0.1f32.sci_mantissa()), NiceFloat(1.6));
/// assert_eq!(NiceFloat(10.0f32.sci_mantissa()), NiceFloat(1.25));
/// assert_eq!(
///     NiceFloat(f32::MIN_POSITIVE_SUBNORMAL.sci_mantissa()),
///     NiceFloat(1.0)
/// );
/// assert_eq!(
///     NiceFloat(f32::MAX_SUBNORMAL.sci_mantissa()),
///     NiceFloat(1.9999998)
/// );
/// assert_eq!(
///     NiceFloat(f32::MIN_POSITIVE_NORMAL.sci_mantissa()),
///     NiceFloat(1.0)
/// );
/// assert_eq!(
///     NiceFloat(f32::MAX_FINITE.sci_mantissa()),
///     NiceFloat(1.9999999)
/// );
/// ```
///
/// # sci_exponent
/// ```
/// use malachite_base::num::basic::floats::PrimitiveFloat;
/// use malachite_base::num::conversion::traits::SciMantissaAndExponent;
///
/// assert_eq!(1.0f32.sci_exponent(), 0);
/// assert_eq!(core::f32::consts::PI.sci_exponent(), 1);
/// assert_eq!(0.1f32.sci_exponent(), -4);
/// assert_eq!(10.0f32.sci_exponent(), 3);
/// assert_eq!(f32::MIN_POSITIVE_SUBNORMAL.sci_exponent(), -149);
/// assert_eq!(f32::MAX_SUBNORMAL.sci_exponent(), -127);
/// assert_eq!(f32::MIN_POSITIVE_NORMAL.sci_exponent(), -126);
/// assert_eq!(f32::MAX_FINITE.sci_exponent(), 127);
/// ```
///
/// # from_sci_mantissa_and_exponent;
/// ```
/// use malachite_base::num::basic::floats::PrimitiveFloat;
/// use malachite_base::num::conversion::traits::SciMantissaAndExponent;
/// use malachite_base::num::float::NiceFloat;
///
/// assert_eq!(u32::from_sci_mantissa_and_exponent(1.5, 1u64), Some(3u32));
/// assert_eq!(u32::from_sci_mantissa_and_exponent(1.51, 1u64), Some(3u32));
/// assert_eq!(
///     u32::from_sci_mantissa_and_exponent(1.921875, 6u64),
///     Some(123u32)
/// );
/// assert_eq!(u32::from_sci_mantissa_and_exponent(1.5, 1u64), Some(3u32));
///
/// assert_eq!(
///     f32::from_sci_mantissa_and_exponent(1.0, 0).map(NiceFloat),
///     Some(NiceFloat(1.0))
/// );
/// assert_eq!(
///     f32::from_sci_mantissa_and_exponent(1.5707964, 1).map(NiceFloat),
///     Some(NiceFloat(core::f32::consts::PI))
/// );
/// assert_eq!(
///     f32::from_sci_mantissa_and_exponent(1.6, -4).map(NiceFloat),
///     Some(NiceFloat(0.1))
/// );
/// assert_eq!(
///     f32::from_sci_mantissa_and_exponent(1.25, 3).map(NiceFloat),
///     Some(NiceFloat(10.0))
/// );
/// assert_eq!(
///     f32::from_sci_mantissa_and_exponent(1.0, -149).map(NiceFloat),
///     Some(NiceFloat(f32::MIN_POSITIVE_SUBNORMAL))
/// );
/// assert_eq!(
///     f32::from_sci_mantissa_and_exponent(1.9999998, -127).map(NiceFloat),
///     Some(NiceFloat(f32::MAX_SUBNORMAL))
/// );
/// assert_eq!(
///     f32::from_sci_mantissa_and_exponent(1.0, -126).map(NiceFloat),
///     Some(NiceFloat(f32::MIN_POSITIVE_NORMAL))
/// );
/// assert_eq!(
///     f32::from_sci_mantissa_and_exponent(1.9999999, 127).map(NiceFloat),
///     Some(NiceFloat(f32::MAX_FINITE))
/// );
///
/// assert_eq!(f32::from_sci_mantissa_and_exponent(2.0, 1), None);
/// assert_eq!(f32::from_sci_mantissa_and_exponent(1.1, -2000), None);
/// assert_eq!(f32::from_sci_mantissa_and_exponent(1.1, 2000), None);
/// assert_eq!(f32::from_sci_mantissa_and_exponent(1.999, -149), None);
/// ```
pub mod mantissa_and_exponent;
/// Traits for converting slices to numbers, slices to [`Vec`]s, or numbers to [`Vec`]s. The traits
/// are [`FromOtherTypeSlice`](traits::FromOtherTypeSlice),
/// [`VecFromOtherTypeSlice`](traits::VecFromOtherTypeSlice), and
/// [`VecFromOtherType`](traits::VecFromOtherType).
///
/// # from_other_type_slice
/// ```
/// use malachite_base::num::conversion::traits::FromOtherTypeSlice;
///
/// let xs: &[u32] = &[];
/// assert_eq!(u32::from_other_type_slice(xs), 0);
/// assert_eq!(u32::from_other_type_slice(&[123u32, 456]), 123);
///
/// assert_eq!(u8::from_other_type_slice(&[0xabcdu16, 0xef01]), 0xcd);
///
/// assert_eq!(u16::from_other_type_slice(&[0xabu8, 0xcd, 0xef]), 0xcdab);
/// assert_eq!(u64::from_other_type_slice(&[0xabu8, 0xcd, 0xef]), 0xefcdab);
/// ```
///
/// # vec_from_other_type_slice
/// ```
/// use malachite_base::num::conversion::traits::VecFromOtherTypeSlice;
///
/// assert_eq!(u32::vec_from_other_type_slice(&[123u32, 456]), &[123, 456]);
/// assert_eq!(
///     u8::vec_from_other_type_slice(&[0xcdabu16, 0x01ef, 0x4523, 0x8967]),
///     &[0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89]
/// );
/// assert_eq!(
///     u16::vec_from_other_type_slice(&[0xabu8, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67]),
///     &[0xcdab, 0x01ef, 0x4523, 0x67]
/// );
/// ```
///
/// # vec_from_other_type
/// ```
/// use malachite_base::num::conversion::traits::VecFromOtherType;
///
/// assert_eq!(u32::vec_from_other_type(123u32), &[123]);
/// assert_eq!(u8::vec_from_other_type(0xcdabu16), &[0xab, 0xcd]);
/// assert_eq!(u16::vec_from_other_type(0xabu8), &[0xab]);
/// ```
pub mod slice;
/// Traits for converting numbers to and from [`String`]s.
pub mod string;
/// Various traits for converting numbers.
pub mod traits;
