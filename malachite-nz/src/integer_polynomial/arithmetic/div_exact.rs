// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2008, 2009 William Hart
//
//      Copyright © 2010, 2011 Sebastian Pancratz
//
//      Copyright © 2023 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use crate::integer_polynomial::arithmetic::pseudo_div_rem::{div_rem_divide_and_conquer, trimmed};
use crate::natural::Natural;
use alloc::vec::Vec;
use core::cmp::min;
use malachite_base::num::arithmetic::traits::{
    DivExact, DivExactAssign, DivisibleBy, NegAssign, SubMulAssign,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::{CheckedDivExact, Polynomial};

// The quotient of the polynomials with coefficients `xs` and `ys`, assuming that the division is
// exact, where `ys` is nonempty, its last coefficient is nonzero, and `xs` is at least as long: the
// low zeros of `ys` are discarded along with as many coefficients of `xs`; then the low half of the
// quotient is found as a power series, dividing by the lowest coefficient of `ys`, and the high
// half from the top, dividing by the highest. If the division is not exact, this may panic or
// return a meaningless result.
//
// This is equivalent to `_gr_poly_divexact_basecase_bidirectional` from
// `gr_poly/divexact_bidirectional.c`, FLINT 3.6.0, with integer coefficients.
crate_test_fn! {div_exact_bidirectional(xs: &[Integer], ys: &[Integer]) -> Vec<Integer> {
    let zeros = ys.iter().take_while(|&y| *y == 0u32).count();
    let (xs, ys) = (&xs[zeros..], &ys[zeros..]);
    let q_len = xs.len() - ys.len() + 1;
    if ys.len() == 1 {
        return xs.iter().map(|x| x.div_exact(&ys[0])).collect();
    }
    let high_len = q_len >> 1;
    let low_len = q_len - high_len;
    let mut qs: Vec<Integer> = Vec::with_capacity(q_len);
    for i in 0..low_len {
        let mut s = xs[i].clone();
        for j in 1..=min(i, ys.len() - 1) {
            s.sub_mul_assign(&ys[j], &qs[i - j]);
        }
        qs.push(s.div_exact(&ys[0]));
    }
    let lead = ys.last().unwrap();
    let xs_high = &xs[low_len..];
    let mut high = vec![Integer::ZERO; high_len];
    for i in 0..high_len {
        let k = high_len - 1 - i;
        let mut s = xs_high[xs_high.len() - 1 - i].clone();
        for l in 1..=min(i, ys.len() - 1) {
            s.sub_mul_assign(&ys[ys.len() - 1 - l], &high[k + l]);
        }
        high[k] = s.div_exact(lead);
    }
    qs.extend(high);
    qs
}}

// The largest number of bits in the absolute values of `xs`.
fn max_bits(xs: &[Integer]) -> u64 {
    xs.iter()
        .map(SignificantBits::significant_bits)
        .max()
        .unwrap_or(0)
}

// The quotient of the polynomials with coefficients `xs` and `ys`, assuming that the division is
// exact, as for `div_exact_bidirectional`: from both ends for short quotients or divisors, and by
// divide-and-conquer otherwise.
//
// This is equivalent to `_fmpz_poly_divexact` from `fmpz_poly/divexact.c`, FLINT 3.6.0.
crate_test_fn! {div_exact_dispatch(xs: &[Integer], ys: &[Integer]) -> Vec<Integer> {
    let q_len = xs.len() - ys.len() + 1;
    if q_len == 1 {
        return vec![xs.last().unwrap().div_exact(ys.last().unwrap())];
    }
    if q_len <= 100 || ys.len() <= 16 {
        return div_exact_bidirectional(xs, ys);
    }
    let x_bits = max_bits(&xs[..q_len]);
    let y_bits = max_bits(&ys[..min(ys.len(), q_len)]);
    let mut y_cutoff = if y_bits > 3000 { 20 } else { 60 };
    let mut q_cutoff = if x_bits > 1000 { 100 } else { 200 };
    if x_bits >= 100 * y_bits {
        q_cutoff <<= 1;
        y_cutoff <<= 1;
    }
    if q_len <= q_cutoff || ys.len() <= y_cutoff {
        div_exact_bidirectional(xs, ys)
    } else {
        div_rem_divide_and_conquer(xs, ys)
            .expect("the division must be exact")
            .0
    }
}}

fn div_exact_helper(p: &IntegerPolynomial, q: &IntegerPolynomial) -> IntegerPolynomial {
    assert_ne!(*q, IntegerPolynomial::ZERO, "division by zero");
    if p.coefficients.len() < q.coefficients.len() {
        return IntegerPolynomial::ZERO;
    }
    trimmed(div_exact_dispatch(&p.coefficients, &q.coefficients))
}

// The sum of the coefficients of the polynomial with coefficients `xs`: its value at 1.
pub(crate) fn sum(xs: &[Integer]) -> Integer {
    xs.iter().sum()
}

// The quotient of `p` by `q`, or `None` if `q` does not divide `p`. The constant coefficients, and
// the values at 1, are checked first, since `q` can only divide `p` if they divide.
//
// This is equivalent to `fmpz_poly_divides` from `fmpz_poly/divides.c`, FLINT 3.6.0, except that
// `None` is returned where FLINT throws on a zero divisor.
pub(crate) fn checked_div_exact_helper(
    p: &IntegerPolynomial,
    q: &IntegerPolynomial,
) -> Option<IntegerPolynomial> {
    if *q == IntegerPolynomial::ZERO {
        return None;
    }
    if *p == IntegerPolynomial::ZERO {
        return Some(IntegerPolynomial::ZERO);
    }
    let (xs, ys) = (&p.coefficients, &q.coefficients);
    if xs.len() < ys.len()
        || !(&xs[0]).divisible_by(&ys[0])
        || xs.len() > 1 && !sum(xs).divisible_by(sum(ys))
    {
        return None;
    }
    let (qs, rs) = div_rem_divide_and_conquer(xs, ys)?;
    if rs.iter().any(|r| *r != 0u32) {
        None
    } else {
        Some(trimmed(qs))
    }
}

impl DivExact<Integer> for IntegerPolynomial {
    type Output = Self;

    /// Divides an [`IntegerPolynomial`] by an [`Integer`], taking both by value. Every coefficient
    /// of the polynomial must be exactly divisible by the [`Integer`]. If one isn't, this function
    /// may panic or return a meaningless result.
    ///
    /// $$
    /// f(p, c) = \frac{p}{c}.
    /// $$
    ///
    /// A polynomial is divisible by $c$ exactly when $|c|$ divides its
    /// [`content`](malachite_base::num::arithmetic::traits::Content::content), so that is how to
    /// check beforehand.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// polynomial's coefficients.
    ///
    /// # Panics
    /// Panics if `c` is zero. May panic if a coefficient of the polynomial is not divisible by `c`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivExact;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2-3*x+9").unwrap();
    /// assert_eq!(
    ///     p.clone().div_exact(Integer::from(3)).to_string(),
    ///     "2*x^2-x+3"
    /// );
    /// assert_eq!(
    ///     p.clone().div_exact(Integer::from(-3)).to_string(),
    ///     "-2*x^2+x-3"
    /// );
    /// assert_eq!(
    ///     IntegerPolynomial::ZERO.div_exact(Integer::from(5)),
    ///     IntegerPolynomial::ZERO
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_scalar_divexact_fmpz` from
    /// `fmpz_poly/scalar_divexact_fmpz.c`, FLINT 3.6.0.
    #[inline]
    fn div_exact(mut self, c: Integer) -> Self {
        self.div_exact_assign(&c);
        self
    }
}

impl<'a> DivExact<&'a Integer> for IntegerPolynomial {
    type Output = Self;

    /// Divides an [`IntegerPolynomial`] by an [`Integer`], taking the polynomial by value and the
    /// [`Integer`] by reference. Every coefficient of the polynomial must be exactly divisible by
    /// the [`Integer`]. If one isn't, this function may panic or return a meaningless result.
    ///
    /// See the documentation for the [`DivExact`] implementation on [`IntegerPolynomial`] that
    /// takes both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// polynomial's coefficients.
    ///
    /// # Panics
    /// Panics if `c` is zero. May panic if a coefficient of the polynomial is not divisible by `c`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivExact;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2-3*x+9").unwrap();
    /// assert_eq!(
    ///     p.clone().div_exact(&Integer::from(3)).to_string(),
    ///     "2*x^2-x+3"
    /// );
    /// assert_eq!(
    ///     p.clone().div_exact(&Integer::from(-3)).to_string(),
    ///     "-2*x^2+x-3"
    /// );
    /// assert_eq!(
    ///     IntegerPolynomial::ZERO.div_exact(&Integer::from(5)),
    ///     IntegerPolynomial::ZERO
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_scalar_divexact_fmpz` from
    /// `fmpz_poly/scalar_divexact_fmpz.c`, FLINT 3.6.0.
    #[inline]
    fn div_exact(mut self, c: &'a Integer) -> Self {
        self.div_exact_assign(c);
        self
    }
}

impl DivExact<Integer> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Divides an [`IntegerPolynomial`] by an [`Integer`], taking the polynomial by reference and
    /// the [`Integer`] by value. Every coefficient of the polynomial must be exactly divisible by
    /// the [`Integer`]. If one isn't, this function may panic or return a meaningless result.
    ///
    /// See the documentation for the [`DivExact`] implementation on [`IntegerPolynomial`] that
    /// takes both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// polynomial's coefficients.
    ///
    /// # Panics
    /// Panics if `c` is zero. May panic if a coefficient of the polynomial is not divisible by `c`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivExact;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2-3*x+9").unwrap();
    /// assert_eq!((&p).div_exact(Integer::from(3)).to_string(), "2*x^2-x+3");
    /// assert_eq!((&p).div_exact(Integer::from(-3)).to_string(), "-2*x^2+x-3");
    /// assert_eq!(
    ///     IntegerPolynomial::ZERO.div_exact(Integer::from(5)),
    ///     IntegerPolynomial::ZERO
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_scalar_divexact_fmpz` from
    /// `fmpz_poly/scalar_divexact_fmpz.c`, FLINT 3.6.0.
    #[inline]
    fn div_exact(self, c: Integer) -> IntegerPolynomial {
        self.div_exact(&c)
    }
}

impl<'a> DivExact<&'a Integer> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Divides an [`IntegerPolynomial`] by an [`Integer`], taking both by reference. Every
    /// coefficient of the polynomial must be exactly divisible by the [`Integer`]. If one isn't,
    /// this function may panic or return a meaningless result.
    ///
    /// See the documentation for the [`DivExact`] implementation on [`IntegerPolynomial`] that
    /// takes both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// polynomial's coefficients.
    ///
    /// # Panics
    /// Panics if `c` is zero. May panic if a coefficient of the polynomial is not divisible by `c`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivExact;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2-3*x+9").unwrap();
    /// assert_eq!((&p).div_exact(&Integer::from(3)).to_string(), "2*x^2-x+3");
    /// assert_eq!((&p).div_exact(&Integer::from(-3)).to_string(), "-2*x^2+x-3");
    /// assert_eq!(
    ///     IntegerPolynomial::ZERO.div_exact(&Integer::from(5)),
    ///     IntegerPolynomial::ZERO
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_scalar_divexact_fmpz` from
    /// `fmpz_poly/scalar_divexact_fmpz.c`, FLINT 3.6.0.
    fn div_exact(self, c: &'a Integer) -> IntegerPolynomial {
        assert_ne!(*c, 0u32, "division by zero");
        // Only a coefficient that is not divisible by `c` can come out as zero, so trimming is what
        // keeps even a meaningless result a valid polynomial.
        IntegerPolynomial::from_coefficients_asc(match *c {
            integer_one!() => self.coefficients.clone(),
            integer_negative_one!() => self.coefficients.iter().map(|x| -x).collect(),
            _ => self.coefficients.iter().map(|x| x.div_exact(c)).collect(),
        })
    }
}

