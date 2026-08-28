// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`From`] for converting values to a
/// [`GaussianRational`](crate::gaussian_rational::GaussianRational).
pub mod from;
/// Implementations of traits for converting a primitive float to a
/// [`GaussianRational`](crate::gaussian_rational::GaussianRational).
///
/// The traits are [`TryFrom`] and
/// [`ConvertibleFrom`](malachite_base::num::conversion::traits::ConvertibleFrom).
///
/// # try_from
/// ```
/// use malachite_q::gaussian_rational::conversion::from_primitive_float::*;
/// use malachite_q::gaussian_rational::GaussianRational;
///
/// assert_eq!(GaussianRational::try_from(123.0f32).unwrap().to_string(), "123");
/// assert_eq!(GaussianRational::try_from(0.5f64).unwrap().to_string(), "1/2");
/// assert_eq!(
///     GaussianRational::try_from(-0.1f32).unwrap().to_string(),
///     "-13421773/134217728"
/// );
///
/// assert_eq!(
///     GaussianRational::try_from(f64::NAN),
///     Err(GaussianRationalFromPrimitiveFloatError)
/// );
/// assert_eq!(
///     GaussianRational::try_from(f64::INFINITY),
///     Err(GaussianRationalFromPrimitiveFloatError)
/// );
/// ```
///
/// # convertible_from
/// ```
/// use malachite_base::num::conversion::traits::ConvertibleFrom;
/// use malachite_q::gaussian_rational::GaussianRational;
///
/// assert_eq!(GaussianRational::convertible_from(123.0f32), true);
/// assert_eq!(GaussianRational::convertible_from(0.5f64), true);
/// assert_eq!(GaussianRational::convertible_from(f64::NAN), false);
/// assert_eq!(GaussianRational::convertible_from(f64::INFINITY), false);
/// ```
pub mod from_primitive_float;
/// Implementations of traits for converting a
/// [`GaussianRational`](crate::gaussian_rational::GaussianRational) to a
/// [`GaussianInteger`](malachite_nz::gaussian_integer::GaussianInteger).
///
/// The traits are [`TryFrom`] and
/// [`ConvertibleFrom`](malachite_base::num::conversion::traits::ConvertibleFrom).
pub mod gaussian_integer_from_gaussian_rational;
/// Implementations of [`ImaginaryFrom`](malachite_base::num::conversion::traits::ImaginaryFrom) for
/// converting values to purely imaginary
/// [`GaussianRational`](crate::gaussian_rational::GaussianRational)s.
pub mod imaginary_from;
/// Implementations of traits for converting a
/// [`GaussianRational`](crate::gaussian_rational::GaussianRational) to an
/// [`Integer`](malachite_nz::integer::Integer).
///
/// The traits are [`TryFrom`] and
/// [`ConvertibleFrom`](malachite_base::num::conversion::traits::ConvertibleFrom).
pub mod integer_from_gaussian_rational;
/// An implementation of
/// [`IsGaussianInteger`](malachite_base::num::conversion::traits::IsGaussianInteger), a trait for
/// determining whether a number is a Gaussian integer.
///
/// A [`GaussianRational`](crate::gaussian_rational::GaussianRational) is a Gaussian integer if and
/// only if its real and imaginary parts are both integers.
pub mod is_gaussian_integer;
/// An implementation of [`IsInteger`](malachite_base::num::conversion::traits::IsInteger), a trait
/// for determining whether a number is an integer.
///
/// A [`GaussianRational`](crate::gaussian_rational::GaussianRational) is an integer if and only if
/// its imaginary part is zero and its real part is an integer.
pub mod is_integer;
/// An implementation of [`IsReal`](malachite_base::num::conversion::traits::IsReal), a trait for
/// determining whether a number is a real number.
///
/// A [`GaussianRational`](crate::gaussian_rational::GaussianRational) is a real number if and only
/// if its imaginary part is zero.
pub mod is_real;
/// Implementations of traits for converting a
/// [`GaussianRational`](crate::gaussian_rational::GaussianRational) to a
/// [`Natural`](malachite_nz::natural::Natural).
///
/// The traits are [`TryFrom`] and
/// [`ConvertibleFrom`](malachite_base::num::conversion::traits::ConvertibleFrom).
pub mod natural_from_gaussian_rational;
/// Implementations of traits for converting a
/// [`GaussianRational`](crate::gaussian_rational::GaussianRational) to a primitive float.
///
/// The traits are [`TryFrom`] and
/// [`ConvertibleFrom`](malachite_base::num::conversion::traits::ConvertibleFrom).
///
/// # try_from
/// ```
/// use malachite_q::gaussian_rational::conversion::primitive_float_from_gaussian_rational::*;
/// use malachite_q::gaussian_rational::GaussianRational;
/// use std::str::FromStr;
///
/// let x = GaussianRational::from_str("123").unwrap();
/// assert_eq!(f32::try_from(&x).unwrap(), 123.0);
///
/// let x = GaussianRational::from_str("1/2").unwrap();
/// assert_eq!(f32::try_from(&x).unwrap(), 0.5);
///
/// let x = GaussianRational::from_str("1/3").unwrap();
/// assert_eq!(
///     f32::try_from(&x),
///     Err(PrimitiveFloatFromGaussianRationalError)
/// );
///
/// let x = GaussianRational::from_str("2-3i").unwrap();
/// assert_eq!(
///     f32::try_from(&x),
///     Err(PrimitiveFloatFromGaussianRationalError)
/// );
/// ```
///
/// # convertible_from
/// ```
/// use malachite_base::num::conversion::traits::ConvertibleFrom;
/// use malachite_q::gaussian_rational::GaussianRational;
/// use std::str::FromStr;
///
/// let x = GaussianRational::from_str("123").unwrap();
/// assert_eq!(f32::convertible_from(&x), true);
///
/// let x = GaussianRational::from_str("1/2").unwrap();
/// assert_eq!(f32::convertible_from(&x), true);
///
/// let x = GaussianRational::from_str("1/3").unwrap();
/// assert_eq!(f32::convertible_from(&x), false);
///
/// let x = GaussianRational::from_str("2-3i").unwrap();
/// assert_eq!(f32::convertible_from(&x), false);
/// ```
pub mod primitive_float_from_gaussian_rational;
/// Implementations of traits for converting a
/// [`GaussianRational`](crate::gaussian_rational::GaussianRational) to a primitive integer.
///
/// The traits are [`TryFrom`] and
/// [`ConvertibleFrom`](malachite_base::num::conversion::traits::ConvertibleFrom).
///
/// # try_from
/// ```
/// use malachite_q::gaussian_rational::conversion::primitive_int_from_gaussian_rational::*;
/// use malachite_q::gaussian_rational::GaussianRational;
/// use std::str::FromStr;
///
/// let x = GaussianRational::from_str("123").unwrap();
/// assert_eq!(u32::try_from(&x).unwrap(), 123);
/// assert_eq!(i32::try_from(&x).unwrap(), 123);
///
/// let x = GaussianRational::from_str("-123").unwrap();
/// assert_eq!(u32::try_from(&x), Err(PrimitiveIntFromGaussianRationalError));
/// assert_eq!(i32::try_from(&x).unwrap(), -123);
///
/// let x = GaussianRational::from_str("22/7").unwrap();
/// assert_eq!(u32::try_from(&x), Err(PrimitiveIntFromGaussianRationalError));
///
/// let x = GaussianRational::from_str("1000000000000").unwrap();
/// assert_eq!(u32::try_from(&x), Err(PrimitiveIntFromGaussianRationalError));
/// assert_eq!(u64::try_from(&x).unwrap(), 1000000000000);
///
/// let x = GaussianRational::from_str("2-3i").unwrap();
/// assert_eq!(u32::try_from(&x), Err(PrimitiveIntFromGaussianRationalError));
/// assert_eq!(i32::try_from(&x), Err(PrimitiveIntFromGaussianRationalError));
/// ```
///
/// # convertible_from
/// ```
/// use malachite_base::num::conversion::traits::ConvertibleFrom;
/// use malachite_q::gaussian_rational::GaussianRational;
/// use std::str::FromStr;
///
/// let x = GaussianRational::from_str("123").unwrap();
/// assert_eq!(u32::convertible_from(&x), true);
/// assert_eq!(i32::convertible_from(&x), true);
///
/// let x = GaussianRational::from_str("-123").unwrap();
/// assert_eq!(u32::convertible_from(&x), false);
/// assert_eq!(i32::convertible_from(&x), true);
///
/// let x = GaussianRational::from_str("22/7").unwrap();
/// assert_eq!(u32::convertible_from(&x), false);
///
/// let x = GaussianRational::from_str("1000000000000").unwrap();
/// assert_eq!(u32::convertible_from(&x), false);
/// assert_eq!(u64::convertible_from(&x), true);
///
/// let x = GaussianRational::from_str("2-3i").unwrap();
/// assert_eq!(u32::convertible_from(&x), false);
/// assert_eq!(i32::convertible_from(&x), false);
/// ```
pub mod primitive_int_from_gaussian_rational;
/// Implementations of traits for converting a
/// [`GaussianRational`](crate::gaussian_rational::GaussianRational) to a
/// [`Rational`](crate::Rational).
///
/// The traits are [`TryFrom`] and
/// [`ConvertibleFrom`](malachite_base::num::conversion::traits::ConvertibleFrom).
pub mod rational_from_gaussian_rational;
/// Conversions to and from strings.
pub mod string;
