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

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::integer_matrix::arithmetic::reduced_row_echelon_form::{
    pivots_and_nonpivots, rref_in_place,
};
use alloc::vec;
use malachite_base::num::arithmetic::traits::Nullspace;
use malachite_base::num::basic::traits::{One, Zero};

// Returns a basis of the right nullspace of `a`, as the columns of an n by n - r matrix, where n is
// the width of `a` and r its rank: column i has the negated denominator of the fraction-free
// reduced row echelon form in the row of the ith non-pivot column, and in the rows of the pivot
// columns, the elements of that non-pivot column of the form.
//
// This is equivalent to `fmpz_mat_nullspace` from `fmpz_mat/nullspace.c`, FLINT 3.6.0, except that
// the result has exactly as many columns as the nullity.
fn nullspace_helper(mut a: IntegerMatrix) -> IntegerMatrix {
    let n = a.width;
    let rank = rref_in_place(&mut a).0;
    let nullity = n - rank;
    let mut res = vec![Integer::ZERO; n * nullity];
    if rank == 0 {
        for i in 0..nullity {
            res[i * (nullity + 1)] = Integer::ONE;
        }
    } else if nullity != 0 {
        let r = &a.elements;
        let pivots_nonpivots = pivots_and_nonpivots(r, n, rank);
        let (pivots, nonpivots) = pivots_nonpivots.split_at(rank);
        let den = &r[pivots[0]];
        for (i, &np) in nonpivots.iter().enumerate() {
            for (j, &pivot) in pivots.iter().enumerate() {
                res[pivot * nullity + i] = r[j * n + np].clone();
            }
            res[np * nullity + i] = -den;
        }
    }
    IntegerMatrix {
        elements: res,
        height: n,
        width: nullity,
    }
}

impl Nullspace for IntegerMatrix {
    type Output = Self;

    /// Computes a basis of the right nullspace of an [`IntegerMatrix`] $A$: the vectors $x$ with
    /// $Ax = 0$, taking the matrix by value.
    ///
    /// If $A$ is $h \times w$ with rank $r$, the result is a $w \times (w - r)$ matrix whose
    /// columns form a basis of the nullspace over the rationals. It is found from the fraction-free
    /// reduced row echelon form $R$ with denominator $d$: column $i$ has $-d$ in the row of the
    /// $i$th non-pivot column of $R$ and, in the row of each pivot column, the element of $R$ in
    /// that pivot's row and the non-pivot column. When $r = 0$, the basis is that of the standard
    /// unit vectors.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^4 (b + \log n)^2 + n^4 (b + \log n) \log^2 (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^3 (b + \log n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum of the height and width of
    /// the matrix, and $b$ is the maximum number of significant bits of its elements: the elements
    /// of the form have $O(n(b + \log n))$ bits, and the multimodular method solves a nonsingular
    /// system within the bounds of
    /// [`solve`](malachite_base::num::arithmetic::traits::Solve::solve).
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Nullspace;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!(a.nullspace().to_string(), "[3; -6; 3]");
    ///
    /// let a = IntegerMatrix::from_str("[2, 1; 4, 2; 6, 3]").unwrap();
    /// assert_eq!(a.nullspace().to_string(), "[1; -2]");
    ///
    /// // A nonsingular matrix has a trivial nullspace.
    /// let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.nullspace().to_string(), "[]_{2,0}");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_nullspace` from `fmpz_mat/nullspace.c`, FLINT 3.6.0, except
    /// that the result has exactly as many columns as the nullity.
    #[inline]
    fn nullspace(self) -> Self {
        nullspace_helper(self)
    }
}

impl Nullspace for &IntegerMatrix {
    type Output = IntegerMatrix;

    /// Computes a basis of the right nullspace of an [`IntegerMatrix`] $A$: the vectors $x$ with
    /// $Ax = 0$, taking the matrix by reference.
    ///
    /// If $A$ is $h \times w$ with rank $r$, the result is a $w \times (w - r)$ matrix whose
    /// columns form a basis of the nullspace over the rationals. It is found from the fraction-free
    /// reduced row echelon form $R$ with denominator $d$: column $i$ has $-d$ in the row of the
    /// $i$th non-pivot column of $R$ and, in the row of each pivot column, the element of $R$ in
    /// that pivot's row and the non-pivot column. When $r = 0$, the basis is that of the standard
    /// unit vectors.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^4 (b + \log n)^2 + n^4 (b + \log n) \log^2 (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^3 (b + \log n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum of the height and width of
    /// the matrix, and $b$ is the maximum number of significant bits of its elements: the elements
    /// of the form have $O(n(b + \log n))$ bits, and the multimodular method solves a nonsingular
    /// system within the bounds of
    /// [`solve`](malachite_base::num::arithmetic::traits::Solve::solve).
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Nullspace;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!((&a).nullspace().to_string(), "[3; -6; 3]");
    ///
    /// let a = IntegerMatrix::from_str("[2, 1; 4, 2; 6, 3]").unwrap();
    /// assert_eq!((&a).nullspace().to_string(), "[1; -2]");
    ///
    /// // A nonsingular matrix has a trivial nullspace.
    /// let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).nullspace().to_string(), "[]_{2,0}");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_nullspace` from `fmpz_mat/nullspace.c`, FLINT 3.6.0, except
    /// that the result has exactly as many columns as the nullity.
    #[inline]
    fn nullspace(self) -> IntegerMatrix {
        nullspace_helper(self.clone())
    }
}
