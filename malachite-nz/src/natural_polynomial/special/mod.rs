// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`RisingFactorialPolynomial`](
/// malachite_base::polynomial::special::traits::RisingFactorialPolynomial) and
/// [`RisingFactorialPolynomialTruncated`](
/// malachite_base::polynomial::special::traits::RisingFactorialPolynomialTruncated), traits for
/// computing rising factorial polynomials, whose coefficients are the unsigned Stirling numbers of
/// the first kind.
pub mod rising_factorial_polynomial;
/// Implementations of [`TouchardPolynomial`](
/// malachite_base::polynomial::special::traits::TouchardPolynomial) and
/// [`TouchardPolynomialTruncated`](
/// malachite_base::polynomial::special::traits::TouchardPolynomialTruncated), traits for computing
/// Touchard polynomials, whose coefficients are the Stirling numbers of the second kind.
pub mod touchard_polynomial;
