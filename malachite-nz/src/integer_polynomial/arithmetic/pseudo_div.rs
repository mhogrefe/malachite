// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2008, 2009 William Hart
//
//      Copyright © 2010, 2011 Sebastian Pancratz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::IntegerPolynomial;
use crate::integer_polynomial::arithmetic::pseudo_div_rem::helper;
use malachite_base::polynomial::{PseudoDiv, PseudoDivAssign};

fn pseudo_div_helper(p: &IntegerPolynomial, q: &IntegerPolynomial) -> (IntegerPolynomial, u64) {
    let (q, _, d) = helper(p, q);
    (q, d)
}

impl PseudoDiv<Self> for IntegerPolynomial {
    type Output = Self;

    /// Pseudo-divides two [`IntegerPolynomial`]s, returning the pseudo-quotient and the exponent,
    /// taking the dividend by value and the divisor by value.
    ///
    /// $$
    /// f(a, b) = (q, d), \quad \ell^d a = bq + r, \quad \deg r < \deg b,
    /// $$
    /// where $\ell$ is the leading coefficient of $b$.
    ///
    /// The exponent $d$ is the least for which the pseudo-quotient has integral coefficients; once
    /// it is fixed, the pseudo-quotient and pseudo-remainder are unique. It is at most $\deg a -
    /// \deg b + 1$, and it is 0 when $\ell$ is $\pm 1$ or $b$ divides $a$. When $\deg a < \deg b$,
    /// the pseudo-quotient is 0, the pseudo-remainder is $a$, and $d$ is 0.
    ///
    /// Short divisors are handled one coefficient at a time, multiplying by $\ell$ only when the
    /// next coefficient of the quotient would not be integral; longer ones are split in half
    /// recursively, which finds the same exponent.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::PseudoDiv;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^4+x+1").unwrap();
    /// let q = IntegerPolynomial::from_str("2*x^2+x+1").unwrap();
    /// let (s, d) = p.pseudo_div(q);
    /// assert_eq!(s.to_string(), "4*x^2-2*x-1");
    /// assert_eq!(d, 3);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_pseudo_div` from `fmpz_poly/pseudo_div.c`, FLINT 3.6.0.
    #[inline]
    fn pseudo_div(self, other: Self) -> (Self, u64) {
        pseudo_div_helper(&self, &other)
    }
}

impl PseudoDiv<&Self> for IntegerPolynomial {
    type Output = Self;

    /// Pseudo-divides two [`IntegerPolynomial`]s, returning the pseudo-quotient and the exponent,
    /// taking the dividend by value and the divisor by reference.
    ///
    /// $$
    /// f(a, b) = (q, d), \quad \ell^d a = bq + r, \quad \deg r < \deg b,
    /// $$
    /// where $\ell$ is the leading coefficient of $b$.
    ///
    /// The exponent $d$ is the least for which the pseudo-quotient has integral coefficients; once
    /// it is fixed, the pseudo-quotient and pseudo-remainder are unique. It is at most $\deg a -
    /// \deg b + 1$, and it is 0 when $\ell$ is $\pm 1$ or $b$ divides $a$. When $\deg a < \deg b$,
    /// the pseudo-quotient is 0, the pseudo-remainder is $a$, and $d$ is 0.
    ///
    /// Short divisors are handled one coefficient at a time, multiplying by $\ell$ only when the
    /// next coefficient of the quotient would not be integral; longer ones are split in half
    /// recursively, which finds the same exponent.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::PseudoDiv;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^4+x+1").unwrap();
    /// let q = IntegerPolynomial::from_str("2*x^2+x+1").unwrap();
    /// let (s, d) = p.pseudo_div(&q);
    /// assert_eq!(s.to_string(), "4*x^2-2*x-1");
    /// assert_eq!(d, 3);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_pseudo_div` from `fmpz_poly/pseudo_div.c`, FLINT 3.6.0.
    #[inline]
    fn pseudo_div(self, other: &Self) -> (Self, u64) {
        pseudo_div_helper(&self, other)
    }
}

impl PseudoDiv<IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Pseudo-divides two [`IntegerPolynomial`]s, returning the pseudo-quotient and the exponent,
    /// taking the dividend by reference and the divisor by value.
    ///
    /// $$
    /// f(a, b) = (q, d), \quad \ell^d a = bq + r, \quad \deg r < \deg b,
    /// $$
    /// where $\ell$ is the leading coefficient of $b$.
    ///
    /// The exponent $d$ is the least for which the pseudo-quotient has integral coefficients; once
    /// it is fixed, the pseudo-quotient and pseudo-remainder are unique. It is at most $\deg a -
    /// \deg b + 1$, and it is 0 when $\ell$ is $\pm 1$ or $b$ divides $a$. When $\deg a < \deg b$,
    /// the pseudo-quotient is 0, the pseudo-remainder is $a$, and $d$ is 0.
    ///
    /// Short divisors are handled one coefficient at a time, multiplying by $\ell$ only when the
    /// next coefficient of the quotient would not be integral; longer ones are split in half
    /// recursively, which finds the same exponent.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::PseudoDiv;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^4+x+1").unwrap();
    /// let q = IntegerPolynomial::from_str("2*x^2+x+1").unwrap();
    /// let (s, d) = (&p).pseudo_div(q);
    /// assert_eq!(s.to_string(), "4*x^2-2*x-1");
    /// assert_eq!(d, 3);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_pseudo_div` from `fmpz_poly/pseudo_div.c`, FLINT 3.6.0.
    #[inline]
    fn pseudo_div(self, other: IntegerPolynomial) -> (IntegerPolynomial, u64) {
        pseudo_div_helper(self, &other)
    }
}