impl DivExactAssign<Integer> for IntegerPolynomial {
    /// Divides an [`IntegerPolynomial`] by an [`Integer`] in place, taking the [`Integer`] by
    /// value. Every coefficient of the polynomial must be exactly divisible by the [`Integer`]. If
    /// one isn't, this function may panic or leave a meaningless result.
    ///
    /// $$
    /// p \gets \frac{p}{c}.
    /// $$
    ///
    /// See the documentation for the [`DivExact`] implementation on [`IntegerPolynomial`] that
    /// takes both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// polynomial's coefficients.
    ///
    /// # Panics
    /// Panics if `c` is zero. May panic if a coefficient of the polynomial is not divisible by `c`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivExactAssign;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("6*x^2-3*x+9").unwrap();
    /// p.div_exact_assign(Integer::from(3));
    /// assert_eq!(p.to_string(), "2*x^2-x+3");
    ///
    /// let mut p = IntegerPolynomial::from_str("6*x^2-3*x+9").unwrap();
    /// p.div_exact_assign(Integer::from(-3));
    /// assert_eq!(p.to_string(), "-2*x^2+x-3");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_scalar_divexact_fmpz` from
    /// `fmpz_poly/scalar_divexact_fmpz.c`, FLINT 3.6.0.
    #[inline]
    fn div_exact_assign(&mut self, c: Integer) {
        self.div_exact_assign(&c);
    }
}

