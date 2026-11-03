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
use crate::integer_polynomial::IntegerPolynomial;
use crate::integer_polynomial::arithmetic::mul::mul_greater_to_out;
use alloc::vec;
use malachite_base::num::arithmetic::traits::{NegAssign, SubMulAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::polynomial::arithmetic::traits::FromRoots;

// The number of roots below which the product is built one factor at a time.
const FROM_ROOTS_BASECASE_CUTOFF: usize = 20;

// Writes into `out`, which has length `xs.len() + 1`, the coefficients of $(x - x_0)(x - x_1)
// \cdots (x - x_{n-1})$.
//
// This is equivalent to `_fmpz_poly_product_roots_fmpz_vec` from
// `fmpz_poly/product_roots_fmpz_vec.c`, FLINT 3.6.0.
fn from_roots_to_out(out: &mut [Integer], xs: &[Integer]) {
    let n = xs.len();
    assert_eq!(out.len(), n + 1);
    if n == 0 {
        out[0] = Integer::ONE;
    } else if n < FROM_ROOTS_BASECASE_CUTOFF {
        out[n] = Integer::ONE;
        out[n - 1] = -&xs[0];
        for i in 1..n {
            out[n - i - 1] = &out[n - i] * &xs[i];
            out[n - i - 1].neg_assign();
            for j in 0..i - 1 {
                let (lo, hi) = out.split_at_mut(n - i + j + 1);
                lo[n - i + j].sub_mul_assign(&hi[0], &xs[i]);
            }
            out[n - 1] -= &xs[i];
        }
    } else {
        let half = n.div_ceil(2);
        let mut tmp = vec![Integer::ZERO; n + 2];
        let (left, right) = tmp.split_at_mut(half + 1);
        from_roots_to_out(left, &xs[..half]);
        from_roots_to_out(right, &xs[half..]);
        mul_greater_to_out(out, left, right);
    }
}

impl FromRoots<Integer> for IntegerPolynomial {
    /// Builds the monic [`IntegerPolynomial`] whose roots, with multiplicity, are the values in
    /// `roots`.
    ///
    /// $$
    /// f((r_i)_{i=0}^{n-1}) = \prod_{i=0}^{n-1} (x - r_i).
    /// $$
    ///
    /// Fewer than 20 factors are multiplied in one at a time; more are split into two halves, whose
    /// products are found the same way and then multiplied.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log^2 (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `roots.len()`, and $m$ is the largest
    /// number of significant bits of any root.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::FromRoots;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(IntegerPolynomial::from_roots(&[]).to_string(), "1");
    /// // (x + 1)(x - 2)(x - 3)
    /// let roots = [-1, 2, 3].map(Integer::from);
    /// assert_eq!(
    ///     IntegerPolynomial::from_roots(&roots).to_string(),
    ///     "x^3-4*x^2+x+6"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_product_roots_fmpz_vec` from
    /// `fmpz_poly/product_roots_fmpz_vec.c`, FLINT 3.6.0.
    fn from_roots(roots: &[Integer]) -> Self {
        let mut out = vec![Integer::ZERO; roots.len() + 1];
        from_roots_to_out(&mut out, roots);
        Self { coefficients: out }
    }
}