impl PseudoDiv<&IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Pseudo-divides two [`IntegerPolynomial`]s, returning the pseudo-quotient and the exponent,
    /// taking the dividend by reference and the divisor by reference.
    ///
    /// $$
    /// f(a, b) = (q, d), \quad \ell^d a = bq + r, \quad \deg r < \deg b,
    /// $$
    /// where $\ell$ is the leading coefficient of $b$.
    ///
    /// The exponent $d$ is the least for which the pseudo-quotient has integral coefficients; once
    /// it is fixed, the pseudo-quotient and pseudo-remainder are unique. It is at most $\deg a -
    /// \deg b + 1$, and it is 0 when $\ell$ is $\pm 1$ or $b$ divides $a$. When $\deg a < \deg b$,
    /// the pseudo-quotient is 0, the pseudo-remainder is $a$, and $d$ is 0.
    ///
    /// Short divisors are handled one coefficient at a time, multiplying by $\ell$ only when the
    /// next coefficient of the quotient would not be integral; longer ones are split in half
    /// recursively, which finds the same exponent.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::PseudoDiv;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^4+x+1").unwrap();
    /// let q = IntegerPolynomial::from_str("2*x^2+x+1").unwrap();
    /// let (s, d) = (&p).pseudo_div(&q);
    /// assert_eq!(s.to_string(), "4*x^2-2*x-1");
    /// assert_eq!(d, 3);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_pseudo_div` from `fmpz_poly/pseudo_div.c`, FLINT 3.6.0.
    #[inline]
    fn pseudo_div(self, other: &IntegerPolynomial) -> (IntegerPolynomial, u64) {
        pseudo_div_helper(self, other)
    }
}

impl PseudoDivAssign<Self> for IntegerPolynomial {
    /// Replaces an [`IntegerPolynomial`] with its pseudo-quotient by another, returning the
    /// exponent, taking the divisor by value.
    ///
    /// $$
    /// a \gets q, \quad \ell^d a = bq + r, \quad \deg r < \deg b,
    /// $$
    /// returning $d$, where $\ell$ is the leading coefficient of $b$.
    ///
    /// The exponent $d$ is the least for which the pseudo-quotient has integral coefficients; once
    /// it is fixed, the pseudo-quotient and pseudo-remainder are unique. It is at most $\deg a -
    /// \deg b + 1$, and it is 0 when $\ell$ is $\pm 1$ or $b$ divides $a$. When $\deg a < \deg b$,
    /// the pseudo-quotient is 0, the pseudo-remainder is $a$, and $d$ is 0.
    ///
    /// Short divisors are handled one coefficient at a time, multiplying by $\ell$ only when the
    /// next coefficient of the quotient would not be integral; longer ones are split in half
    /// recursively, which finds the same exponent.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::PseudoDivAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^4+x+1").unwrap();
    /// let q = IntegerPolynomial::from_str("2*x^2+x+1").unwrap();
    /// assert_eq!(p.pseudo_div_assign(q), 3);
    /// assert_eq!(p.to_string(), "4*x^2-2*x-1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_pseudo_div` from `fmpz_poly/pseudo_div.c`, FLINT 3.6.0.
    #[inline]
    fn pseudo_div_assign(&mut self, other: Self) -> u64 {
        let (x, d) = pseudo_div_helper(self, &other);
        *self = x;
        d
    }
}

impl PseudoDivAssign<&Self> for IntegerPolynomial {
    /// Replaces an [`IntegerPolynomial`] with its pseudo-quotient by another, returning the
    /// exponent, taking the divisor by reference.
    ///
    /// $$
    /// a \gets q, \quad \ell^d a = bq + r, \quad \deg r < \deg b,
    /// $$
    /// returning $d$, where $\ell$ is the leading coefficient of $b$.
    ///
    /// The exponent $d$ is the least for which the pseudo-quotient has integral coefficients; once
    /// it is fixed, the pseudo-quotient and pseudo-remainder are unique. It is at most $\deg a -
    /// \deg b + 1$, and it is 0 when $\ell$ is $\pm 1$ or $b$ divides $a$. When $\deg a < \deg b$,
    /// the pseudo-quotient is 0, the pseudo-remainder is $a$, and $d$ is 0.
    ///
    /// Short divisors are handled one coefficient at a time, multiplying by $\ell$ only when the
    /// next coefficient of the quotient would not be integral; longer ones are split in half
    /// recursively, which finds the same exponent.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::PseudoDivAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^4+x+1").unwrap();
    /// let q = IntegerPolynomial::from_str("2*x^2+x+1").unwrap();
    /// assert_eq!(p.pseudo_div_assign(&q), 3);
    /// assert_eq!(p.to_string(), "4*x^2-2*x-1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_pseudo_div` from `fmpz_poly/pseudo_div.c`, FLINT 3.6.0.
    #[inline]
    fn pseudo_div_assign(&mut self, other: &Self) -> u64 {
        let (x, d) = pseudo_div_helper(self, other);
        *self = x;
        d
    }
}
