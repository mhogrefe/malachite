// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_vector::RationalVector;
use malachite_base::num::arithmetic::traits::{AbsSquared, Square};
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;

// The squared norm of the vector `numerators / denominator`: the squared norm of the integer
// numerators over the square of the denominator, reduced once.
fn abs_squared_over((numerators, denominator): (IntegerVector, Natural)) -> Rational {
    Rational::from_naturals(numerators.abs_squared(), denominator.square())
}

impl AbsSquared for RationalVector {
    type Output = Rational;

    /// Computes the squared norm of a [`RationalVector`], the sum of the squares of its elements,
    /// taking the vector by value.
    ///
    /// This is the dot product of the vector with itself, $\|v\|^2$. The squared norm of the
    /// 0-dimensional vector is 0. The vector is written as integer numerators over a common
    /// denominator, as in [`Dot`](malachite_base::num::arithmetic::traits::Dot), so that one
    /// integer squared norm and one reduction suffice.
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
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3)").unwrap();
    /// assert_eq!(v.abs_squared().to_string(), "25/36");
    /// let v = RationalVector::from_str("()").unwrap();
    /// assert_eq!(v.abs_squared(), 0);
    /// ```
    #[inline]
    fn abs_squared(self) -> Rational {
        abs_squared_over(self.into_numerators_and_denominator())
    }
}

impl AbsSquared for &RationalVector {
    type Output = Rational;

    /// Computes the squared norm of a [`RationalVector`], the sum of the squares of its elements,
    /// taking the vector by reference.
    ///
    /// This is the dot product of the vector with itself, $\|v\|^2$. The squared norm of the
    /// 0-dimensional vector is 0. The vector is written as integer numerators over a common
    /// denominator, as in [`Dot`](malachite_base::num::arithmetic::traits::Dot), so that one
    /// integer squared norm and one reduction suffice.
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
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3)").unwrap();
    /// assert_eq!((&v).abs_squared().to_string(), "25/36");
    /// let v = RationalVector::from_str("()").unwrap();
    /// assert_eq!((&v).abs_squared(), 0);
    /// ```
    #[inline]
    fn abs_squared(self) -> Rational {
        abs_squared_over(self.to_numerators_and_denominator())
    }
}
