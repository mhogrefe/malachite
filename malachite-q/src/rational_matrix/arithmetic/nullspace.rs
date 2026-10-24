// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_matrix::RationalMatrix;
use crate::rational_matrix::arithmetic::reduced_row_echelon_form::rref_in_place;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::Nullspace;
use malachite_base::num::basic::traits::{One, Zero};

// Returns the basis of the right nullspace of `a` given by its reduced row echelon form R, as the
// columns of an n by n - r matrix, where n is the width of `a` and r its rank: for the ith
// non-pivot column, the column of the basis has 1 in the row of that non-pivot column and, in the
// row of each pivot column, the negated element of R in that pivot's row and the non-pivot column.
fn nullspace_helper(mut a: RationalMatrix) -> RationalMatrix {
    let n = a.width;
    let rank = rref_in_place(&mut a);
    let nullity = n - rank;
    let r = &a.elements;
    let mut pivots = Vec::with_capacity(rank);
    let mut nonpivots = Vec::with_capacity(nullity);
    let mut j = 0;
    for row in r.chunks_exact(n.max(1)).take(rank) {
        while row[j] == 0u32 {
            nonpivots.push(j);
            j += 1;
        }
        pivots.push(j);
        j += 1;
    }
    nonpivots.extend(j..n);
    let mut res = vec![Rational::ZERO; n * nullity];
    for (i, &np) in nonpivots.iter().enumerate() {
        for (j, &pivot) in pivots.iter().enumerate() {
            res[pivot * nullity + i] = -&r[j * n + np];
        }
        res[np * nullity + i] = Rational::ONE;
    }
    RationalMatrix {
        elements: res,
        height: n,
        width: nullity,
    }
}

impl Nullspace for RationalMatrix {
    type Output = Self;

    /// Computes a basis of the right nullspace of a [`RationalMatrix`] $A$: the vectors $x$ with
    /// $Ax = 0$, taking the matrix by value.
    ///
    /// If $A$ is $h \times w$ with rank $r$, the result is a $w \times (w - r)$ matrix whose
    /// columns form the basis given by the reduced row echelon form $R$: column $i$ has 1 in the
    /// row of the $i$th non-pivot column of $R$, zeros in the rows of the other non-pivot columns,
    /// and, in the row of each pivot column, the negated element of $R$ in that pivot's row and the
    /// non-pivot column. The basis is unique.
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
    /// use malachite_base::num::arithmetic::traits::Nullspace;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!(a.nullspace().to_string(), "[1; -2; 1]");
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1/3, 1/4; 1, 2/3, 1/2]").unwrap();
    /// assert_eq!(a.nullspace().to_string(), "[-2/3, -1/2; 1, 0; 0, 1]");
    ///
    /// // A nonsingular matrix has a trivial nullspace.
    /// let a = RationalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.nullspace().to_string(), "[]_{2,0}");
    /// ```
    #[inline]
    fn nullspace(self) -> Self {
        nullspace_helper(self)
    }
}

impl Nullspace for &RationalMatrix {
    type Output = RationalMatrix;

    /// Computes a basis of the right nullspace of a [`RationalMatrix`] $A$: the vectors $x$ with
    /// $Ax = 0$, taking the matrix by reference.
    ///
    /// If $A$ is $h \times w$ with rank $r$, the result is a $w \times (w - r)$ matrix whose
    /// columns form the basis given by the reduced row echelon form $R$: column $i$ has 1 in the
    /// row of the $i$th non-pivot column of $R$, zeros in the rows of the other non-pivot columns,
    /// and, in the row of each pivot column, the negated element of $R$ in that pivot's row and the
    /// non-pivot column. The basis is unique.
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
    /// use malachite_base::num::arithmetic::traits::Nullspace;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!((&a).nullspace().to_string(), "[1; -2; 1]");
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1/3, 1/4; 1, 2/3, 1/2]").unwrap();
    /// assert_eq!((&a).nullspace().to_string(), "[-2/3, -1/2; 1, 0; 0, 1]");
    ///
    /// // A nonsingular matrix has a trivial nullspace.
    /// let a = RationalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).nullspace().to_string(), "[]_{2,0}");
    /// ```
    #[inline]
    fn nullspace(self) -> RationalMatrix {
        nullspace_helper(self.clone())
    }
}
