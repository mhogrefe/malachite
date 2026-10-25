// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use core::cmp::Ordering;
use malachite_base::num::arithmetic::traits::DeterminantSign;
use malachite_base::num::comparison::traits::OrdProducts;

impl DeterminantSign for NaturalMatrix {
    /// Determines the sign of the determinant of a square [`NaturalMatrix`], taking it by value.
    ///
    /// The result is the ordering of the determinant against 0. For dimension 2 it is
    /// [`Natural::cmp_products`](malachite_base::num::comparison::traits::OrdProducts) on the two
    /// diagonals, so the products are formed only if cheaper checks cannot decide. Otherwise it is
    /// the sign for the [`IntegerMatrix`] with the same elements, which is certified without
    /// computing the determinant unless the matrix is singular or ill-conditioned.
    ///
    /// $$
    /// f(A) = \operatorname{sgn}(\det A).
    /// $$
    ///
    /// # Worst-case complexity
    /// Same as the complexity of `determinant_sign` for an [`IntegerMatrix`] with the same
    /// elements.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DeterminantSign;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.clone().determinant_sign(), Less);
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// assert_eq!(a.clone().determinant_sign(), Less);
    ///
    /// let a = NaturalMatrix::from_str("[2, 1, 1; 1, 2, 1; 1, 1, 2]").unwrap();
    /// assert_eq!(a.clone().determinant_sign(), Greater);
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!(a.clone().determinant_sign(), Equal);
    /// ```
    fn determinant_sign(self) -> Ordering {
        assert_eq!(self.height, self.width, "the matrix must be square");
        if let [a00, a01, a10, a11] = self.elements.as_slice() {
            Natural::cmp_products(a00, a11, a01, a10)
        } else {
            IntegerMatrix::from(self).determinant_sign()
        }
    }
}

impl DeterminantSign for &NaturalMatrix {
    /// Determines the sign of the determinant of a square [`NaturalMatrix`], taking it by
    /// reference.
    ///
    /// The result is the ordering of the determinant against 0. For dimension 2 it is
    /// [`Natural::cmp_products`](malachite_base::num::comparison::traits::OrdProducts) on the two
    /// diagonals, so the products are formed only if cheaper checks cannot decide. Otherwise it is
    /// the sign for the [`IntegerMatrix`] with the same elements, which is certified without
    /// computing the determinant unless the matrix is singular or ill-conditioned.
    ///
    /// $$
    /// f(A) = \operatorname{sgn}(\det A).
    /// $$
    ///
    /// # Worst-case complexity
    /// Same as the complexity of `determinant_sign` for an [`IntegerMatrix`] with the same
    /// elements.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DeterminantSign;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).determinant_sign(), Less);
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// assert_eq!((&a).determinant_sign(), Less);
    ///
    /// let a = NaturalMatrix::from_str("[2, 1, 1; 1, 2, 1; 1, 1, 2]").unwrap();
    /// assert_eq!((&a).determinant_sign(), Greater);
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!((&a).determinant_sign(), Equal);
    /// ```
    fn determinant_sign(self) -> Ordering {
        assert_eq!(self.height, self.width, "the matrix must be square");
        if let [a00, a01, a10, a11] = self.elements.as_slice() {
            Natural::cmp_products(a00, a11, a01, a10)
        } else {
            IntegerMatrix::from(self).determinant_sign()
        }
    }
}
