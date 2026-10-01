// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::square::square_ref;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_power_of_2_mul::reduce_coefficients;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2IsReduced, ModPowerOf2Square, ModPowerOf2SquareAssign,
};

pub(crate) fn assert_reduced(p: &NaturalPolynomial, pow: u64) {
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
}

impl ModPowerOf2Square for NaturalPolynomial {
    type Output = Self;

    /// Squares a [`NaturalPolynomial`] modulo $2^k$, taking it by value. The coefficients must
    /// already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, k) = p^2 \\bmod 2^k.
    /// $$
    ///
    /// The leading coefficient of the square can vanish modulo $2^k$, and then the degree of the
    /// square is lower than twice the degree of the polynomial.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the polynomial, and $m$ is
    /// `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Square;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The coefficients wrap around modulo 8.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_square(3)
    ///         .to_string(),
    ///     "x^4+6*x^3+5*x^2+4*x+4"
    /// );
    /// // The leading coefficient vanishes modulo 16, so the degree drops.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("4*x+1")
    ///         .unwrap()
    ///         .mod_power_of_2_square(4)
    ///         .to_string(),
    ///     "8*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_sqr` from `fmpz_mod_poly/sqr.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    #[inline]
    fn mod_power_of_2_square(mut self, pow: u64) -> Self {
        self.mod_power_of_2_square_assign(pow);
        self
    }
}

impl ModPowerOf2Square for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Squares a [`NaturalPolynomial`] modulo $2^k$, taking it by reference. The coefficients must
    /// already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, k) = p^2 \\bmod 2^k.
    /// $$
    ///
    /// The leading coefficient of the square can vanish modulo $2^k$, and then the degree of the
    /// square is lower than twice the degree of the polynomial.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the polynomial, and $m$ is
    /// `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Square;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The coefficients wrap around modulo 8.
    /// assert_eq!(
    ///     &NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_square(3)
    ///         .to_string(),
    ///     "x^4+6*x^3+5*x^2+4*x+4"
    /// );
    /// // The leading coefficient vanishes modulo 16, so the degree drops.
    /// assert_eq!(
    ///     &NaturalPolynomial::from_str("4*x+1")
    ///         .unwrap()
    ///         .mod_power_of_2_square(4)
    ///         .to_string(),
    ///     "8*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_sqr` from `fmpz_mod_poly/sqr.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    fn mod_power_of_2_square(self, pow: u64) -> NaturalPolynomial {
        assert_reduced(self, pow);
        reduce_coefficients(square_ref(&self.coefficients), pow)
    }
}

impl ModPowerOf2SquareAssign for NaturalPolynomial {
    /// Squares a [`NaturalPolynomial`] modulo $2^k$ in place. The coefficients must already be
    /// reduced modulo $2^k$.
    ///
    /// $$
    /// p \\gets p^2 \\bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the polynomial, and $m$ is
    /// `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SquareAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_power_of_2_square_assign(3);
    /// assert_eq!(p.to_string(), "x^4+6*x^3+5*x^2+4*x+4");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_sqr` from `fmpz_mod_poly/sqr.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    fn mod_power_of_2_square_assign(&mut self, pow: u64) {
        assert_reduced(self, pow);
        // The square of a constant is computed in place.
        if let [c] = self.coefficients.as_mut_slice() {
            c.mod_power_of_2_square_assign(pow);
            self.trim();
        } else {
            *self = reduce_coefficients(square_ref(&self.coefficients), pow);
        }
    }
}
