// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_matrix::RationalMatrix;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{DivExact, LcmAssign};
use malachite_base::num::basic::traits::One;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural::Natural;

impl RationalMatrix {
    /// Clears the denominators of a [`RationalMatrix`], returning a matrix of [`Integer`]
    /// numerators and a single positive [`Natural`] denominator whose quotients are the elements.
    ///
    /// The denominator is the least common multiple of the elements' denominators, the smallest
    /// that works, and each numerator is its element times that denominator. Because the elements
    /// are in lowest terms, so is the result: no prime divides the denominator and every numerator.
    /// The shape is unchanged, and a matrix with no elements gives the denominator 1. This is
    /// FLINT's `fmpq_mat_get_fmpz_mat_matwise`.
    ///
    /// This is the form in which a sequence of matrix operations over one denominator can be done
    /// in integer arithmetic;
    /// [`from_numerators_and_denominator`](RationalMatrix::from_numerators_and_denominator)
    /// converts back.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn (\log n)^2 \log\log n)$
    ///
    /// $M(n, m) = O(mn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
    /// elements' numerators and denominators, and $m$ is `self.height() * self.width()`. The
    /// denominator can have nearly $n$ bits, and each numerator is scaled up to it.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, -2/3; 5, 1/4]").unwrap();
    /// let (ns, d) = a.to_numerators_and_denominator();
    /// assert_eq!(ns.to_string(), "[6, -8; 60, 3]");
    /// assert_eq!(d, 12);
    ///
    /// let a = RationalMatrix::from_str("[]_{2,0}").unwrap();
    /// let (ns, d) = a.to_numerators_and_denominator();
    /// assert_eq!(ns.to_string(), "[]_{2,0}");
    /// assert_eq!(d, 1);
    /// ```
    pub fn to_numerators_and_denominator(&self) -> (IntegerMatrix, Natural) {
        let mut d = Natural::ONE;
        for x in &self.elements {
            d.lcm_assign(x.denominator_ref());
        }
        let numerators = self
            .elements
            .iter()
            .map(|x| {
                Integer::from_sign_and_abs(
                    *x >= 0u32,
                    (&d).div_exact(x.denominator_ref()) * x.numerator_ref(),
                )
            })
            .collect();
        (
            IntegerMatrix::from_owned_elements(self.height(), self.width(), numerators),
            d,
        )
    }

    /// Clears the denominators of a [`RationalMatrix`], taking the matrix by value and returning a
    /// matrix of [`Integer`] numerators and a single positive [`Natural`] denominator whose
    /// quotients are the elements.
    ///
    /// This is [`to_numerators_and_denominator`](RationalMatrix::to_numerators_and_denominator),
    /// except that each numerator is scaled in place rather than cloned first, and an element whose
    /// denominator is already the common one is moved over unchanged.
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
    /// let (ns, d) = a.into_numerators_and_denominator();
    /// assert_eq!(ns.to_string(), "[6, -8; 60, 3]");
    /// assert_eq!(d, 12);
    ///
    /// let a = RationalMatrix::from_str("[]_{2,0}").unwrap();
    /// let (ns, d) = a.into_numerators_and_denominator();
    /// assert_eq!(ns.to_string(), "[]_{2,0}");
    /// assert_eq!(d, 1);
    /// ```
    pub fn into_numerators_and_denominator(self) -> (IntegerMatrix, Natural) {
        let mut d = Natural::ONE;
        for x in &self.elements {
            d.lcm_assign(x.denominator_ref());
        }
        let (height, width) = (self.height(), self.width());
        let numerators = self
            .elements
            .into_iter()
            .map(|x| {
                let sign = x >= 0u32;
                let (mut n, x_d) = x.into_numerator_and_denominator();
                if x_d != d {
                    n *= (&d).div_exact(x_d);
                }
                Integer::from_sign_and_abs(sign, n)
            })
            .collect();
        (
            IntegerMatrix::from_owned_elements(height, width, numerators),
            d,
        )
    }
}
