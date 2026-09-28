// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::add::{add_assign_ref, add_assign_val};
use malachite_base::polynomial::{AddTruncated, AddTruncatedAssign, Polynomial};

// The first `len` elements of `xs`, or all of them if there are fewer.
fn prefix(xs: &[Natural], len: u64) -> &[Natural] {
    &xs[..usize::try_from(len).map_or(xs.len(), |len| len.min(xs.len()))]
}

impl AddTruncated<Self> for NaturalPolynomial {
    type Output = Self;

    /// Adds two [`NaturalPolynomial`]s, keeping only the coefficients of $x^i$ for $i$ less than
    /// `len`, taking both by value.
    ///
    /// $$
    /// f(p, q, n) = (p + q) \bmod x^n.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the sum of their images modulo $x^n$,
    /// so only the first `len` coefficients of each are read. Natural coefficients cannot cancel,
    /// but the kept part of either operand can end in zeros, so the sum is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// first `len` coefficients of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::AddTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^3+2*x^2+x+5")
    ///         .unwrap()
    ///         .add_truncated(NaturalPolynomial::from_str("4*x^2+x+2").unwrap(), 3)
    ///         .to_string(),
    ///     "6*x^2+2*x+7"
    /// );
    /// // Only the constant and linear coefficients are kept.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^3+2*x^2+x+5")
    ///         .unwrap()
    ///         .add_truncated(NaturalPolynomial::from_str("4*x^2+x+2").unwrap(), 2)
    ///         .to_string(),
    ///     "2*x+7"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_add_series` from `fmpz_poly/add_series.c`, FLINT 3.6.0.
    fn add_truncated(mut self, mut other: Self, len: u64) -> Self {
        self.truncate_assign(len);
        other.truncate_assign(len);
        add_assign_val(&mut self.coefficients, other.coefficients);
        self.trim();
        self
    }
}

impl AddTruncated<&Self> for NaturalPolynomial {
    type Output = Self;

    /// Adds two [`NaturalPolynomial`]s, keeping only the coefficients of $x^i$ for $i$ less than
    /// `len`, taking the first by value and the second by reference.
    ///
    /// $$
    /// f(p, q, n) = (p + q) \bmod x^n.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the sum of their images modulo $x^n$,
    /// so only the first `len` coefficients of each are read. Natural coefficients cannot cancel,
    /// but the kept part of either operand can end in zeros, so the sum is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// first `len` coefficients of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::AddTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^3+2*x^2+x+5")
    ///         .unwrap()
    ///         .add_truncated(&NaturalPolynomial::from_str("4*x^2+x+2").unwrap(), 3)
    ///         .to_string(),
    ///     "6*x^2+2*x+7"
    /// );
    /// // Only the constant and linear coefficients are kept.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^3+2*x^2+x+5")
    ///         .unwrap()
    ///         .add_truncated(&NaturalPolynomial::from_str("4*x^2+x+2").unwrap(), 2)
    ///         .to_string(),
    ///     "2*x+7"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_add_series` from `fmpz_poly/add_series.c`, FLINT 3.6.0.
    fn add_truncated(mut self, other: &Self, len: u64) -> Self {
        self.truncate_assign(len);
        add_assign_ref(&mut self.coefficients, prefix(&other.coefficients, len));
        self.trim();
        self
    }
}

impl AddTruncated<NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Adds two [`NaturalPolynomial`]s, keeping only the coefficients of $x^i$ for $i$ less than
    /// `len`, taking the first by reference and the second by value.
    ///
    /// $$
    /// f(p, q, n) = (p + q) \bmod x^n.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the sum of their images modulo $x^n$,
    /// so only the first `len` coefficients of each are read. Natural coefficients cannot cancel,
    /// but the kept part of either operand can end in zeros, so the sum is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// first `len` coefficients of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::AddTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^3+2*x^2+x+5").unwrap())
    ///         .add_truncated(NaturalPolynomial::from_str("4*x^2+x+2").unwrap(), 3)
    ///         .to_string(),
    ///     "6*x^2+2*x+7"
    /// );
    /// // Only the constant and linear coefficients are kept.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^3+2*x^2+x+5").unwrap())
    ///         .add_truncated(NaturalPolynomial::from_str("4*x^2+x+2").unwrap(), 2)
    ///         .to_string(),
    ///     "2*x+7"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_add_series` from `fmpz_poly/add_series.c`, FLINT 3.6.0.
    fn add_truncated(self, mut other: NaturalPolynomial, len: u64) -> NaturalPolynomial {
        other.truncate_assign(len);
        add_assign_ref(&mut other.coefficients, prefix(&self.coefficients, len));
        other.trim();
        other
    }
}

