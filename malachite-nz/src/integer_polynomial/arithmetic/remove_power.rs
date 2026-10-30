// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2011 Sebastian Pancratz
//
//      Copyright © 2011 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::IntegerPolynomial;
use crate::integer_polynomial::arithmetic::div_exact::{checked_div_exact_helper, sum};
use malachite_base::num::arithmetic::traits::{Abs, Content, DivExact, Pow, UnsignedAbs};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::factorization::traits::{RemovePower, RemovePowerAssign};

// The largest power of `q` that divides `p`, divided out of `p`, and its exponent. A constant `q`
// divides `p` as often as `|q|` divides the content of `p`. Otherwise the exponent is at most the
// number of times `deg q` fits into `deg p`, and, when the values of the polynomials at 1 are
// nonzero and the divisor's is not 1, the number of times the divisor's value divides the
// dividend's; starting from that bound, the power of `q` is reduced until it divides `p`.
//
// This is equivalent to `fmpz_poly_remove` from `fmpz_poly/remove.c`, FLINT 3.6.0, except that
// constant divisors are handled by the content, where FLINT divides by zero when the dividend's
// value at 1 is 0, and that the exponent is first capped at the degree bound.
fn remove_power_helper(p: &IntegerPolynomial, q: &IntegerPolynomial) -> (IntegerPolynomial, u64) {
    assert_ne!(*q, IntegerPolynomial::ZERO, "division by zero");
    let (xs, ys) = (&p.coefficients, &q.coefficients);
    if let [c] = ys.as_slice() {
        assert!(
            c.unsigned_abs_ref() != &1u32,
            "the divisor must not be a unit"
        );
        if *p == IntegerPolynomial::ZERO {
            return (IntegerPolynomial::ZERO, 0);
        }
        let e = p.content().remove_power(c.unsigned_abs_ref()).1;
        let f = c.pow(e);
        return (
            IntegerPolynomial {
                coefficients: xs.iter().map(|x| x.div_exact(&f)).collect(),
            },
            e,
        );
    }
    if xs.len() < ys.len() {
        return (p.clone(), 0);
    }
    let degree_bound = u64::exact_from((xs.len() - 1) / (ys.len() - 1));
    let p_sum = sum(xs).abs();
    let q_sum = sum(ys).abs();
    let mut e = if q_sum == 0u32 {
        if p_sum != 0u32 {
            return (p.clone(), 0);
        }
        degree_bound
    } else if p_sum == 0u32 || q_sum == 1u32 {
        degree_bound
    } else {
        p_sum
            .unsigned_abs()
            .remove_power(q_sum.unsigned_abs())
            .1
            .min(degree_bound)
    };
    let mut power = q.pow(e);
    while e != 0 {
        if let Some(s) = checked_div_exact_helper(p, &power) {
            return (s, e);
        }
        power = (&power).div_exact(q);
        e -= 1;
    }
    (p.clone(), 0)
}

impl RemovePower<Self> for IntegerPolynomial {
    type Output = Self;

    /// Removes the largest power of an [`IntegerPolynomial`] that divides another, returning the
    /// reduced polynomial and the exponent of that power, taking the polynomial by value and the
    /// factor by value.
    ///
    /// $$
    /// f(a, b) = (q, e), \quad a = b^e q, \quad b \nmid q.
    /// $$
    ///
    /// 0 is left alone, with an exponent of 0, since every power of the factor divides it. A
    /// constant factor divides the polynomial as often as its absolute value divides the content.
    /// Otherwise the exponent is bounded by the degrees and by the values at 1, and the power is
    /// reduced from that bound until it divides.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n \log\log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0, 1, or -1.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::factorization::traits::RemovePower;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("2*x^4+9*x^3+15*x^2+11*x+3").unwrap();
    /// let q = IntegerPolynomial::from_str("x+1").unwrap();
    /// let (s, e) = p.remove_power(q);
    /// assert_eq!(s.to_string(), "2*x+3");
    /// assert_eq!(e, 3);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_remove` from `fmpz_poly/remove.c`, FLINT 3.6.0, except that
    /// a constant factor is handled by the content, where FLINT divides by zero when the dividend's
    /// value at 1 is 0.
    #[inline]
    fn remove_power(self, other: Self) -> (Self, u64) {
        remove_power_helper(&self, &other)
    }
}

impl RemovePower<&Self> for IntegerPolynomial {
    type Output = Self;

    /// Removes the largest power of an [`IntegerPolynomial`] that divides another, returning the
    /// reduced polynomial and the exponent of that power, taking the polynomial by value and the
    /// factor by reference.
    ///
    /// $$
    /// f(a, b) = (q, e), \quad a = b^e q, \quad b \nmid q.
    /// $$
    ///
    /// 0 is left alone, with an exponent of 0, since every power of the factor divides it. A
    /// constant factor divides the polynomial as often as its absolute value divides the content.
    /// Otherwise the exponent is bounded by the degrees and by the values at 1, and the power is
    /// reduced from that bound until it divides.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n \log\log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0, 1, or -1.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::factorization::traits::RemovePower;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("2*x^4+9*x^3+15*x^2+11*x+3").unwrap();
    /// let q = IntegerPolynomial::from_str("x+1").unwrap();
    /// let (s, e) = p.remove_power(&q);
    /// assert_eq!(s.to_string(), "2*x+3");
    /// assert_eq!(e, 3);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_remove` from `fmpz_poly/remove.c`, FLINT 3.6.0, except that
    /// a constant factor is handled by the content, where FLINT divides by zero when the dividend's
    /// value at 1 is 0.
    #[inline]
    fn remove_power(self, other: &Self) -> (Self, u64) {
        remove_power_helper(&self, other)
    }
}

