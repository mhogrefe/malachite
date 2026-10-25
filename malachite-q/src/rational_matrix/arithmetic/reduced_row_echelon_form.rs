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

use crate::Rational;
use crate::rational_matrix::RationalMatrix;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    ReducedRowEchelonForm, ReducedRowEchelonFormAssign, SubMulAssign,
};
use malachite_base::num::basic::traits::{One, Zero};

// If the element of `a` in row `r` and column `c` is zero, swaps row `r` with the first row below
// it whose element in column `c` is nonzero. Returns 1 if the element was already nonzero, -1 if a
// row was swapped, and 0 if there is no nonzero element at or below row `r` in column `c`.
//
// This is equivalent to `fmpq_mat_pivot` from `fmpq_mat/pivot.c`, FLINT 3.6.0, without the
// permutation.
fn pivot(a: &mut RationalMatrix, r: usize, c: usize) -> i8 {
    let w = a.width;
    if a.elements[r * w + c] != 0u32 {
        return 1;
    }
    for j in r + 1..a.height {
        if a.elements[j * w + c] != 0u32 {
            for k in 0..w {
                a.elements.swap(j * w + k, r * w + k);
            }
            return -1;
        }
    }
    0
}

// Replaces `a` with its reduced row echelon form by Gauss–Jordan elimination with rational
// arithmetic, returning the rank.
//
// This is equivalent to `fmpq_mat_rref_classical` from `fmpq_mat/rref_classical.c`, FLINT 3.6.0.
crate_test_fn! {
rref_classical(a: &mut RationalMatrix) -> usize {
    let (m, n) = (a.height, a.width);
    if m == 0 || n == 0 {
        return 0;
    }
    let mut rank = 0;
    let mut pivot_row = 0;
    let mut pivot_col = 0;
    while pivot_row < m && pivot_col < n {
        if pivot(a, pivot_row, pivot_col) == 0 {
            pivot_col += 1;
            continue;
        }
        rank += 1;
        let b = &mut a.elements;
        // Scale the row to have 1 as its leading element.
        let p = b[pivot_row * n + pivot_col].clone();
        for j in pivot_col + 1..n {
            b[pivot_row * n + j] /= &p;
        }
        // Eliminate the rows above and below.
        for i in 0..m {
            if i == pivot_row || b[i * n + pivot_col] == 0u32 {
                continue;
            }
            let factor = b[i * n + pivot_col].clone();
            for j in pivot_col + 1..n {
                let x = b[pivot_row * n + j].clone();
                b[i * n + j].sub_mul_assign(x, &factor);
            }
        }
        // Clear the pivot column.
        for i in 0..m {
            b[i * n + pivot_col] = if i == pivot_row {
                Rational::ONE
            } else {
                Rational::ZERO
            };
        }
        pivot_row += 1;
        pivot_col += 1;
    }
    rank
}}

// Replaces `a` with its reduced row echelon form, found by clearing the denominators of each row
// and computing the fraction-free reduced row echelon form of the resulting integer matrix,
// returning the rank.
//
// This is equivalent to `fmpq_mat_rref_fraction_free` from `fmpq_mat/rref_fraction_free.c`, FLINT
// 3.6.0.
crate_test_fn! {
rref_fraction_free(a: &mut RationalMatrix) -> usize {
    let (m, n) = (a.height, a.width);
    if m == 0 || n == 0 {
        return 0;
    }
    let numerators = a.to_numerators_and_row_denominators().0;
    let (r, den) = numerators.reduced_row_echelon_form();
    let rank = r
        .elements_ref()
        .chunks_exact(n)
        .take_while(|row| row.iter().any(|x| *x != 0u32))
        .count();
    for (x, y) in a.elements.iter_mut().zip(r.elements_ref()) {
        *x = if rank == 0 {
            Rational::ZERO
        } else {
            Rational::from_integers_ref(&y, &den)
        };
    }
    rank
}}

// Replaces `a` with its reduced row echelon form, returning the rank: by Gauss–Jordan elimination
// with rational arithmetic when `a` has at most 2 rows or columns, and by clearing denominators
// otherwise.
//
// This is equivalent to `fmpq_mat_rref` from `fmpq_mat/rref.c`, FLINT 3.6.0.
pub(crate) fn rref_in_place(a: &mut RationalMatrix) -> usize {
    if a.height <= 2 || a.width <= 2 {
        rref_classical(a)
    } else {
        rref_fraction_free(a)
    }
}

impl ReducedRowEchelonForm for RationalMatrix {
    type Output = Self;

