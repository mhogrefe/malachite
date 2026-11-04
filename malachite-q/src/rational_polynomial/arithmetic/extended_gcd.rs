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

use crate::Rational;
use crate::rational_polynomial::RationalPolynomial;
use crate::rational_polynomial::arithmetic::make_monic::monic_from_primitive_part;
use malachite_base::num::arithmetic::traits::{
    Content, DivExact, ExtendedGcd, Gcd, PrimitivePart, Reciprocal, UnsignedAbs,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::arithmetic::traits::MakeMonic;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::resultant_extended_gcd::resultant_extended_gcd;
use malachite_nz::natural::Natural;

// The constant polynomial `x`.
fn constant(x: Rational) -> RationalPolynomial {
    RationalPolynomial::from_coefficients_asc(vec![x])
}

// `s * d / c`, for the polynomial `s`, the natural number `d`, and the nonzero integer `c`.
fn scaled(s: IntegerPolynomial, d: &Natural, c: Integer) -> RationalPolynomial {
    let d = Integer::from_sign_and_abs_ref(c >= 0u32, d);
    let s = if d == 1u32 {
        s
    } else {
        IntegerPolynomial::from_coefficients_asc(
            s.into_coefficients_asc()
                .into_iter()
                .map(|x| x * &d)
                .collect::<Vec<_>>(),
        )
    };
    RationalPolynomial::from_numerator_and_denominator(s, c.unsigned_abs())
}

// The monic GCD $g$ of `p` and `q`, together with $s$ and $t$ such that $sp + tq = g$. When the GCD
// is 0, so are $s$ and $t$.
//
// This is equivalent to `fmpq_poly_xgcd` from `fmpq_poly/xgcd.c`, FLINT 3.6.0.
fn extended_gcd_helper(
    p: &RationalPolynomial,
    q: &RationalPolynomial,
) -> (RationalPolynomial, RationalPolynomial, RationalPolynomial) {
    if p.len() < q.len() {
        let (g, t, s) = extended_gcd_helper(q, p);
        return (g, s, t);
    }
    if *p == RationalPolynomial::ZERO {
        return (
            RationalPolynomial::ZERO,
            RationalPolynomial::ZERO,
            RationalPolynomial::ZERO,
        );
    }
    if *q == RationalPolynomial::ZERO {
        return (
            p.make_monic(),
            constant(p.leading_coefficient().reciprocal()),
            RationalPolynomial::ZERO,
        );
    }
    if q.len() == 1 {
        return (
            RationalPolynomial::one(),
            RationalPolynomial::ZERO,
            constant(q.leading_coefficient().reciprocal()),
        );
    }
    let a_content = (&p.numerator).content();
    let b_content = (&q.numerator).content();
    let a = (&p.numerator).primitive_part();
    let b = (&q.numerator).primitive_part();
    let g = (&a).gcd(&b);
    let (c, d) = if g.len() > 1 {
        ((&a).div_exact(&g), (&b).div_exact(&g))
    } else {
        (a, b)
    };
    let (r, s, t) = resultant_extended_gcd(&c, &d);
    let g_lead = g.leading_coefficient().clone();
    let s = scaled(s, &p.denominator, Integer::from(a_content) * &r * &g_lead);
    let t = scaled(t, &q.denominator, Integer::from(b_content) * r * g_lead);
    (monic_from_primitive_part(g), s, t)
}

impl ExtendedGcd<Self> for RationalPolynomial {
    type Gcd = Self;
    type Cofactor = Self;

    /// Computes the monic GCD of two [`RationalPolynomial`]s, together with cofactors, taking both
    /// by value.
    ///
    /// $$
    /// f(a, b) = (g, s, t), \quad sa + tb = g,
    /// $$
    ///
    /// where $g = \gcd(a, b)$ is monic, or 0 if both polynomials are 0. If $a$ and $b$ are nonzero
    /// and $\deg g < \min(\deg a, \deg b)$, then $\deg s < \deg b - \deg g$ and $\deg t < \deg a -
    /// \deg g$. If $b$ is 0, the result is $(a/c, 1/c, 0)$, where $c$ is the leading coefficient of
    /// $a$; if both are 0, all three are 0.
    ///
    /// The GCD $h$ of the primitive parts of the numerators is found first. The cofactors then come
    /// from an extended GCD of the primitive parts divided by $h$, found modulo many primes and
    /// reconstructed with the Chinese remainder theorem, which also yields their resultant.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3)$
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerators and denominators of both polynomials' coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ExtendedGcd;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2-1/2").unwrap();
    /// let q = RationalPolynomial::from_str("3*x^2+6*x+3").unwrap();
    /// let (g, s, t) = p.extended_gcd(q);
    /// assert_eq!(g.to_string(), "x+1");
    /// assert_eq!(s.to_string(), "-1");
    /// assert_eq!(t.to_string(), "1/6");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_xgcd` from `fmpq_poly/xgcd.c`, FLINT 3.6.0.
    #[inline]
    fn extended_gcd(self, other: Self) -> (Self, Self, Self) {
        extended_gcd_helper(&self, &other)
    }
}

impl ExtendedGcd<&Self> for RationalPolynomial {
    type Gcd = Self;
    type Cofactor = Self;

    /// Computes the monic GCD of two [`RationalPolynomial`]s, together with cofactors, taking the
    /// first by value and the second by reference.
    ///
    /// $$
    /// f(a, b) = (g, s, t), \quad sa + tb = g,
    /// $$
    ///
    /// where $g = \gcd(a, b)$ is monic, or 0 if both polynomials are 0. If $a$ and $b$ are nonzero
    /// and $\deg g < \min(\deg a, \deg b)$, then $\deg s < \deg b - \deg g$ and $\deg t < \deg a -
    /// \deg g$. If $b$ is 0, the result is $(a/c, 1/c, 0)$, where $c$ is the leading coefficient of
    /// $a$; if both are 0, all three are 0.
    ///
    /// The GCD $h$ of the primitive parts of the numerators is found first. The cofactors then come
    /// from an extended GCD of the primitive parts divided by $h$, found modulo many primes and
    /// reconstructed with the Chinese remainder theorem, which also yields their resultant.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3)$
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerators and denominators of both polynomials' coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ExtendedGcd;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2-1/2").unwrap();
    /// let q = RationalPolynomial::from_str("3*x^2+6*x+3").unwrap();
    /// let (g, s, t) = p.extended_gcd(&q);
    /// assert_eq!(g.to_string(), "x+1");
    /// assert_eq!(s.to_string(), "-1");
    /// assert_eq!(t.to_string(), "1/6");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_xgcd` from `fmpq_poly/xgcd.c`, FLINT 3.6.0.
    #[inline]
    fn extended_gcd(self, other: &Self) -> (Self, Self, Self) {
        extended_gcd_helper(&self, other)
    }
}

impl ExtendedGcd<RationalPolynomial> for &RationalPolynomial {
    type Gcd = RationalPolynomial;
    type Cofactor = RationalPolynomial;

    /// Computes the monic GCD of two [`RationalPolynomial`]s, together with cofactors, taking the
    /// first by reference and the second by value.
    ///
    /// $$
    /// f(a, b) = (g, s, t), \quad sa + tb = g,
    /// $$
    ///
    /// where $g = \gcd(a, b)$ is monic, or 0 if both polynomials are 0. If $a$ and $b$ are nonzero
    /// and $\deg g < \min(\deg a, \deg b)$, then $\deg s < \deg b - \deg g$ and $\deg t < \deg a -
    /// \deg g$. If $b$ is 0, the result is $(a/c, 1/c, 0)$, where $c$ is the leading coefficient of
    /// $a$; if both are 0, all three are 0.
    ///
    /// The GCD $h$ of the primitive parts of the numerators is found first. The cofactors then come
    /// from an extended GCD of the primitive parts divided by $h$, found modulo many primes and
    /// reconstructed with the Chinese remainder theorem, which also yields their resultant.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3)$
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerators and denominators of both polynomials' coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ExtendedGcd;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2-1/2").unwrap();
    /// let q = RationalPolynomial::from_str("3*x^2+6*x+3").unwrap();
    /// let (g, s, t) = (&p).extended_gcd(q);
    /// assert_eq!(g.to_string(), "x+1");
    /// assert_eq!(s.to_string(), "-1");
    /// assert_eq!(t.to_string(), "1/6");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_xgcd` from `fmpq_poly/xgcd.c`, FLINT 3.6.0.
    #[inline]
    fn extended_gcd(
        self,
        other: RationalPolynomial,
    ) -> (RationalPolynomial, RationalPolynomial, RationalPolynomial) {
        extended_gcd_helper(self, &other)
    }
}

impl ExtendedGcd<&RationalPolynomial> for &RationalPolynomial {
    type Gcd = RationalPolynomial;
    type Cofactor = RationalPolynomial;

    /// Computes the monic GCD of two [`RationalPolynomial`]s, together with cofactors, taking both
    /// by reference.
    ///
    /// $$
    /// f(a, b) = (g, s, t), \quad sa + tb = g,
    /// $$
    ///
    /// where $g = \gcd(a, b)$ is monic, or 0 if both polynomials are 0. If $a$ and $b$ are nonzero
    /// and $\deg g < \min(\deg a, \deg b)$, then $\deg s < \deg b - \deg g$ and $\deg t < \deg a -
    /// \deg g$. If $b$ is 0, the result is $(a/c, 1/c, 0)$, where $c$ is the leading coefficient of
    /// $a$; if both are 0, all three are 0.
    ///
    /// The GCD $h$ of the primitive parts of the numerators is found first. The cofactors then come
    /// from an extended GCD of the primitive parts divided by $h$, found modulo many primes and
    /// reconstructed with the Chinese remainder theorem, which also yields their resultant.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3)$
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerators and denominators of both polynomials' coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ExtendedGcd;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2-1/2").unwrap();
    /// let q = RationalPolynomial::from_str("3*x^2+6*x+3").unwrap();
    /// let (g, s, t) = (&p).extended_gcd(&q);
    /// assert_eq!(g.to_string(), "x+1");
    /// assert_eq!(s.to_string(), "-1");
    /// assert_eq!(t.to_string(), "1/6");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_xgcd` from `fmpq_poly/xgcd.c`, FLINT 3.6.0.
    #[inline]
    fn extended_gcd(
        self,
        other: &RationalPolynomial,
    ) -> (RationalPolynomial, RationalPolynomial, RationalPolynomial) {
        extended_gcd_helper(self, other)
    }
}
