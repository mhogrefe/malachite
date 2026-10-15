// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use malachite_base::num::arithmetic::traits::{AbsSquared, CheckedAbs, CheckedSqrt};

impl CheckedAbs for NaturalVector {
    type Output = Natural;

    /// Computes the Euclidean norm of a [`NaturalVector`], the square root of the sum of the
    /// squares of its elements, taking the vector by value, and returns it if it is an integer.
    ///
    /// $$
    /// f(v) = \begin{cases}
    ///     \operatorname{Some}(\|v\|) & \text{if} \\ \|v\| \in \mathbb{Z}, \\\\
    ///     \operatorname{None} & \textrm{otherwise},
    /// \end{cases}
    /// $$
    ///
    /// where $\|v\| = \sqrt{\sum_{i=0}^{n-1} v_i^2}$ and $n$ is the dimension of $v$. The sum of
    /// the squares is an integer, so its square root is an integer exactly when the sum is a
    /// perfect square. The norm of the 0-dimensional vector is 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O((m + \log n) \log (m + \log n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vector, and $m$ is
    /// the largest number of significant bits of any element: the squared norm has $O(m + \log n)$
    /// bits, and its square root costs no more than computing it.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CheckedAbs;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(3, 4)").unwrap();
    /// assert_eq!(v.clone().checked_abs(), Some(Natural::from(5u32)));
    /// let v = NaturalVector::from_str("(2, 3, 6)").unwrap();
    /// assert_eq!(v.clone().checked_abs(), Some(Natural::from(7u32)));
    /// let v = NaturalVector::from_str("(1, 1)").unwrap();
    /// assert_eq!(v.clone().checked_abs(), None);
    /// let v = NaturalVector::from_str("()").unwrap();
    /// assert_eq!(v.clone().checked_abs(), Some(Natural::ZERO));
    /// ```
    #[inline]
    fn checked_abs(self) -> Option<Natural> {
        self.abs_squared().checked_sqrt()
    }
}

impl CheckedAbs for &NaturalVector {
    type Output = Natural;

    /// Computes the Euclidean norm of a [`NaturalVector`], the square root of the sum of the
    /// squares of its elements, taking the vector by reference, and returns it if it is an integer.
    ///
    /// $$
    /// f(v) = \begin{cases}
    ///     \operatorname{Some}(\|v\|) & \text{if} \\ \|v\| \in \mathbb{Z}, \\\\
    ///     \operatorname{None} & \textrm{otherwise},
    /// \end{cases}
    /// $$
    ///
    /// where $\|v\| = \sqrt{\sum_{i=0}^{n-1} v_i^2}$ and $n$ is the dimension of $v$. The sum of
    /// the squares is an integer, so its square root is an integer exactly when the sum is a
    /// perfect square. The norm of the 0-dimensional vector is 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O((m + \log n) \log (m + \log n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vector, and $m$ is
    /// the largest number of significant bits of any element: the squared norm has $O(m + \log n)$
    /// bits, and its square root costs no more than computing it.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CheckedAbs;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(3, 4)").unwrap();
    /// assert_eq!((&v).checked_abs(), Some(Natural::from(5u32)));
    /// let v = NaturalVector::from_str("(2, 3, 6)").unwrap();
    /// assert_eq!((&v).checked_abs(), Some(Natural::from(7u32)));
    /// let v = NaturalVector::from_str("(1, 1)").unwrap();
    /// assert_eq!((&v).checked_abs(), None);
    /// let v = NaturalVector::from_str("()").unwrap();
    /// assert_eq!((&v).checked_abs(), Some(Natural::ZERO));
    /// ```
    #[inline]
    fn checked_abs(self) -> Option<Natural> {
        self.abs_squared().checked_sqrt()
    }
}
