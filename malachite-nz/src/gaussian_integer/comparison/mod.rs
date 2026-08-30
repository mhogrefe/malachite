// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`Ord`] and [`PartialOrd`] for
/// [`ComparableGaussianInteger`](crate::gaussian_integer::ComparableGaussianInteger) and
/// [`ComparableGaussianIntegerRef`](crate::gaussian_integer::ComparableGaussianIntegerRef),
/// ordering [`GaussianInteger`](crate::gaussian_integer::GaussianInteger)s lexicographically: first
/// by real part, then by imaginary part.
pub mod cmp;
/// Implementations of [`OrdAbs`](malachite_base::num::comparison::traits::OrdAbs) and
/// [`PartialOrdAbs`](malachite_base::num::comparison::traits::PartialOrdAbs) for
/// [`GaussianInteger`](crate::gaussian_integer::GaussianInteger), comparing absolute values
/// (distances from the origin).
pub mod cmp_abs;
