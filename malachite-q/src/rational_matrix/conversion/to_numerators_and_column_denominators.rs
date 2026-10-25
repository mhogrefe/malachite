// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_matrix::RationalMatrix;
use crate::rational_matrix::conversion::to_numerators_and_row_denominators::{
    into_scaled_numerators, scaled_numerators,
};
use alloc::vec::Vec;

use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::LcmAssign;
use malachite_base::num::basic::traits::One;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;

impl RationalMatrix {
    /// Clears the denominators of each column of a [`RationalMatrix`], returning a matrix of
    /// [`Integer`](malachite_nz::integer::Integer) numerators and a [`NaturalVector`] of positive
    /// denominators, one for each column.
    ///
    /// The denominator of column $j$ is the least common multiple of the denominators of the
    /// elements in that column, the smallest that works, and each numerator is its element times
    /// its column's denominator, so that dividing every element of column $j$ of the numerators by
    /// element $j$ of the denominators gives back the matrix. Because the elements are in lowest
    /// terms, no prime divides a column's denominator and every numerator in that column. The shape
    /// is unchanged; a matrix with no rows gives every column the denominator 1, and a matrix with
    /// no columns gives no denominators. This is FLINT's `fmpq_mat_get_fmpz_mat_colwise`, except
    /// that FLINT leaves the denominators unset when the matrix has no rows.
    ///
    /// Clearing the columns separately, rather than the whole matrix as
    /// [`to_numerators_and_denominator`](RationalMatrix::to_numerators_and_denominator) does, keeps
    /// the numerators smaller when the columns have different denominators, and is what the product
    /// of two [`RationalMatrix`]s uses: the columns of the second factor are cleared.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn (\log n)^2 \log\log n)$
    ///
    /// $M(n, m) = O(mn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
    /// elements' numerators and denominators, and $m$ is `self.height() * self.width()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, -2/3; 5, 1/4]").unwrap();
    /// let (ns, ds) = a.to_numerators_and_column_denominators();
    /// assert_eq!(ns.to_string(), "[1, -8; 10, 3]");
    /// assert_eq!(ds.to_string(), "(2, 12)");
    ///
    /// let a = RationalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let (ns, ds) = a.to_numerators_and_column_denominators();
    /// assert_eq!(ns.to_string(), "[1, 2; 3, 4]");
    /// assert_eq!(ds.to_string(), "(1, 1)");
    ///
    /// let a = RationalMatrix::from_str("[]_{2,0}").unwrap();
    /// let (ns, ds) = a.to_numerators_and_column_denominators();
    /// assert_eq!(ns.to_string(), "[]_{2,0}");
    /// assert_eq!(ds.to_string(), "()");
    ///
    /// let a = RationalMatrix::from_str("[]_{0,3}").unwrap();
    /// let (ns, ds) = a.to_numerators_and_column_denominators();
    /// assert_eq!(ns.to_string(), "[]_{0,3}");
    /// assert_eq!(ds.to_string(), "(1, 1, 1)");
    /// ```
    pub fn to_numerators_and_column_denominators(&self) -> (IntegerMatrix, NaturalVector) {
        let denominators = self.column_denominators();
        let numerators = self.scale_by_column_denominators(&denominators);
        (
            numerators,
            NaturalVector {
                elements: denominators,
            },
        )
    }

    /// Clears the denominators of each column of a [`RationalMatrix`], taking the matrix by value
    /// and returning a matrix of [`Integer`](malachite_nz::integer::Integer) numerators and a
    /// [`NaturalVector`] of positive denominators, one for each column.
    ///
    /// This is
    /// [`to_numerators_and_column_denominators`](Self::to_numerators_and_column_denominators),
    /// except that each numerator is scaled in place rather than cloned first, and an element whose
    /// denominator is already its column's is moved over unchanged.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn (\log n)^2 \log\log n)$
    ///
    /// $M(n, m) = O(mn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
    /// elements' numerators and denominators, and $m$ is `self.height() * self.width()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, -2/3; 5, 1/4]").unwrap();
    /// let (ns, ds) = a.into_numerators_and_column_denominators();
    /// assert_eq!(ns.to_string(), "[1, -8; 10, 3]");
    /// assert_eq!(ds.to_string(), "(2, 12)");
    ///
    /// let a = RationalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let (ns, ds) = a.into_numerators_and_column_denominators();
    /// assert_eq!(ns.to_string(), "[1, 2; 3, 4]");
    /// assert_eq!(ds.to_string(), "(1, 1)");
    ///
    /// let a = RationalMatrix::from_str("[]_{2,0}").unwrap();
    /// let (ns, ds) = a.into_numerators_and_column_denominators();
    /// assert_eq!(ns.to_string(), "[]_{2,0}");
    /// assert_eq!(ds.to_string(), "()");
    ///
    /// let a = RationalMatrix::from_str("[]_{0,3}").unwrap();
    /// let (ns, ds) = a.into_numerators_and_column_denominators();
    /// assert_eq!(ns.to_string(), "[]_{0,3}");
    /// assert_eq!(ds.to_string(), "(1, 1, 1)");
    /// ```
    pub fn into_numerators_and_column_denominators(self) -> (IntegerMatrix, NaturalVector) {
        let denominators = self.column_denominators();
        let numerators = self.into_scaled_by_column_denominators(&denominators);
        (
            numerators,
            NaturalVector {
                elements: denominators,
            },
        )
    }

    // The least common multiple of the denominators in each column.
    pub(crate) fn column_denominators(&self) -> Vec<Natural> {
        (0..self.width())
            .map(|j| {
                let mut d = Natural::ONE;
                for x in self.column_iter(j) {
                    d.lcm_assign(x.denominator_ref());
                }
                d
            })
            .collect()
    }

    // The elements times the denominators of their columns, each a multiple of its elements'
    // denominators.
    pub(crate) fn scale_by_column_denominators(&self, denominators: &[Natural]) -> IntegerMatrix {
        IntegerMatrix::from_owned_elements(
            self.height(),
            self.width(),
            scaled_numerators(&self.elements, denominators.iter().cycle()),
        )
    }

    // The elements times the denominators of their columns, taking the matrix by value.
    pub(crate) fn into_scaled_by_column_denominators(
        self,
        denominators: &[Natural],
    ) -> IntegerMatrix {
        let (height, width) = (self.height(), self.width());
        IntegerMatrix::from_owned_elements(
            height,
            width,
            into_scaled_numerators(self.elements, denominators.iter().cycle()),
        )
    }
}
