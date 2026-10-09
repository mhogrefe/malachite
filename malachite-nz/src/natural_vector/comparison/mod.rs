// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`EntrywiseMax`](malachite_base::num::arithmetic::traits::EntrywiseMax) and
/// [`EntrywiseMaxAssign`](malachite_base::num::arithmetic::traits::EntrywiseMaxAssign), which take
/// the larger of each pair of corresponding elements of two vectors.
pub mod entrywise_max;
/// Implementations of [`EntrywiseMin`](malachite_base::num::arithmetic::traits::EntrywiseMin) and
/// [`EntrywiseMinAssign`](malachite_base::num::arithmetic::traits::EntrywiseMinAssign), which take
/// the smaller of each pair of corresponding elements of two vectors.
pub mod entrywise_min;
/// Equality of [`NaturalVector`](super::NaturalVector)s and
/// [`UnsignedVector`](malachite_base::unsigned_vector::UnsignedVector)s.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_base::unsigned_vector::UnsignedVector;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// assert!(
///     NaturalVector::from_str("()").unwrap() == UnsignedVector::<u8>::from_str("()").unwrap()
/// );
/// assert!(
///     NaturalVector::from_str("(3, 255)").unwrap()
///         == UnsignedVector::<u8>::from_str("(3, 255)").unwrap()
/// );
/// assert!(
///     NaturalVector::from_str("(3, 256)").unwrap()
///         != UnsignedVector::<u8>::from_str("(3, 0)").unwrap()
/// );
/// assert!(
///     NaturalVector::from_str("(1, 0)").unwrap()
///         != UnsignedVector::<u32>::from_str("(1)").unwrap()
/// );
///
/// assert!(
///     UnsignedVector::<u16>::from_str("(1, 2)").unwrap()
///         == NaturalVector::from_str("(1, 2)").unwrap()
/// );
/// assert!(
///     UnsignedVector::<u128>::from_str("(1, 2)").unwrap()
///         != NaturalVector::from_str("(2, 1)").unwrap()
/// );
/// ```
pub mod partial_eq_unsigned_vector;
/// Implementations of [`Ord`] and [`PartialOrd`] for
/// [`ShortlexNaturalVector`](super::ShortlexNaturalVector) and
/// [`ShortlexNaturalVectorRef`](super::ShortlexNaturalVectorRef), comparing two vectors by
/// dimension and then lexicographically.
pub mod shortlex_cmp;
