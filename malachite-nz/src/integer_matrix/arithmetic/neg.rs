// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;
use core::ops::Neg;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::NegAssign;

impl Neg for IntegerMatrix {
    type Output = Self;

    /// Negates an [`IntegerMatrix`], taking it by value.
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
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, -3; 2, 0]").unwrap();
    /// assert_eq!((-a).to_string(), "[-1, 3; -2, 0]");
    /// assert_eq!(
    ///     (-IntegerMatrix::from_str("[]_{0,0}").unwrap()).to_string(),
    ///     "[]_{0,0}"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_neg` from `fmpz_mat/neg.c`, FLINT 3.6.0.
    #[inline]
    fn neg(mut self) -> Self {
        self.neg_assign();
        self
    }
}

impl Neg for &IntegerMatrix {
    type Output = IntegerMatrix;

    /// Negates an [`IntegerMatrix`], taking it by reference.
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
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, -3; 2, 0]").unwrap();
    /// assert_eq!((-&a).to_string(), "[-1, 3; -2, 0]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_neg` from `fmpz_mat/neg.c`, FLINT 3.6.0.
    #[inline]
    fn neg(self) -> IntegerMatrix {
        IntegerMatrix::from_owned_elements(
            self.height(),
            self.width(),
            self.elements.iter().map(|x| -x).collect(),
        )
    }
}

impl NegAssign for IntegerMatrix {
    /// Negates an [`IntegerMatrix`] in place.
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
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let mut a = IntegerMatrix::from_str("[1, -3; 2, 0]").unwrap();
    /// a.neg_assign();
    /// assert_eq!(a.to_string(), "[-1, 3; -2, 0]");
    /// ```
    #[inline]
    fn neg_assign(&mut self) {
        for x in &mut self.elements {
            x.neg_assign();
        }
    }
}
