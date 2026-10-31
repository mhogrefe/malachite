// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2011 Sebastian Pancratz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use crate::rational_polynomial::arithmetic::div_exact::content_and_primitive_part;
use crate::rational_polynomial::arithmetic::div_rem::{mul_integer, over_integer};
use alloc::borrow::Cow;
use core::mem::take;
use malachite_base::num::arithmetic::traits::{Content, DivExact, Pow};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::factorization::traits::{RemovePower, RemovePowerAssign};
use malachite_base::polynomial::Polynomial;
use malachite_nz::integer::Integer;

// The largest power of `q` that divides `p`, divided out of `p`, and its exponent. Over the
// rationals, a power of `q` divides `p` exactly when the same power of the primitive part of the
// numerator of `q` divides the primitive part of the numerator of `p` over the integers, by Gauss's
// lemma; the cofactor is then scaled by the contents and denominators.
//
// This is equivalent to `fmpq_poly_remove` from `fmpq_poly/remove.c`, FLINT 3.6.0, with the search
// for the exponent done by `IntegerPolynomial::remove_power` on the primitive parts.
fn remove_power_helper(
    p: Cow<'_, RationalPolynomial>,
    q: &RationalPolynomial,
) -> (RationalPolynomial, u64) {
    assert_ne!(*q, RationalPolynomial::ZERO, "division by zero");
    assert_ne!(q.numerator.len(), 1, "the divisor must not be a unit");
    if p.numerator.len() < q.numerator.len() {
        return (p.into_owned(), 0);
    }
    let (c_2, pp_2) = content_and_primitive_part(Cow::Borrowed(q));
    let c_1 = (&p.numerator).content();
    let pp_1 = if c_1 == 1u32 {
        Cow::Borrowed(&p.numerator)
    } else {
        Cow::Owned((&p.numerator).div_exact(Integer::from(&c_1)))
    };
    let (cofactor, e) = pp_1.as_ref().remove_power(pp_2);
    if e == 0 {
        return (p.into_owned(), 0);
    }
    // p / q^e = (c_1 / a) (b / c_2)^e cofactor
    let numerator = mul_integer(cofactor, &Integer::from(c_1 * (&q.denominator).pow(e)));
    let denominator = Integer::from(&p.denominator * c_2.pow(e));
    (over_integer(numerator, denominator), e)
}

impl RemovePower<Self> for RationalPolynomial {
    type Output = Self;

    /// Divides a [`RationalPolynomial`] by the largest power of another that divides it, returning
    /// the cofactor and the exponent, taking both by value.
    ///
    /// $$
    /// f(a, b) = (a / b^e, e), \quad b^e \mid a, \quad b^{e+1} \nmid a.
    /// $$
    ///
    /// Over the rationals, a power of `other` divides `self` exactly when the same power of the
    /// primitive part of its numerator divides the primitive part of `self`'s numerator over the
    /// integers, so the search runs there. If `other` does not divide `self`, or `self` is 0, the
    /// exponent is 0 and `self` is returned unchanged.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerators and denominators of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0 or a nonzero constant, which divides every polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::factorization::traits::RemovePower;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let (r, e) = (RationalPolynomial::from_str("x^3-3*x^2+3*x-1").unwrap())
    ///     .remove_power(RationalPolynomial::from_str("2*x-2").unwrap());
    /// assert_eq!(r.to_string(), "1/8");
    /// assert_eq!(e, 3);
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_remove` from `fmpq_poly/remove.c`, FLINT 3.6.0.
    #[inline]
    fn remove_power(self, other: Self) -> (Self, u64) {
        remove_power_helper(Cow::Owned(self), &other)
    }
}

impl RemovePower<&Self> for RationalPolynomial {
    type Output = Self;

    /// Divides a [`RationalPolynomial`] by the largest power of another that divides it, returning
    /// the cofactor and the exponent, taking the first by value and the second by reference.
    ///
    /// $$
    /// f(a, b) = (a / b^e, e), \quad b^e \mid a, \quad b^{e+1} \nmid a.
    /// $$
    ///
    /// Over the rationals, a power of `other` divides `self` exactly when the same power of the
    /// primitive part of its numerator divides the primitive part of `self`'s numerator over the
    /// integers, so the search runs there. If `other` does not divide `self`, or `self` is 0, the
    /// exponent is 0 and `self` is returned unchanged.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerators and denominators of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0 or a nonzero constant, which divides every polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::factorization::traits::RemovePower;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let (r, e) = (RationalPolynomial::from_str("x^3-3*x^2+3*x-1").unwrap())
    ///     .remove_power(&RationalPolynomial::from_str("2*x-2").unwrap());
    /// assert_eq!(r.to_string(), "1/8");
    /// assert_eq!(e, 3);
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_remove` from `fmpq_poly/remove.c`, FLINT 3.6.0.
    #[inline]
    fn remove_power(self, other: &Self) -> (Self, u64) {
        remove_power_helper(Cow::Owned(self), other)
    }
}

