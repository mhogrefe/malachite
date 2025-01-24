// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of traits for working with the digits of [`Natural`](crate::natural::Natural)s.
pub mod digits;
/// An implementation of the [`From`] trait for converting a [`bool`] to a
/// [`Natural`](crate::natural::Natural).
pub mod from_bool;
/// Functions for constructing a [`Natural`](crate::natural::Natural) from [`Limb`](crate#limbs)s.
pub mod from_limbs;
/// Implementations of traits for converting a primitive float to a
/// [`Natural`](crate::natural::Natural).
///
/// The traits are [`TryFrom`],
/// [`ConvertibleFrom`](malachite_base::num::conversion::traits::ConvertibleFrom), and
/// [`RoundingFrom`](malachite_base::num::conversion::traits::RoundingFrom).
///
/// # rounding_from
/// ```
/// use malachite_base::num::conversion::traits::RoundingFrom;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(
///     Natural::rounding_from(0.0, Exact).to_debug_string(),
///     "(0, Equal)"
/// );
/// assert_eq!(
///     Natural::rounding_from(-0.0, Exact).to_debug_string(),
///     "(0, Equal)"
/// );
/// assert_eq!(
///     Natural::rounding_from(123.0, Exact).to_debug_string(),
///     "(123, Equal)"
/// );
/// assert_eq!(
///     Natural::rounding_from(1.0e9, Exact).to_debug_string(),
///     "(1000000000, Equal)"
/// );
/// assert_eq!(
///     Natural::rounding_from(1.0e9, Exact).to_debug_string(),
///     "(1000000000, Equal)"
/// );
/// assert_eq!(
///     Natural::rounding_from(4294967295.0, Exact).to_debug_string(),
///     "(4294967295, Equal)"
/// );
/// assert_eq!(
///     Natural::rounding_from(4294967296.0, Exact).to_debug_string(),
///     "(4294967296, Equal)"
/// );
/// assert_eq!(
///     Natural::rounding_from(1.0e100, Exact).to_debug_string(),
///     "(1000000000000000015902891109759918046836080856394528138978132755774783877217038106081346\
///     9985856815104, Equal)"
/// );
/// assert_eq!(
///     Natural::rounding_from(123.1, Floor).to_debug_string(),
///     "(123, Less)"
/// );
/// assert_eq!(
///     Natural::rounding_from(123.1, Ceiling).to_debug_string(),
///     "(124, Greater)"
/// );
/// assert_eq!(
///     Natural::rounding_from(123.1, Nearest).to_debug_string(),
///     "(123, Less)"
/// );
/// assert_eq!(
///     Natural::rounding_from(123.9, Floor).to_debug_string(),
///     "(123, Less)"
/// );
/// assert_eq!(
///     Natural::rounding_from(123.9, Ceiling).to_debug_string(),
///     "(124, Greater)"
/// );
/// assert_eq!(
///     Natural::rounding_from(123.9, Nearest).to_debug_string(),
///     "(124, Greater)"
/// );
/// assert_eq!(
///     Natural::rounding_from(123.5, Nearest).to_debug_string(),
///     "(124, Greater)"
/// );
/// assert_eq!(
///     Natural::rounding_from(124.5, Nearest).to_debug_string(),
///     "(124, Less)"
/// );
/// assert_eq!(
///     Natural::rounding_from(-0.99, Ceiling).to_debug_string(),
///     "(0, Greater)"
/// );
/// assert_eq!(
///     Natural::rounding_from(-0.499, Nearest).to_debug_string(),
///     "(0, Greater)"
/// );
/// assert_eq!(
///     Natural::rounding_from(-0.5, Nearest).to_debug_string(),
///     "(0, Greater)"
/// );
/// ```
///
/// # try_from
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(
///     Natural::try_from(f64::NAN).to_debug_string(),
///     "Err(FloatInfiniteOrNan)"
/// );
/// assert_eq!(
///     Natural::try_from(f64::INFINITY).to_debug_string(),
///     "Err(FloatInfiniteOrNan)"
/// );
/// assert_eq!(
///     Natural::try_from(f64::NEGATIVE_INFINITY).to_debug_string(),
///     "Err(FloatInfiniteOrNan)"
/// );
/// assert_eq!(Natural::try_from(0.0).to_debug_string(), "Ok(0)");
/// assert_eq!(Natural::try_from(-0.0).to_debug_string(), "Ok(0)");
/// assert_eq!(Natural::try_from(123.0).to_debug_string(), "Ok(123)");
/// assert_eq!(Natural::try_from(1.0e9).to_debug_string(), "Ok(1000000000)");
/// assert_eq!(
///     Natural::try_from(4294967295.0).to_debug_string(),
///     "Ok(4294967295)"
/// );
/// assert_eq!(
///     Natural::try_from(4294967296.0).to_debug_string(),
///     "Ok(4294967296)"
/// );
/// assert_eq!(
///     Natural::try_from(1.0e100).to_debug_string(),
///     "Ok(10000000000000000159028911097599180468360808563945281389781327557747838772170381060813\
///     469985856815104)"
/// );
/// assert_eq!(
///     Natural::try_from(123.1).to_debug_string(),
///     "Err(FloatNonIntegerOrOutOfRange)"
/// );
/// assert_eq!(
///     Natural::try_from(123.9).to_debug_string(),
///     "Err(FloatNonIntegerOrOutOfRange)"
/// );
/// assert_eq!(
///     Natural::try_from(123.5).to_debug_string(),
///     "Err(FloatNonIntegerOrOutOfRange)"
/// );
/// assert_eq!(
///     Natural::try_from(124.5).to_debug_string(),
///     "Err(FloatNonIntegerOrOutOfRange)"
/// );
/// assert_eq!(
///     Natural::try_from(-0.499).to_debug_string(),
///     "Err(FloatNegative)"
/// );
/// assert_eq!(
///     Natural::try_from(-0.5).to_debug_string(),
///     "Err(FloatNegative)"
/// );
/// assert_eq!(
///     Natural::try_from(-123.0).to_debug_string(),
///     "Err(FloatNegative)"
/// );
/// ```
///
/// # convertible_from
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::conversion::traits::ConvertibleFrom;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(Natural::convertible_from(f64::NAN), false);
/// assert_eq!(Natural::convertible_from(f64::INFINITY), false);
/// assert_eq!(Natural::convertible_from(f64::NEGATIVE_INFINITY), false);
/// assert_eq!(Natural::convertible_from(0.0), true);
/// assert_eq!(Natural::convertible_from(-0.0), true);
/// assert_eq!(Natural::convertible_from(123.0), true);
/// assert_eq!(Natural::convertible_from(1.0e9), true);
/// assert_eq!(Natural::convertible_from(4294967295.0), true);
/// assert_eq!(Natural::convertible_from(4294967296.0), true);
/// assert_eq!(Natural::convertible_from(1.0e100), true);
/// assert_eq!(Natural::convertible_from(123.1), false);
/// assert_eq!(Natural::convertible_from(123.9), false);
/// assert_eq!(Natural::convertible_from(123.5), false);
/// assert_eq!(Natural::convertible_from(124.5), false);
/// assert_eq!(Natural::convertible_from(-0.499), false);
/// assert_eq!(Natural::convertible_from(-0.5), false);
/// assert_eq!(Natural::convertible_from(-123.0), false);
/// ```
pub mod from_primitive_float;
/// Implementations of traits for converting a primitive integer to a
/// [`Natural`](crate::natural::Natural).
///
/// The traits are [`From`], [`TryFrom`],
/// [`ConvertibleFrom`](malachite_base::num::conversion::traits::ConvertibleFrom), and
/// [`SaturatingFrom`](malachite_base::num::conversion::traits::SaturatingFrom).
///
/// # from
/// ```
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(Natural::from(123u64), 123);
/// assert_eq!(Natural::from(123u8), 123);
/// assert_eq!(Natural::from(123u128), 123);
/// ```
///
/// # try_from
/// ```
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(Natural::try_from(123i32).to_debug_string(), "Ok(123)");
/// assert_eq!(
///     Natural::try_from(-123i32).to_debug_string(),
///     "Err(NaturalFromSignedError)"
/// );
/// ```
///
/// # convertible_from
/// ```
/// use malachite_base::num::conversion::traits::ConvertibleFrom;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(Natural::convertible_from(123i32), true);
/// assert_eq!(Natural::convertible_from(-123i32), false);
/// ```
///
/// # saturating_from
/// ```
/// use malachite_base::num::conversion::traits::SaturatingFrom;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(Natural::saturating_from(123i32), 123);
/// assert_eq!(Natural::saturating_from(-123i32), 0);
/// ```
pub mod from_primitive_int;
/// An implementation of [`IsInteger`](malachite_base::num::conversion::traits::IsInteger), a trait
/// for determining whether a number is an integer.
///
/// A [`Natural`](crate::natural::Natural) is always an integer.
pub mod is_integer;
/// A function for counting a [`Natural`](crate::natural::Natural)'s [`Limb`](crate#limbs)s.
pub mod limb_count;
/// Implementations of traits for converting numbers to and from mantissa-and-exponent
/// representations.
///
/// See [`PrimitiveFloat`](malachite_base::num::basic::floats::PrimitiveFloat) for a description of
/// the different types of mantissas and exponents. The traits are
/// [`IntegerMantissaAndExponent`](malachite_base::num::conversion::traits::IntegerMantissaAndExponent)
/// and [`SciMantissaAndExponent`](malachite_base::num::conversion::traits::SciMantissaAndExponent).
///
/// Here are some examples of the macro-generated functions:
///
/// # sci_mantissa_and_exponent
/// ```
/// use malachite_base::num::arithmetic::traits::Pow;
/// use malachite_base::num::conversion::traits::SciMantissaAndExponent;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_nz::natural::Natural;
///
/// let test = |n: Natural, mantissa: f32, exponent: u64| {
///     let (m, e) = n.sci_mantissa_and_exponent();
///     assert_eq!(NiceFloat(m), NiceFloat(mantissa));
///     assert_eq!(e, exponent);
/// };
/// test(Natural::from(3u32), 1.5, 1);
/// test(Natural::from(123u32), 1.921875, 6);
/// test(Natural::from(1000000000u32), 1.8626451, 29);
/// test(Natural::from(10u32).pow(52), 1.670478, 172);
/// ```
///
/// # from_sci_mantissa_and_exponent
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::conversion::traits::SciMantissaAndExponent;
/// use malachite_nz::natural::Natural;
///
/// let test = |mantissa: f32, exponent: u64, out: Option<Natural>| {
///     assert_eq!(
///         <&Natural as SciMantissaAndExponent<_, _, _>>::from_sci_mantissa_and_exponent(
///             mantissa, exponent
///         ),
///         out
///     );
/// };
/// test(1.5, 1, Some(Natural::from(3u32)));
/// test(1.51, 1, Some(Natural::from(3u32)));
/// test(1.921875, 6, Some(Natural::from(123u32)));
/// test(
///     1.670478,
///     172,
///     Some(Natural::from_str("10000000254586612611935772707803116801852191350456320").unwrap()),
/// );
///
/// test(2.0, 1, None);
/// test(10.0, 1, None);
/// test(0.5, 1, None);
/// ```
pub mod mantissa_and_exponent;
/// Implementations of traits for converting a [`Natural`](crate::natural::Natural) to a primitive
/// float.
///
/// The traits are [`TryFrom`],
/// [`ConvertibleFrom`](malachite_base::num::conversion::traits::ConvertibleFrom), and
/// [`RoundingFrom`](malachite_base::num::conversion::traits::RoundingFrom).
///
/// # rounding_from
/// ```
/// use core::cmp::Ordering::*;
/// use core::str::FromStr;
/// use malachite_base::num::conversion::traits::RoundingFrom;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(
///     f32::rounding_from(&Natural::from_str("123").unwrap(), Exact),
///     (123.0, Equal)
/// );
/// assert_eq!(
///     f32::rounding_from(&Natural::from_str("1000000001").unwrap(), Floor),
///     (1.0e9, Less)
/// );
/// assert_eq!(
///     f32::rounding_from(&Natural::from_str("1000000001").unwrap(), Ceiling),
///     (1.00000006e9, Greater)
/// );
/// assert_eq!(
///     f32::rounding_from(
///         &Natural::from_str("10000000000000000000000000000000000000000000000000000").unwrap(),
///         Nearest
///     ),
///     (3.4028235e38, Less)
/// );
/// ```
///
/// # try_from
/// ```
/// use core::str::FromStr;
/// use malachite_nz::natural::conversion::primitive_float_from_natural::*;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(f32::try_from(&Natural::from_str("123").unwrap()), Ok(123.0));
/// assert_eq!(
///     f32::try_from(&Natural::from_str("1000000000").unwrap()),
///     Ok(1.0e9)
/// );
/// assert_eq!(
///     f32::try_from(&Natural::from_str("1000000001").unwrap()),
///     Err(PrimitiveFloatFromNaturalError)
/// );
/// assert_eq!(
///     f32::try_from(
///         &Natural::from_str("10000000000000000000000000000000000000000000000000000").unwrap()
///     ),
///     Err(PrimitiveFloatFromNaturalError)
/// );
/// ```
///
/// # convertible_from
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::conversion::traits::ConvertibleFrom;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(
///     f32::convertible_from(&Natural::from_str("123").unwrap()),
///     true
/// );
/// assert_eq!(
///     f32::convertible_from(&Natural::from_str("1000000000").unwrap()),
///     true
/// );
/// assert_eq!(
///     f32::convertible_from(&Natural::from_str("1000000001").unwrap()),
///     false
/// );
/// assert_eq!(
///     f32::convertible_from(
///         &Natural::from_str("10000000000000000000000000000000000000000000000000000").unwrap()
///     ),
///     false
/// );
/// ```
pub mod primitive_float_from_natural;
/// Implementations of traits for converting a [`Natural`](crate::natural::Natural) to a primitive
/// integer.
///
/// The traits are [`TryFrom`],
/// [`ConvertibleFrom`](malachite_base::num::conversion::traits::ConvertibleFrom),
/// [`OverflowingFrom`](malachite_base::num::conversion::traits::OverflowingFrom),
/// [`SaturatingFrom`](malachite_base::num::conversion::traits::SaturatingFrom), and
/// [`WrappingFrom`](malachite_base::num::conversion::traits::WrappingFrom).
///
/// # try_from
/// ```
/// use malachite_base::num::arithmetic::traits::Pow;
/// use malachite_base::num::basic::traits::One;
/// use malachite_nz::natural::conversion::primitive_int_from_natural::{
///     SignedFromNaturalError, UnsignedFromNaturalError,
/// };
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(u32::try_from(&Natural::from(123u32)), Ok(123));
/// assert_eq!(
///     u32::try_from(&Natural::from(10u32).pow(12)),
///     Err(UnsignedFromNaturalError)
/// );
/// assert_eq!(u8::try_from(&Natural::from(123u32)), Ok(123));
/// assert_eq!(
///     u8::try_from(&Natural::from(10u32).pow(12)),
///     Err(UnsignedFromNaturalError)
/// );
/// assert_eq!(u64::try_from(&Natural::from(123u32)), Ok(123));
/// assert_eq!(
///     u64::try_from(&(Natural::ONE << 100)),
///     Err(UnsignedFromNaturalError)
/// );
///
/// assert_eq!(i32::try_from(&Natural::from(123u32)), Ok(123));
/// assert_eq!(
///     i32::try_from(&Natural::from(10u32).pow(12)),
///     Err(SignedFromNaturalError)
/// );
/// assert_eq!(i8::try_from(&Natural::from(123u32)), Ok(123));
/// assert_eq!(
///     i8::try_from(&Natural::from(10u32).pow(12)),
///     Err(SignedFromNaturalError)
/// );
/// assert_eq!(i64::try_from(&Natural::from(123u32)), Ok(123));
/// assert_eq!(
///     i64::try_from(&(Natural::ONE << 100)),
///     Err(SignedFromNaturalError)
/// );
/// ```
///
/// # wrapping_from
/// ```
/// use malachite_base::num::arithmetic::traits::Pow;
/// use malachite_base::num::basic::traits::One;
/// use malachite_base::num::conversion::traits::WrappingFrom;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(u32::wrapping_from(&Natural::from(123u32)), 123);
/// assert_eq!(
///     u32::wrapping_from(&Natural::from(10u32).pow(12)),
///     3567587328
/// );
/// assert_eq!(u8::wrapping_from(&Natural::from(123u32)), 123);
/// assert_eq!(u8::wrapping_from(&Natural::from(10u32).pow(12)), 0);
/// assert_eq!(u64::wrapping_from(&Natural::from(123u32)), 123);
/// assert_eq!(u64::wrapping_from(&(Natural::ONE << 100)), 0);
///
/// assert_eq!(i32::wrapping_from(&Natural::from(123u32)), 123);
/// assert_eq!(
///     i32::wrapping_from(&Natural::from(10u32).pow(12)),
///     -727379968
/// );
/// assert_eq!(i8::wrapping_from(&Natural::from(123u32)), 123);
/// assert_eq!(i8::wrapping_from(&Natural::from(10u32).pow(12)), 0);
/// assert_eq!(i64::wrapping_from(&Natural::from(123u32)), 123);
/// assert_eq!(i64::wrapping_from(&(Natural::ONE << 100)), 0);
/// ```
///
/// # saturating_from
/// ```
/// use malachite_base::num::arithmetic::traits::Pow;
/// use malachite_base::num::basic::traits::One;
/// use malachite_base::num::conversion::traits::SaturatingFrom;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(u32::saturating_from(&Natural::from(123u32)), 123);
/// assert_eq!(
///     u32::saturating_from(&Natural::from(10u32).pow(12)),
///     u32::MAX
/// );
/// assert_eq!(u8::saturating_from(&Natural::from(123u32)), 123);
/// assert_eq!(u8::saturating_from(&Natural::from(10u32).pow(12)), 255);
/// assert_eq!(u64::saturating_from(&Natural::from(123u32)), 123);
/// assert_eq!(
///     u64::saturating_from(&(Natural::ONE << 100)),
///     18446744073709551615
/// );
///
/// assert_eq!(i32::saturating_from(&Natural::from(123u32)), 123);
/// assert_eq!(
///     i32::saturating_from(&Natural::from(10u32).pow(12)),
///     2147483647
/// );
/// assert_eq!(i8::saturating_from(&Natural::from(123u32)), 123);
/// assert_eq!(i8::saturating_from(&Natural::from(10u32).pow(12)), 127);
/// assert_eq!(i64::saturating_from(&Natural::from(123u32)), 123);
/// assert_eq!(
///     i64::saturating_from(&(Natural::ONE << 100)),
///     9223372036854775807
/// );
/// ```
///
/// # overflowing_from
/// ```
/// use malachite_base::num::arithmetic::traits::Pow;
/// use malachite_base::num::basic::traits::One;
/// use malachite_base::num::conversion::traits::OverflowingFrom;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(u32::overflowing_from(&Natural::from(123u32)), (123, false));
/// assert_eq!(
///     u32::overflowing_from(&Natural::from(10u32).pow(12)),
///     (3567587328, true)
/// );
/// assert_eq!(u8::overflowing_from(&Natural::from(123u32)), (123, false));
/// assert_eq!(
///     u8::overflowing_from(&Natural::from(10u32).pow(12)),
///     (0, true)
/// );
/// assert_eq!(u64::overflowing_from(&Natural::from(123u32)), (123, false));
/// assert_eq!(u64::overflowing_from(&(Natural::ONE << 100)), (0, true));
///
/// assert_eq!(i32::overflowing_from(&Natural::from(123u32)), (123, false));
/// assert_eq!(
///     i32::overflowing_from(&Natural::from(10u32).pow(12)),
///     (-727379968, true)
/// );
/// assert_eq!(i8::overflowing_from(&Natural::from(123u32)), (123, false));
/// assert_eq!(
///     i8::overflowing_from(&Natural::from(10u32).pow(12)),
///     (0, true)
/// );
/// assert_eq!(i64::overflowing_from(&Natural::from(123u32)), (123, false));
/// assert_eq!(i64::overflowing_from(&(Natural::ONE << 100)), (0, true));
/// ```
///
/// # convertible_from
/// ```
/// use malachite_base::num::arithmetic::traits::Pow;
/// use malachite_base::num::basic::traits::One;
/// use malachite_base::num::conversion::traits::ConvertibleFrom;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(u32::convertible_from(&Natural::from(123u32)), true);
/// assert_eq!(u32::convertible_from(&Natural::from(10u32).pow(12)), false);
/// assert_eq!(u8::convertible_from(&Natural::from(123u32)), true);
/// assert_eq!(u8::convertible_from(&Natural::from(10u32).pow(12)), false);
/// assert_eq!(u64::convertible_from(&Natural::from(123u32)), true);
/// assert_eq!(u64::convertible_from(&(Natural::ONE << 100)), false);
///
/// assert_eq!(i32::convertible_from(&Natural::from(123u32)), true);
/// assert_eq!(i32::convertible_from(&Natural::from(10u32).pow(12)), false);
/// assert_eq!(i8::convertible_from(&Natural::from(123u32)), true);
/// assert_eq!(i8::convertible_from(&Natural::from(10u32).pow(12)), false);
/// assert_eq!(i64::convertible_from(&Natural::from(123u32)), true);
/// assert_eq!(i64::convertible_from(&(Natural::ONE << 100)), false);
/// ```
pub mod primitive_int_from_natural;
/// Implementations of traits for conversions between Python integers and
/// [`Natural`](crate::natural::Natural)s using [pyo3](https://pyo3.rs/).
pub mod pyo3;
/// Implementations of traits for serialization and deserialization using
/// [serde](https://serde.rs/).
pub mod serde;
/// Implementations of traits for converting [`Natural`](crate::natural::Natural)s to and from
/// [`String`]s.
pub mod string;
/// Functions for extracting [`Limb`](crate#limbs)s from a [`Natural`](crate::natural::Natural).
pub mod to_limbs;
