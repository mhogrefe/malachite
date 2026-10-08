// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Functions for converting a [`Vec`](alloc::vec::Vec) or slice of
/// [`Natural`](crate::natural::Natural)s to a [`NaturalVector`](super::NaturalVector).
pub mod from_elements;
/// Implementations of traits for converting an
/// [`UnsignedVector`](malachite_base::unsigned_vector::UnsignedVector) to a
/// [`NaturalVector`](super::NaturalVector).
pub mod from_unsigned_vector;
/// Functions for converting a [`NaturalVector`](super::NaturalVector) to and from a [`String`].
pub mod string;
/// Functions for converting a [`NaturalVector`](super::NaturalVector) to a [`Vec`](alloc::vec::Vec)
/// or slice of [`Natural`](crate::natural::Natural)s.
pub mod to_elements;
/// Implementations of traits for converting a [`NaturalVector`](super::NaturalVector) to an
/// [`UnsignedVector`](malachite_base::unsigned_vector::UnsignedVector).
///
/// # try_from
/// ```
/// use core::str::FromStr;
/// use malachite_base::unsigned_vector::UnsignedVector;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// let v = NaturalVector::from_str("(3, 255)").unwrap();
/// assert_eq!(
///     UnsignedVector::<u8>::try_from(&v).unwrap().to_string(),
///     "(3, 255)"
/// );
/// assert_eq!(
///     UnsignedVector::<u64>::try_from(v).unwrap().to_string(),
///     "(3, 255)"
/// );
///
/// let v = NaturalVector::from_str("(3, 256)").unwrap();
/// assert!(UnsignedVector::<u8>::try_from(&v).is_err());
/// assert_eq!(
///     UnsignedVector::<u16>::try_from(&v).unwrap().to_string(),
///     "(3, 256)"
/// );
///
/// assert_eq!(
///     UnsignedVector::<u32>::try_from(NaturalVector::from_str("()").unwrap())
///         .unwrap()
///         .to_string(),
///     "()"
/// );
/// ```
///
/// # convertible_from
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::conversion::traits::ConvertibleFrom;
/// use malachite_base::unsigned_vector::UnsignedVector;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// assert_eq!(
///     UnsignedVector::<u8>::convertible_from(&NaturalVector::from_str("(3, 255)").unwrap()),
///     true
/// );
/// assert_eq!(
///     UnsignedVector::<u8>::convertible_from(&NaturalVector::from_str("(3, 256)").unwrap()),
///     false
/// );
/// assert_eq!(
///     UnsignedVector::<u16>::convertible_from(&NaturalVector::from_str("(3, 256)").unwrap()),
///     true
/// );
/// assert_eq!(
///     UnsignedVector::<u32>::convertible_from(&NaturalVector::from_str("()").unwrap()),
///     true
/// );
/// ```
pub mod unsigned_vector_from_natural_vector;
