// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use malachite_base::num::arithmetic::traits::{ModAbsSquared, ModDot};

impl ModAbsSquared<Natural> for NaturalVector {
    type Output = Natural;

    /// Computes the squared norm of a [`NaturalVector`], the sum of the squares of its elements,
    /// modulo $m$, taking the vector by value and the modulus by value. The elements must already
    /// be reduced.
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
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vector, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero or if any element is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAbsSquared;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v.clone().mod_abs_squared(Natural::from(10u32)), 4);
    /// ```
    #[inline]
    fn mod_abs_squared(self, m: Natural) -> Natural {
        (&self).mod_dot(&self, m)
    }
}

impl ModAbsSquared<&Natural> for NaturalVector {
    type Output = Natural;

    /// Computes the squared norm of a [`NaturalVector`], the sum of the squares of its elements,
    /// modulo $m$, taking the vector by value and the modulus by reference. The elements must
    /// already be reduced.
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
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vector, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero or if any element is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAbsSquared;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v.clone().mod_abs_squared(&Natural::from(10u32)), 4);
    /// ```
    #[inline]
    fn mod_abs_squared(self, m: &Natural) -> Natural {
        (&self).mod_dot(&self, m)
    }
}

impl ModAbsSquared<Natural> for &NaturalVector {
    type Output = Natural;

    /// Computes the squared norm of a [`NaturalVector`], the sum of the squares of its elements,
    /// modulo $m$, taking the vector by reference and the modulus by value. The elements must
    /// already be reduced.
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
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vector, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero or if any element is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAbsSquared;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!((&v).mod_abs_squared(Natural::from(10u32)), 4);
    /// ```
    #[inline]
    fn mod_abs_squared(self, m: Natural) -> Natural {
        self.mod_dot(self, m)
    }
}

impl ModAbsSquared<&Natural> for &NaturalVector {
    type Output = Natural;

    /// Computes the squared norm of a [`NaturalVector`], the sum of the squares of its elements,
    /// modulo $m$, taking the vector by reference and the modulus by reference. The elements must
    /// already be reduced.
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
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vector, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero or if any element is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAbsSquared;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!((&v).mod_abs_squared(&Natural::from(10u32)), 4);
    /// ```
    #[inline]
    fn mod_abs_squared(self, m: &Natural) -> Natural {
        self.mod_dot(self, m)
    }
}