impl AddTruncated<&NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Adds two [`NaturalPolynomial`]s, keeping only the coefficients of $x^i$ for $i$ less than
    /// `len`, taking both by reference.
    ///
    /// $$
    /// f(p, q, n) = (p + q) \bmod x^n.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the sum of their images modulo $x^n$,
    /// so only the first `len` coefficients of each are read. Natural coefficients cannot cancel,
    /// but the kept part of either operand can end in zeros, so the sum is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// first `len` coefficients of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::AddTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^3+2*x^2+x+5").unwrap())
    ///         .add_truncated(&NaturalPolynomial::from_str("4*x^2+x+2").unwrap(), 3)
    ///         .to_string(),
    ///     "6*x^2+2*x+7"
    /// );
    /// // Only the constant and linear coefficients are kept.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^3+2*x^2+x+5").unwrap())
    ///         .add_truncated(&NaturalPolynomial::from_str("4*x^2+x+2").unwrap(), 2)
    ///         .to_string(),
    ///     "2*x+7"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_add_series` from `fmpz_poly/add_series.c`, FLINT 3.6.0.
    fn add_truncated(self, other: &NaturalPolynomial, len: u64) -> NaturalPolynomial {
        let mut coefficients = prefix(&self.coefficients, len).to_vec();
        add_assign_ref(&mut coefficients, prefix(&other.coefficients, len));
        NaturalPolynomial::from_coefficients_asc(coefficients)
    }
}

impl AddTruncatedAssign<Self> for NaturalPolynomial {
    /// Adds a [`NaturalPolynomial`] to a [`NaturalPolynomial`] in place, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`, taking the second polynomial by value.
    ///
    /// $$
    /// p \gets (p + q) \bmod x^n.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the sum of their images modulo $x^n$,
    /// so only the first `len` coefficients of each are read. Natural coefficients cannot cancel,
    /// but the kept part of either operand can end in zeros, so the sum is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// first `len` coefficients of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::AddTruncatedAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x^2+x+5").unwrap();
    /// p.add_truncated_assign(NaturalPolynomial::from_str("4*x^2+x+2").unwrap(), 3);
    /// assert_eq!(p.to_string(), "6*x^2+2*x+7");
    ///
    /// // Only the constant and linear coefficients are kept.
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x^2+x+5").unwrap();
    /// p.add_truncated_assign(NaturalPolynomial::from_str("4*x^2+x+2").unwrap(), 2);
    /// assert_eq!(p.to_string(), "2*x+7");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_add_series` from `fmpz_poly/add_series.c`, FLINT 3.6.0.
    fn add_truncated_assign(&mut self, mut other: Self, len: u64) {
        self.truncate_assign(len);
        other.truncate_assign(len);
        add_assign_val(&mut self.coefficients, other.coefficients);
        self.trim();
    }
}

impl AddTruncatedAssign<&Self> for NaturalPolynomial {
    /// Adds a [`NaturalPolynomial`] to a [`NaturalPolynomial`] in place, keeping only the
    /// coefficients of $x^i$ for $i$ less than `len`, taking the second polynomial by reference.
    ///
    /// $$
    /// p \gets (p + q) \bmod x^n.
    /// $$
    ///
    /// The polynomials need not already be truncated: this is the sum of their images modulo $x^n$,
    /// so only the first `len` coefficients of each are read. Natural coefficients cannot cancel,
    /// but the kept part of either operand can end in zeros, so the sum is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// first `len` coefficients of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::AddTruncatedAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x^2+x+5").unwrap();
    /// p.add_truncated_assign(&NaturalPolynomial::from_str("4*x^2+x+2").unwrap(), 3);
    /// assert_eq!(p.to_string(), "6*x^2+2*x+7");
    ///
    /// // Only the constant and linear coefficients are kept.
    /// let mut p = NaturalPolynomial::from_str("x^3+2*x^2+x+5").unwrap();
    /// p.add_truncated_assign(&NaturalPolynomial::from_str("4*x^2+x+2").unwrap(), 2);
    /// assert_eq!(p.to_string(), "2*x+7");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_add_series` from `fmpz_poly/add_series.c`, FLINT 3.6.0.
    fn add_truncated_assign(&mut self, other: &Self, len: u64) {
        self.truncate_assign(len);
        add_assign_ref(&mut self.coefficients, prefix(&other.coefficients, len));
        self.trim();
    }
}
