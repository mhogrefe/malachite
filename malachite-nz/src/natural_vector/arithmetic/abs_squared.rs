// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_vector::arithmetic::dot_general::vec_dot_general;
use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use malachite_base::num::arithmetic::traits::AbsSquared;

impl AbsSquared for NaturalVector {
    type Output = Natural;

    /// Computes the squared norm of a [`NaturalVector`], the sum of the squares of its elements,
    /// taking the vector by value.
    ///
    /// This is the dot product of the vector with itself, $\|v\|^2$. The squared norm of the
    /// 0-dimensional vector is 0.
    ///
    /// $$
    /// f(v) = \|v\|^2 = \sum_{i=0}^{n-1} v_i^2,
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
    /// the largest number of significant bits of any element.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::AbsSquared;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(3, 4, 12)").unwrap();
    /// assert_eq!(v.abs_squared(), 169);
    /// let v = NaturalVector::from_str("()").unwrap();
    /// assert_eq!(v.abs_squared(), 0);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_dot` from `fmpz_vec/dot.c`, FLINT 3.6.0, with both vectors
    /// the same, which squares each element instead of multiplying it by itself.
    #[inline]
    fn abs_squared(self) -> Natural {
        vec_dot_general(None, false, &self.elements, &self.elements, false)
    }
}

impl AbsSquared for &NaturalVector {
    type Output = Natural;

    /// Computes the squared norm of a [`NaturalVector`], the sum of the squares of its elements,
    /// taking the vector by reference.
    ///
    /// This is the dot product of the vector with itself, $\|v\|^2$. The squared norm of the
    /// 0-dimensional vector is 0.
    ///
    /// $$
    /// f(v) = \|v\|^2 = \sum_{i=0}^{n-1} v_i^2,
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
    /// the largest number of significant bits of any element.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::AbsSquared;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(3, 4, 12)").unwrap();
    /// assert_eq!((&v).abs_squared(), 169);
    /// let v = NaturalVector::from_str("()").unwrap();
    /// assert_eq!((&v).abs_squared(), 0);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_dot` from `fmpz_vec/dot.c`, FLINT 3.6.0, with both vectors
    /// the same, which squares each element instead of multiplying it by itself.
    #[inline]
    fn abs_squared(self) -> Natural {
        vec_dot_general(None, false, &self.elements, &self.elements, false)
    }
}
