// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_polynomial::NaturalPolynomial;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2Assign};

impl ModPowerOf2 for NaturalPolynomial {
    type Output = Self;

    /// Divides every coefficient of a [`NaturalPolynomial`] by $2^k$, keeping the remainders,
    /// taking the polynomial by value.
    ///
    /// The result is reduced modulo $2^k$, which is to say that
    /// [`mod_power_of_2_is_reduced`](malachite_base::num::arithmetic::traits::
    /// ModPowerOf2IsReduced::mod_power_of_2_is_reduced) returns `true` for it.
    ///
    /// Reducing can lower the degree, and can even give the zero polynomial: a leading coefficient
    /// that is a multiple of $2^k$ becomes zero, and a polynomial does not hold trailing zero
    /// coefficients. So $4x^2 + 3$ modulo $4$ is the constant $3$, not a quadratic with a zero
    /// leading coefficient.
    ///
    /// Unlike a [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial),
    /// there is no power wide enough to leave every polynomial alone: a
    /// [`Natural`](crate::natural::Natural) coefficient can be larger than any $2^k$, so a large
    /// $k$ is as meaningful as a small one.
    ///
    /// $$
    /// f(p, k) = q, \\quad \text{where} \\quad q_i = p_i - 2^k \left \lfloor \frac{p_i}{2^k}
    /// \right \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // Every coefficient is taken modulo 2.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2(1)
    ///         .to_string(),
    ///     "x^2+x"
    /// );
    ///
    /// // Reducing the leading coefficient to zero lowers the degree.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("4*x^2+3")
    ///         .unwrap()
    ///         .mod_power_of_2(2)
    ///         .to_string(),
    ///     "3"
    /// );
    ///
    /// // A power too wide for a `u64` still does something here.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("340282366920938463463374607431768211457*x")
    ///         .unwrap()
    ///         .mod_power_of_2(64)
    ///         .to_string(),
    ///     "x"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2(mut self, pow: u64) -> Self {
        self.mod_power_of_2_assign(pow);
        self
    }
}

impl ModPowerOf2 for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Divides every coefficient of a [`NaturalPolynomial`] by $2^k$, keeping the remainders,
    /// taking the polynomial by reference.
    ///
    /// See the documentation for the [`ModPowerOf2`] implementation on [`NaturalPolynomial`] for
    /// details, including how reducing can lower the degree.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("4*x^2+3").unwrap();
    /// assert_eq!((&p).mod_power_of_2(2).to_string(), "3");
    /// // The polynomial is left alone.
    /// assert_eq!(p.to_string(), "4*x^2+3");
    /// ```
    #[inline]
    fn mod_power_of_2(self, pow: u64) -> NaturalPolynomial {
        // `from_coefficients_asc` trims, which is what makes the degree fall when the leading
        // coefficient reduces to zero.
        NaturalPolynomial::from_coefficients_asc(
            self.coefficients_asc()
                .iter()
                .map(|c| c.mod_power_of_2(pow))
                .collect::<Vec<_>>(),
        )
    }
}

impl ModPowerOf2Assign for NaturalPolynomial {
    /// Divides every coefficient of a [`NaturalPolynomial`] by $2^k$, replacing the polynomial by
    /// the one whose coefficients are the remainders.
    ///
    /// See the documentation for the [`ModPowerOf2`] implementation on [`NaturalPolynomial`] for
    /// details, including how reducing can lower the degree.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Assign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_power_of_2_assign(1);
    /// assert_eq!(p.to_string(), "x^2+x");
    ///
    /// let mut p = NaturalPolynomial::from_str("4*x^2+3").unwrap();
    /// p.mod_power_of_2_assign(2);
    /// assert_eq!(p.to_string(), "3");
    /// ```
    fn mod_power_of_2_assign(&mut self, pow: u64) {
        for c in &mut self.coefficients {
            c.mod_power_of_2_assign(pow);
        }
        self.trim();
    }
}
