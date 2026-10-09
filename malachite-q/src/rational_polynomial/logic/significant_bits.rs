// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use malachite_base::num::arithmetic::traits::{DivExact, Gcd};
use malachite_base::num::logic::traits::SignificantBits;

impl SignificantBits for &RationalPolynomial {
    /// Returns the sum of the numbers of significant bits of the coefficients of a
    /// [`RationalPolynomial`], each coefficient's count being the sum of the bits of its numerator
    /// and denominator in lowest terms, as for
    /// [`Rational`](crate::Rational#impl-SignificantBits-for-%26Rational).
    ///
    /// This is the number of bits needed to store all of the coefficients one by one, so it agrees
    /// with the [`RationalVector`](crate::rational_vector::RationalVector) of the coefficients. It
    /// is 0 for the zero polynomial, and a zero coefficient below the leading one, being $0/1$,
    /// contributes 1. The polynomial does not store its coefficients one by one, so each one's
    /// numerator and denominator are reduced by their GCD before being counted.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerator's coefficients, plus the number of coefficients times the number of bits of the
    /// denominator.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::logic::traits::SignificantBits;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// // The coefficients are 1/2, 0, and 1/3.
    /// assert_eq!(
    ///     RationalPolynomial::from_str("1/3*x^2+1/2")
    ///         .unwrap()
    ///         .significant_bits(),
    ///     7
    /// );
    /// assert_eq!(
    ///     RationalPolynomial::from_str("0")
    ///         .unwrap()
    ///         .significant_bits(),
    ///     0
    /// );
    /// ```
    fn significant_bits(self) -> u64 {
        let d = &self.denominator;
        let d_bits = d.significant_bits();
        self.numerator
            .coefficients_asc()
            .iter()
            .map(|n| {
                let n = n.unsigned_abs_ref();
                let g = n.gcd(d);
                if g == 1u32 {
                    n.significant_bits() + d_bits
                } else {
                    n.div_exact(&g).significant_bits() + d.div_exact(&g).significant_bits()
                }
            })
            .sum()
    }
}
