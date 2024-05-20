// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// An implementation of the [`From`] trait for converting a [`bool`] to an
/// [`Integer`](crate::integer::Integer).
pub mod from_bool;
/// Functions for converting a [`Natural`](crate::natural::Natural) to an
/// [`Integer`](crate::integer::Integer), and an implementation of the [`From`] trait.
pub mod from_natural;
/// Implementations of traits for converting a primitive float to an
/// [`Integer`](crate::integer::Integer).
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
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     Integer::rounding_from(0.0, Exact).to_debug_string(),
///     "(0, Equal)"
/// );
/// assert_eq!(
///     Integer::rounding_from(-0.0, Exact).to_debug_string(),
///     "(0, Equal)"
/// );
/// assert_eq!(
///     Integer::rounding_from(123.0, Exact).to_debug_string(),
///     "(123, Equal)"
/// );
/// assert_eq!(
///     Integer::rounding_from(1.0e9, Exact).to_debug_string(),
///     "(1000000000, Equal)"
/// );
/// assert_eq!(
///     Integer::rounding_from(1.0e9, Exact).to_debug_string(),
///     "(1000000000, Equal)"
/// );
/// assert_eq!(
///     Integer::rounding_from(4294967295.0, Exact).to_debug_string(),
///     "(4294967295, Equal)"
/// );
/// assert_eq!(
///     Integer::rounding_from(4294967296.0, Exact).to_debug_string(),
///     "(4294967296, Equal)"
/// );
/// assert_eq!(
///     Integer::rounding_from(1.0e100, Exact).to_debug_string(),
///     "(1000000000000000015902891109759918046836080856394528138978132755774783877217038106081346\
///     9985856815104, Equal)"
/// );
/// assert_eq!(
///     Integer::rounding_from(123.1, Floor).to_debug_string(),
///     "(123, Less)"
/// );
/// assert_eq!(
///     Integer::rounding_from(123.1, Ceiling).to_debug_string(),
///     "(124, Greater)"
/// );
/// assert_eq!(
///     Integer::rounding_from(123.1, Nearest).to_debug_string(),
///     "(123, Less)"
/// );
/// assert_eq!(
///     Integer::rounding_from(123.9, Floor).to_debug_string(),
///     "(123, Less)"
/// );
/// assert_eq!(
///     Integer::rounding_from(123.9, Ceiling).to_debug_string(),
///     "(124, Greater)"
/// );
/// assert_eq!(
///     Integer::rounding_from(123.9, Nearest).to_debug_string(),
///     "(124, Greater)"
/// );
/// assert_eq!(
///     Integer::rounding_from(123.5, Nearest).to_debug_string(),
///     "(124, Greater)"
/// );
/// assert_eq!(
///     Integer::rounding_from(124.5, Nearest).to_debug_string(),
///     "(124, Less)"
/// );
/// assert_eq!(
///     Integer::rounding_from(-0.99, Ceiling).to_debug_string(),
///     "(0, Greater)"
/// );
/// assert_eq!(
///     Integer::rounding_from(-0.499, Nearest).to_debug_string(),
///     "(0, Greater)"
/// );
/// assert_eq!(
///     Integer::rounding_from(-0.5, Nearest).to_debug_string(),
///     "(0, Greater)"
/// );
/// ```
///
/// # try_from
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::strings::ToDebugString;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     Integer::try_from(f64::NAN).to_debug_string(),
///     "Err(FloatInfiniteOrNan)"
/// );
/// assert_eq!(
///     Integer::try_from(f64::INFINITY).to_debug_string(),
///     "Err(FloatInfiniteOrNan)"
/// );
/// assert_eq!(
///     Integer::try_from(f64::NEGATIVE_INFINITY).to_debug_string(),
///     "Err(FloatInfiniteOrNan)"
/// );
/// assert_eq!(Integer::try_from(0.0).to_debug_string(), "Ok(0)");
/// assert_eq!(Integer::try_from(-0.0).to_debug_string(), "Ok(0)");
/// assert_eq!(Integer::try_from(123.0).to_debug_string(), "Ok(123)");
/// assert_eq!(Integer::try_from(-123.0).to_debug_string(), "Ok(-123)");
/// assert_eq!(Integer::try_from(1.0e9).to_debug_string(), "Ok(1000000000)");
/// assert_eq!(
///     Integer::try_from(4294967295.0).to_debug_string(),
///     "Ok(4294967295)"
/// );
/// assert_eq!(
///     Integer::try_from(4294967296.0).to_debug_string(),
///     "Ok(4294967296)"
/// );
/// assert_eq!(
///     Integer::try_from(1.0e100).to_debug_string(),
///     "Ok(10000000000000000159028911097599180468360808563945281389781327557747838772170381060813\
///     469985856815104)"
/// );
/// assert_eq!(
///     Integer::try_from(123.1).to_debug_string(),
///     "Err(FloatNonIntegerOrOutOfRange)"
/// );
/// assert_eq!(
///     Integer::try_from(123.9).to_debug_string(),
///     "Err(FloatNonIntegerOrOutOfRange)"
/// );
/// assert_eq!(
///     Integer::try_from(123.5).to_debug_string(),
///     "Err(FloatNonIntegerOrOutOfRange)"
/// );
/// assert_eq!(
///     Integer::try_from(124.5).to_debug_string(),
///     "Err(FloatNonIntegerOrOutOfRange)"
/// );
/// assert_eq!(
///     Integer::try_from(-0.499).to_debug_string(),
///     "Err(FloatNonIntegerOrOutOfRange)"
/// );
/// assert_eq!(
///     Integer::try_from(-0.5).to_debug_string(),
///     "Err(FloatNonIntegerOrOutOfRange)"
/// );
/// ```
///
/// # convertible_from
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::conversion::traits::ConvertibleFrom;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(Integer::convertible_from(f64::NAN), false);
/// assert_eq!(Integer::convertible_from(f64::INFINITY), false);
/// assert_eq!(Integer::convertible_from(f64::NEGATIVE_INFINITY), false);
/// assert_eq!(Integer::convertible_from(0.0), true);
/// assert_eq!(Integer::convertible_from(-0.0), true);
/// assert_eq!(Integer::convertible_from(123.0), true);
/// assert_eq!(Integer::convertible_from(-123.0), true);
/// assert_eq!(Integer::convertible_from(1.0e9), true);
/// assert_eq!(Integer::convertible_from(4294967295.0), true);
/// assert_eq!(Integer::convertible_from(4294967296.0), true);
/// assert_eq!(Integer::convertible_from(1.0e100), true);
/// assert_eq!(Integer::convertible_from(123.1), false);
/// assert_eq!(Integer::convertible_from(123.9), false);
/// assert_eq!(Integer::convertible_from(123.5), false);
/// assert_eq!(Integer::convertible_from(124.5), false);
/// assert_eq!(Integer::convertible_from(-0.499), false);
/// assert_eq!(Integer::convertible_from(-0.5), false);
/// ```
pub mod from_primitive_float;
/// Implementations of traits for converting a primitive integer to an
/// [`Integer`](crate::integer::Integer).
///
/// The traits are [`From`], [`TryFrom`],
/// [`ConvertibleFrom`](malachite_base::num::conversion::traits::ConvertibleFrom), and
/// [`SaturatingFrom`](malachite_base::num::conversion::traits::SaturatingFrom).
///
/// # from
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(Integer::from(123u32).to_string(), "123");
/// assert_eq!(Integer::from(-123i32).to_string(), "-123");
/// ```
pub mod from_primitive_int;
/// Functions for constructing an [`Integer`](crate::integer::Integer) from two's complement
/// [`Limb`](crate#limbs)s.
pub mod from_twos_complement_limbs;
/// An implementation of [`IsInteger`](malachite_base::num::conversion::traits::IsInteger), a trait
/// for determining whether a number is an integer.
///
/// An [`Integer`](crate::integer::Integer) is always an integer.
pub mod is_integer;
/// Implementations of traits for converting an [`Integer`](crate::integer::Integer) to a
/// [`Natural`](crate::natural::Natural).
///
/// The traits are [`TryFrom`],
/// [`ConvertibleFrom`](malachite_base::num::conversion::traits::ConvertibleFrom), and
/// [`SaturatingFrom`](malachite_base::num::conversion::traits::SaturatingFrom).
pub mod natural_from_integer;
/// Implementations of traits for converting an [`Integer`](crate::integer::Integer) to a primitive
/// float.
///
/// The traits are [`TryFrom`]
/// [`ConvertibleFrom`](malachite_base::num::conversion::traits::ConvertibleFrom), and
/// [`RoundingFrom`](malachite_base::num::conversion::traits::RoundingFrom).
///
/// # rounding_from
/// ```
/// use core::cmp::Ordering::*;
/// use core::str::FromStr;
/// use malachite_base::num::conversion::traits::RoundingFrom;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     f32::rounding_from(&Integer::from_str("123").unwrap(), Exact),
///     (123.0, Equal)
/// );
/// assert_eq!(
///     f32::rounding_from(&Integer::from_str("1000000001").unwrap(), Floor),
///     (1.0e9, Less)
/// );
/// assert_eq!(
///     f32::rounding_from(&Integer::from_str("1000000001").unwrap(), Ceiling),
///     (1.00000006e9, Greater)
/// );
/// assert_eq!(
///     f32::rounding_from(&Integer::from_str("-1000000001").unwrap(), Floor),
///     (-1.00000006e9, Less)
/// );
/// assert_eq!(
///     f32::rounding_from(&Integer::from_str("-1000000001").unwrap(), Ceiling),
///     (-1.0e9, Greater)
/// );
/// assert_eq!(
///     f32::rounding_from(
///         &Integer::from_str("10000000000000000000000000000000000000000000000000000").unwrap(),
///         Nearest
///     ),
///     (3.4028235e38, Less)
/// );
/// ```
///
/// # try_from
/// ```
/// use core::str::FromStr;
/// use malachite_nz::integer::conversion::primitive_float_from_integer::*;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(f32::try_from(&Integer::from_str("123").unwrap()), Ok(123.0));
/// assert_eq!(
///     f32::try_from(&Integer::from_str("-1000000000").unwrap()),
///     Ok(-1.0e9)
/// );
/// assert_eq!(
///     f32::try_from(&Integer::from_str("1000000001").unwrap()),
///     Err(PrimitiveFloatFromIntegerError)
/// );
/// assert_eq!(
///     f32::try_from(
///         &Integer::from_str("-10000000000000000000000000000000000000000000000000000").unwrap()
///     ),
///     Err(PrimitiveFloatFromIntegerError)
/// );
/// ```
///
/// # convertible_from
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::conversion::traits::ConvertibleFrom;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     f32::convertible_from(&Integer::from_str("123").unwrap()),
///     true
/// );
/// assert_eq!(
///     f32::convertible_from(&Integer::from_str("-1000000000").unwrap()),
///     true
/// );
/// assert_eq!(
///     f32::convertible_from(&Integer::from_str("1000000001").unwrap()),
///     false
/// );
/// assert_eq!(
///     f32::convertible_from(
///         &Integer::from_str("-10000000000000000000000000000000000000000000000000000").unwrap()
///     ),
///     false
/// );
/// ```
pub mod primitive_float_from_integer;
/// Implementations of traits for converting an [`Integer`](crate::integer::Integer) to a primitive
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
/// use malachite_nz::integer::conversion::primitive_int_from_integer::{
///     SignedFromIntegerError, UnsignedFromIntegerError,
/// };
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(u32::try_from(&Integer::from(123)), Ok(123));
/// assert_eq!(
///     u32::try_from(&Integer::from(-123)),
///     Err(UnsignedFromIntegerError)
/// );
/// assert_eq!(
///     u32::try_from(&Integer::from(10u32).pow(12)),
///     Err(UnsignedFromIntegerError)
/// );
/// assert_eq!(
///     u32::try_from(&-Integer::from(10u32).pow(12)),
///     Err(UnsignedFromIntegerError)
/// );
///
/// assert_eq!(i32::try_from(&Integer::from(123)), Ok(123));
/// assert_eq!(i32::try_from(&Integer::from(-123)), Ok(-123));
/// assert_eq!(
///     i32::try_from(&Integer::from(10u32).pow(12)),
///     Err(SignedFromIntegerError)
/// );
/// assert_eq!(
///     i32::try_from(&-Integer::from(10u32).pow(12)),
///     Err(SignedFromIntegerError)
/// );
/// ```
///
/// # wrapping_from
/// ```
/// use malachite_base::num::arithmetic::traits::Pow;
/// use malachite_base::num::conversion::traits::WrappingFrom;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(u32::wrapping_from(&Integer::from(123)), 123);
/// assert_eq!(u32::wrapping_from(&Integer::from(-123)), 4294967173);
/// assert_eq!(
///     u32::wrapping_from(&Integer::from(10u32).pow(12)),
///     3567587328
/// );
/// assert_eq!(
///     u32::wrapping_from(&-Integer::from(10u32).pow(12)),
///     727379968
/// );
///
/// assert_eq!(i32::wrapping_from(&Integer::from(123)), 123);
/// assert_eq!(i32::wrapping_from(&Integer::from(-123)), -123);
/// assert_eq!(
///     i32::wrapping_from(&Integer::from(10u32).pow(12)),
///     -727379968
/// );
/// assert_eq!(
///     i32::wrapping_from(&-Integer::from(10u32).pow(12)),
///     727379968
/// );
/// ```
///
/// # saturating_from
/// ```
/// use malachite_base::num::arithmetic::traits::Pow;
/// use malachite_base::num::conversion::traits::SaturatingFrom;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(u32::saturating_from(&Integer::from(123)), 123);
/// assert_eq!(u32::saturating_from(&Integer::from(-123)), 0);
/// assert_eq!(
///     u32::saturating_from(&Integer::from(10u32).pow(12)),
///     u32::MAX
/// );
/// assert_eq!(u32::saturating_from(&-Integer::from(10u32).pow(12)), 0);
///
/// assert_eq!(i32::saturating_from(&Integer::from(123)), 123);
/// assert_eq!(i32::saturating_from(&Integer::from(-123)), -123);
/// assert_eq!(
///     i32::saturating_from(&Integer::from(10u32).pow(12)),
///     2147483647
/// );
/// assert_eq!(
///     i32::saturating_from(&-Integer::from(10u32).pow(12)),
///     -2147483648
/// );
/// ```
///
/// # overflowing_from
/// ```
/// use malachite_base::num::arithmetic::traits::Pow;
/// use malachite_base::num::conversion::traits::OverflowingFrom;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(u32::overflowing_from(&Integer::from(123)), (123, false));
/// assert_eq!(
///     u32::overflowing_from(&Integer::from(-123)),
///     (4294967173, true)
/// );
/// assert_eq!(
///     u32::overflowing_from(&Integer::from(10u32).pow(12)),
///     (3567587328, true)
/// );
/// assert_eq!(
///     u32::overflowing_from(&-Integer::from(10u32).pow(12)),
///     (727379968, true)
/// );
///
/// assert_eq!(i32::overflowing_from(&Integer::from(123)), (123, false));
/// assert_eq!(i32::overflowing_from(&Integer::from(-123)), (-123, false));
/// assert_eq!(
///     i32::overflowing_from(&Integer::from(10u32).pow(12)),
///     (-727379968, true)
/// );
/// assert_eq!(
///     i32::overflowing_from(&-Integer::from(10u32).pow(12)),
///     (727379968, true)
/// );
/// ```
///
/// # convertible_from
/// ```
/// use malachite_base::num::arithmetic::traits::Pow;
/// use malachite_base::num::conversion::traits::ConvertibleFrom;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(u32::convertible_from(&Integer::from(123)), true);
/// assert_eq!(u32::convertible_from(&Integer::from(-123)), false);
/// assert_eq!(u32::convertible_from(&Integer::from(10u32).pow(12)), false);
/// assert_eq!(u32::convertible_from(&-Integer::from(10u32).pow(12)), false);
///
/// assert_eq!(i32::convertible_from(&Integer::from(123)), true);
/// assert_eq!(i32::convertible_from(&Integer::from(-123)), true);
/// assert_eq!(i32::convertible_from(&Integer::from(10u32).pow(12)), false);
/// assert_eq!(i32::convertible_from(&-Integer::from(10u32).pow(12)), false);
/// ```
pub mod primitive_int_from_integer;
/// Implementations of traits for conversions between Python integers and
/// [`Integer`](crate::integer::Integer)s using [pyo3](https://pyo3.rs/).
pub mod pyo3;
/// Implementations of traits for serialization and deserialization using
/// [serde](https://serde.rs/).
pub mod serde;
/// Implementations of traits for converting [`Integer`](crate::integer::Integer)s to and from
/// [`String`]s.
pub mod string;
/// Functions for extracting two's complement [`Limb`](crate#limbs)s from an
/// [`Integer`](crate::integer::Integer).
pub mod to_twos_complement_limbs;
