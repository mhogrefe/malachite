// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`EntrywiseMax`](crate::num::arithmetic::traits::EntrywiseMax) and
/// [`EntrywiseMaxAssign`](crate::num::arithmetic::traits::EntrywiseMaxAssign), which take the
/// larger of each pair of corresponding elements of two vectors.
pub mod entrywise_max;
/// Implementations of [`EntrywiseMin`](crate::num::arithmetic::traits::EntrywiseMin) and
/// [`EntrywiseMinAssign`](crate::num::arithmetic::traits::EntrywiseMinAssign), which take the
/// smaller of each pair of corresponding elements of two vectors.
pub mod entrywise_min;
/// Implementations of [`Ord`] and [`PartialOrd`] for
/// [`ShortlexUnsignedVector`](super::ShortlexUnsignedVector) and
/// [`ShortlexUnsignedVectorRef`](super::ShortlexUnsignedVectorRef), comparing two vectors by
/// dimension and then lexicographically.
pub mod shortlex_cmp;
