// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// An implementation of [`From`] for converting an
/// [`IntegerMatrix`](malachite_nz::integer_matrix::IntegerMatrix) to a
/// [`RationalMatrix`](super::RationalMatrix).
pub mod from_integer_matrix;
/// An implementation of [`From`] for converting a
/// [`NaturalMatrix`](malachite_nz::natural_matrix::NaturalMatrix) to a
/// [`RationalMatrix`](super::RationalMatrix).
pub mod from_natural_matrix;
/// A function for building a [`RationalMatrix`](super::RationalMatrix) from a matrix of numerators
/// and a single denominator.
pub mod from_numerators_and_denominator;
/// An implementation of [`From`] for converting an
/// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix) to a
/// [`RationalMatrix`](super::RationalMatrix).
pub mod from_unsigned_matrix;
/// Implementations of traits for converting a [`RationalMatrix`](super::RationalMatrix) to an
/// [`IntegerMatrix`](malachite_nz::integer_matrix::IntegerMatrix).
///
/// # try_from
/// ```
/// use core::str::FromStr;
/// use malachite_nz::integer_matrix::IntegerMatrix;
/// use malachite_q::rational_matrix::RationalMatrix;
///
/// let m = RationalMatrix::from_str("[3, -4; 0, 1]").unwrap();
/// assert_eq!(
///     IntegerMatrix::try_from(&m).unwrap().to_string(),
///     "[3, -4; 0, 1]"
/// );
/// assert_eq!(
///     IntegerMatrix::try_from(m).unwrap().to_string(),
///     "[3, -4; 0, 1]"
/// );
///
/// assert!(IntegerMatrix::try_from(RationalMatrix::from_str("[1/2]").unwrap()).is_err());
/// assert!(
///     IntegerMatrix::try_from(&RationalMatrix::from_str("[3, -4; 0, 1/3]").unwrap()).is_err()
/// );
///
/// assert_eq!(
///     IntegerMatrix::try_from(RationalMatrix::from_str("[]_{0,2}").unwrap())
///         .unwrap()
///         .to_string(),
///     "[]_{0,2}"
/// );
/// ```
///
/// # convertible_from
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::conversion::traits::ConvertibleFrom;
/// use malachite_nz::integer_matrix::IntegerMatrix;
/// use malachite_q::rational_matrix::RationalMatrix;
///
/// assert_eq!(
///     IntegerMatrix::convertible_from(&RationalMatrix::from_str("[3, -4; 0, 1]").unwrap()),
///     true
/// );
/// assert_eq!(
///     IntegerMatrix::convertible_from(&RationalMatrix::from_str("[1/2]").unwrap()),
///     false
/// );
/// assert_eq!(
///     IntegerMatrix::convertible_from(&RationalMatrix::from_str("[]_{0,2}").unwrap()),
///     true
/// );
/// ```
pub mod integer_matrix_from_rational_matrix;
/// Implementations of traits for converting a [`RationalMatrix`](super::RationalMatrix) to a
/// [`NaturalMatrix`](malachite_nz::natural_matrix::NaturalMatrix).
///
/// # try_from
/// ```
/// use core::str::FromStr;
/// use malachite_nz::natural_matrix::NaturalMatrix;
/// use malachite_q::rational_matrix::RationalMatrix;
///
/// let m = RationalMatrix::from_str("[3, 4; 0, 1]").unwrap();
/// assert_eq!(
///     NaturalMatrix::try_from(&m).unwrap().to_string(),
///     "[3, 4; 0, 1]"
/// );
/// assert_eq!(
///     NaturalMatrix::try_from(m).unwrap().to_string(),
///     "[3, 4; 0, 1]"
/// );
///
/// assert!(NaturalMatrix::try_from(RationalMatrix::from_str("[1/2]").unwrap()).is_err());
/// assert!(NaturalMatrix::try_from(&RationalMatrix::from_str("[3, -4; 0, 1]").unwrap()).is_err());
///
/// assert_eq!(
///     NaturalMatrix::try_from(RationalMatrix::from_str("[]_{2,0}").unwrap())
///         .unwrap()
///         .to_string(),
///     "[]_{2,0}"
/// );
/// ```
///
/// # convertible_from
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::conversion::traits::ConvertibleFrom;
/// use malachite_nz::natural_matrix::NaturalMatrix;
/// use malachite_q::rational_matrix::RationalMatrix;
///
/// assert_eq!(
///     NaturalMatrix::convertible_from(&RationalMatrix::from_str("[3, 4; 0, 1]").unwrap()),
///     true
/// );
/// assert_eq!(
///     NaturalMatrix::convertible_from(&RationalMatrix::from_str("[1/2]").unwrap()),
///     false
/// );
/// assert_eq!(
///     NaturalMatrix::convertible_from(&RationalMatrix::from_str("[]_{2,0}").unwrap()),
///     true
/// );
/// ```
pub mod natural_matrix_from_rational_matrix;
#[cfg(feature = "serde")]
/// Implementations of traits for serialization and deserialization using
/// [serde](https://serde.rs/).
pub mod serde;
/// Conversions between [`RationalMatrix`](super::RationalMatrix)s and strings.
pub mod string;
/// Functions for clearing the denominators of a [`RationalMatrix`](super::RationalMatrix), giving a
/// matrix of numerators and a single denominator.
pub mod to_numerators_and_denominator;
/// Implementations of traits for converting a [`RationalMatrix`](super::RationalMatrix) to an
/// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix).
///
/// # try_from
/// ```
/// use core::str::FromStr;
/// use malachite_base::unsigned_matrix::UnsignedMatrix;
/// use malachite_q::rational_matrix::RationalMatrix;
///
/// let m = RationalMatrix::from_str("[3, 255; 0, 1]").unwrap();
/// assert_eq!(
///     UnsignedMatrix::<u8>::try_from(&m).unwrap().to_string(),
///     "[3, 255; 0, 1]"
/// );
/// assert_eq!(
///     UnsignedMatrix::<u8>::try_from(m).unwrap().to_string(),
///     "[3, 255; 0, 1]"
/// );
///
/// assert!(UnsignedMatrix::<u8>::try_from(RationalMatrix::from_str("[3, 256]").unwrap()).is_err());
/// assert!(
///     UnsignedMatrix::<u8>::try_from(&RationalMatrix::from_str("[3, -1; 1/2, 1]").unwrap())
///         .is_err()
/// );
///
/// assert_eq!(
///     UnsignedMatrix::<u8>::try_from(RationalMatrix::from_str("[]_{0,3}").unwrap())
///         .unwrap()
///         .to_string(),
///     "[]_{0,3}"
/// );
/// ```
///
/// # convertible_from
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::conversion::traits::ConvertibleFrom;
/// use malachite_base::unsigned_matrix::UnsignedMatrix;
/// use malachite_q::rational_matrix::RationalMatrix;
///
/// assert_eq!(
///     UnsignedMatrix::<u8>::convertible_from(
///         &RationalMatrix::from_str("[3, 255; 0, 1]").unwrap()
///     ),
///     true
/// );
/// assert_eq!(
///     UnsignedMatrix::<u8>::convertible_from(&RationalMatrix::from_str("[3, 256]").unwrap()),
///     false
/// );
/// assert_eq!(
///     UnsignedMatrix::<u8>::convertible_from(&RationalMatrix::from_str("[]_{0,3}").unwrap()),
///     true
/// );
/// ```
pub mod unsigned_matrix_from_rational_matrix;