impl<'a> DivExactAssign<&'a Integer> for IntegerPolynomial {
    /// Divides an [`IntegerPolynomial`] by an [`Integer`] in place, taking the [`Integer`] by
    /// reference. Every coefficient of the polynomial must be exactly divisible by the [`Integer`].
    /// If one isn't, this function may panic or leave a meaningless result.
    ///
    /// $$
    /// p \gets \frac{p}{c}.
    /// $$
    ///
    /// See the documentation for the [`DivExact`] implementation on [`IntegerPolynomial`] that
    /// takes both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// polynomial's coefficients.
    ///
    /// # Panics
    /// Panics if `c` is zero. May panic if a coefficient of the polynomial is not divisible by `c`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivExactAssign;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("6*x^2-3*x+9").unwrap();
    /// p.div_exact_assign(&Integer::from(3));
    /// assert_eq!(p.to_string(), "2*x^2-x+3");
    ///
    /// let mut p = IntegerPolynomial::from_str("6*x^2-3*x+9").unwrap();
    /// p.div_exact_assign(&Integer::from(-3));
    /// assert_eq!(p.to_string(), "-2*x^2+x-3");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_scalar_divexact_fmpz` from
    /// `fmpz_poly/scalar_divexact_fmpz.c`, FLINT 3.6.0.
    fn div_exact_assign(&mut self, c: &'a Integer) {
        assert_ne!(*c, 0u32, "division by zero");
        match *c {
            integer_one!() => {}
            integer_negative_one!() => self.neg_assign(),
            _ => {
                for x in &mut self.coefficients {
                    x.div_exact_assign(c);
                }
                // Only a coefficient that is not divisible by `c` can come out as zero, so trimming
                // is what keeps even a meaningless result a valid polynomial.
                self.trim();
            }
        }
    }
}

