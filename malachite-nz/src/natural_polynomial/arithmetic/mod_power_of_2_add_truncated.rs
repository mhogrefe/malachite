// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_power_of_2_add::{
    add_assign_ref, add_assign_val, assert_reduced,
};
use malachite_base::polynomial::{
    ModPowerOf2AddTruncated, ModPowerOf2AddTruncatedAssign, Polynomial,
};

// The first `len` elements of `xs`, or all of them if there are fewer.
fn prefix(xs: &[Natural], len: u64) -> &[Natural] {
    &xs[..usize::try_from(len).map_or(xs.len(), |len| len.min(xs.len()))]
}

impl ModPowerOf2AddTruncated<Self> for NaturalPolynomial {
    type Output = Self;

    /// Adds two [`NaturalPolynomial`]s modulo $2^k$, keeping only the coefficients of $x^i$ for $i$
    /// less than `len`, taking both by value. The coefficients of both must already be reduced
    /// modulo $2^k$.
    ///
    /// $$
    /// f(p, q, n, k) = ((p + q) \bmod x^n) \bmod 2^k.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. The sum is trimmed, so when
    /// coefficients cancel modulo $2^k$ at the top of the kept range, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2AddTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The linear coefficients wrap around to 0.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^3+2*x^2+x+5")
    ///         .unwrap()
    ///         .mod_power_of_2_add_truncated(
    ///             NaturalPolynomial::from_str("4*x^2+7*x+2").unwrap(),
    ///             3,
    ///             3
    ///         )
    ///         .to_string(),
    ///     "6*x^2+7"
    /// );
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^3+2*x^2+x+5")
    ///         .unwrap()
    ///         .mod_power_of_2_add_truncated(
    ///             NaturalPolynomial::from_str("4*x^2+7*x+2").unwrap(),
    ///             1,
    ///             3
    ///         )
    ///         .to_string(),
    ///     "7"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_add_series` from `fmpz_mod_poly/add_series.c`, FLINT
    /// 3.6.0, with the modulus $2^k$.
    fn mod_power_of_2_add_truncated(mut self, mut other: Self, len: u64, pow: u64) -> Self {
        assert_reduced(&self, &other, pow);
        self.truncate_assign(len);
        other.truncate_assign(len);
        add_assign_val(&mut self.coefficients, other.coefficients, pow);
        self.trim();
        self
    }
}

impl ModPowerOf2AddTruncated<&Self> for NaturalPolynomial {
    type Output = Self;

    /// Adds two [`NaturalPolynomial`]s modulo $2^k$, keeping only the coefficients of $x^i$ for $i$
    /// less than `len`, taking the first by value and the second by reference. The coefficients of
    /// both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, n, k) = ((p + q) \bmod x^n) \bmod 2^k.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. The sum is trimmed, so when
    /// coefficients cancel modulo $2^k$ at the top of the kept range, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2AddTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The linear coefficients wrap around to 0.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^3+2*x^2+x+5")
    ///         .unwrap()
    ///         .mod_power_of_2_add_truncated(
    ///             &NaturalPolynomial::from_str("4*x^2+7*x+2").unwrap(),
    ///             3,
    ///             3
    ///         )
    ///         .to_string(),
    ///     "6*x^2+7"
    /// );
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^3+2*x^2+x+5")
    ///         .unwrap()
    ///         .mod_power_of_2_add_truncated(
    ///             &NaturalPolynomial::from_str("4*x^2+7*x+2").unwrap(),
    ///             1,
    ///             3
    ///         )
    ///         .to_string(),
    ///     "7"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_add_series` from `fmpz_mod_poly/add_series.c`, FLINT
    /// 3.6.0, with the modulus $2^k$.
    fn mod_power_of_2_add_truncated(mut self, other: &Self, len: u64, pow: u64) -> Self {
        assert_reduced(&self, other, pow);
        self.truncate_assign(len);
        add_assign_ref(
            &mut self.coefficients,
            prefix(&other.coefficients, len),
            pow,
        );
        self.trim();
        self
    }
}

impl ModPowerOf2AddTruncated<NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Adds two [`NaturalPolynomial`]s modulo $2^k$, keeping only the coefficients of $x^i$ for $i$
    /// less than `len`, taking the first by reference and the second by value. The coefficients of
    /// both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, n, k) = ((p + q) \bmod x^n) \bmod 2^k.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. The sum is trimmed, so when
    /// coefficients cancel modulo $2^k$ at the top of the kept range, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2AddTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The linear coefficients wrap around to 0.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^3+2*x^2+x+5").unwrap())
    ///         .mod_power_of_2_add_truncated(
    ///             NaturalPolynomial::from_str("4*x^2+7*x+2").unwrap(),
    ///             3,
    ///             3
    ///         )
    ///         .to_string(),
    ///     "6*x^2+7"
    /// );
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^3+2*x^2+x+5").unwrap())
    ///         .mod_power_of_2_add_truncated(
    ///             NaturalPolynomial::from_str("4*x^2+7*x+2").unwrap(),
    ///             1,
    ///             3
    ///         )
    ///         .to_string(),
    ///     "7"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_add_series` from `fmpz_mod_poly/add_series.c`, FLINT
    /// 3.6.0, with the modulus $2^k$.
    fn mod_power_of_2_add_truncated(
        self,
        mut other: NaturalPolynomial,
        len: u64,
        pow: u64,
    ) -> NaturalPolynomial {
        assert_reduced(self, &other, pow);
        other.truncate_assign(len);
        add_assign_ref(
            &mut other.coefficients,
            prefix(&self.coefficients, len),
            pow,
        );
        other.trim();
        other
    }
}

