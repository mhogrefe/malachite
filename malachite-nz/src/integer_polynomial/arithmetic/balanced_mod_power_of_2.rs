// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::IntegerPolynomial;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{BalancedModPowerOf2, BalancedModPowerOf2Assign};
use malachite_base::polynomial::Polynomial;

impl BalancedModPowerOf2 for IntegerPolynomial {
    type Output = Self;

    /// Reduces every coefficient of an [`IntegerPolynomial`] modulo $2^k$ to the representative
    /// closest to zero, taking the polynomial by value.
    ///
    /// Each coefficient $r_i$ of the result satisfies $-2^{k-1} < r_i \leq 2^{k-1}$ and is
    /// congruent to the original coefficient modulo $2^k$, which determine it uniquely, as with
    /// [`BalancedModPowerOf2`] for [`Integer`](crate::integer::Integer)s; for $k = 0$ every
    /// coefficient is 0. A remainder of exactly $2^{k-1}$ is positive.
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
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// // Each coefficient goes to its balanced remainder modulo 8, and half the modulus
    /// // stays positive.
    /// let p = IntegerPolynomial::from_str("-19*x^2+23*x-20").unwrap();
    /// assert_eq!(
    ///     p.clone().balanced_mod_power_of_2(3).to_string(),
    ///     "-3*x^2-x+4"
    /// );
    /// let p = IntegerPolynomial::from_str("-8*x^2+7*x-5").unwrap();
    /// assert_eq!(p.clone().balanced_mod_power_of_2(3).to_string(), "-x+3");
    /// ```
    #[inline]
    fn balanced_mod_power_of_2(mut self, pow: u64) -> Self {
        self.balanced_mod_power_of_2_assign(pow);
        self
    }
}

impl BalancedModPowerOf2 for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Reduces every coefficient of an [`IntegerPolynomial`] modulo $2^k$ to the representative
    /// closest to zero, taking the polynomial by reference.
    ///
    /// See the documentation for the [`BalancedModPowerOf2`] implementation on
    /// [`IntegerPolynomial`] that takes the polynomial by value for details.
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
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// // Each coefficient goes to its balanced remainder modulo 8, and half the modulus
    /// // stays positive.
    /// let p = IntegerPolynomial::from_str("-19*x^2+23*x-20").unwrap();
    /// assert_eq!((&p).balanced_mod_power_of_2(3).to_string(), "-3*x^2-x+4");
    /// let p = IntegerPolynomial::from_str("-8*x^2+7*x-5").unwrap();
    /// assert_eq!((&p).balanced_mod_power_of_2(3).to_string(), "-x+3");
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

impl BalancedModPowerOf2Assign for IntegerPolynomial {
    /// Reduces every coefficient of an [`IntegerPolynomial`] modulo $2^k$ to the representative
    /// closest to zero, in place.
    ///
    /// See the documentation for the [`BalancedModPowerOf2`] implementation on
    /// [`IntegerPolynomial`] that takes the polynomial by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm)$
    ///
    /// $M(n, m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of coefficients, and $m$ is
    /// `pow`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::BalancedModPowerOf2Assign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("-19*x^2+23*x-20").unwrap();
    /// p.balanced_mod_power_of_2_assign(3);
    /// assert_eq!(p.to_string(), "-3*x^2-x+4");
    /// ```
    fn balanced_mod_power_of_2_assign(&mut self, pow: u64) {
        for c in &mut self.coefficients {
            c.balanced_mod_power_of_2_assign(pow);
        }
        self.trim();
    }
}
