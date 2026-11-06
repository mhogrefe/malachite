// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Computes the $n$th Touchard polynomial, whose coefficients are the Stirling numbers of the
/// second kind $S(n, k)$.
pub trait TouchardPolynomial {
    fn touchard_polynomial(n: u64) -> Self;
}

/// Computes the $n$th Touchard polynomial truncated to its first `len` coefficients, which are the
/// Stirling numbers of the second kind $S(n, k)$ for $k < \text{len}$.
pub trait TouchardPolynomialTruncated {
    fn touchard_polynomial_truncated(n: u64, len: u64) -> Self;
}
