// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::num::arithmetic::traits::{Height, ModPowerOf2IsReduced};

impl ModPowerOf2IsReduced for NaturalPolynomial {
    /// Returns whether a [`NaturalPolynomial`] is reduced modulo $2^k$; in other words, whether
    /// every one of its coefficients has no more than $k$ significant bits.
    ///
    /// Asking that of every coefficient is asking it of the largest, so this is the number of
    /// significant bits of the polynomial's [`height`](Height::to_height) — which bit length
    /// being monotone means is the largest of the coefficients' bit lengths, so the height itself
    /// never has to be built or borrowed. The zero polynomial has no coefficients and is reduced
    /// modulo every power of 2, including $2^0$.
    ///
    /// $f(p, k) = (\max_i p_i < 2^k)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2IsReduced;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The largest coefficient is 3, which needs two bits.
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(p.mod_power_of_2_is_reduced(2), true);
    /// assert_eq!(p.mod_power_of_2_is_reduced(1), false);
    ///
    /// // The zero polynomial is reduced modulo every power of 2.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("0")
    ///         .unwrap()
    ///         .mod_power_of_2_is_reduced(0),
    ///     true
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_is_reduced(&self, pow: u64) -> bool {
        self.height_significant_bits() <= pow
    }
}
