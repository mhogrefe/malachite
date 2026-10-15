// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_vector::RationalVector;
use malachite_base::num::arithmetic::traits::{AbsSquared, CheckedAbs, CheckedSqrt};

impl CheckedAbs for RationalVector {
    type Output = Rational;

    /// Computes the Euclidean norm of a [`RationalVector`], the square root of the sum of the
    /// squares of its elements, taking the vector by value, and returns it if it is rational.
    ///
    /// $$
    /// f(v) = \begin{cases}
    ///     \operatorname{Some}(\|v\|) & \text{if} \\ \|v\| \in \mathbb{Q}, \\\\
    ///     \operatorname{None} & \textrm{otherwise},
    /// \end{cases}
    /// $$
    ///
    /// where $\|v\| = \sqrt{\sum_{i=0}^{n-1} v_i^2}$ and $n$ is the dimension of $v$. The sum of
    /// the squares is a rational number in lowest terms, so its square root is rational exactly
    /// when both its numerator and its denominator are perfect squares. The norm of the
    /// 0-dimensional vector is 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O((m + \log n) \log (m + \log n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vector, and $m$ is
    /// the largest number of significant bits of any element numerator or denominator: the squared
    /// norm has $O(m + \log n)$ bits, and its square root costs no more than computing it.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CheckedAbs;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(3/5, -4/5)").unwrap();
    /// assert_eq!(v.clone().checked_abs(), Some(Rational::ONE));
    /// let v = RationalVector::from_str("(1/3, -2/3, 2/3)").unwrap();
    /// assert_eq!(v.clone().checked_abs(), Some(Rational::ONE));
    /// let v = RationalVector::from_str("(1/2, 1/2)").unwrap();
    /// assert_eq!(v.clone().checked_abs(), None);
    /// let v = RationalVector::from_str("(3/2, 2)").unwrap();
    /// assert_eq!(
    ///     v.clone().checked_abs(),
    ///     Some(Rational::from_unsigneds(5u32, 2))
    /// );
    /// ```
    #[inline]
    fn checked_abs(self) -> Option<Rational> {
        self.abs_squared().checked_sqrt()
    }
}

impl CheckedAbs for &RationalVector {
    type Output = Rational;

    /// Computes the Euclidean norm of a [`RationalVector`], the square root of the sum of the
    /// squares of its elements, taking the vector by reference, and returns it if it is rational.
    ///
    /// $$
    /// f(v) = \begin{cases}
    ///     \operatorname{Some}(\|v\|) & \text{if} \\ \|v\| \in \mathbb{Q}, \\\\
    ///     \operatorname{None} & \textrm{otherwise},
    /// \end{cases}
    /// $$
    ///
    /// where $\|v\| = \sqrt{\sum_{i=0}^{n-1} v_i^2}$ and $n$ is the dimension of $v$. The sum of
    /// the squares is a rational number in lowest terms, so its square root is rational exactly
    /// when both its numerator and its denominator are perfect squares. The norm of the
    /// 0-dimensional vector is 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O((m + \log n) \log (m + \log n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vector, and $m$ is
    /// the largest number of significant bits of any element numerator or denominator: the squared
    /// norm has $O(m + \log n)$ bits, and its square root costs no more than computing it.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CheckedAbs;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(3/5, -4/5)").unwrap();
    /// assert_eq!((&v).checked_abs(), Some(Rational::ONE));
    /// let v = RationalVector::from_str("(1/3, -2/3, 2/3)").unwrap();
    /// assert_eq!((&v).checked_abs(), Some(Rational::ONE));
    /// let v = RationalVector::from_str("(1/2, 1/2)").unwrap();
    /// assert_eq!((&v).checked_abs(), None);
    /// let v = RationalVector::from_str("(3/2, 2)").unwrap();
    /// assert_eq!((&v).checked_abs(), Some(Rational::from_unsigneds(5u32, 2)));
    /// ```
    #[inline]
    fn checked_abs(self) -> Option<Rational> {
        self.abs_squared().checked_sqrt()
    }
}
