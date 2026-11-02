// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use crate::integer_polynomial::arithmetic::compositional_inverse_truncated::*;
use crate::test_util::integer_polynomial::arithmetic::compose_truncated::*;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::Polynomial;
use malachite_base::unsigned_polynomial::arithmetic::mod_compositional_inverse_truncated::*;

// The compositional inverse of `p` to `len` terms, one coefficient at a time: the coefficient of
// $x^i$ in $p(q)$ is that of $x^i$ in $q$, times the coefficient $c = \pm 1$ of $x$ in $p$, plus
// terms from lower coefficients of $q$, so each new coefficient makes one composition's coefficient
// vanish.
pub fn compositional_inverse_truncated_naive(
    p: &IntegerPolynomial,
    len: u64,
) -> Option<IntegerPolynomial> {
    let xs = p.coefficients_asc();
    if xs.len() < 2 || xs[0] != 0u32 || xs[1].unsigned_abs_ref() != &1u32 {
        return None;
    }
    let c = xs[1].clone();
    let n = usize::exact_from(len);
    let mut q = vec![Integer::ZERO; n];
    if n > 1 {
        q[1] = c.clone();
    }
    for i in 2..n {
        let composed = integer_polynomial_compose_truncated_naive(
            p,
            &IntegerPolynomial::from_coefficients_asc(q.clone()),
            u64::exact_from(i + 1),
        );
        let ci = composed
            .coefficients_asc()
            .get(i)
            .cloned()
            .unwrap_or(Integer::ZERO);
        q[i] = -ci * &c;
    }
    Some(IntegerPolynomial::from_coefficients_asc(q))
}

fn run(
    p: &IntegerPolynomial,
    len: u64,
    kernel: impl Fn(&IntegerRing, &[Integer], usize) -> Option<Vec<Integer>>,
) -> Option<IntegerPolynomial> {
    let xs = p.coefficients_asc();
    let n = usize::exact_from(len);
    if n < 3 || xs.len() < 2 || xs[0] != 0u32 || xs[1].unsigned_abs_ref() != &1u32 {
        return compositional_inverse_truncated_naive(p, len);
    }
    kernel(&IntegerRing, xs, n).map(IntegerPolynomial::from_coefficients_asc)
}

pub fn compositional_inverse_truncated_lagrange_wrapper(
    p: &IntegerPolynomial,
    len: u64,
) -> Option<IntegerPolynomial> {
    run(p, len, |r, xs, n| {
        compositional_inverse_truncated_lagrange(r, xs, n)
    })
}

pub fn compositional_inverse_truncated_lagrange_fast_wrapper(
    p: &IntegerPolynomial,
    len: u64,
) -> Option<IntegerPolynomial> {
    run(p, len, |r, xs, n| {
        compositional_inverse_truncated_lagrange_fast(r, xs, n)
    })
}

pub fn compositional_inverse_truncated_newton_wrapper(
    p: &IntegerPolynomial,
    len: u64,
) -> Option<IntegerPolynomial> {
    run(p, len, |r, xs, n| {
        Some(compositional_inverse_truncated_newton(r, xs, n))
    })
}
