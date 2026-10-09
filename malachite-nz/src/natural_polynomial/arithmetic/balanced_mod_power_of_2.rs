// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::IntegerPolynomial;
use crate::natural_polynomial::NaturalPolynomial;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::BalancedModPowerOf2;
use malachite_base::polynomial::Polynomial;

impl BalancedModPowerOf2 for NaturalPolynomial {
    type Output = IntegerPolynomial;

    /// Reduces every coefficient of a [`NaturalPolynomial`] modulo $2^k$ to the representative
    /// closest to zero, taking the polynomial by value.
    ///
    /// Each coefficient $r_i$ of the result satisfies $-2^{k-1} < r_i \leq 2^{k-1}$ and is
    /// congruent to the original coefficient modulo $2^k$, which determine it uniquely, as with
    /// [`BalancedModPowerOf2`] for [`Natural`](crate::natural::Natural)s; for $k = 0$ every
    /// coefficient is 0. A remainder of exactly $2^{k-1}$ is positive. Remainders above $2^{k-1}$
    /// become negative, which is why the result is an
    /// [`IntegerPolynomial`](crate::integer_polynomial::IntegerPolynomial).
    ///
    /// Reducing can lower the degree, and can even give the zero polynomial: a leading coefficient
    /// that is a multiple of $2^k$ becomes zero, and a polynomial does not hold trailing zero
    /// coefficients.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of coefficients, and $m$ is
    /// `pow`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::BalancedModPowerOf2;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // Each coefficient goes to its balanced remainder modulo 8, and half the modulus
    /// // stays positive.
    /// let p = NaturalPolynomial::from_str("23*x^2+20*x+19").unwrap();
    /// assert_eq!(
    ///     p.clone().balanced_mod_power_of_2(3).to_string(),
    ///     "-x^2+4*x+3"
    /// );
    /// let p = NaturalPolynomial::from_str("8*x^2+7*x+5").unwrap();
    /// assert_eq!(p.clone().balanced_mod_power_of_2(3).to_string(), "-x-3");
    /// ```
    #[inline]
    fn balanced_mod_power_of_2(self, pow: u64) -> IntegerPolynomial {
        IntegerPolynomial::from_coefficients_asc(
            self.coefficients
                .into_iter()
                .map(|c| c.balanced_mod_power_of_2(pow))
                .collect::<Vec<_>>(),
        )
    }
}

impl BalancedModPowerOf2 for &NaturalPolynomial {
    type Output = IntegerPolynomial;

    /// Reduces every coefficient of a [`NaturalPolynomial`] modulo $2^k$ to the representative
    /// closest to zero, taking the polynomial by reference.
    ///
    /// See the documentation for the [`BalancedModPowerOf2`] implementation on
    /// [`NaturalPolynomial`] that takes the polynomial by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of coefficients, and $m$ is
    /// `pow`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::BalancedModPowerOf2;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // Each coefficient goes to its balanced remainder modulo 8, and half the modulus
    /// // stays positive.
    /// let p = NaturalPolynomial::from_str("23*x^2+20*x+19").unwrap();
    /// assert_eq!((&p).balanced_mod_power_of_2(3).to_string(), "-x^2+4*x+3");
    /// let p = NaturalPolynomial::from_str("8*x^2+7*x+5").unwrap();
    /// assert_eq!((&p).balanced_mod_power_of_2(3).to_string(), "-x-3");
    /// ```
    #[inline]
    fn balanced_mod_power_of_2(self, pow: u64) -> IntegerPolynomial {
        // `from_coefficients_asc` trims, which is what makes the degree fall when the leading
        // coefficient reduces to zero.
        IntegerPolynomial::from_coefficients_asc(
            self.coefficients
                .iter()
                .map(|c| c.balanced_mod_power_of_2(pow))
                .collect::<Vec<_>>(),
        )
    }
}
