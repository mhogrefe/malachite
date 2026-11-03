// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use alloc::vec::Vec;
use malachite_base::polynomial::arithmetic::traits::{Evaluate, EvaluateMany};

impl EvaluateMany<Integer> for &IntegerPolynomial {
    type Output = Integer;

    /// Evaluates an [`IntegerPolynomial`] at each of several [`Integer`]s.
    ///
    /// $$
    /// f(p, (x_j)_{j=0}^{k-1}) = \left ( \sum_{i=0}^{n-1} c_i x_j^i \right )_{j=0}^{k-1},
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length.
    ///
    /// Each value is found as by
    /// [`evaluate`](malachite_base::polynomial::arithmetic::traits::Evaluate::evaluate), which
    /// chooses between Horner's rule and divide and conquer by the length of the polynomial and the
    /// size of the value.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(kn \log^2 n \log\log n)$
    ///
    /// $M(n, k) = O(kn \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $k$ is `xs.len()`, and $n$ is `self.len()`
    /// times the larger of the greatest number of bits of any coefficient and the greatest number
    /// of bits of any value in `xs`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::EvaluateMany;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^2-3*x+2").unwrap();
    /// let xs = [-1i32, 0, 1, 2, 3].map(Integer::from);
    /// assert_eq!(
    ///     (&p).evaluate_many(&xs),
    ///     [6i32, 2, 0, 0, 2].map(Integer::from)
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_evaluate_fmpz_vec` from `fmpz_poly/evaluate_fmpz_vec.c`,
    /// FLINT 3.6.0.
    #[inline]
    fn evaluate_many(self, xs: &[Integer]) -> Vec<Integer> {
        xs.iter().map(|x| self.evaluate(x)).collect()
    }
}
