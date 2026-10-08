// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::vector::Vector;

/// Iterators that generate [`NaturalVector`]s without repetition.
pub mod exhaustive;
#[cfg(feature = "random")]
/// Iterators that generate [`NaturalVector`]s randomly.
pub mod random;

/// A vector whose elements are [`Natural`]s.
///
/// This is malachite-base's generic [`Vector`], and everything defined on it applies; see its
/// documentation. This alias is here for convenience, and to hold the generators of vectors of
/// [`Natural`]s.
pub type NaturalVector = Vector<Natural>;
