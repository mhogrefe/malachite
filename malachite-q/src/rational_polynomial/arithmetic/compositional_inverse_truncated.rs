// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 Sebastian Pancratz Copyright © 2011 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_polynomial::RationalPolynomial;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    AddMulAssign, DivExact, FloorSqrt, LcmAssign, Reciprocal,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::*;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;

// The lengths at which Newton's iteration is used rather than the fast Lagrange algorithm. Measured
// on polynomials with small fractional coefficients, Newton's iteration was 15% to 35% faster from
// 10 to 18 terms, and the fast Lagrange algorithm was faster from 20 terms on, 19 times as fast at
// 128. Below 10 terms they tie, since Newton's iteration then starts from the fast Lagrange
// algorithm. FLINT uses the fast Lagrange algorithm below 20 terms and Newton's iteration above.
const NEWTON_LENGTHS: core::ops::Range<usize> = 10..20;

// The number of terms below which Newton's iteration stops halving and starts from the fast
// Lagrange algorithm.
//
// This is FLINT's `FLINT_REVERSE_NEWTON_CUTOFF`.
const NEWTON_BASECASE: usize = 8;

// The first `len` coefficients of the numerator of `p`, padded with zeros.
fn padded_numerator(p: &RationalPolynomial, len: usize) -> Vec<Integer> {
    let mut xs = p.numerator_ref().coefficients_asc().to_vec();
    xs.resize(len, Integer::ZERO);
    xs
}

// The reversion of $cx$, which is $x/c$.
fn reversion_of_linear(f: &RationalPolynomial) -> RationalPolynomial {
    RationalPolynomial::from_coefficients_asc(vec![Rational::ZERO, f.coefficient(1).reciprocal()])
}

// The reversion of `f` to `n` terms by the fast Lagrange algorithm, where `f` has constant
// coefficient 0 and a nonzero coefficient of $x$, and `n` is at least 2. The coefficient of $x^i$
// is that of $x^{i-1}$ in $(x/f)^i$, divided by $i$. The powers $(x/f)^j$ for $j \leq \lfloor \sqrt
// n \rfloor$ are kept, and the other powers are reached by multiplying by the largest of them;
// between multiplications, the needed coefficients are dot products of the numerators. Each
// coefficient gets its own denominator, and they are brought to a common one at the end.
//
// This is equivalent to `_fmpq_poly_revert_series_lagrange_fast` from
// `fmpq_poly/revert_series_lagrange_fast.c`, FLINT 3.6.0.
pub(crate) fn compositional_inverse_truncated_lagrange_fast_rational(
    f: &RationalPolynomial,
    n: usize,
) -> RationalPolynomial {
    if f.len() <= 2 {
        return reversion_of_linear(f);
    }
    let m = n.floor_sqrt();
    let f_over_x = RationalPolynomial::from_numerator_and_denominator(
        IntegerPolynomial::from_coefficients_asc(
            f.numerator_ref().coefficients_asc()[1..].to_vec(),
        ),
        f.denominator_ref().clone(),
    );
    // rs[j - 1] is (x / f)^j
    let mut rs: Vec<RationalPolynomial> = Vec::with_capacity(m);
    rs.push(f_over_x.inverse_truncated(u64::exact_from(n - 1)).unwrap());
    for i in 1..m {
        let power = (&rs[i - 1]).mul_truncated(&rs[0], u64::exact_from(n - 1));
        rs.push(power);
    }
    let r_numerators: Vec<Vec<Integer>> = rs.iter().map(|r| padded_numerator(r, n - 1)).collect();
    let mut numerators = vec![Integer::ZERO; n];
    let mut denominators = vec![Natural::ONE; n];
    for i in 1..m {
        numerators[i] = r_numerators[i - 1][i - 1].clone();
        denominators[i] = rs[i - 1].denominator_ref() * Natural::from(i);
    }
    let mut s = rs[m - 1].clone();
    let mut i = m;
    while i < n {
        let s_numerator = padded_numerator(&s, n - 1);
        numerators[i] = s_numerator[i - 1].clone();
        denominators[i] = s.denominator_ref() * Natural::from(i);
        let mut j = 1;
        while j < m && i + j < n {
            let mut dot = Integer::ZERO;
            for (x, y) in s_numerator[..i + j]
                .iter()
                .zip(r_numerators[j - 1][..i + j].iter().rev())
            {
                dot.add_mul_assign(x, y);
            }
            numerators[i + j] = dot;
            denominators[i + j] =
                s.denominator_ref() * rs[j - 1].denominator_ref() * Natural::from(i + j);
            j += 1;
        }
        if i + 1 < n {
            s = s.mul_truncated(&rs[m - 1], u64::exact_from(n - 1));
        }
        i += m;
    }
    let mut denominator = Natural::ONE;
    for d in &denominators {
        denominator.lcm_assign(d);
    }
    for (x, d) in numerators.iter_mut().zip(denominators.iter()) {
        *x *= Integer::from((&denominator).div_exact(d));
    }
    RationalPolynomial::from_numerator_and_denominator(
        IntegerPolynomial::from_coefficients_asc(numerators),
        denominator,
    )
}