impl DivExact<Self> for IntegerPolynomial {
    type Output = Self;

    /// Divides two [`IntegerPolynomial`]s, assuming that the second divides the first, taking the
    /// dividend by value and the divisor by value.
    ///
    /// $$
    /// f(a, b) = q, \quad a = bq.
    /// $$
    ///
    /// If the divisor does not divide the dividend, this may panic or return a meaningless result.
    ///
    /// Short quotients are found from both ends: the low half as a power series, dividing by the
    /// divisor's lowest nonzero coefficient, and the high half from the top; long ones by
    /// divide-and-conquer.
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
    /// Panics if `other` is 0. May panic if `other` does not divide `self`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivExact;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+13*x+5").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x+5").unwrap();
    /// assert_eq!(p.div_exact(q).to_string(), "2*x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_divexact` from `fmpz_poly/divexact.c`, FLINT 3.6.0.
    #[inline]
    fn div_exact(self, other: Self) -> Self {
        div_exact_helper(&self, &other)
    }
}

impl DivExact<&Self> for IntegerPolynomial {
    type Output = Self;

    /// Divides two [`IntegerPolynomial`]s, assuming that the second divides the first, taking the
    /// dividend by value and the divisor by reference.
    ///
    /// $$
    /// f(a, b) = q, \quad a = bq.
    /// $$
    ///
    /// If the divisor does not divide the dividend, this may panic or return a meaningless result.
    ///
    /// Short quotients are found from both ends: the low half as a power series, dividing by the
    /// divisor's lowest nonzero coefficient, and the high half from the top; long ones by
    /// divide-and-conquer.
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
    /// Panics if `other` is 0. May panic if `other` does not divide `self`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivExact;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+13*x+5").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x+5").unwrap();
    /// assert_eq!(p.div_exact(&q).to_string(), "2*x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_divexact` from `fmpz_poly/divexact.c`, FLINT 3.6.0.
    #[inline]
    fn div_exact(self, other: &Self) -> Self {
        div_exact_helper(&self, other)
    }
}

