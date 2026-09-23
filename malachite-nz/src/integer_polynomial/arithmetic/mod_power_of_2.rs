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
use malachite_base::num::arithmetic::traits::{ModPowerOf2, RemPowerOf2, RemPowerOf2Assign};
use malachite_base::polynomial::Polynomial;

impl ModPowerOf2 for IntegerPolynomial {
    type Output = NaturalPolynomial;

    /// Divides every coefficient of an [`IntegerPolynomial`] by $2^k$, keeping the remainders as a
    /// [`NaturalPolynomial`], taking the polynomial by value.
    ///
    /// Each remainder is non-negative, as with [`ModPowerOf2`] for
    /// [`Integer`](crate::integer::Integer): a negative coefficient $c$ becomes $2^k - (-c \bmod
    /// 2^k)$ unless it is a multiple of $2^k$. So the result has natural coefficients, and is
    /// reduced modulo $2^k$, which is to say that [`mod_power_of_2_is_reduced`](
    /// malachite_base::num::arithmetic::traits::ModPowerOf2IsReduced::mod_power_of_2_is_reduced)
    /// returns `true` for it.
    ///
    /// Reducing can lower the degree, and can even give the zero polynomial: a leading coefficient
    /// that is a multiple of $2^k$ becomes zero, and a polynomial does not hold trailing zero
    /// coefficients. So $-4x^2 + 3$ modulo $4$ is the constant $3$.
    ///
    /// $$
    /// f(p, k) = q, \quad \text{where} \quad q_i = p_i - 2^k \left \lfloor \frac{p_i}{2^k}
    /// \right \rfloor.
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::{
    ///     ModPowerOf2, RemPowerOf2, RemPowerOf2Assign,
    /// };
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// // Every coefficient is taken modulo 4, and negative ones become non-negative.
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("x^2-3*x-2")
    ///         .unwrap()
    ///         .mod_power_of_2(2)
    ///         .to_string(),
    ///     "x^2+x+2"
    /// );
    ///
    /// // Reducing the leading coefficient to zero lowers the degree.
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("-4*x^2+3")
    ///         .unwrap()
    ///         .mod_power_of_2(2)
    ///         .to_string(),
    ///     "3"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2(self, pow: u64) -> NaturalPolynomial {
        // `from_coefficients_asc` trims, which is what makes the degree fall when the leading
        // coefficient reduces to zero.
        NaturalPolynomial::from_coefficients_asc(
            self.coefficients
                .into_iter()
                .map(|c| c.mod_power_of_2(pow))
                .collect::<Vec<_>>(),
        )
    }
}

impl ModPowerOf2 for &IntegerPolynomial {
    type Output = NaturalPolynomial;

    /// Divides every coefficient of an [`IntegerPolynomial`] by $2^k$, keeping the remainders as a
    /// [`NaturalPolynomial`], taking the polynomial by reference.
    ///
    /// See the documentation for the [`ModPowerOf2`] implementation on [`IntegerPolynomial`] for
    /// details, including how negative coefficients are handled and how reducing can lower the
    /// degree.
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
    /// use malachite_base::num::arithmetic::traits::{
    ///     ModPowerOf2, RemPowerOf2, RemPowerOf2Assign,
    /// };
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// // Every coefficient is taken modulo 4, and negative ones become non-negative.
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^2-3*x-2").unwrap())
    ///         .mod_power_of_2(2)
    ///         .to_string(),
    ///     "x^2+x+2"
    /// );
    ///
    /// // Reducing the leading coefficient to zero lowers the degree.
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("-4*x^2+3").unwrap())
    ///         .mod_power_of_2(2)
    ///         .to_string(),
    ///     "3"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2(self, pow: u64) -> NaturalPolynomial {
        NaturalPolynomial::from_coefficients_asc(
            self.coefficients
                .iter()
                .map(|c| c.mod_power_of_2(pow))
                .collect::<Vec<_>>(),
        )
    }
}

impl RemPowerOf2 for IntegerPolynomial {
    type Output = Self;

