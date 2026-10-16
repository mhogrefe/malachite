// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Equality of [`NaturalMatrix`](super::NaturalMatrix)s and
/// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)s.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_base::unsigned_matrix::UnsignedMatrix;
/// use malachite_nz::natural_matrix::NaturalMatrix;
///
/// assert!(
///     NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap()
///         == UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap()
/// );
/// assert!(
///     NaturalMatrix::from_str("[3, 256]").unwrap()
///         != UnsignedMatrix::<u8>::from_str("[3, 0]").unwrap()
/// );
/// // The same elements in a different shape are not equal.
/// assert!(
///     NaturalMatrix::from_str("[1, 2, 3, 4]").unwrap()
///         != UnsignedMatrix::<u32>::from_str("[1, 2; 3, 4]").unwrap()
/// );
/// assert!(
///     NaturalMatrix::from_str("[]_{0,3}").unwrap()
///         != UnsignedMatrix::<u32>::from_str("[]_{3,0}").unwrap()
/// );
///
/// assert!(
///     UnsignedMatrix::<u16>::from_str("[]_{2,0}").unwrap()
///         == NaturalMatrix::from_str("[]_{2,0}").unwrap()
/// );
/// assert!(
///     UnsignedMatrix::<u128>::from_str("[1; 2]").unwrap()
///         != NaturalMatrix::from_str("[2; 1]").unwrap()
/// );
/// ```
pub mod partial_eq_unsigned_matrix;