impl DivExact<IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Divides two [`IntegerPolynomial`]s, assuming that the second divides the first, taking the
    /// dividend by reference and the divisor by value.
    ///
    /// $$
    /// f(a, b) = q, \quad a = bq.
    /// $$
    ///
    /// If the divisor does not divide the dividend, this may panic or return a meaningless result.
    ///
    /// Short quotients are found from both ends: the low half as a power series, dividing by the
    /// divisor's lowest nonzero coefficient, and the high half from the top; long ones by
    /// divide-and-conquer.
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
    /// Panics if `other` is 0. May panic if `other` does not divide `self`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivExact;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+13*x+5").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x+5").unwrap();
    /// assert_eq!((&p).div_exact(q).to_string(), "2*x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_divexact` from `fmpz_poly/divexact.c`, FLINT 3.6.0.
    #[inline]
    fn div_exact(self, other: IntegerPolynomial) -> IntegerPolynomial {
        div_exact_helper(self, &other)
    }
}

impl DivExact<&IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Divides two [`IntegerPolynomial`]s, assuming that the second divides the first, taking the
    /// dividend by reference and the divisor by reference.
    ///
    /// $$
    /// f(a, b) = q, \quad a = bq.
    /// $$
    ///
    /// If the divisor does not divide the dividend, this may panic or return a meaningless result.
    ///
    /// Short quotients are found from both ends: the low half as a power series, dividing by the
    /// divisor's lowest nonzero coefficient, and the high half from the top; long ones by
    /// divide-and-conquer.
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
    /// Panics if `other` is 0. May panic if `other` does not divide `self`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivExact;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+13*x+5").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x+5").unwrap();
    /// assert_eq!((&p).div_exact(&q).to_string(), "2*x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_divexact` from `fmpz_poly/divexact.c`, FLINT 3.6.0.
    #[inline]
    fn div_exact(self, other: &IntegerPolynomial) -> IntegerPolynomial {
        div_exact_helper(self, other)
    }
}

impl DivExactAssign<Self> for IntegerPolynomial {
    /// Replaces an [`IntegerPolynomial`] with its quotient by another, assuming that the second
    /// divides the first, taking the divisor by value.
    ///
    /// $$
    /// a \gets q, \quad a = bq.
    /// $$
    ///
    /// If the divisor does not divide the dividend, this may panic or assign a meaningless result.
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
    /// Panics if `other` is 0. May panic if `other` does not divide `self`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivExactAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("6*x^2+13*x+5").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x+5").unwrap();
    /// p.div_exact_assign(q);
    /// assert_eq!(p.to_string(), "2*x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_divexact` from `fmpz_poly/divexact.c`, FLINT 3.6.0.
    #[inline]
    fn div_exact_assign(&mut self, other: Self) {
        *self = div_exact_helper(self, &other);
    }
}

impl DivExactAssign<&Self> for IntegerPolynomial {
    /// Replaces an [`IntegerPolynomial`] with its quotient by another, assuming that the second
    /// divides the first, taking the divisor by reference.
    ///
    /// $$
    /// a \gets q, \quad a = bq.
    /// $$
    ///
    /// If the divisor does not divide the dividend, this may panic or assign a meaningless result.
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
    /// Panics if `other` is 0. May panic if `other` does not divide `self`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivExactAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("6*x^2+13*x+5").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x+5").unwrap();
    /// p.div_exact_assign(&q);
    /// assert_eq!(p.to_string(), "2*x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_divexact` from `fmpz_poly/divexact.c`, FLINT 3.6.0.
    #[inline]
    fn div_exact_assign(&mut self, other: &Self) {
        *self = div_exact_helper(self, other);
    }
}

impl CheckedDivExact<Self> for IntegerPolynomial {
    type Output = Self;

