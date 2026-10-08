// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Functions for converting a [`Vec`](alloc::vec::Vec) or slice of [`Rational`](crate::Rational)s
/// to a [`RationalVector`](super::RationalVector).
pub mod from_elements;
/// Implementations of traits for converting an
/// [`IntegerVector`](malachite_nz::integer_vector::IntegerVector) to a
/// [`RationalVector`](super::RationalVector).
pub mod from_integer_vector;
/// Implementations of traits for converting a
/// [`NaturalVector`](malachite_nz::natural_vector::NaturalVector) to a
/// [`RationalVector`](super::RationalVector).
pub mod from_natural_vector;
/// A function for building a [`RationalVector`](super::RationalVector) from a vector of numerators
/// and a single denominator.
pub mod from_numerators_and_denominator;
/// Implementations of traits for converting an
/// [`UnsignedVector`](malachite_base::unsigned_vector::UnsignedVector) to a
/// [`RationalVector`](super::RationalVector).
pub mod from_unsigned_vector;
/// Implementations of traits for converting a [`RationalVector`](super::RationalVector) to an
/// [`IntegerVector`](malachite_nz::integer_vector::IntegerVector).
///
/// # try_from
/// ```
/// use core::str::FromStr;
/// use malachite_nz::integer_vector::IntegerVector;
/// use malachite_q::rational_vector::RationalVector;
///
/// let v = RationalVector::from_str("(4/2, -3)").unwrap();
/// assert_eq!(IntegerVector::try_from(&v).unwrap().to_string(), "(2, -3)");
/// assert_eq!(IntegerVector::try_from(v).unwrap().to_string(), "(2, -3)");
///
/// let v = RationalVector::from_str("(3, 1/2)").unwrap();
/// assert!(IntegerVector::try_from(&v).is_err());
/// assert!(IntegerVector::try_from(v).is_err());
/// ```
///
/// # convertible_from
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::conversion::traits::ConvertibleFrom;
/// use malachite_nz::integer_vector::IntegerVector;
/// use malachite_q::rational_vector::RationalVector;
///
/// assert_eq!(
///     IntegerVector::convertible_from(&RationalVector::from_str("(4/2, -3)").unwrap()),
///     true
/// );
/// assert_eq!(
///     IntegerVector::convertible_from(&RationalVector::from_str("(3, 1/2)").unwrap()),
///     false
/// );
/// assert_eq!(
///     IntegerVector::convertible_from(&RationalVector::from_str("()").unwrap()),
///     true
/// );
/// ```
pub mod integer_vector_from_rational_vector;
/// Implementations of traits for converting a [`RationalVector`](super::RationalVector) to a
/// [`NaturalVector`](malachite_nz::natural_vector::NaturalVector).
///
/// # try_from
/// ```
/// use core::str::FromStr;
/// use malachite_nz::natural_vector::NaturalVector;
/// use malachite_q::rational_vector::RationalVector;
///
/// let v = RationalVector::from_str("(4/2, 3)").unwrap();
/// assert_eq!(NaturalVector::try_from(&v).unwrap().to_string(), "(2, 3)");
/// assert_eq!(NaturalVector::try_from(v).unwrap().to_string(), "(2, 3)");
///
/// // A negative element never fits.
/// let v = RationalVector::from_str("(3, -1)").unwrap();
/// assert!(NaturalVector::try_from(&v).is_err());
///
/// let v = RationalVector::from_str("(3, 1/2)").unwrap();
/// assert!(NaturalVector::try_from(&v).is_err());
/// assert!(NaturalVector::try_from(v).is_err());
/// ```
///
/// # convertible_from
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::conversion::traits::ConvertibleFrom;
/// use malachite_nz::natural_vector::NaturalVector;
/// use malachite_q::rational_vector::RationalVector;
///
/// assert_eq!(
///     NaturalVector::convertible_from(&RationalVector::from_str("(4/2, 3)").unwrap()),
///     true
/// );
/// assert_eq!(
///     NaturalVector::convertible_from(&RationalVector::from_str("(3, -1)").unwrap()),
///     false
/// );
/// assert_eq!(
///     NaturalVector::convertible_from(&RationalVector::from_str("(3, 1/2)").unwrap()),
///     false
/// );
/// assert_eq!(
///     NaturalVector::convertible_from(&RationalVector::from_str("()").unwrap()),
///     true
/// );
/// ```
pub mod natural_vector_from_rational_vector;
/// Functions for converting a [`RationalVector`](super::RationalVector) to and from a [`String`].
pub mod string;
/// Functions for converting a [`RationalVector`](super::RationalVector) to a
/// [`Vec`](alloc::vec::Vec) or slice of [`Rational`](crate::Rational)s.
pub mod to_elements;
/// A function for clearing the denominators of a [`RationalVector`](super::RationalVector), giving
/// a vector of numerators and a single denominator.
pub mod to_numerators_and_denominator;
/// Implementations of traits for converting a [`RationalVector`](super::RationalVector) to an
/// [`UnsignedVector`](malachite_base::unsigned_vector::UnsignedVector).
///
/// # try_from
/// ```
/// use core::str::FromStr;
/// use malachite_base::unsigned_vector::UnsignedVector;
/// use malachite_q::rational_vector::RationalVector;
///
/// let v = RationalVector::from_str("(3, 510/2)").unwrap();
/// assert_eq!(
///     UnsignedVector::<u8>::try_from(&v).unwrap().to_string(),
///     "(3, 255)"
/// );
/// assert_eq!(
///     UnsignedVector::<u8>::try_from(v).unwrap().to_string(),
///     "(3, 255)"
/// );
///
/// let v = RationalVector::from_str("(3, 256)").unwrap();
/// assert!(UnsignedVector::<u8>::try_from(&v).is_err());
/// assert_eq!(
///     UnsignedVector::<u16>::try_from(&v).unwrap().to_string(),
///     "(3, 256)"
/// );
///
/// // A negative element never fits.
/// let v = RationalVector::from_str("(3, -1)").unwrap();
/// assert!(UnsignedVector::<u64>::try_from(&v).is_err());
///
/// let v = RationalVector::from_str("(3, 1/2)").unwrap();
/// assert!(UnsignedVector::<u8>::try_from(&v).is_err());
/// assert!(UnsignedVector::<u8>::try_from(v).is_err());
/// ```
///
/// # convertible_from
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::conversion::traits::ConvertibleFrom;
/// use malachite_base::unsigned_vector::UnsignedVector;
/// use malachite_q::rational_vector::RationalVector;
///
/// assert_eq!(
///     UnsignedVector::<u8>::convertible_from(&RationalVector::from_str("(3, 255)").unwrap()),
///     true
/// );
/// assert_eq!(
///     UnsignedVector::<u8>::convertible_from(&RationalVector::from_str("(3, 256)").unwrap()),
///     false
/// );
/// assert_eq!(
///     UnsignedVector::<u8>::convertible_from(&RationalVector::from_str("(3, -1)").unwrap()),
///     false
/// );
/// assert_eq!(
///     UnsignedVector::<u8>::convertible_from(&RationalVector::from_str("(3, 1/2)").unwrap()),
///     false
/// );
/// assert_eq!(
///     UnsignedVector::<u8>::convertible_from(&RationalVector::from_str("()").unwrap()),
///     true
/// );
/// ```
pub mod unsigned_vector_from_rational_vector;
