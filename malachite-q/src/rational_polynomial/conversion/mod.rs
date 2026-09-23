// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of traits for converting an
/// [`IntegerPolynomial`](malachite_nz::integer_polynomial::IntegerPolynomial) to a
/// [`RationalPolynomial`](super::RationalPolynomial).
pub mod from_integer_polynomial;
/// Implementations of traits for converting a
/// [`NaturalPolynomial`](malachite_nz::natural_polynomial::NaturalPolynomial) to a
/// [`RationalPolynomial`](super::RationalPolynomial).
pub mod from_natural_polynomial;
/// Implementations of traits for converting a value that a [`Rational`](crate::Rational) can be
/// converted from into a constant [`RationalPolynomial`](super::RationalPolynomial).
pub mod from_rational;
/// Implementations of traits for converting a
/// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial) to a
/// [`RationalPolynomial`](super::RationalPolynomial).
pub mod from_unsigned_polynomial;
/// Implementations of traits for converting a [`RationalPolynomial`](super::RationalPolynomial) to
/// an [`IntegerPolynomial`](malachite_nz::integer_polynomial::IntegerPolynomial).
///
/// # try_from
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer_polynomial::IntegerPolynomial;
/// use malachite_q::rational_polynomial::RationalPolynomial;
///
/// let p = RationalPolynomial::from_str("-3*x^2+4/2").unwrap();
/// assert_eq!(
///     IntegerPolynomial::try_from(&p).unwrap().to_string(),
///     "-3*x^2+2"
/// );
/// assert_eq!(
///     IntegerPolynomial::try_from(p).unwrap().to_string(),
///     "-3*x^2+2"
/// );
///
/// let p = RationalPolynomial::from_str("x^2+1/2").unwrap();
/// assert!(IntegerPolynomial::try_from(&p).is_err());
/// assert!(IntegerPolynomial::try_from(p).is_err());
///
/// assert_eq!(
///     IntegerPolynomial::try_from(RationalPolynomial::ZERO),
///     Ok(IntegerPolynomial::ZERO)
/// );
/// ```
pub mod integer_polynomial_from_rational_polynomial;
/// Implementations of traits for converting a [`RationalPolynomial`](super::RationalPolynomial) to
/// a [`NaturalPolynomial`](malachite_nz::natural_polynomial::NaturalPolynomial).
///
/// # try_from
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::natural_polynomial::NaturalPolynomial;
/// use malachite_q::rational_polynomial::RationalPolynomial;
///
/// let p = RationalPolynomial::from_str("3*x^2+4/2").unwrap();
/// assert_eq!(
///     NaturalPolynomial::try_from(&p).unwrap().to_string(),
///     "3*x^2+2"
/// );
/// assert_eq!(
///     NaturalPolynomial::try_from(p).unwrap().to_string(),
///     "3*x^2+2"
/// );
///
/// // Not an integer, or negative.
/// assert!(NaturalPolynomial::try_from(RationalPolynomial::from_str("x^2+1/2").unwrap()).is_err());
/// assert!(NaturalPolynomial::try_from(RationalPolynomial::from_str("x^2-1").unwrap()).is_err());
///
/// assert_eq!(
///     NaturalPolynomial::try_from(RationalPolynomial::ZERO),
///     Ok(NaturalPolynomial::ZERO)
/// );
/// ```
pub mod natural_polynomial_from_rational_polynomial;
/// Implementations of traits for serialization and deserialization using
/// [serde](https://serde.rs/).
pub mod serde;
/// Functions for converting an [`RationalPolynomial`](super::RationalPolynomial) to and from a
/// [`String`](alloc::string::String).
pub mod string;
/// Implementations of traits for converting a [`RationalPolynomial`](super::RationalPolynomial) to
/// an [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial).
///
/// # try_from
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
/// use malachite_q::rational_polynomial::RationalPolynomial;
///
/// let p = RationalPolynomial::from_str("3*x^2+255").unwrap();
/// assert_eq!(
///     UnsignedPolynomial::<u8>::try_from(&p).unwrap().to_string(),
///     "3*x^2+255"
/// );
/// assert_eq!(
///     UnsignedPolynomial::<u64>::try_from(p).unwrap().to_string(),
///     "3*x^2+255"
/// );
///
/// // Too large for `u8`, negative, or not an integer.
/// let p = RationalPolynomial::from_str("3*x^2+256").unwrap();
/// assert!(UnsignedPolynomial::<u8>::try_from(&p).is_err());
/// assert!(UnsignedPolynomial::<u16>::try_from(&p).is_ok());
/// let p = RationalPolynomial::from_str("3*x^2-1").unwrap();
/// assert!(UnsignedPolynomial::<u64>::try_from(&p).is_err());
/// let p = RationalPolynomial::from_str("3*x^2+1/2").unwrap();
/// assert!(UnsignedPolynomial::<u64>::try_from(&p).is_err());
///
/// assert_eq!(
///     UnsignedPolynomial::<u32>::try_from(RationalPolynomial::ZERO),
///     Ok(UnsignedPolynomial::<u32>::ZERO)
/// );
/// ```
pub mod unsigned_polynomial_from_rational_polynomial;
