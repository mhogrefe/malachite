// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::vector::Vector;

/// Functions for converting a [`RationalVector`] to and from a vector of numerators and a single
/// denominator.
pub mod conversion;
/// Iterators that generate [`RationalVector`]s without repetition.
pub mod exhaustive;
#[cfg(feature = "random")]
/// Iterators that generate [`RationalVector`]s randomly.
pub mod random;

/// A vector whose elements are [`Rational`]s.
///
/// This is malachite-base's generic [`Vector`], and everything defined on it applies; see its
/// documentation. This alias is here for convenience, and to hold the generators of vectors of
/// [`Rational`]s.
pub type RationalVector = Vector<Rational>;
