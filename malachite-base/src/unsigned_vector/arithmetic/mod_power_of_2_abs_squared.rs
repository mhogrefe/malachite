// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModPowerOf2AbsSquared, ModPowerOf2Dot};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;

impl<T: PrimitiveUnsigned> ModPowerOf2AbsSquared for UnsignedVector<T> {
    type Output = T;

    /// Computes the squared norm of an [`UnsignedVector`], the sum of the squares of its elements,
    /// modulo $2^k$, taking the vector by value. The elements must already be reduced.
    ///
    /// The squared norm of a 0-dimensional vector is 0.
    ///
    /// $$
    /// f(v, k) = \sum_{i=0}^{n-1} v_i^2 \bmod 2^k,
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
    /// Panics if `pow` is greater than `T::WIDTH` or if any element is greater than or equal to
    /// $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AbsSquared;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v.clone().mod_power_of_2_abs_squared(3), 6);
    /// ```
    #[inline]
    fn mod_power_of_2_abs_squared(self, pow: u64) -> T {
        (&self).mod_power_of_2_dot(&self, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2AbsSquared for &UnsignedVector<T> {
    type Output = T;

    /// Computes the squared norm of an [`UnsignedVector`], the sum of the squares of its elements,
    /// modulo $2^k$, taking the vector by reference. The elements must already be reduced.
    ///
    /// The squared norm of a 0-dimensional vector is 0.
    ///
    /// $$
    /// f(v, k) = \sum_{i=0}^{n-1} v_i^2 \bmod 2^k,
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
    /// Panics if `pow` is greater than `T::WIDTH` or if any element is greater than or equal to
    /// $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AbsSquared;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!((&v).mod_power_of_2_abs_squared(3), 6);
    /// ```
    #[inline]
    fn mod_power_of_2_abs_squared(self, pow: u64) -> T {
        self.mod_power_of_2_dot(self, pow)
    }
}
