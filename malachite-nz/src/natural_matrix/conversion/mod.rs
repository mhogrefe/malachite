// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// An implementation of [`From`] for converting an
/// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix) to a
/// [`NaturalMatrix`](super::NaturalMatrix).
pub mod from_unsigned_matrix;
#[cfg(feature = "serde")]
/// Implementations of traits for serialization and deserialization using
/// [serde](https://serde.rs/).
pub mod serde;
/// Conversions between [`NaturalMatrix`](super::NaturalMatrix)s and strings.
pub mod string;
/// Implementations of traits for converting a [`NaturalMatrix`](super::NaturalMatrix) to an
/// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix).
///
/// # try_from
/// ```
/// use core::str::FromStr;
/// use malachite_base::unsigned_matrix::UnsignedMatrix;
/// use malachite_nz::natural_matrix::NaturalMatrix;
///
/// let m = NaturalMatrix::from_str("[3, 255; 0, 1]").unwrap();
/// assert_eq!(
///     UnsignedMatrix::<u8>::try_from(&m).unwrap().to_string(),
///     "[3, 255; 0, 1]"
/// );
/// assert_eq!(
///     UnsignedMatrix::<u64>::try_from(m).unwrap().to_string(),
///     "[3, 255; 0, 1]"
/// );
///
/// let m = NaturalMatrix::from_str("[3, 256; 0, 1]").unwrap();
/// assert!(UnsignedMatrix::<u8>::try_from(&m).is_err());
/// assert_eq!(
///     UnsignedMatrix::<u16>::try_from(&m).unwrap().to_string(),
///     "[3, 256; 0, 1]"
/// );
///
/// assert_eq!(
///     UnsignedMatrix::<u32>::try_from(NaturalMatrix::from_str("[]_{2,0}").unwrap())
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
/// use malachite_base::unsigned_matrix::UnsignedMatrix;
/// use malachite_nz::natural_matrix::NaturalMatrix;
///
/// assert_eq!(
///     UnsignedMatrix::<u8>::convertible_from(&NaturalMatrix::from_str("[3, 255]").unwrap()),
///     true
/// );
/// assert_eq!(
///     UnsignedMatrix::<u8>::convertible_from(&NaturalMatrix::from_str("[3; 256]").unwrap()),
///     false
/// );
/// assert_eq!(
///     UnsignedMatrix::<u16>::convertible_from(&NaturalMatrix::from_str("[3; 256]").unwrap()),
///     true
/// );
/// assert_eq!(
///     UnsignedMatrix::<u32>::convertible_from(&NaturalMatrix::from_str("[]_{0,3}").unwrap()),
///     true
/// );
/// ```
pub mod unsigned_matrix_from_natural_matrix;
