// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use malachite_base::num::arithmetic::traits::{ModPowerOf2AbsSquared, ModPowerOf2Dot};

impl ModPowerOf2AbsSquared for NaturalVector {
    type Output = Natural;

    /// Computes the squared norm of a [`NaturalVector`], the sum of the squares of its elements,
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
    /// $T(n, k) = O(nk \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vector, and $k$ is
    /// `pow`.
    ///
    /// # Panics
    /// Panics if any element is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AbsSquared;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v.clone().mod_power_of_2_abs_squared(3), 6);
    /// ```
    #[inline]
    fn mod_power_of_2_abs_squared(self, pow: u64) -> Natural {
        (&self).mod_power_of_2_dot(&self, pow)
    }
}

impl ModPowerOf2AbsSquared for &NaturalVector {
    type Output = Natural;

    /// Computes the squared norm of a [`NaturalVector`], the sum of the squares of its elements,
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
    /// $T(n, k) = O(nk \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vector, and $k$ is
    /// `pow`.
    ///
    /// # Panics
    /// Panics if any element is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AbsSquared;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!((&v).mod_power_of_2_abs_squared(3), 6);
    /// ```
    #[inline]
    fn mod_power_of_2_abs_squared(self, pow: u64) -> Natural {
        self.mod_power_of_2_dot(self, pow)
    }
}
