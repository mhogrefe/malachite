// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2Assign};
use crate::num::basic::integers::PrimitiveInt;
use crate::u64_polynomial::U64Polynomial;
use alloc::vec::Vec;

impl ModPowerOf2 for U64Polynomial {
    type Output = Self;

    /// Divides every coefficient of a [`U64Polynomial`] by $2^k$, keeping the remainders, taking
    /// the polynomial by value.
    ///
    /// The result is reduced modulo $2^k$, which is to say that
    /// [`mod_power_of_2_is_reduced`](crate::num::arithmetic::traits::ModPowerOf2IsReduced::
    /// mod_power_of_2_is_reduced) returns `true` for it.
    ///
    /// Reducing can lower the degree, and can even give the zero polynomial: a leading coefficient
    /// that is a multiple of $2^k$ becomes zero, and a polynomial does not hold trailing zero
    /// coefficients. So $4x^2 + 3$ modulo $4$ is the constant $3$, not a quadratic with a zero
    /// leading coefficient.
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
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2;
    /// use malachite_base::u64_polynomial::U64Polynomial;
    ///
    /// // Every coefficient is taken modulo 2.
    /// assert_eq!(
    ///     U64Polynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2(1)
    ///         .to_string(),
    ///     "x^2+x"
    /// );
    ///
    /// // Reducing the leading coefficient to zero lowers the degree.
    /// assert_eq!(
    ///     U64Polynomial::from_str("4*x^2+3")
    ///         .unwrap()
    ///         .mod_power_of_2(2)
    ///         .to_string(),
    ///     "3"
    /// );
    ///
    /// // Modulo 2^0 every coefficient is zero, so the whole polynomial is.
    /// assert_eq!(
    ///     U64Polynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2(0)
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2(mut self, pow: u64) -> Self {
        self.mod_power_of_2_assign(pow);
        self
    }
}

impl ModPowerOf2 for &U64Polynomial {
    type Output = U64Polynomial;

    /// Divides every coefficient of a [`U64Polynomial`] by $2^k$, keeping the remainders, taking
    /// the polynomial by reference.
    ///
    /// See the documentation for the [`ModPowerOf2`] implementation on [`U64Polynomial`] for
    /// details, including how reducing can lower the degree.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2;
    /// use malachite_base::u64_polynomial::U64Polynomial;
    ///
    /// let p = U64Polynomial::from_str("4*x^2+3").unwrap();
    /// assert_eq!((&p).mod_power_of_2(2).to_string(), "3");
    /// // The polynomial is left alone.
    /// assert_eq!(p.to_string(), "4*x^2+3");
    /// ```
    #[inline]
    fn mod_power_of_2(self, pow: u64) -> U64Polynomial {
        // `from_coefficients_asc` trims, which is what makes the degree fall when the leading
        // coefficient reduces to zero.
        U64Polynomial::from_coefficients_asc(
            self.coefficients_asc()
                .iter()
                .map(|&c| c.mod_power_of_2(pow))
                .collect::<Vec<_>>(),
        )
    }
}

impl ModPowerOf2Assign for U64Polynomial {
    /// Divides every coefficient of a [`U64Polynomial`] by $2^k$, replacing the polynomial by the
    /// one whose coefficients are the remainders.
    ///
    /// See the documentation for the [`ModPowerOf2`] implementation on [`U64Polynomial`] for
    /// details, including how reducing can lower the degree.
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
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Assign;
    /// use malachite_base::u64_polynomial::U64Polynomial;
    ///
    /// let mut p = U64Polynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_power_of_2_assign(1);
    /// assert_eq!(p.to_string(), "x^2+x");
    ///
    /// let mut p = U64Polynomial::from_str("4*x^2+3").unwrap();
    /// p.mod_power_of_2_assign(2);
    /// assert_eq!(p.to_string(), "3");
    /// ```
    fn mod_power_of_2_assign(&mut self, pow: u64) {
        // A shortcut rather than a necessity: no `u64` reaches 2^64, so reducing by a power that
        // wide is already the identity on every coefficient, and the loop below would do the same
        // work to no effect.
        if pow >= u64::WIDTH {
            return;
        }
        for c in &mut self.coefficients {
            c.mod_power_of_2_assign(pow);
        }
        self.trim();
    }
}
