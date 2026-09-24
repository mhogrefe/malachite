// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::{EqTruncated, slices_eq_truncated};
use crate::unsigned_polynomial::UnsignedPolynomial;

impl<T: PrimitiveUnsigned> EqTruncated for UnsignedPolynomial<T> {
    /// Determines whether an [`UnsignedPolynomial`] and another agree below $x^{\mathrm{len}}$:
    /// that is, whether they have the same coefficient of $x^i$ for every $i$ less than `len`.
    ///
    /// Any two polynomials agree below $x^0$, and once `len` is at least both of their lengths,
    /// they agree exactly when they are equal.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `min(len, max(self.len(),
    /// other.len()))`.
    ///
    /// # Examples
    /// See [here](super::eq_truncated#eq_truncated).
    ///
    /// This is equivalent to `nmod_poly_equal_trunc` from `nmod_poly/equal_trunc.c`, FLINT 3.6.0.
    #[inline]
    fn eq_truncated(&self, other: &Self, len: u64) -> bool {
        slices_eq_truncated(
            &self.coefficients,
            &other.coefficients,
            len,
            |&x| x == T::ZERO,
            |&y| y == T::ZERO,
            |x, y| x == y,
        )
    }
}
