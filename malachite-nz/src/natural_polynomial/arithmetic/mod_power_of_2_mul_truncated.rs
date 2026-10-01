// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::mul_truncated::{
    mul_truncated_ref_ref, mul_truncated_val_ref, mul_truncated_val_val,
};
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_power_of_2_add::assert_reduced;
use crate::natural_polynomial::arithmetic::mod_power_of_2_mul::reduce_coefficients;
use core::mem::take;
use malachite_base::polynomial::{ModPowerOf2MulTruncated, ModPowerOf2MulTruncatedAssign};

impl ModPowerOf2MulTruncated<Self> for NaturalPolynomial {
    type Output = Self;

    /// Multiplies two [`NaturalPolynomial`]s modulo $2^k$, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking both by value. The coefficients of both must already be
    /// reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, n, k) = (pq \\bmod x^n) \\bmod 2^k.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the product of their images modulo
    /// $x^n$, so only their first `len` coefficients are read.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2MulTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The product is 2*x^3+11*x^2+19*x+10; its low two coefficients, modulo 16.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_mul_truncated(NaturalPolynomial::from_str("2*x+5").unwrap(), 2, 4)
    ///         .to_string(),
    ///     "3*x+10"
    /// );
    /// // The linear coefficient of the product, 16, vanishes modulo 16.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x+15")
    ///         .unwrap()
    ///         .mod_power_of_2_mul_truncated(NaturalPolynomial::from_str("x+1").unwrap(), 2, 4)
    ///         .to_string(),
    ///     "15"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0,
    /// with the modulus $2^k$.
    fn mod_power_of_2_mul_truncated(self, other: Self, len: u64, pow: u64) -> Self {
        assert_reduced(&self, &other, pow);
        reduce_coefficients(
            mul_truncated_val_val(self.coefficients, other.coefficients, len),
            pow,
        )
    }
}

impl ModPowerOf2MulTruncated<&Self> for NaturalPolynomial {
    type Output = Self;

    /// Multiplies two [`NaturalPolynomial`]s modulo $2^k$, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking the first by value and the second by reference. The
    /// coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, n, k) = (pq \\bmod x^n) \\bmod 2^k.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the product of their images modulo
    /// $x^n$, so only their first `len` coefficients are read.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2MulTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The product is 2*x^3+11*x^2+19*x+10; its low two coefficients, modulo 16.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_mul_truncated(&NaturalPolynomial::from_str("2*x+5").unwrap(), 2, 4)
    ///         .to_string(),
    ///     "3*x+10"
    /// );
    /// // The linear coefficient of the product, 16, vanishes modulo 16.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x+15")
    ///         .unwrap()
    ///         .mod_power_of_2_mul_truncated(&NaturalPolynomial::from_str("x+1").unwrap(), 2, 4)
    ///         .to_string(),
    ///     "15"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0,
    /// with the modulus $2^k$.
    fn mod_power_of_2_mul_truncated(self, other: &Self, len: u64, pow: u64) -> Self {
        assert_reduced(&self, other, pow);
        reduce_coefficients(
            mul_truncated_val_ref(self.coefficients, &other.coefficients, len),
            pow,
        )
    }
}

impl ModPowerOf2MulTruncated<NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Multiplies two [`NaturalPolynomial`]s modulo $2^k$, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking the first by reference and the second by value. The
    /// coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, n, k) = (pq \\bmod x^n) \\bmod 2^k.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the product of their images modulo
    /// $x^n$, so only their first `len` coefficients are read.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2MulTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The product is 2*x^3+11*x^2+19*x+10; its low two coefficients, modulo 16.
    /// assert_eq!(
    ///     &NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_mul_truncated(NaturalPolynomial::from_str("2*x+5").unwrap(), 2, 4)
    ///         .to_string(),
    ///     "3*x+10"
    /// );
    /// // The linear coefficient of the product, 16, vanishes modulo 16.
    /// assert_eq!(
    ///     &NaturalPolynomial::from_str("x+15")
    ///         .unwrap()
    ///         .mod_power_of_2_mul_truncated(NaturalPolynomial::from_str("x+1").unwrap(), 2, 4)
    ///         .to_string(),
    ///     "15"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0,
    /// with the modulus $2^k$.
    fn mod_power_of_2_mul_truncated(
        self,
        other: NaturalPolynomial,
        len: u64,
        pow: u64,
    ) -> NaturalPolynomial {
        assert_reduced(self, &other, pow);
        reduce_coefficients(
            mul_truncated_val_ref(other.coefficients, &self.coefficients, len),
            pow,
        )
    }
}

