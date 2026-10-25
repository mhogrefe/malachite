// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_matrix::RationalMatrix;
use core::ops::Neg;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::NegAssign;

impl Neg for RationalMatrix {
    type Output = Self;

    /// Negates a [`RationalMatrix`], taking it by value.
    ///
    /// Every element is negated, so the shape is unchanged.
    ///
    /// $$
    /// f(A) = -A.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, -3; 2/3, 0]").unwrap();
    /// assert_eq!((-a).to_string(), "[-1/2, 3; -2/3, 0]");
    /// assert_eq!(
    ///     (-RationalMatrix::from_str("[]_{0,0}").unwrap()).to_string(),
    ///     "[]_{0,0}"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_neg` from `fmpq_mat/neg.c`, FLINT 3.6.0.
    #[inline]
    fn neg(mut self) -> Self {
        self.neg_assign();
        self
    }
}

impl Neg for &RationalMatrix {
    type Output = RationalMatrix;

    /// Negates a [`RationalMatrix`], taking it by reference.
    ///
    /// Every element is negated, so the shape is unchanged.
    ///
    /// $$
    /// f(A) = -A.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements' numerators and denominators.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, -3; 2/3, 0]").unwrap();
    /// assert_eq!((-&a).to_string(), "[-1/2, 3; -2/3, 0]");
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_neg` from `fmpq_mat/neg.c`, FLINT 3.6.0.
    #[inline]
    fn neg(self) -> RationalMatrix {
        RationalMatrix::from_owned_elements(
            self.height(),
            self.width(),
            self.elements.iter().map(|x| -x).collect(),
        )
    }
}

impl NegAssign for RationalMatrix {
    /// Negates a [`RationalMatrix`] in place.
    ///
    /// $$
    /// A \gets -A.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::NegAssign;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let mut a = RationalMatrix::from_str("[1/2, -3; 2/3, 0]").unwrap();
    /// a.neg_assign();
    /// assert_eq!(a.to_string(), "[-1/2, 3; -2/3, 0]");
    /// ```
    #[inline]
    fn neg_assign(&mut self) {
        for x in &mut self.elements {
            x.neg_assign();
        }
    }
}