// The reversion of `f` to `n` terms by Newton's iteration, where `f` has constant coefficient 0 and
// a nonzero coefficient of $x$, and `n` is at least 2: from an approximation $g$, $g - (f(g) - x)
// g' / f(g)'$ is correct to twice as many terms. The first terms come from the fast Lagrange
// algorithm.
//
// This is equivalent to `_fmpq_poly_revert_series_newton` from `fmpq_poly/revert_series_newton.c`,
// FLINT 3.6.0.
pub(crate) fn compositional_inverse_truncated_newton_rational(
    f: &RationalPolynomial,
    n: usize,
) -> RationalPolynomial {
    if f.len() <= 2 {
        return reversion_of_linear(f);
    }
    let mut steps = vec![n];
    let mut k = n;
    while k > NEWTON_BASECASE {
        k = k.div_ceil(2);
        steps.push(k);
    }
    let mut res = compositional_inverse_truncated_lagrange_fast_rational(f, k);
    for &k in steps.iter().rev().skip(1) {
        let k = u64::exact_from(k);
        let t = f.compose_truncated(&res, k);
        let u = (&t).derivative();
        // The coefficient of x in t is 1, since res is correct to at least two terms; FLINT zeroes
        // it.
        let t = t - RationalPolynomial::x();
        let v = t.div_truncated(u, k);
        res -= v.mul_truncated((&res).derivative(), k);
    }
    res
}

// The compositional inverse of `p` to `len` terms, or `None` if its constant coefficient is not 0
// or its coefficient of $x$ is 0. A polynomial with integer coefficients and coefficient of $x$
// equal to 1 or -1 has an inverse with integer coefficients, which is computed over the integers.
//
// This is equivalent to `fmpq_poly_revert_series` from `fmpq_poly/revert_series.c`, FLINT 3.6.0,
// except that `None` is returned where FLINT throws, and that the choice of algorithm was measured
// (see `NEWTON_LENGTHS`).
pub(crate) fn compositional_inverse_truncated_rational_helper(
    p: &RationalPolynomial,
    len: u64,
) -> Option<RationalPolynomial> {
    let numerator = p.numerator_ref().coefficients_asc();
    if numerator.len() < 2 || numerator[0] != 0u32 || numerator[1] == 0u32 {
        return None;
    }
    if len < 2 {
        return Some(RationalPolynomial::ZERO);
    }
    if *p.denominator_ref() == 1u32 && numerator[1].unsigned_abs_ref() == &1u32 {
        return p
            .numerator_ref()
            .compositional_inverse_truncated(len)
            .map(RationalPolynomial::from);
    }
    let n = usize::exact_from(len);
    Some(if NEWTON_LENGTHS.contains(&n) {
        compositional_inverse_truncated_newton_rational(p, n)
    } else {
        compositional_inverse_truncated_lagrange_fast_rational(p, n)
    })
}

impl CompositionalInverseTruncated for RationalPolynomial {
    type Output = Self;

