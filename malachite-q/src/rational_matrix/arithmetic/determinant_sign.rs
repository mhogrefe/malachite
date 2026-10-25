// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_matrix::RationalMatrix;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{DeterminantSign, Sign};
use malachite_base::num::comparison::traits::OrdProducts;

impl DeterminantSign for RationalMatrix {
    /// Determines the sign of the determinant of a square [`RationalMatrix`], taking it by value.
    ///
    /// The result is the ordering of the determinant against 0. For dimension 2 it is
    /// [`Rational::cmp_products`](malachite_base::num::comparison::traits::OrdProducts) on the two
    /// diagonals, so the products are formed only if cheaper checks cannot decide. Otherwise each
    /// row is multiplied by the least common multiple of its denominators, which leaves the sign
    /// unchanged, and the sign is that for the resulting
    /// [`IntegerMatrix`](malachite_nz::integer_matrix::IntegerMatrix), which is certified without
    /// computing the determinant unless the matrix is singular or ill-conditioned.
    ///
    /// $$
    /// f(A) = \operatorname{sgn}(\det A).
    /// $$
    ///
    /// # Worst-case complexity
    /// Same as the complexity of `determinant`.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DeterminantSign;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, 2/3; 3/4, 4/5]").unwrap();
    /// assert_eq!(a.clone().determinant_sign(), Less);
    ///
    /// let a = RationalMatrix::from_str("[1/2, -2, 3; 4, 5/3, -6; -7, 8, 10]").unwrap();
    /// assert_eq!(a.clone().determinant_sign(), Greater);
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1, 3/2; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!(a.clone().determinant_sign(), Equal);
    /// ```
    fn determinant_sign(self) -> Ordering {
        assert_eq!(self.height, self.width, "the matrix must be square");
        match self.elements.as_slice() {
            [] => Greater,
            [a00] => a00.sign(),
            [a00, a01, a10, a11] => Rational::cmp_products(a00, a11, a01, a10),
            _ => self
                .to_numerators_and_row_denominators()
                .0
                .determinant_sign(),
        }
    }
}

impl DeterminantSign for &RationalMatrix {
    /// Determines the sign of the determinant of a square [`RationalMatrix`], taking it by
    /// reference.
    ///
    /// The result is the ordering of the determinant against 0. For dimension 2 it is
    /// [`Rational::cmp_products`](malachite_base::num::comparison::traits::OrdProducts) on the two
    /// diagonals, so the products are formed only if cheaper checks cannot decide. Otherwise each
    /// row is multiplied by the least common multiple of its denominators, which leaves the sign
    /// unchanged, and the sign is that for the resulting
    /// [`IntegerMatrix`](malachite_nz::integer_matrix::IntegerMatrix), which is certified without
    /// computing the determinant unless the matrix is singular or ill-conditioned.
    ///
    /// $$
    /// f(A) = \operatorname{sgn}(\det A).
    /// $$
    ///
    /// # Worst-case complexity
    /// Same as the complexity of `determinant`.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DeterminantSign;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, 2/3; 3/4, 4/5]").unwrap();
    /// assert_eq!((&a).determinant_sign(), Less);
    ///
    /// let a = RationalMatrix::from_str("[1/2, -2, 3; 4, 5/3, -6; -7, 8, 10]").unwrap();
    /// assert_eq!((&a).determinant_sign(), Greater);
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1, 3/2; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!((&a).determinant_sign(), Equal);
    /// ```
    fn determinant_sign(self) -> Ordering {
        assert_eq!(self.height, self.width, "the matrix must be square");
        match self.elements.as_slice() {
            [] => Greater,
            [a00] => a00.sign(),
            [a00, a01, a10, a11] => Rational::cmp_products(a00, a11, a01, a10),
            _ => self
                .to_numerators_and_row_denominators()
                .0
                .determinant_sign(),
        }
    }
}