    /// Computes the reduced row echelon form of a [`RationalMatrix`], taking it by value.
    ///
    /// The nonzero rows of the result come first, and their number is the rank; the first nonzero
    /// element of each is 1, the only nonzero element of its column. The form is unique.
    ///
    /// Matrices with at most 2 rows or columns are reduced by Gauss–Jordan elimination with
    /// rational arithmetic; larger ones by clearing the denominators of each row and computing the
    /// fraction-free reduced row echelon form of the resulting
    /// [`IntegerMatrix`](malachite_nz::integer_matrix::IntegerMatrix).
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^6 b^2 + n^5 b \log^2 (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^4 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum of the height and width of
    /// the matrix, and $b$ is the maximum number of significant bits of the numerators and
    /// denominators of its elements: clearing the denominators of the rows gives integer elements
    /// of $O(nb)$ bits.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ReducedRowEchelonForm;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!(
    ///     a.reduced_row_echelon_form().to_string(),
    ///     "[1, 0, -1; 0, 1, 2; 0, 0, 0]"
    /// );
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1/3, 1/4; 1, 2/3, 1/2]").unwrap();
    /// assert_eq!(
    ///     a.reduced_row_echelon_form().to_string(),
    ///     "[1, 2/3, 1/2; 0, 0, 0]"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_rref` from `fmpq_mat/rref.c`, FLINT 3.6.0, except that the
    /// rank is not returned.
    #[inline]
    fn reduced_row_echelon_form(mut self) -> Self {
        rref_in_place(&mut self);
        self
    }
}

impl ReducedRowEchelonForm for &RationalMatrix {
    type Output = RationalMatrix;

    /// Computes the reduced row echelon form of a [`RationalMatrix`], taking it by reference.
    ///
    /// The nonzero rows of the result come first, and their number is the rank; the first nonzero
    /// element of each is 1, the only nonzero element of its column. The form is unique.
    ///
    /// Matrices with at most 2 rows or columns are reduced by Gauss–Jordan elimination with
    /// rational arithmetic; larger ones by clearing the denominators of each row and computing the
    /// fraction-free reduced row echelon form of the resulting
    /// [`IntegerMatrix`](malachite_nz::integer_matrix::IntegerMatrix).
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^6 b^2 + n^5 b \log^2 (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^4 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum of the height and width of
    /// the matrix, and $b$ is the maximum number of significant bits of the numerators and
    /// denominators of its elements: clearing the denominators of the rows gives integer elements
    /// of $O(nb)$ bits.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ReducedRowEchelonForm;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!(
    ///     (&a).reduced_row_echelon_form().to_string(),
    ///     "[1, 0, -1; 0, 1, 2; 0, 0, 0]"
    /// );
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1/3, 1/4; 1, 2/3, 1/2]").unwrap();
    /// assert_eq!(
    ///     (&a).reduced_row_echelon_form().to_string(),
    ///     "[1, 2/3, 1/2; 0, 0, 0]"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_rref` from `fmpq_mat/rref.c`, FLINT 3.6.0, except that the
    /// rank is not returned.
    #[inline]
    fn reduced_row_echelon_form(self) -> RationalMatrix {
        let mut a = self.clone();
        rref_in_place(&mut a);
        a
    }
}

impl ReducedRowEchelonFormAssign for RationalMatrix {
    type Output = ();

    /// Replaces a [`RationalMatrix`] with its reduced row echelon form.
    ///
    /// The nonzero rows of the result come first, and their number is the rank; the first nonzero
    /// element of each is 1, the only nonzero element of its column. The form is unique.
    ///
    /// Matrices with at most 2 rows or columns are reduced by Gauss–Jordan elimination with
    /// rational arithmetic; larger ones by clearing the denominators of each row and computing the
    /// fraction-free reduced row echelon form of the resulting
    /// [`IntegerMatrix`](malachite_nz::integer_matrix::IntegerMatrix).
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^6 b^2 + n^5 b \log^2 (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^4 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum of the height and width of
    /// the matrix, and $b$ is the maximum number of significant bits of the numerators and
    /// denominators of its elements: clearing the denominators of the rows gives integer elements
    /// of $O(nb)$ bits.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ReducedRowEchelonFormAssign;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let mut a = RationalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// a.reduced_row_echelon_form_assign();
    /// assert_eq!(a.to_string(), "[1, 0, -1; 0, 1, 2; 0, 0, 0]");
    ///
    /// let mut a = RationalMatrix::from_str("[1/2, 1/3, 1/4; 1, 2/3, 1/2]").unwrap();
    /// a.reduced_row_echelon_form_assign();
    /// assert_eq!(a.to_string(), "[1, 2/3, 1/2; 0, 0, 0]");
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_rref` from `fmpq_mat/rref.c`, FLINT 3.6.0, except that the
    /// rank is not returned.
    #[inline]
    fn reduced_row_echelon_form_assign(&mut self) {
        rref_in_place(self);
    }
}