impl ModPowerOf2AddTruncated<&NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Adds two [`NaturalPolynomial`]s modulo $2^k$, keeping only the coefficients of $x^i$ for $i$
    /// less than `len`, taking both by reference. The coefficients of both must already be reduced
    /// modulo $2^k$.
    ///
    /// $$
    /// f(p, q, n, k) = ((p + q) \bmod x^n) \bmod 2^k.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. The sum is trimmed, so when
    /// coefficients cancel modulo $2^k$ at the top of the kept range, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2AddTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The linear coefficients wrap around to 0.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^3+2*x^2+x+5").unwrap())
    ///         .mod_power_of_2_add_truncated(
    ///             &NaturalPolynomial::from_str("4*x^2+7*x+2").unwrap(),
    ///             3,
    ///             3
    ///         )
    ///         .to_string(),
    ///     "6*x^2+7"
    /// );
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^3+2*x^2+x+5").unwrap())
    ///         .mod_power_of_2_add_truncated(
    ///             &NaturalPolynomial::from_str("4*x^2+7*x+2").unwrap(),
    ///             1,
    ///             3
    ///         )
    ///         .to_string(),
    ///     "7"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_add_series` from `fmpz_mod_poly/add_series.c`, FLINT
    /// 3.6.0, with the modulus $2^k$.
    fn mod_power_of_2_add_truncated(
        self,
        other: &NaturalPolynomial,
        len: u64,
        pow: u64,
    ) -> NaturalPolynomial {
        assert_reduced(self, other, pow);
        let mut coefficients = prefix(&self.coefficients, len).to_vec();
        add_assign_ref(&mut coefficients, prefix(&other.coefficients, len), pow);
        let mut result = NaturalPolynomial { coefficients };
        result.trim();
        result
    }
}

impl ModPowerOf2AddTruncatedAssign<Self> for NaturalPolynomial {
    /// Adds a [`NaturalPolynomial`] to a [`NaturalPolynomial`] modulo $2^k$ in place, keeping only
    /// the coefficients of $x^i$ for $i$ less than `len`, taking the second polynomial by value.
    /// The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// p \gets ((p + q) \bmod x^n) \bmod 2^k.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. The sum is trimmed, so when
    /// coefficients cancel modulo $2^k$ at the top of the kept range, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2AddTruncatedAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The linear coefficients wrap around to 0.
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x^2+x+5").unwrap();
    /// p.mod_power_of_2_add_truncated_assign(
    ///     NaturalPolynomial::from_str("4*x^2+7*x+2").unwrap(),
    ///     3,
    ///     3,
    /// );
    /// assert_eq!(p.to_string(), "6*x^2+7");
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x^2+x+5").unwrap();
    /// p.mod_power_of_2_add_truncated_assign(
    ///     NaturalPolynomial::from_str("4*x^2+7*x+2").unwrap(),
    ///     1,
    ///     3,
    /// );
    /// assert_eq!(p.to_string(), "7");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_add_series` from `fmpz_mod_poly/add_series.c`, FLINT
    /// 3.6.0, with the modulus $2^k$.
    fn mod_power_of_2_add_truncated_assign(&mut self, mut other: Self, len: u64, pow: u64) {
        assert_reduced(self, &other, pow);
        self.truncate_assign(len);
        other.truncate_assign(len);
        add_assign_val(&mut self.coefficients, other.coefficients, pow);
        self.trim();
    }
}

impl ModPowerOf2AddTruncatedAssign<&Self> for NaturalPolynomial {
    /// Adds a [`NaturalPolynomial`] to a [`NaturalPolynomial`] modulo $2^k$ in place, keeping only
    /// the coefficients of $x^i$ for $i$ less than `len`, taking the second polynomial by
    /// reference. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// p \gets ((p + q) \bmod x^n) \bmod 2^k.
    /// $$
    ///
    /// Only the first `len` coefficients of each polynomial are read. The sum is trimmed, so when
    /// coefficients cancel modulo $2^k$ at the top of the kept range, the degree is lower still.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2AddTruncatedAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The linear coefficients wrap around to 0.
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x^2+x+5").unwrap();
    /// p.mod_power_of_2_add_truncated_assign(
    ///     &NaturalPolynomial::from_str("4*x^2+7*x+2").unwrap(),
    ///     3,
    ///     3,
    /// );
    /// assert_eq!(p.to_string(), "6*x^2+7");
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x^2+x+5").unwrap();
    /// p.mod_power_of_2_add_truncated_assign(
    ///     &NaturalPolynomial::from_str("4*x^2+7*x+2").unwrap(),
    ///     1,
    ///     3,
    /// );
    /// assert_eq!(p.to_string(), "7");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_add_series` from `fmpz_mod_poly/add_series.c`, FLINT
    /// 3.6.0, with the modulus $2^k$.
    fn mod_power_of_2_add_truncated_assign(&mut self, other: &Self, len: u64, pow: u64) {
        assert_reduced(self, other, pow);
        self.truncate_assign(len);
        add_assign_ref(
            &mut self.coefficients,
            prefix(&other.coefficients, len),
            pow,
        );
        self.trim();
    }
}
