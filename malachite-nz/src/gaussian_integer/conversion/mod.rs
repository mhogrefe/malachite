// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`From`] for converting values to a
/// [`GaussianInteger`](crate::gaussian_integer::GaussianInteger).
pub mod from;
/// Implementations of traits for converting a primitive float to a
/// [`GaussianInteger`](crate::gaussian_integer::GaussianInteger).
///
/// The traits are [`TryFrom`] and
/// [`ConvertibleFrom`](malachite_base::num::conversion::traits::ConvertibleFrom).
///
/// # try_from
/// ```
/// use malachite_nz::gaussian_integer::GaussianInteger;
/// use malachite_nz::gaussian_integer::conversion::from_primitive_float::*;
///
/// assert_eq!(
///     GaussianInteger::try_from(123.0f32).unwrap().to_string(),
///     "123"
/// );
/// assert_eq!(
///     GaussianInteger::try_from(-123.0f64).unwrap().to_string(),
///     "-123"
/// );
///
/// assert_eq!(
///     GaussianInteger::try_from(0.5f32),
///     Err(GaussianIntegerFromPrimitiveFloatError)
/// );
/// assert_eq!(
///     GaussianInteger::try_from(f64::NAN),
///     Err(GaussianIntegerFromPrimitiveFloatError)
/// );
/// assert_eq!(
///     GaussianInteger::try_from(f64::INFINITY),
///     Err(GaussianIntegerFromPrimitiveFloatError)
/// );
/// ```
///
/// # convertible_from
/// ```
/// use malachite_base::num::conversion::traits::ConvertibleFrom;
/// use malachite_nz::gaussian_integer::GaussianInteger;
///
/// assert_eq!(GaussianInteger::convertible_from(123.0f32), true);
/// assert_eq!(GaussianInteger::convertible_from(-123.0f64), true);
/// assert_eq!(GaussianInteger::convertible_from(0.5f32), false);
/// assert_eq!(GaussianInteger::convertible_from(f64::NAN), false);
/// assert_eq!(GaussianInteger::convertible_from(f64::INFINITY), false);
/// ```
pub mod from_primitive_float;
/// Implementations of [`ImaginaryFrom`](malachite_base::num::conversion::traits::ImaginaryFrom) for
/// converting values to purely imaginary
/// [`GaussianInteger`](crate::gaussian_integer::GaussianInteger)s.
pub mod imaginary_from;
/// Implementations of traits for converting a
/// [`GaussianInteger`](crate::gaussian_integer::GaussianInteger) to an
/// [`Integer`](crate::integer::Integer).
///
/// The traits are [`TryFrom`] and
/// [`ConvertibleFrom`](malachite_base::num::conversion::traits::ConvertibleFrom).
pub mod integer_from_gaussian_integer;
/// An implementation of
/// [`IsGaussianInteger`](malachite_base::num::conversion::traits::IsGaussianInteger), a trait for
/// determining whether a number is a Gaussian integer.
///
/// A [`GaussianInteger`](crate::gaussian_integer::GaussianInteger) is always a Gaussian integer.
pub mod is_gaussian_integer;
/// An implementation of [`IsInteger`](malachite_base::num::conversion::traits::IsInteger), a trait
/// for determining whether a number is an integer.
///
/// A [`GaussianInteger`](crate::gaussian_integer::GaussianInteger) is an integer if and only if its
/// imaginary part is zero.
pub mod is_integer;
/// An implementation of [`IsReal`](malachite_base::num::conversion::traits::IsReal), a trait for
/// determining whether a number is a real number.
///
/// A [`GaussianInteger`](crate::gaussian_integer::GaussianInteger) is a real number if and only if
/// its imaginary part is zero.
pub mod is_real;
/// Implementations of traits for converting a
/// [`GaussianInteger`](crate::gaussian_integer::GaussianInteger) to a
/// [`Natural`](crate::natural::Natural).
///
/// The traits are [`TryFrom`] and
/// [`ConvertibleFrom`](malachite_base::num::conversion::traits::ConvertibleFrom).
pub mod natural_from_gaussian_integer;
/// Implementations of traits for converting a
/// [`GaussianInteger`](crate::gaussian_integer::GaussianInteger) to a primitive float.
///
/// The traits are [`TryFrom`] and
/// [`ConvertibleFrom`](malachite_base::num::conversion::traits::ConvertibleFrom).
///
/// # try_from
/// ```
/// use malachite_nz::gaussian_integer::GaussianInteger;
/// use malachite_nz::gaussian_integer::conversion::primitive_float_from_gaussian_integer::*;
/// use std::str::FromStr;
///
/// let x = GaussianInteger::from_str("123").unwrap();
/// assert_eq!(f32::try_from(&x).unwrap(), 123.0);
///
/// let x = GaussianInteger::from_str("-123").unwrap();
/// assert_eq!(f32::try_from(&x).unwrap(), -123.0);
///
/// let x = GaussianInteger::from_str("16777217").unwrap();
/// assert_eq!(
///     f32::try_from(&x),
///     Err(PrimitiveFloatFromGaussianIntegerError)
/// );
/// assert_eq!(f64::try_from(&x).unwrap(), 16777217.0);
///
/// let x = GaussianInteger::from_str("2-3i").unwrap();
/// assert_eq!(
///     f32::try_from(&x),
///     Err(PrimitiveFloatFromGaussianIntegerError)
/// );
/// ```
///
/// # convertible_from
/// ```
/// use malachite_base::num::conversion::traits::ConvertibleFrom;
/// use malachite_nz::gaussian_integer::GaussianInteger;
/// use std::str::FromStr;
///
/// let x = GaussianInteger::from_str("123").unwrap();
/// assert_eq!(f32::convertible_from(&x), true);
///
/// let x = GaussianInteger::from_str("16777217").unwrap();
/// assert_eq!(f32::convertible_from(&x), false);
/// assert_eq!(f64::convertible_from(&x), true);
///
/// let x = GaussianInteger::from_str("2-3i").unwrap();
/// assert_eq!(f32::convertible_from(&x), false);
/// ```
pub mod primitive_float_from_gaussian_integer;
/// Implementations of traits for converting a
/// [`GaussianInteger`](crate::gaussian_integer::GaussianInteger) to a primitive integer.
///
/// The traits are [`TryFrom`] and
/// [`ConvertibleFrom`](malachite_base::num::conversion::traits::ConvertibleFrom).
///
/// # try_from
/// ```
/// use malachite_nz::gaussian_integer::GaussianInteger;
/// use malachite_nz::gaussian_integer::conversion::primitive_int_from_gaussian_integer::*;
/// use std::str::FromStr;
///
/// let x = GaussianInteger::from_str("123").unwrap();
/// assert_eq!(u32::try_from(&x).unwrap(), 123);
/// assert_eq!(i32::try_from(&x).unwrap(), 123);
///
/// let x = GaussianInteger::from_str("-123").unwrap();
/// assert_eq!(u32::try_from(&x), Err(PrimitiveIntFromGaussianIntegerError));
/// assert_eq!(i32::try_from(&x).unwrap(), -123);
///
/// let x = GaussianInteger::from_str("1000000000000").unwrap();
/// assert_eq!(u32::try_from(&x), Err(PrimitiveIntFromGaussianIntegerError));
/// assert_eq!(u64::try_from(&x).unwrap(), 1000000000000);
///
/// let x = GaussianInteger::from_str("2-3i").unwrap();
/// assert_eq!(u32::try_from(&x), Err(PrimitiveIntFromGaussianIntegerError));
/// assert_eq!(i32::try_from(&x), Err(PrimitiveIntFromGaussianIntegerError));
/// ```
///
/// # convertible_from
/// ```
/// use malachite_base::num::conversion::traits::ConvertibleFrom;
/// use malachite_nz::gaussian_integer::GaussianInteger;
/// use std::str::FromStr;
///
/// let x = GaussianInteger::from_str("123").unwrap();
/// assert_eq!(u32::convertible_from(&x), true);
/// assert_eq!(i32::convertible_from(&x), true);
///
/// let x = GaussianInteger::from_str("-123").unwrap();
/// assert_eq!(u32::convertible_from(&x), false);
/// assert_eq!(i32::convertible_from(&x), true);
///
/// let x = GaussianInteger::from_str("1000000000000").unwrap();
/// assert_eq!(u32::convertible_from(&x), false);
/// assert_eq!(u64::convertible_from(&x), true);
///
/// let x = GaussianInteger::from_str("2-3i").unwrap();
/// assert_eq!(u32::convertible_from(&x), false);
/// assert_eq!(i32::convertible_from(&x), false);
/// ```
pub mod primitive_int_from_gaussian_integer;
/// Conversions to and from strings.
pub mod string;
