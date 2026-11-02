// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`AddTruncated`](malachite_base::polynomial::series::traits::AddTruncated)
/// and [`AddTruncatedAssign`](malachite_base::polynomial::series::traits::AddTruncatedAssign), for
/// adding two polynomials and keeping only their low coefficients.
pub mod add_truncated;
/// Implementations of
/// [`ComposeTruncated`](malachite_base::polynomial::series::traits::ComposeTruncated) and
/// [`ComposeTruncatedAssign`](malachite_base::polynomial::series::traits::ComposeTruncatedAssign),
/// for composing two polynomials as power series.
pub mod compose_truncated;
/// Series reversion: the compositional inverse of a polynomial as a power series.
pub mod compositional_inverse_truncated;
/// Implementations of [`DivTruncated`](malachite_base::polynomial::series::traits::DivTruncated),
/// [`DivTruncatedAssign`](malachite_base::polynomial::series::traits::DivTruncatedAssign), and
/// [`CheckedDivTruncated`](malachite_base::polynomial::series::traits::CheckedDivTruncated), for
/// dividing two polynomials as power series.
pub mod div_truncated;
/// Implementations of
/// [`InverseTruncated`](malachite_base::polynomial::series::traits::InverseTruncated), for
/// inverting a polynomial as a power series.
pub mod inverse_truncated;
/// Implementations of [`MulTruncated`](malachite_base::polynomial::series::traits::MulTruncated)
/// and [`MulTruncatedAssign`](malachite_base::polynomial::series::traits::MulTruncatedAssign), for
/// multiplying two polynomials and keeping only the low coefficients of the product.
pub mod mul_truncated;
/// Implementations of [`PowTruncated`](malachite_base::polynomial::series::traits::PowTruncated)
/// and [`PowTruncatedAssign`](malachite_base::polynomial::series::traits::PowTruncatedAssign), for
/// raising a polynomial to a power and keeping only the low coefficients of the power.
pub mod pow_truncated;
/// Implementations of
/// [`SquareTruncated`](malachite_base::polynomial::series::traits::SquareTruncated) and
/// [`SquareTruncatedAssign`](malachite_base::polynomial::series::traits::SquareTruncatedAssign),
/// for squaring a polynomial and keeping only the low coefficients of the square.
pub mod square_truncated;
/// Implementations of [`SubTruncated`](malachite_base::polynomial::series::traits::SubTruncated)
/// and [`SubTruncatedAssign`](malachite_base::polynomial::series::traits::SubTruncatedAssign), for
/// subtracting one polynomial from another and keeping only their low coefficients.
pub mod sub_truncated;
