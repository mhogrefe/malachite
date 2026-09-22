// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{Height, ModIsReduced};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_polynomial::UnsignedPolynomial;

impl<T: PrimitiveUnsigned> ModIsReduced<T> for UnsignedPolynomial<T> {
    /// Returns whether a [`UnsignedPolynomial`] is reduced modulo a [`u64`] $m$; in other words,
    /// whether every one of its coefficients is less than $m$.
    ///
    /// Asking that of every coefficient is asking it of the largest, so this is a comparison
    /// against the polynomial's [`Height`](Height::to_height). The zero polynomial has no
    /// coefficients and is reduced modulo every $m$.
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
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients.
    ///
    /// # Panics
    /// Panics if `m` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModIsReduced;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The coefficients are 2, 3, and 1, so 4 is large enough and 3 is not.
    /// let p = UnsignedPolynomial::<u64>::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(p.mod_is_reduced(&4), true);
    /// assert_eq!(p.mod_is_reduced(&3), false);
    ///
    /// // The zero polynomial is reduced modulo everything.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u64>::from_str("0")
    ///         .unwrap()
    ///         .mod_is_reduced(&1),
    ///     true
    /// );
    /// ```
    #[inline]
    fn mod_is_reduced(&self, m: &T) -> bool {
        assert_ne!(*m, T::ZERO);
        self.to_height() < *m
    }
}