impl ModPowerOf2MulTruncated<&NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Multiplies two [`NaturalPolynomial`]s modulo $2^k$, keeping only the coefficients of $x^i$
    /// for $i$ less than `len`, taking both by reference. The coefficients of both must already be
    /// reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, n, k) = (pq \\bmod x^n) \\bmod 2^k.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the product of their images modulo
    /// $x^n$, so only their first `len` coefficients are read.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2MulTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The product is 2*x^3+11*x^2+19*x+10; its low two coefficients, modulo 16.
    /// assert_eq!(
    ///     &NaturalPolynomial::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_mul_truncated(&NaturalPolynomial::from_str("2*x+5").unwrap(), 2, 4)
    ///         .to_string(),
    ///     "3*x+10"
    /// );
    /// // The linear coefficient of the product, 16, vanishes modulo 16.
    /// assert_eq!(
    ///     &NaturalPolynomial::from_str("x+15")
    ///         .unwrap()
    ///         .mod_power_of_2_mul_truncated(&NaturalPolynomial::from_str("x+1").unwrap(), 2, 4)
    ///         .to_string(),
    ///     "15"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0,
    /// with the modulus $2^k$.
    fn mod_power_of_2_mul_truncated(
        self,
        other: &NaturalPolynomial,
        len: u64,
        pow: u64,
    ) -> NaturalPolynomial {
        assert_reduced(self, other, pow);
        reduce_coefficients(
            mul_truncated_ref_ref(&self.coefficients, &other.coefficients, len),
            pow,
        )
    }
}

impl ModPowerOf2MulTruncatedAssign<Self> for NaturalPolynomial {
    /// Multiplies a [`NaturalPolynomial`] by another [`NaturalPolynomial`] modulo $2^k$ in place,
    /// keeping only the coefficients of $x^i$ for $i$ less than `len`, taking the right-hand side
    /// by value. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// p \\gets (pq \\bmod x^n) \\bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2MulTruncatedAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_power_of_2_mul_truncated_assign(NaturalPolynomial::from_str("2*x+5").unwrap(), 2, 4);
    /// assert_eq!(p.to_string(), "3*x+10");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0,
    /// with the modulus $2^k$.
    fn mod_power_of_2_mul_truncated_assign(&mut self, other: Self, len: u64, pow: u64) {
        assert_reduced(self, &other, pow);
        *self = reduce_coefficients(
            mul_truncated_val_val(take(&mut self.coefficients), other.coefficients, len),
            pow,
        );
    }
}

impl ModPowerOf2MulTruncatedAssign<&Self> for NaturalPolynomial {
    /// Multiplies a [`NaturalPolynomial`] by another [`NaturalPolynomial`] modulo $2^k$ in place,
    /// keeping only the coefficients of $x^i$ for $i$ less than `len`, taking the right-hand side
    /// by reference. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// p \\gets (pq \\bmod x^n) \\bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2MulTruncatedAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// p.mod_power_of_2_mul_truncated_assign(&NaturalPolynomial::from_str("2*x+5").unwrap(), 2, 4);
    /// assert_eq!(p.to_string(), "3*x+10");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mullow` from `fmpz_mod_poly/mullow.c`, FLINT 3.6.0,
    /// with the modulus $2^k$.
    fn mod_power_of_2_mul_truncated_assign(&mut self, other: &Self, len: u64, pow: u64) {
        assert_reduced(self, other, pow);
        *self = reduce_coefficients(
            mul_truncated_val_ref(take(&mut self.coefficients), &other.coefficients, len),
            pow,
        );
    }
}
