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
use alloc::vec::Vec;
use core::iter::repeat_n;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{DivExact, LcmAssign};
use malachite_base::num::basic::traits::One;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;

impl RationalMatrix {
    /// Clears the denominators of each row of a [`RationalMatrix`], returning a matrix of
    /// [`Integer`] numerators and a [`NaturalVector`] of positive denominators, one for each row.
    ///
    /// The denominator of row $i$ is the least common multiple of the denominators of the elements
    /// in that row, the smallest that works, and each numerator is its element times its row's
    /// denominator, so that dividing every element of row $i$ of the numerators by element $i$ of
    /// the denominators gives back the matrix. Because the elements are in lowest terms, no prime
    /// divides a row's denominator and every numerator in that row. The shape is unchanged; a
    /// matrix with no columns gives every row the denominator 1, and a matrix with no rows gives no
    /// denominators. This is FLINT's `fmpq_mat_get_fmpz_mat_rowwise`, except that FLINT leaves the
    /// denominators unset when the matrix has no columns.
    ///
    /// Clearing the rows separately, rather than the whole matrix as
    /// [`to_numerators_and_denominator`](RationalMatrix::to_numerators_and_denominator) does, keeps
    /// the numerators smaller when the rows have different denominators, and is what the product of
    /// two [`RationalMatrix`]s uses: the rows of the first factor are cleared.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn (\log n)^2 \log\log n)$
    ///
    /// $M(n, m) = O(mn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
    /// elements' numerators and denominators, and $m$ is `self.elements.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, -2/3; 5, 1/4]").unwrap();
    /// let (ns, ds) = a.to_numerators_and_row_denominators();
    /// assert_eq!(ns.to_string(), "[3, -4; 20, 1]");
    /// assert_eq!(ds.to_string(), "(6, 4)");
    ///
    /// let a = RationalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let (ns, ds) = a.to_numerators_and_row_denominators();
    /// assert_eq!(ns.to_string(), "[1, 2; 3, 4]");
    /// assert_eq!(ds.to_string(), "(1, 1)");
    ///
    /// let a = RationalMatrix::from_str("[]_{2,0}").unwrap();
    /// let (ns, ds) = a.to_numerators_and_row_denominators();
    /// assert_eq!(ns.to_string(), "[]_{2,0}");
    /// assert_eq!(ds.to_string(), "(1, 1)");
    ///
    /// let a = RationalMatrix::from_str("[]_{0,3}").unwrap();
    /// let (ns, ds) = a.to_numerators_and_row_denominators();
    /// assert_eq!(ns.to_string(), "[]_{0,3}");
    /// assert_eq!(ds.to_string(), "()");
    /// ```
    pub fn to_numerators_and_row_denominators(&self) -> (IntegerMatrix, NaturalVector) {
        let mut denominators = Vec::with_capacity(self.height);
        for i in 0..self.height() {
            let mut d = Natural::ONE;
            for x in self.row_iter(i) {
                d.lcm_assign(x.denominator_ref());
            }
            denominators.push(d);
        }
        let numerators = self
            .elements
            .iter()
            .zip(denominators.iter().flat_map(|d| repeat_n(d, self.width)))
            .map(|(x, d)| {
                let n = if *d == 1u32 {
                    x.numerator_ref().clone()
                } else {
                    d.div_exact(x.denominator_ref()) * x.numerator_ref()
                };
                Integer::from_sign_and_abs(*x >= 0u32, n)
            })
            .collect();
        (
            IntegerMatrix::from_owned_elements(self.height(), self.width(), numerators),
            NaturalVector {
                elements: denominators,
            },
        )
    }

    /// Clears the denominators of each row of a [`RationalMatrix`], taking the matrix by value and
    /// returning a matrix of [`Integer`] numerators and a [`NaturalVector`] of positive
    /// denominators, one for each row.
    ///
    /// This is [`to_numerators_and_row_denominators`](Self::to_numerators_and_row_denominators),
    /// except that each numerator is scaled in place rather than cloned first, and an element whose
    /// denominator is already its row's is moved over unchanged.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn (\log n)^2 \log\log n)$
    ///
    /// $M(n, m) = O(mn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
    /// elements' numerators and denominators, and $m$ is `self.elements.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, -2/3; 5, 1/4]").unwrap();
    /// let (ns, ds) = a.into_numerators_and_row_denominators();
    /// assert_eq!(ns.to_string(), "[3, -4; 20, 1]");
    /// assert_eq!(ds.to_string(), "(6, 4)");
    ///
    /// let a = RationalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let (ns, ds) = a.into_numerators_and_row_denominators();
    /// assert_eq!(ns.to_string(), "[1, 2; 3, 4]");
    /// assert_eq!(ds.to_string(), "(1, 1)");
    ///
    /// let a = RationalMatrix::from_str("[]_{2,0}").unwrap();
    /// let (ns, ds) = a.into_numerators_and_row_denominators();
    /// assert_eq!(ns.to_string(), "[]_{2,0}");
    /// assert_eq!(ds.to_string(), "(1, 1)");
    ///
    /// let a = RationalMatrix::from_str("[]_{0,3}").unwrap();
    /// let (ns, ds) = a.into_numerators_and_row_denominators();
    /// assert_eq!(ns.to_string(), "[]_{0,3}");
    /// assert_eq!(ds.to_string(), "()");
    /// ```
    pub fn into_numerators_and_row_denominators(self) -> (IntegerMatrix, NaturalVector) {
        let mut denominators = Vec::with_capacity(self.height);
        for i in 0..self.height() {
            let mut d = Natural::ONE;
            for x in self.row_iter(i) {
                d.lcm_assign(x.denominator_ref());
            }
            denominators.push(d);
        }
        let (height, width) = (self.height(), self.width());
        let width_usize = self.width;
        let numerators = self
            .elements
            .into_iter()
            .zip(denominators.iter().flat_map(|d| repeat_n(d, width_usize)))
            .map(|(x, d)| {
                let sign = x >= 0u32;
                let (mut n, x_d) = x.into_numerator_and_denominator();
                if x_d != *d {
                    n *= d.div_exact(x_d);
                }
                Integer::from_sign_and_abs(sign, n)
            })
            .collect();
        (
            IntegerMatrix::from_owned_elements(height, width, numerators),
            NaturalVector {
                elements: denominators,
            },
        )
    }
}
