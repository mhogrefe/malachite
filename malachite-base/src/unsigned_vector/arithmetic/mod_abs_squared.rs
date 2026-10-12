// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModAbsSquared, ModDot};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;

impl<T: PrimitiveUnsigned> ModAbsSquared<T> for UnsignedVector<T> {
    type Output = T;

    /// Computes the squared norm of an [`UnsignedVector`], the sum of the squares of its elements,
    /// modulo $m$, taking the vector by value. The elements must already be reduced.
    ///
    /// The squared norm of a 0-dimensional vector is 0.
    ///
    /// $$
    /// f(v, m) = \sum_{i=0}^{n-1} v_i^2 \bmod m,
    /// $$
    ///
    /// where $n$ is the dimension of $v$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the dimension of the vector.
    ///
    /// # Panics
    /// Panics if `m` is zero or if any element is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAbsSquared;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v.clone().mod_abs_squared(10), 4);
    /// ```
    #[inline]
    fn mod_abs_squared(self, m: T) -> T {
        (&self).mod_dot(&self, m)
    }
}

impl<T: PrimitiveUnsigned> ModAbsSquared<T> for &UnsignedVector<T> {
    type Output = T;

    /// Computes the squared norm of an [`UnsignedVector`], the sum of the squares of its elements,
    /// modulo $m$, taking the vector by reference. The elements must already be reduced.
    ///
    /// The squared norm of a 0-dimensional vector is 0.
    ///
    /// $$
    /// f(v, m) = \sum_{i=0}^{n-1} v_i^2 \bmod m,
    /// $$
    ///
    /// where $n$ is the dimension of $v$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the dimension of the vector.
    ///
    /// # Panics
    /// Panics if `m` is zero or if any element is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAbsSquared;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!((&v).mod_abs_squared(10), 4);
    /// ```
    #[inline]
    fn mod_abs_squared(self, m: T) -> T {
        self.mod_dot(self, m)
    }
}