    /// Finds the compositional inverse of a [`RationalPolynomial`] as a power series, keeping only
    /// the coefficients of $x^i$ for $i$ less than `len`, taking it by value.
    ///
    /// $$
    /// f(p, n) = q, \quad \deg q < n, \quad p(q(x)) \equiv q(p(x)) \equiv x \pmod{x^n}.
    /// $$
    ///
    /// Returns `None` unless the constant coefficient is 0 and the coefficient of $x$ is nonzero;
    /// otherwise the inverse exists and is unique. When the inverse exists and `len` is less than
    /// 2, the result is the zero polynomial.
    ///
    /// When the coefficients are integers and the coefficient of $x$ is 1 or $-1$, the inverse has
    /// integer coefficients and is computed as for an [`IntegerPolynomial`]. Otherwise it is
    /// computed by the fast Lagrange algorithm, in which the coefficient of $x^i$ is that of
    /// $x^{i-1}$ in $(x/p)^i$, divided by $i$, with a common denominator only at the end; when
    /// `len` is from 10 to 19, Newton's iteration is used instead, in which each step doubles the
    /// number of correct coefficients with a composition and a division.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^3 b \log (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^{5/2} b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $b$ is the number of bits of
    /// the largest numerator or denominator of a coefficient of `self` plus the number of bits of
    /// `len`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CompositionalInverseTruncated;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("1/3*x^2+1/2*x").unwrap())
    ///         .compositional_inverse_truncated(5)
    ///         .unwrap()
    ///         .to_string(),
    ///     "-640/27*x^4+64/9*x^3-8/3*x^2+2*x"
    /// );
    /// // The coefficient of x is 0.
    /// assert!(
    ///     (RationalPolynomial::from_str("1/2*x^2").unwrap())
    ///         .compositional_inverse_truncated(5)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_revert_series` from `fmpq_poly/revert_series.c`, FLINT
    /// 3.6.0, except that `None` is returned where FLINT throws, that the inverse modulo $x^0$ or
    /// $x^1$ is 0, and that the choice of algorithm was measured.
    #[inline]
    fn compositional_inverse_truncated(self, len: u64) -> Option<Self> {
        compositional_inverse_truncated_rational_helper(&self, len)
    }
}

impl CompositionalInverseTruncated for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Finds the compositional inverse of a [`RationalPolynomial`] as a power series, keeping only
    /// the coefficients of $x^i$ for $i$ less than `len`, taking it by reference.
    ///
    /// $$
    /// f(p, n) = q, \quad \deg q < n, \quad p(q(x)) \equiv q(p(x)) \equiv x \pmod{x^n}.
    /// $$
    ///
    /// Returns `None` unless the constant coefficient is 0 and the coefficient of $x$ is nonzero;
    /// otherwise the inverse exists and is unique. When the inverse exists and `len` is less than
    /// 2, the result is the zero polynomial.
    ///
    /// When the coefficients are integers and the coefficient of $x$ is 1 or $-1$, the inverse has
    /// integer coefficients and is computed as for an [`IntegerPolynomial`]. Otherwise it is
    /// computed by the fast Lagrange algorithm, in which the coefficient of $x^i$ is that of
    /// $x^{i-1}$ in $(x/p)^i$, divided by $i$, with a common denominator only at the end; when
    /// `len` is from 10 to 19, Newton's iteration is used instead, in which each step doubles the
    /// number of correct coefficients with a composition and a division.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^3 b \log (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^{5/2} b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $b$ is the number of bits of
    /// the largest numerator or denominator of a coefficient of `self` plus the number of bits of
    /// `len`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CompositionalInverseTruncated;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/3*x^2+1/2*x").unwrap())
    ///         .compositional_inverse_truncated(5)
    ///         .unwrap()
    ///         .to_string(),
    ///     "-640/27*x^4+64/9*x^3-8/3*x^2+2*x"
    /// );
    /// // The coefficient of x is 0.
    /// assert!(
    ///     (&RationalPolynomial::from_str("1/2*x^2").unwrap())
    ///         .compositional_inverse_truncated(5)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_revert_series` from `fmpq_poly/revert_series.c`, FLINT
    /// 3.6.0, except that `None` is returned where FLINT throws, that the inverse modulo $x^0$ or
    /// $x^1$ is 0, and that the choice of algorithm was measured.
    #[inline]
    fn compositional_inverse_truncated(self, len: u64) -> Option<RationalPolynomial> {
        compositional_inverse_truncated_rational_helper(self, len)
    }
}
