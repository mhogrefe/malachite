// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of traits for converting a value that a [`Integer`](crate::integer::Integer) can
/// be converted from into a constant [`IntegerPolynomial`](super::IntegerPolynomial).
pub mod from_integer;
/// Implementations of traits for converting a
/// [`NaturalPolynomial`](crate::natural_polynomial::NaturalPolynomial) to an
/// [`IntegerPolynomial`](super::IntegerPolynomial).
pub mod from_natural_polynomial;
/// Implementations of traits for converting a
/// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial) to an
/// [`IntegerPolynomial`](super::IntegerPolynomial).
pub mod from_unsigned_polynomial;
/// Implementations of traits for converting an [`IntegerPolynomial`](super::IntegerPolynomial) to a
/// [`NaturalPolynomial`](crate::natural_polynomial::NaturalPolynomial).
///
/// # try_from
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer_polynomial::IntegerPolynomial;
/// use malachite_nz::natural_polynomial::NaturalPolynomial;
///
/// let p = IntegerPolynomial::from_str("3*x^2+1000000000000000000000000").unwrap();
/// assert_eq!(
///     NaturalPolynomial::try_from(&p).unwrap().to_string(),
///     "3*x^2+1000000000000000000000000"
/// );
/// assert_eq!(
///     NaturalPolynomial::try_from(p).unwrap().to_string(),
///     "3*x^2+1000000000000000000000000"
/// );
///
/// let p = IntegerPolynomial::from_str("3*x^2-1").unwrap();
/// assert!(NaturalPolynomial::try_from(&p).is_err());
/// assert!(NaturalPolynomial::try_from(p).is_err());
///
/// assert_eq!(
///     NaturalPolynomial::try_from(IntegerPolynomial::ZERO),
///     Ok(NaturalPolynomial::ZERO)
/// );
/// ```
pub mod natural_polynomial_from_integer_polynomial;
/// Implementations of traits for serialization and deserialization using
/// [serde](https://serde.rs/).
pub mod serde;
/// Functions for converting an [`IntegerPolynomial`](super::IntegerPolynomial) to and from a
/// [`String`](alloc::string::String).
pub mod string;
/// Implementations of traits for converting an [`IntegerPolynomial`](super::IntegerPolynomial) to
/// an [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial).
///
/// # try_from
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
/// use malachite_nz::integer_polynomial::IntegerPolynomial;
///
/// let p = IntegerPolynomial::from_str("3*x^2+255").unwrap();
/// assert_eq!(
///     UnsignedPolynomial::<u8>::try_from(&p).unwrap().to_string(),
///     "3*x^2+255"
/// );
/// assert_eq!(
///     UnsignedPolynomial::<u64>::try_from(p).unwrap().to_string(),
///     "3*x^2+255"
/// );
///
/// // Too large for `u8`, or negative.
/// let p = IntegerPolynomial::from_str("3*x^2+256").unwrap();
/// assert!(UnsignedPolynomial::<u8>::try_from(&p).is_err());
/// assert_eq!(
///     UnsignedPolynomial::<u16>::try_from(&p).unwrap().to_string(),
///     "3*x^2+256"
/// );
/// let p = IntegerPolynomial::from_str("3*x^2-1").unwrap();
/// assert!(UnsignedPolynomial::<u64>::try_from(&p).is_err());
///
/// assert_eq!(
///     UnsignedPolynomial::<u32>::try_from(IntegerPolynomial::ZERO),
///     Ok(UnsignedPolynomial::<u32>::ZERO)
/// );
/// ```
pub mod unsigned_polynomial_from_integer_polynomial;
