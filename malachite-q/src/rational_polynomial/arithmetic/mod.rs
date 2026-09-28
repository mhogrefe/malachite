// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`Add`](core::ops::Add) and [`AddAssign`](core::ops::AddAssign), for adding
/// two polynomials.
pub mod add;
/// Implementations of [`AddTruncated`](malachite_base::polynomial::AddTruncated) and
/// [`AddTruncatedAssign`](malachite_base::polynomial::AddTruncatedAssign), for adding two
/// polynomials and keeping only their low coefficients.
pub mod add_truncated;
/// Implementations of
/// [`CanonicalizeUnit`](malachite_base::num::arithmetic::traits::CanonicalizeUnit) and
/// [`CanonicalizeUnitAssign`](malachite_base::num::arithmetic::traits::CanonicalizeUnitAssign),
/// which bring a polynomial into canonical unit form.
pub mod canonicalize_unit;
/// Implementations of [`Content`](malachite_base::polynomial::Content),
/// [`PrimitivePart`](malachite_base::polynomial::PrimitivePart), and
/// [`ContentAndPrimitivePart`](malachite_base::polynomial::ContentAndPrimitivePart) for
/// [`RationalPolynomial`](super::RationalPolynomial)s.
pub mod content;
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
/// An implementation of [`L2NormSquared`](malachite_base::polynomial::L2NormSquared), the sum of
/// the squares of a polynomial's coefficients.
pub mod l2_norm_squared;
/// Implementations of [`MakeMonic`](malachite_base::polynomial::MakeMonic) and
/// [`MakeMonicAssign`](malachite_base::polynomial::MakeMonicAssign), which divide a polynomial by
/// its leading coefficient.
pub mod make_monic;
/// Implementations of [`Neg`](core::ops::Neg) and
/// [`NegAssign`](malachite_base::num::arithmetic::traits::NegAssign), for negating a polynomial.
pub mod neg;
/// Implementations of [`Sub`](core::ops::Sub) and [`SubAssign`](core::ops::SubAssign), for
/// subtracting one polynomial from another.
pub mod sub;
/// Implementations of [`SubTruncated`](malachite_base::polynomial::SubTruncated) and
/// [`SubTruncatedAssign`](malachite_base::polynomial::SubTruncatedAssign), for subtracting one
/// polynomial from another and keeping only their low coefficients.
pub mod sub_truncated;