    /// Divides every coefficient of an [`IntegerPolynomial`] by $2^k$, keeping the remainders,
    /// taking the polynomial by value.
    ///
    /// Each remainder has the sign of its coefficient and a smaller absolute value than $2^k$, as
    /// with [`RemPowerOf2`] for [`Integer`](crate::integer::Integer)s. This is the remainder of
    /// truncating division, and the result stays an [`IntegerPolynomial`]; for a remainder that is
    /// always non-negative, and a [`NaturalPolynomial`] result, use [`ModPowerOf2`].
    ///
    /// Reducing can lower the degree, and can even give the zero polynomial: a leading coefficient
    /// that is a multiple of $2^k$ becomes zero, and a polynomial does not hold trailing zero
    /// coefficients. So $-4x^2 - 3$ modulo $4$ is the constant $-3$.
    ///
    /// $$
    /// f(p, k) = r, \quad \text{where} \quad r_i = p_i - 2^k \operatorname{sgn}(p_i)
    ///     \left \lfloor \frac{|p_i|}{2^k} \right \rfloor.
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
    /// use malachite_base::num::arithmetic::traits::RemPowerOf2;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// // Every coefficient is taken modulo 4, keeping its sign.
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("x^2-7*x-2")
    ///         .unwrap()
    ///         .rem_power_of_2(2)
    ///         .to_string(),
    ///     "x^2-3*x-2"
    /// );
    ///
    /// // Reducing the leading coefficient to zero lowers the degree.
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("-4*x^2-3")
    ///         .unwrap()
    ///         .rem_power_of_2(2)
    ///         .to_string(),
    ///     "-3"
    /// );
    /// ```
    #[inline]
    fn rem_power_of_2(mut self, pow: u64) -> Self {
        self.rem_power_of_2_assign(pow);
        self
    }
}

impl RemPowerOf2 for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Divides every coefficient of an [`IntegerPolynomial`] by $2^k$, keeping the remainders,
    /// taking the polynomial by reference.
    ///
    /// See the documentation for the [`RemPowerOf2`] implementation on [`IntegerPolynomial`] for
    /// details, including the signs of the remainders and how reducing can lower the degree.
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
    /// use malachite_base::num::arithmetic::traits::RemPowerOf2;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// // Every coefficient is taken modulo 4, keeping its sign.
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^2-7*x-2").unwrap())
    ///         .rem_power_of_2(2)
    ///         .to_string(),
    ///     "x^2-3*x-2"
    /// );
    ///
    /// // Reducing the leading coefficient to zero lowers the degree.
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("-4*x^2-3").unwrap())
    ///         .rem_power_of_2(2)
    ///         .to_string(),
    ///     "-3"
    /// );
    /// ```
    #[inline]
    fn rem_power_of_2(self, pow: u64) -> IntegerPolynomial {
        // `from_coefficients_asc` trims, which is what makes the degree fall when the leading
        // coefficient reduces to zero.
        IntegerPolynomial::from_coefficients_asc(
            self.coefficients
                .iter()
                .map(|c| c.rem_power_of_2(pow))
                .collect::<Vec<_>>(),
        )
    }
}

impl RemPowerOf2Assign for IntegerPolynomial {
    /// Divides every coefficient of an [`IntegerPolynomial`] by $2^k$, replacing the polynomial by
    /// the one whose coefficients are the remainders.
    ///
    /// See the documentation for the [`RemPowerOf2`] implementation on [`IntegerPolynomial`] for
    /// details, including the signs of the remainders and how reducing can lower the degree.
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
    /// use malachite_base::num::arithmetic::traits::RemPowerOf2Assign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^2-7*x-2").unwrap();
    /// p.rem_power_of_2_assign(2);
    /// assert_eq!(p.to_string(), "x^2-3*x-2");
    ///
    /// let mut p = IntegerPolynomial::from_str("-4*x^2-3").unwrap();
    /// p.rem_power_of_2_assign(2);
    /// assert_eq!(p.to_string(), "-3");
    /// ```
    fn rem_power_of_2_assign(&mut self, pow: u64) {
        for c in &mut self.coefficients {
            c.rem_power_of_2_assign(pow);
        }
        self.trim();
    }
}
