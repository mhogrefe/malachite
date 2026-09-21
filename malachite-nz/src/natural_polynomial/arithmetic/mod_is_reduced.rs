// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::num::arithmetic::traits::{HeightRef, ModIsReduced};
use malachite_base::num::basic::traits::Zero;

impl ModIsReduced<Natural> for NaturalPolynomial {
    /// Returns whether a [`NaturalPolynomial`] is reduced modulo a [`Natural`] $m$; in other words,
    /// whether every one of its coefficients is less than $m$.
    ///
    /// Asking that of every coefficient is asking it of the largest, so this is a comparison
    /// against the polynomial's [`height`](HeightRef::height_ref). The height is one of the
    /// coefficients, so it is borrowed rather than built. The zero polynomial has no coefficients
    /// and is reduced modulo every $m$.
    ///
    /// $m$ cannot be zero.
    ///
    /// $f(p, m) = (\max_i p_i < m)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients.
    ///
    /// # Panics
    /// Panics if `m` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModIsReduced;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The coefficients are 2, 3, and 1, so 4 is large enough and 3 is not.
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(p.mod_is_reduced(&Natural::from(4u32)), true);
    /// assert_eq!(p.mod_is_reduced(&Natural::from(3u32)), false);
    ///
    /// // The zero polynomial is reduced modulo everything.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("0")
    ///         .unwrap()
    ///         .mod_is_reduced(&Natural::ONE),
    ///     true
    /// );
    /// ```
    #[inline]
    fn mod_is_reduced(&self, m: &Natural) -> bool {
        assert_ne!(*m, Natural::ZERO);
        self.height_ref() < m
    }
}
