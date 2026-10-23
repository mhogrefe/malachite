// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010-2011 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::integer_matrix::arithmetic::solve::solve_nonsingular;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{Inverse, MulSubMul};
use malachite_base::num::basic::traits::{One, Zero};

// Returns the inverse of a square matrix A as a matrix B and a denominator d, with AB = dI, or
// `None` if A is singular.
//
// This is equivalent to `fmpz_mat_inv` from `fmpz_mat/inv.c`, FLINT 3.6.0, except that it panics if
// the matrix is not square.
fn inverse_helper(a: &IntegerMatrix) -> Option<(IntegerMatrix, Integer)> {
    assert_eq!(a.height, a.width, "the matrix must be square");
    let n = a.height;
    let matrix = |elements: Vec<Integer>| IntegerMatrix {
        elements,
        height: n,
        width: n,
    };
    match n {
        0 => Some((matrix(Vec::new()), Integer::ONE)),
        1 => {
            let den = a.elements[0].clone();
            if den == 0u32 {
                None
            } else {
                Some((matrix(vec![Integer::ONE]), den))
            }
        }
        2 => {
            let [a00, a01, a10, a11] = a.elements.as_slice() else {
                unreachable!()
            };
            let den = a00.mul_sub_mul(a11, a01, a10);
            if den == 0u32 {
                None
            } else {
                Some((matrix(vec![a11.clone(), -a01, -a10, a00.clone()]), den))
            }
        }
        _ => {
            let mut identity = vec![Integer::ZERO; n * n];
            for i in 0..n {
                identity[i * (n + 1)] = Integer::ONE;
            }
            solve_nonsingular(a, &matrix(identity))
        }
    }
}

impl Inverse for IntegerMatrix {
    type Output = (Self, Integer);

    /// Finds the inverse of a square [`IntegerMatrix`] $A$, as a matrix $B$ and a nonzero
    /// [`Integer`] $d$ with $AB = BA = dI$, so that $A^{-1} = B/d$, taking the matrix by value.
    ///
    /// `None` is returned if $A$ is singular. If $A$ has dimension at most 15, $d$ is $\det A$;
    /// otherwise, it is the least positive common denominator of $A^{-1}$. In general, $B$ and $d$
    /// may have a common factor, and $d$ may be negative.
    ///
    /// Matrices of dimension at most 2 are inverted directly, and larger ones by solving $AB = dI$
    /// as [`solve`](malachite_base::num::arithmetic::traits::Solve::solve) does.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^4 (b + \log n)^2 + n^4 (b + \log n) \log^2 (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^3 (b + \log n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of $A$, and $b$ is the
    /// maximum number of significant bits of the elements of $A$: the elements of the solution have
    /// $O(n(b + \log n))$ bits, and the multimodular and $p$-adic algorithms use $O(n(b + \log n))$
    /// word-sized primes or lifting steps, each costing $O(n^3)$ word operations and $O(n^2)$
    /// updates of numbers of that many bits, with $O(\log (nb))$ checks of candidate solutions.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Inverse;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// // The inverse is [-2, 1; 3/2, -1/2].
    /// let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let (b, d) = a.inverse().unwrap();
    /// assert_eq!(b.to_string(), "[4, -2; -3, 1]");
    /// assert_eq!(d.to_string(), "-2");
    ///
    /// // A singular matrix
    /// let a = IntegerMatrix::from_str("[2, 4; 1, 2]").unwrap();
    /// assert_eq!(a.inverse(), None);
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_inv` from `fmpz_mat/inv.c`, FLINT 3.6.0, except that it
    /// panics if the matrix is not square.
    #[inline]
    fn inverse(self) -> Option<(Self, Integer)> {
        inverse_helper(&self)
    }
}

impl Inverse for &IntegerMatrix {
    type Output = (IntegerMatrix, Integer);

    /// Finds the inverse of a square [`IntegerMatrix`] $A$, as a matrix $B$ and a nonzero
    /// [`Integer`] $d$ with $AB = BA = dI$, so that $A^{-1} = B/d$, taking the matrix by reference.
    ///
    /// `None` is returned if $A$ is singular. If $A$ has dimension at most 15, $d$ is $\det A$;
    /// otherwise, it is the least positive common denominator of $A^{-1}$. In general, $B$ and $d$
    /// may have a common factor, and $d$ may be negative.
    ///
    /// Matrices of dimension at most 2 are inverted directly, and larger ones by solving $AB = dI$
    /// as [`solve`](malachite_base::num::arithmetic::traits::Solve::solve) does.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^4 (b + \log n)^2 + n^4 (b + \log n) \log^2 (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^3 (b + \log n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of $A$, and $b$ is the
    /// maximum number of significant bits of the elements of $A$: the elements of the solution have
    /// $O(n(b + \log n))$ bits, and the multimodular and $p$-adic algorithms use $O(n(b + \log n))$
    /// word-sized primes or lifting steps, each costing $O(n^3)$ word operations and $O(n^2)$
    /// updates of numbers of that many bits, with $O(\log (nb))$ checks of candidate solutions.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Inverse;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// // The inverse is [-2, 1; 3/2, -1/2].
    /// let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// let (b, d) = (&a).inverse().unwrap();
    /// assert_eq!(b.to_string(), "[4, -2; -3, 1]");
    /// assert_eq!(d.to_string(), "-2");
    ///
    /// // A singular matrix
    /// let a = IntegerMatrix::from_str("[2, 4; 1, 2]").unwrap();
    /// assert_eq!((&a).inverse(), None);
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_inv` from `fmpz_mat/inv.c`, FLINT 3.6.0, except that it
    /// panics if the matrix is not square.
    #[inline]
    fn inverse(self) -> Option<(IntegerMatrix, Integer)> {
        inverse_helper(self)
    }
}
