// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// A function for building a [`RationalVector`](super::RationalVector) from a vector of numerators
/// and a single denominator.
pub mod from_numerators_and_denominator;
/// A function for clearing the denominators of a [`RationalVector`](super::RationalVector), giving
/// a vector of numerators and a single denominator.
pub mod to_numerators_and_denominator;
