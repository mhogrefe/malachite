// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2026 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;
use crate::integer_matrix::arithmetic::mag::Mag;
use crate::natural::Natural;
use alloc::vec;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::WrappingFrom;

// An upper bound for the absolute values of the coefficients of the characteristic polynomial of
// the square matrix `a`: the coefficient of $x^{n-k}$ is a sum of $\binom{n}{k}$ principal minors
// of size $k$, each bounded by Hadamard's inequality with the $k$ largest row norms, or the $k$
// largest column norms.
//
// This is equivalent to `fmpz_mat_charpoly_bound` from `fmpz_mat/charpoly_bound.c`, FLINT 3.6.0.
pub(crate) fn characteristic_polynomial_bound_helper(a: &IntegerMatrix) -> Natural {
    let n = a.height;
    if n == 0 {
        return Natural::ONE;
    }
    let mut norms = vec![Mag::ZERO; n << 1];
    let (row_norms, column_norms) = norms.split_at_mut(n);
    for (i, row) in a.elements.chunks_exact(n).enumerate() {
        for (j, x) in row.iter().enumerate() {
            let t = Mag::from_natural(x.unsigned_abs_ref());
            row_norms[i].add_mul_assign(t, t);
            column_norms[j].add_mul_assign(t, t);
        }
    }
    for x in row_norms.iter_mut().chain(column_norms.iter_mut()) {
        *x = x.sqrt();
    }
    // Sort the norms in ascending order; they are read in reverse.
    row_norms.sort_unstable_by(|&x, &y| Mag::cmp(x, y));
    column_norms.sort_unstable_by(|&x, &y| Mag::cmp(x, y));
    let n_u64 = u64::wrapping_from(n);
    let mut binomials = vec![Mag::ZERO; (n >> 1) + 2];
    binomials[0] = Mag::ONE;
    binomials[1] = Mag::from_u64(n_u64);
    for i in 2..=n >> 1 {
        let i_u64 = u64::wrapping_from(i);
        binomials[i] = binomials[i - 1].mul_u64(n_u64 - i_u64 + 1).div_u64(i_u64);
    }
    let mut row_product = Mag::ONE;
    let mut column_product = Mag::ONE;
    let mut row_bound = Mag::ONE;
    let mut column_bound = Mag::ONE;
    for j in 0..n {
        let k = j + 1;
        let binomial = binomials[if k <= n >> 1 { k } else { n - k }];
        row_product = Mag::mul(row_product, row_norms[n - 1 - j]);
        column_product = Mag::mul(column_product, column_norms[n - 1 - j]);
        row_bound = Mag::max(row_bound, Mag::mul(binomial, row_product));
        column_bound = Mag::max(column_bound, Mag::mul(binomial, column_product));
    }
    Mag::min(row_bound, column_bound).ceiling()
}

impl IntegerMatrix {
    /// Returns an upper bound for the absolute values of the coefficients of the characteristic
    /// polynomial of a square [`IntegerMatrix`].
    ///
    /// The coefficient of $x^{n-k}$ in the characteristic polynomial of an $n \times n$ matrix is
    /// $(-1)^k$ times the sum of its $\binom{n}{k}$ principal minors of size $k$. By Hadamard's
    /// inequality, each of those is at most the product of the $k$ largest Euclidean norms of the
    /// rows, and also at most the product of the $k$ largest Euclidean norms of the columns. The
    /// bound is the smaller of the row and column bounds, each the largest, over $k$, of
    /// $\binom{n}{k}$ times the product of the $k$ largest norms, and of 1, for the leading
    /// coefficient. The norms, binomial coefficients, and products are computed with floating-point
    /// magnitudes whose 30-bit mantissas are always rounded up, so the bound is rigorous but may
    /// exceed the exact one slightly. The bound is 1 for the $0 \times 0$ matrix and for a zero
    /// matrix.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height()`.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[]_{0,0}").unwrap();
    /// assert_eq!(a.characteristic_polynomial_bound(), 1u32);
    ///
    /// // The characteristic polynomial is x^2-5*x-2.
    /// let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.characteristic_polynomial_bound(), 12u32);
    ///
    /// // The characteristic polynomial is x^2-6*x+25.
    /// let a = IntegerMatrix::from_str("[3, -4; 4, 3]").unwrap();
    /// assert_eq!(a.characteristic_polynomial_bound(), 26u32);
    ///
    /// let a = IntegerMatrix::from_str("[0, 0; 0, 0]").unwrap();
    /// assert_eq!(a.characteristic_polynomial_bound(), 1u32);
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_charpoly_bound` from `fmpz_mat/charpoly_bound.c`, FLINT
    /// 3.6.0, except that it panics if the matrix is not square.
    pub fn characteristic_polynomial_bound(&self) -> Natural {
        assert_eq!(self.height, self.width, "the matrix must be square");
        characteristic_polynomial_bound_helper(self)
    }
}
