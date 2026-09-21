// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use core::cmp::max;
use malachite_base::num::arithmetic::traits::{DivExact, Gcd, Height};
use malachite_base::num::basic::traits::One;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::Natural;

// The height of the `i`th coefficient, which is the larger of the magnitudes of that coefficient in
// lowest terms.
//
// The coefficients share one denominator, so a coefficient's own denominator has to be worked out
// before its height can be: the stored numerator and the shared denominator may still have a factor
// in common, even though the polynomial as a whole is canonical.
fn coefficient_height(numerator: &Natural, denominator: &Natural) -> Natural {
    let gcd = numerator.gcd(denominator);
    max(numerator.div_exact(&gcd), denominator.div_exact(&gcd))
}

impl Height for RationalPolynomial {
    type Output = Natural;

    /// Returns the height of a [`RationalPolynomial`]: the largest of the heights of its
    /// coefficients.
    ///
    /// A [`Rational`](crate::Rational)'s height is the larger of the absolute value of its
    /// numerator and its denominator, in lowest terms. The zero polynomial has no coefficients, and
    /// its height is 1, which is the height of the rational number 0 — zero is $0/1$, and its
    /// denominator is 1.
    ///
    /// $$
    /// f(p) = H(p) = \max_i H(p_i).
    /// $$
    ///
    /// The coefficients share one denominator, which is why this cannot lend its result the way
    /// [`HeightRef`](malachite_base::num::arithmetic::traits::HeightRef) does for the other types:
    /// the stored numerator of a coefficient and the shared denominator may still have a factor in
    /// common, so each coefficient has to be reduced before its height is known, and the answer is
    /// not one of the numbers the polynomial holds.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerator coefficients and the denominator.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Height;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// // The coefficients are 1/3 and 1/2, whose heights are 3 and 2.
    /// assert_eq!(
    ///     RationalPolynomial::from_str("1/2*x+1/3")
    ///         .unwrap()
    ///         .to_height(),
    ///     3
    /// );
    /// assert_eq!(
    ///     RationalPolynomial::from_str("x^2-3*x+2")
    ///         .unwrap()
    ///         .to_height(),
    ///     3
    /// );
    /// // The zero polynomial's height is the height of the rational number 0.
    /// assert_eq!(RationalPolynomial::from_str("0").unwrap().to_height(), 1);
    /// ```
    fn to_height(&self) -> Natural {
        let denominator = self.denominator_ref();
        self.numerator_ref()
            .coefficients_asc()
            .iter()
            .map(|c| coefficient_height(c.unsigned_abs_ref(), denominator))
            .max()
            .unwrap_or(Natural::ONE)
    }

    /// Returns the height of a [`RationalPolynomial`], taking it by value.
    ///
    /// Every coefficient has to be reduced against the shared denominator before its height is
    /// known, so nothing can be moved out of the polynomial and this is the same work as
    /// [`to_height`](Height::to_height).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerator coefficients and the denominator.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Height;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     RationalPolynomial::from_str("1/2*x+1/3")
    ///         .unwrap()
    ///         .into_height(),
    ///     3
    /// );
    /// assert_eq!(RationalPolynomial::from_str("0").unwrap().into_height(), 1);
    /// ```
    #[inline]
    fn into_height(self) -> Natural {
        self.to_height()
    }

    /// Returns the number of significant bits of the height of a [`RationalPolynomial`].
    ///
    /// Since bit length is monotone, this is the largest of the coefficients' height bit lengths.
    /// The heights themselves still have to be worked out, so unlike the other types this is no
    /// cheaper than materializing the height.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerator coefficients and the denominator.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Height;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     RationalPolynomial::from_str("1/2*x+1/3")
    ///         .unwrap()
    ///         .height_significant_bits(),
    ///     2
    /// );
    /// assert_eq!(
    ///     RationalPolynomial::from_str("0")
    ///         .unwrap()
    ///         .height_significant_bits(),
    ///     1
    /// );
    /// ```
    #[inline]
    fn height_significant_bits(&self) -> u64 {
        self.to_height().significant_bits()
    }
}