impl RemovePower<IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Removes the largest power of an [`IntegerPolynomial`] that divides another, returning the
    /// reduced polynomial and the exponent of that power, taking the polynomial by reference and
    /// the factor by value.
    ///
    /// $$
    /// f(a, b) = (q, e), \quad a = b^e q, \quad b \nmid q.
    /// $$
    ///
    /// 0 is left alone, with an exponent of 0, since every power of the factor divides it. A
    /// constant factor divides the polynomial as often as its absolute value divides the content.
    /// Otherwise the exponent is bounded by the degrees and by the values at 1, and the power is
    /// reduced from that bound until it divides.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n \log\log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0, 1, or -1.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::factorization::traits::RemovePower;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("2*x^4+9*x^3+15*x^2+11*x+3").unwrap();
    /// let q = IntegerPolynomial::from_str("x+1").unwrap();
    /// let (s, e) = (&p).remove_power(q);
    /// assert_eq!(s.to_string(), "2*x+3");
    /// assert_eq!(e, 3);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_remove` from `fmpz_poly/remove.c`, FLINT 3.6.0, except that
    /// a constant factor is handled by the content, where FLINT divides by zero when the dividend's
    /// value at 1 is 0.
    #[inline]
    fn remove_power(self, other: IntegerPolynomial) -> (IntegerPolynomial, u64) {
        remove_power_helper(self, &other)
    }
}

impl RemovePower<&IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Removes the largest power of an [`IntegerPolynomial`] that divides another, returning the
    /// reduced polynomial and the exponent of that power, taking the polynomial by reference and
    /// the factor by reference.
    ///
    /// $$
    /// f(a, b) = (q, e), \quad a = b^e q, \quad b \nmid q.
    /// $$
    ///
    /// 0 is left alone, with an exponent of 0, since every power of the factor divides it. A
    /// constant factor divides the polynomial as often as its absolute value divides the content.
    /// Otherwise the exponent is bounded by the degrees and by the values at 1, and the power is
    /// reduced from that bound until it divides.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n \log\log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0, 1, or -1.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::factorization::traits::RemovePower;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("2*x^4+9*x^3+15*x^2+11*x+3").unwrap();
    /// let q = IntegerPolynomial::from_str("x+1").unwrap();
    /// let (s, e) = (&p).remove_power(&q);
    /// assert_eq!(s.to_string(), "2*x+3");
    /// assert_eq!(e, 3);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_remove` from `fmpz_poly/remove.c`, FLINT 3.6.0, except that
    /// a constant factor is handled by the content, where FLINT divides by zero when the dividend's
    /// value at 1 is 0.
    #[inline]
    fn remove_power(self, other: &IntegerPolynomial) -> (IntegerPolynomial, u64) {
        remove_power_helper(self, other)
    }
}

impl RemovePowerAssign<Self> for IntegerPolynomial {
    /// Replaces an [`IntegerPolynomial`] with itself divided by the largest power of another that
    /// divides it, returning the exponent of that power, taking the factor by value.
    ///
    /// $$
    /// a \gets q, \quad a = b^e q, \quad b \nmid q,
    /// $$
    /// returning $e$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n \log\log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0, 1, or -1.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::factorization::traits::RemovePowerAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("2*x^4+9*x^3+15*x^2+11*x+3").unwrap();
    /// let q = IntegerPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(p.remove_power_assign(q), 3);
    /// assert_eq!(p.to_string(), "2*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_remove` from `fmpz_poly/remove.c`, FLINT 3.6.0, except that
    /// a constant factor is handled by the content.
    #[inline]
    fn remove_power_assign(&mut self, other: Self) -> u64 {
        let (q, e) = remove_power_helper(self, &other);
        *self = q;
        e
    }
}

impl RemovePowerAssign<&Self> for IntegerPolynomial {
    /// Replaces an [`IntegerPolynomial`] with itself divided by the largest power of another that
    /// divides it, returning the exponent of that power, taking the factor by reference.
    ///
    /// $$
    /// a \gets q, \quad a = b^e q, \quad b \nmid q,
    /// $$
    /// returning $e$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n \log\log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0, 1, or -1.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::factorization::traits::RemovePowerAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("2*x^4+9*x^3+15*x^2+11*x+3").unwrap();
    /// let q = IntegerPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(p.remove_power_assign(&q), 3);
    /// assert_eq!(p.to_string(), "2*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_remove` from `fmpz_poly/remove.c`, FLINT 3.6.0, except that
    /// a constant factor is handled by the content.
    #[inline]
    fn remove_power_assign(&mut self, other: &Self) -> u64 {
        let (q, e) = remove_power_helper(self, other);
        *self = q;
        e
    }
}
