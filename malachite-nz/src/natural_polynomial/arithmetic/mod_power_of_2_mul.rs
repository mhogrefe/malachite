// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::mul::{mul_ref_ref, mul_val_ref, mul_val_val};
use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_power_of_2_add::assert_reduced;
use alloc::vec::Vec;
use core::mem::take;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Assign, ModPowerOf2Mul, ModPowerOf2MulAssign,
};

// The polynomial whose coefficients are `xs`, each reduced modulo $2^k$, where $k$ is `pow`, and
// trimmed.
pub(crate) fn reduce_coefficients(mut xs: Vec<Natural>, pow: u64) -> NaturalPolynomial {
    for x in &mut xs {
        x.mod_power_of_2_assign(pow);
    }
    let mut p = NaturalPolynomial { coefficients: xs };
    p.trim();
    p
}

impl ModPowerOf2Mul<Self> for NaturalPolynomial {
    type Output = Self;

    /// Multiplies two [`NaturalPolynomial`]s modulo $2^k$, taking both by value. The coefficients
    /// of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, k) = pq \\bmod 2^k.
    /// $$
    ///
    /// The leading coefficient of the product can vanish modulo $2^k$, and then the degree of the
    /// product is lower than the sum of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The coefficients wrap around modulo 16.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_mul(NaturalPolynomial::from_str("2*x+5").unwrap(), 4)
    ///         .to_string(),
    ///     "2*x^3+11*x^2+3*x+10"
    /// );
    /// // The leading coefficient vanishes modulo 16, so the degree drops.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("8*x+1")
    ///         .unwrap()
    ///         .mod_power_of_2_mul(NaturalPolynomial::from_str("2*x+1").unwrap(), 4)
    ///         .to_string(),
    ///     "10*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mul` from `fmpz_mod_poly/mul.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    fn mod_power_of_2_mul(self, other: Self, pow: u64) -> Self {
        assert_reduced(&self, &other, pow);
        reduce_coefficients(mul_val_val(self.coefficients, other.coefficients), pow)
    }
}

impl ModPowerOf2Mul<&Self> for NaturalPolynomial {
    type Output = Self;

    /// Multiplies two [`NaturalPolynomial`]s modulo $2^k$, taking the first by value and the second
    /// by reference. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, k) = pq \\bmod 2^k.
    /// $$
    ///
    /// The leading coefficient of the product can vanish modulo $2^k$, and then the degree of the
    /// product is lower than the sum of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The coefficients wrap around modulo 16.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_mul(&NaturalPolynomial::from_str("2*x+5").unwrap(), 4)
    ///         .to_string(),
    ///     "2*x^3+11*x^2+3*x+10"
    /// );
    /// // The leading coefficient vanishes modulo 16, so the degree drops.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("8*x+1")
    ///         .unwrap()
    ///         .mod_power_of_2_mul(&NaturalPolynomial::from_str("2*x+1").unwrap(), 4)
    ///         .to_string(),
    ///     "10*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mul` from `fmpz_mod_poly/mul.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    fn mod_power_of_2_mul(self, other: &Self, pow: u64) -> Self {
        assert_reduced(&self, other, pow);
        reduce_coefficients(mul_val_ref(self.coefficients, &other.coefficients), pow)
    }
}

impl ModPowerOf2Mul<NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Multiplies two [`NaturalPolynomial`]s modulo $2^k$, taking the first by reference and the
    /// second by value. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, k) = pq \\bmod 2^k.
    /// $$
    ///
    /// The leading coefficient of the product can vanish modulo $2^k$, and then the degree of the
    /// product is lower than the sum of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The coefficients wrap around modulo 16.
    /// assert_eq!(
    ///     &NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_mul(NaturalPolynomial::from_str("2*x+5").unwrap(), 4)
    ///         .to_string(),
    ///     "2*x^3+11*x^2+3*x+10"
    /// );
    /// // The leading coefficient vanishes modulo 16, so the degree drops.
    /// assert_eq!(
    ///     &NaturalPolynomial::from_str("8*x+1")
    ///         .unwrap()
    ///         .mod_power_of_2_mul(NaturalPolynomial::from_str("2*x+1").unwrap(), 4)
    ///         .to_string(),
    ///     "10*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mul` from `fmpz_mod_poly/mul.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    fn mod_power_of_2_mul(self, other: NaturalPolynomial, pow: u64) -> NaturalPolynomial {
        assert_reduced(self, &other, pow);
        reduce_coefficients(mul_val_ref(other.coefficients, &self.coefficients), pow)
    }
}

impl ModPowerOf2Mul<&NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Multiplies two [`NaturalPolynomial`]s modulo $2^k$, taking both by reference. The
    /// coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, k) = pq \\bmod 2^k.
    /// $$
    ///
    /// The leading coefficient of the product can vanish modulo $2^k$, and then the degree of the
    /// product is lower than the sum of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The coefficients wrap around modulo 16.
    /// assert_eq!(
    ///     &NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_mul(&NaturalPolynomial::from_str("2*x+5").unwrap(), 4)
    ///         .to_string(),
    ///     "2*x^3+11*x^2+3*x+10"
    /// );
    /// // The leading coefficient vanishes modulo 16, so the degree drops.
    /// assert_eq!(
    ///     &NaturalPolynomial::from_str("8*x+1")
    ///         .unwrap()
    ///         .mod_power_of_2_mul(&NaturalPolynomial::from_str("2*x+1").unwrap(), 4)
    ///         .to_string(),
    ///     "10*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mul` from `fmpz_mod_poly/mul.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    fn mod_power_of_2_mul(self, other: &NaturalPolynomial, pow: u64) -> NaturalPolynomial {
        assert_reduced(self, other, pow);
        reduce_coefficients(mul_ref_ref(&self.coefficients, &other.coefficients), pow)
    }
}

impl ModPowerOf2MulAssign<Self> for NaturalPolynomial {
    /// Multiplies a [`NaturalPolynomial`] by another [`NaturalPolynomial`] modulo $2^k$ in place,
    /// taking the right-hand side by value. The coefficients of both must already be reduced modulo
    /// $2^k$.
    ///
    /// $$
    /// p \\gets pq \\bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2MulAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_power_of_2_mul_assign(NaturalPolynomial::from_str("2*x+5").unwrap(), 4);
    /// assert_eq!(p.to_string(), "2*x^3+11*x^2+3*x+10");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mul` from `fmpz_mod_poly/mul.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    fn mod_power_of_2_mul_assign(&mut self, other: Self, pow: u64) {
        assert_reduced(self, &other, pow);
        *self = reduce_coefficients(
            mul_val_val(take(&mut self.coefficients), other.coefficients),
            pow,
        );
    }
}

impl ModPowerOf2MulAssign<&Self> for NaturalPolynomial {
    /// Multiplies a [`NaturalPolynomial`] by another [`NaturalPolynomial`] modulo $2^k$ in place,
    /// taking the right-hand side by reference. The coefficients of both must already be reduced
    /// modulo $2^k$.
    ///
    /// $$
    /// p \\gets pq \\bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the length of the longer polynomial, and
    /// $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2MulAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_power_of_2_mul_assign(&NaturalPolynomial::from_str("2*x+5").unwrap(), 4);
    /// assert_eq!(p.to_string(), "2*x^3+11*x^2+3*x+10");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mul` from `fmpz_mod_poly/mul.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    fn mod_power_of_2_mul_assign(&mut self, other: &Self, pow: u64) {
        assert_reduced(self, other, pow);
        *self = reduce_coefficients(
            mul_val_ref(take(&mut self.coefficients), &other.coefficients),
            pow,
        );
    }
}
