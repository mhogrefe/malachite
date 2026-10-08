// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`Ord`] and [`PartialOrd`] for
/// [`ShortlexVector`](super::ShortlexVector) and [`ShortlexVectorRef`](super::ShortlexVectorRef),
/// comparing two vectors by dimension and then lexicographically.
pub mod shortlex_cmp;