    /// Divides two [`IntegerPolynomial`]s if the second divides the first, returning `None`
    /// otherwise, taking the dividend by value and the divisor by value.
    ///
    /// $$
    /// f(a, b) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } a = bq, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// The constant coefficients, and the values at 1, are compared first, since the divisor can
    /// only divide the dividend if they divide. Returns `None` if `other` is 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedDivExact;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+13*x+5").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x+5").unwrap();
    /// assert_eq!(p.checked_div_exact(q).unwrap().to_string(), "2*x+1");
    ///
    /// let p = IntegerPolynomial::from_str("x^2+1").unwrap();
    /// let q = IntegerPolynomial::from_str("x+1").unwrap();
    /// assert!(p.checked_div_exact(q).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_divides` from `fmpz_poly/divides.c`, FLINT 3.6.0, except
    /// that `None` is returned for a divisor of 0, where FLINT throws.
    #[inline]
    fn checked_div_exact(self, other: Self) -> Option<Self> {
        checked_div_exact_helper(&self, &other)
    }
}

impl CheckedDivExact<&Self> for IntegerPolynomial {
    type Output = Self;

    /// Divides two [`IntegerPolynomial`]s if the second divides the first, returning `None`
    /// otherwise, taking the dividend by value and the divisor by reference.
    ///
    /// $$
    /// f(a, b) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } a = bq, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// The constant coefficients, and the values at 1, are compared first, since the divisor can
    /// only divide the dividend if they divide. Returns `None` if `other` is 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedDivExact;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+13*x+5").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x+5").unwrap();
    /// assert_eq!(p.checked_div_exact(&q).unwrap().to_string(), "2*x+1");
    ///
    /// let p = IntegerPolynomial::from_str("x^2+1").unwrap();
    /// let q = IntegerPolynomial::from_str("x+1").unwrap();
    /// assert!(p.checked_div_exact(&q).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_divides` from `fmpz_poly/divides.c`, FLINT 3.6.0, except
    /// that `None` is returned for a divisor of 0, where FLINT throws.
    #[inline]
    fn checked_div_exact(self, other: &Self) -> Option<Self> {
        checked_div_exact_helper(&self, other)
    }
}

impl CheckedDivExact<IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Divides two [`IntegerPolynomial`]s if the second divides the first, returning `None`
    /// otherwise, taking the dividend by reference and the divisor by value.
    ///
    /// $$
    /// f(a, b) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } a = bq, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// The constant coefficients, and the values at 1, are compared first, since the divisor can
    /// only divide the dividend if they divide. Returns `None` if `other` is 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedDivExact;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+13*x+5").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x+5").unwrap();
    /// assert_eq!((&p).checked_div_exact(q).unwrap().to_string(), "2*x+1");
    ///
    /// let p = IntegerPolynomial::from_str("x^2+1").unwrap();
    /// let q = IntegerPolynomial::from_str("x+1").unwrap();
    /// assert!((&p).checked_div_exact(q).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_divides` from `fmpz_poly/divides.c`, FLINT 3.6.0, except
    /// that `None` is returned for a divisor of 0, where FLINT throws.
    #[inline]
    fn checked_div_exact(self, other: IntegerPolynomial) -> Option<IntegerPolynomial> {
        checked_div_exact_helper(self, &other)
    }
}

impl CheckedDivExact<&IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Divides two [`IntegerPolynomial`]s if the second divides the first, returning `None`
    /// otherwise, taking the dividend by reference and the divisor by reference.
    ///
    /// $$
    /// f(a, b) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } a = bq, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// The constant coefficients, and the values at 1, are compared first, since the divisor can
    /// only divide the dividend if they divide. Returns `None` if `other` is 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedDivExact;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+13*x+5").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x+5").unwrap();
    /// assert_eq!((&p).checked_div_exact(&q).unwrap().to_string(), "2*x+1");
    ///
    /// let p = IntegerPolynomial::from_str("x^2+1").unwrap();
    /// let q = IntegerPolynomial::from_str("x+1").unwrap();
    /// assert!((&p).checked_div_exact(&q).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_divides` from `fmpz_poly/divides.c`, FLINT 3.6.0, except
    /// that `None` is returned for a divisor of 0, where FLINT throws.
    #[inline]
    fn checked_div_exact(self, other: &IntegerPolynomial) -> Option<IntegerPolynomial> {
        checked_div_exact_helper(self, other)
    }
}
