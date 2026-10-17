// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ModIsReduced;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;

impl<T: PrimitiveUnsigned> ModIsReduced<T> for UnsignedMatrix<T> {
    /// Returns whether an [`UnsignedMatrix`] is reduced modulo a `T` $m$; in other words, whether
    /// every one of its elements is less than $m$.
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
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.elements.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModIsReduced;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// // The elements are 2, 3, and 1, so 4 is large enough and 3 is not.
    /// let a = UnsignedMatrix::<u64>::from_str("[2, 3, 1]").unwrap();
    /// assert_eq!(a.mod_is_reduced(&4), true);
    /// assert_eq!(a.mod_is_reduced(&3), false);
    ///
    /// // A matrix with no elements is reduced modulo everything.
    /// assert_eq!(
    ///     UnsignedMatrix::<u64>::from_str("[]_{0,0}")
    ///         .unwrap()
    ///         .mod_is_reduced(&1),
    ///     true
    /// );
    /// ```
    #[inline]
    fn mod_is_reduced(&self, m: &T) -> bool {
        assert_ne!(*m, T::ZERO, "the modulus is zero");
        self.elements.iter().all(|x| x < m)
    }
}