impl RemovePower<RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Divides a [`RationalPolynomial`] by the largest power of another that divides it, returning
    /// the cofactor and the exponent, taking the first by reference and the second by value.
    ///
    /// $$
    /// f(a, b) = (a / b^e, e), \quad b^e \mid a, \quad b^{e+1} \nmid a.
    /// $$
    ///
    /// Over the rationals, a power of `other` divides `self` exactly when the same power of the
    /// primitive part of its numerator divides the primitive part of `self`'s numerator over the
    /// integers, so the search runs there. If `other` does not divide `self`, or `self` is 0, the
    /// exponent is 0 and `self` is returned unchanged.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerators and denominators of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0 or a nonzero constant, which divides every polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::factorization::traits::RemovePower;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let (r, e) = (&RationalPolynomial::from_str("x^3-3*x^2+3*x-1").unwrap())
    ///     .remove_power(RationalPolynomial::from_str("2*x-2").unwrap());
    /// assert_eq!(r.to_string(), "1/8");
    /// assert_eq!(e, 3);
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_remove` from `fmpq_poly/remove.c`, FLINT 3.6.0.
    #[inline]
    fn remove_power(self, other: RationalPolynomial) -> (RationalPolynomial, u64) {
        remove_power_helper(Cow::Borrowed(self), &other)
    }
}

impl RemovePower<&RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Divides a [`RationalPolynomial`] by the largest power of another that divides it, returning
    /// the cofactor and the exponent, taking both by reference.
    ///
    /// $$
    /// f(a, b) = (a / b^e, e), \quad b^e \mid a, \quad b^{e+1} \nmid a.
    /// $$
    ///
    /// Over the rationals, a power of `other` divides `self` exactly when the same power of the
    /// primitive part of its numerator divides the primitive part of `self`'s numerator over the
    /// integers, so the search runs there. If `other` does not divide `self`, or `self` is 0, the
    /// exponent is 0 and `self` is returned unchanged.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerators and denominators of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0 or a nonzero constant, which divides every polynomial.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::factorization::traits::RemovePower;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let (r, e) = (&RationalPolynomial::from_str("x^3-3*x^2+3*x-1").unwrap())
    ///     .remove_power(&RationalPolynomial::from_str("2*x-2").unwrap());
    /// assert_eq!(r.to_string(), "1/8");
    /// assert_eq!(e, 3);
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_remove` from `fmpq_poly/remove.c`, FLINT 3.6.0.
    #[inline]
    fn remove_power(self, other: &RationalPolynomial) -> (RationalPolynomial, u64) {
        remove_power_helper(Cow::Borrowed(self), other)
    }
}

impl RemovePowerAssign<Self> for RationalPolynomial {
    /// Divides a [`RationalPolynomial`] in place by the largest power of another that divides it,
    /// returning the exponent, taking the divisor by value.
    ///
    /// $$
    /// a \gets a / b^e, \quad f(a, b) = e, \quad b^e \mid a, \quad b^{e+1} \nmid a.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerators and denominators of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0 or a nonzero constant.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::factorization::traits::RemovePowerAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("x^3-3*x^2+3*x-1").unwrap();
    /// assert_eq!(
    ///     p.remove_power_assign(RationalPolynomial::from_str("2*x-2").unwrap()),
    ///     3
    /// );
    /// assert_eq!(p.to_string(), "1/8");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_remove` from `fmpq_poly/remove.c`, FLINT 3.6.0.
    #[inline]
    fn remove_power_assign(&mut self, other: Self) -> u64 {
        let (q, e) = remove_power_helper(Cow::Owned(take(self)), &other);
        *self = q;
        e
    }
}

impl RemovePowerAssign<&Self> for RationalPolynomial {
    /// Divides a [`RationalPolynomial`] in place by the largest power of another that divides it,
    /// returning the exponent, taking the divisor by reference.
    ///
    /// $$
    /// a \gets a / b^e, \quad f(a, b) = e, \quad b^e \mid a, \quad b^{e+1} \nmid a.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerators and denominators of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0 or a nonzero constant.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::factorization::traits::RemovePowerAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("x^3-3*x^2+3*x-1").unwrap();
    /// assert_eq!(
    ///     p.remove_power_assign(&RationalPolynomial::from_str("2*x-2").unwrap()),
    ///     3
    /// );
    /// assert_eq!(p.to_string(), "1/8");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_remove` from `fmpq_poly/remove.c`, FLINT 3.6.0.
    #[inline]
    fn remove_power_assign(&mut self, other: &Self) -> u64 {
        let (q, e) = remove_power_helper(Cow::Owned(take(self)), other);
        *self = q;
        e
    }
}
