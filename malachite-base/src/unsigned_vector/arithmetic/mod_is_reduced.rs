// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ModIsReduced;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;

impl<T: PrimitiveUnsigned> ModIsReduced<T> for UnsignedVector<T> {
    /// Returns whether an [`UnsignedVector`] is reduced modulo a `T` $m$; in other words, whether
    /// every one of its elements is less than $m$.
    ///
    /// The 0-dimensional vector has no elements and is reduced modulo every $m$.
    ///
    /// $m$ cannot be zero.
    ///
    /// $f(v, m) = (\max_i v_i < m)$, with the maximum of no elements taken to be 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `m` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModIsReduced;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// // The elements are 2, 3, and 1, so 4 is large enough and 3 is not.
    /// let v = UnsignedVector::<u64>::from_str("(2, 3, 1)").unwrap();
    /// assert_eq!(v.mod_is_reduced(&4), true);
    /// assert_eq!(v.mod_is_reduced(&3), false);
    ///
    /// // The 0-dimensional vector is reduced modulo everything.
    /// assert_eq!(
    ///     UnsignedVector::<u64>::from_str("()")
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
