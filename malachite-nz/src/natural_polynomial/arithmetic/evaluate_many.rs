// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use alloc::vec::Vec;
use malachite_base::polynomial::arithmetic::traits::{Evaluate, EvaluateMany};

impl EvaluateMany<Natural> for &NaturalPolynomial {
    type Output = Natural;

    /// Evaluates a [`NaturalPolynomial`] at each of several [`Natural`]s.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("5*x^2+3*x+7").unwrap();
    /// let xs = [0u32, 1, 2, 3].map(Natural::from);
    /// assert_eq!(
    ///     (&p).evaluate_many(&xs),
    ///     [7u32, 15, 33, 61].map(Natural::from)
    /// );
    /// ```
    #[inline]
    fn evaluate_many(self, xs: &[Natural]) -> Vec<Natural> {
        xs.iter().map(|x| self.evaluate(x)).collect()
    }
}
