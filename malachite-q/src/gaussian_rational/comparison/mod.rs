// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`Ord`] and [`PartialOrd`] for
/// [`ComparableGaussianRational`](crate::gaussian_rational::ComparableGaussianRational) and
/// [`ComparableGaussianRationalRef`](crate::gaussian_rational::ComparableGaussianRationalRef),
/// ordering [`GaussianRational`](crate::gaussian_rational::GaussianRational)s lexicographically:
/// first by real part, then by imaginary part.
pub mod cmp;
