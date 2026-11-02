// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_polynomial::RationalPolynomial;
use crate::rational_polynomial::arithmetic::compositional_inverse_truncated::*;
use crate::test_util::rational_polynomial::arithmetic::compose_truncated::*;
use malachite_base::num::arithmetic::traits::Reciprocal;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::*;
use malachite_base::unsigned_polynomial::arithmetic::mod_compositional_inverse_truncated::*;

fn padded(p: RationalPolynomial, len: usize) -> Vec<Rational> {
    let mut xs = p.into_coefficients_asc();
    xs.resize(len, Rational::ZERO);
    xs
}

fn polynomial(xs: &[Rational]) -> RationalPolynomial {
    RationalPolynomial::from_coefficients_asc(xs.to_vec())
}

// The field of rationals, for running the generic reversion kernels on rational coefficients.
pub struct RationalRing;

impl SeriesReversionRing for RationalRing {
    type C = Rational;

    fn zero(&self) -> Rational {
        Rational::ZERO
    }

    fn is_zero(&self, x: &Rational) -> bool {
        *x == 0u32
    }

    fn add(&self, x: &Rational, y: &Rational) -> Rational {
        x + y
    }

    fn sub(&self, x: &Rational, y: &Rational) -> Rational {
        x - y
    }

    fn mul(&self, x: &Rational, y: &Rational) -> Rational {
        x * y
    }

    fn inverse(&self, x: &Rational) -> Option<Rational> {
        if *x == 0u32 {
            None
        } else {
            Some(x.reciprocal())
        }
    }

    fn div_by_index(&self, x: &Rational, i: u64) -> Option<Rational> {
        Some(x / Rational::from(i))
    }

    fn mul_by_index(&self, x: &Rational, i: u64) -> Rational {
        x * Rational::from(i)
    }

    fn mul_truncated(&self, xs: &[Rational], ys: &[Rational], len: usize) -> Vec<Rational> {
        padded(
            polynomial(xs).mul_truncated(polynomial(ys), u64::exact_from(len)),
            len,
        )
    }

    fn inverse_truncated(&self, xs: &[Rational], len: usize) -> Vec<Rational> {
        padded(
            polynomial(xs)
                .inverse_truncated(u64::exact_from(len))
                .unwrap(),
            len,
        )
    }

    fn div_truncated(&self, xs: &[Rational], ys: &[Rational], len: usize) -> Vec<Rational> {
        padded(
            polynomial(xs).div_truncated(polynomial(ys), u64::exact_from(len)),
            len,
        )
    }

    fn compose_truncated(&self, xs: &[Rational], ys: &[Rational], len: usize) -> Vec<Rational> {
        padded(
            polynomial(xs).compose_truncated(polynomial(ys), u64::exact_from(len)),
            len,
        )
    }
}

// The compositional inverse of `p` to `len` terms, one coefficient at a time: the coefficient of
// $x^i$ in $p(q)$ is that of $x^i$ in $q$, times the coefficient $c$ of $x$ in $p$, plus terms from
// lower coefficients of $q$, so each new coefficient makes one composition's coefficient vanish.
pub fn compositional_inverse_truncated_naive(
    p: &RationalPolynomial,
    len: u64,
) -> Option<RationalPolynomial> {
    let xs = p.to_coefficients_asc();
    if xs.len() < 2 || xs[0] != 0u32 || xs[1] == 0u32 {
        return None;
    }
    let c_inverse = (&xs[1]).reciprocal();
    let n = usize::exact_from(len);
    let mut q = vec![Rational::ZERO; n];
    if n > 1 {
        q[1] = c_inverse.clone();
    }
    for i in 2..n {
        let composed = rational_polynomial_compose_truncated_naive(
            p,
            &RationalPolynomial::from_coefficients_asc(q.clone()),
            u64::exact_from(i + 1),
        );
        let ci = composed.coefficient(u64::exact_from(i));
        q[i] = -ci * &c_inverse;
    }
    Some(RationalPolynomial::from_coefficients_asc(q))
}

fn run(
    p: &RationalPolynomial,
    len: u64,
    kernel: impl Fn(&RationalRing, &[Rational], usize) -> Option<Vec<Rational>>,
) -> Option<RationalPolynomial> {
    let xs = p.to_coefficients_asc();
    let n = usize::exact_from(len);
    if n < 3 || xs.len() < 2 || xs[0] != 0u32 || xs[1] == 0u32 {
        return compositional_inverse_truncated_naive(p, len);
    }
    kernel(&RationalRing, &xs, n).map(RationalPolynomial::from_coefficients_asc)
}

pub fn compositional_inverse_truncated_lagrange_wrapper(
    p: &RationalPolynomial,
    len: u64,
) -> Option<RationalPolynomial> {
    run(p, len, |r, xs, n| {
        compositional_inverse_truncated_lagrange(r, xs, n)
    })
}

pub fn compositional_inverse_truncated_lagrange_fast_wrapper(
    p: &RationalPolynomial,
    len: u64,
) -> Option<RationalPolynomial> {
    if len < 2 {
        return compositional_inverse_truncated_naive(p, len);
    }
    let xs = p.to_coefficients_asc();
    if xs.len() < 2 || xs[0] != 0u32 || xs[1] == 0u32 {
        return None;
    }
    Some(compositional_inverse_truncated_lagrange_fast_rational(
        p,
        usize::exact_from(len),
    ))
}

pub fn compositional_inverse_truncated_newton_wrapper(
    p: &RationalPolynomial,
    len: u64,
) -> Option<RationalPolynomial> {
    if len < 2 {
        return compositional_inverse_truncated_naive(p, len);
    }
    let xs = p.to_coefficients_asc();
    if xs.len() < 2 || xs[0] != 0u32 || xs[1] == 0u32 {
        return None;
    }
    Some(compositional_inverse_truncated_newton_rational(
        p,
        usize::exact_from(len),
    ))
}
