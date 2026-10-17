// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use malachite_base::num::arithmetic::traits::ModIsReduced;

impl ModIsReduced<Natural> for NaturalMatrix {
    /// Returns whether a [`NaturalMatrix`] is reduced modulo a [`Natural`] $m$; in other words,
    /// whether every one of its elements is less than $m$.
    ///
    /// A matrix with no elements, of any shape, is reduced modulo every $m$.
    ///
    /// $m$ cannot be zero.
    ///
    /// $f(A, m) = (\max_{i,j} A_{ij} < m)$, with the maximum of no elements taken to be 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// // The elements are 2, 3, and 1, so 4 is large enough and 3 is not.
    /// let a = NaturalMatrix::from_str("[2, 3, 1]").unwrap();
    /// assert_eq!(a.mod_is_reduced(&Natural::from(4u32)), true);
    /// assert_eq!(a.mod_is_reduced(&Natural::from(3u32)), false);
    ///
    /// // A matrix with no elements is reduced modulo everything.
    /// assert_eq!(
    ///     NaturalMatrix::from_str("[]_{0,0}")
    ///         .unwrap()
    ///         .mod_is_reduced(&Natural::ONE),
    ///     true
    /// );
    /// ```
    #[inline]
    fn mod_is_reduced(&self, m: &Natural) -> bool {
        assert_ne!(*m, 0u32, "the modulus is zero");
        self.elements.iter().all(|x| x < m)
    }
}
