// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`Ord`] and [`PartialOrd`] for
/// [`IntegerPolynomial`](super::IntegerPolynomial), comparing two polynomials by their behavior for
/// large arguments.
pub mod cmp;
/// Implementations of [`Ord`] and [`PartialOrd`] for
/// [`ShortlexIntegerPolynomial`](super::ShortlexIntegerPolynomial) and
/// [`ShortlexIntegerPolynomialRef`](super::ShortlexIntegerPolynomialRef), comparing two polynomials
/// by degree and then by their coefficients from highest to lowest.
pub mod shortlex_cmp;
