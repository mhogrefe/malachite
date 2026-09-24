// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`Evaluate`](malachite_base::polynomial::Evaluate) for
/// [`IntegerPolynomial`](malachite_nz::integer_polynomial::IntegerPolynomial)s and
/// [`RationalPolynomial`](super::RationalPolynomial)s at [`Rational`](crate::Rational)s, and for
/// [`RationalPolynomial`](super::RationalPolynomial)s at
/// [`Integer`](malachite_nz::integer::Integer)s.
pub mod evaluate;
/// An implementation of [`Height`](malachite_base::num::arithmetic::traits::Height), the largest of
/// the heights of a polynomial's coefficients.
pub mod height;
/// An implementation of [`IsUnit`](malachite_base::num::arithmetic::traits::IsUnit), a trait for
/// determining whether a number is a unit of its ring.
pub mod is_unit;
