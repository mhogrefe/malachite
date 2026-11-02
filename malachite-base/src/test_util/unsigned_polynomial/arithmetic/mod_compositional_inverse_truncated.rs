// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::polynomial::Polynomial;
use crate::test_util::unsigned_polynomial::arithmetic::mod_compose_truncated::*;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_compositional_inverse_truncated::*;

// The compositional inverse modulo `m` of `p` to `len` terms, one coefficient at a time: the
// coefficient of $x^i$ in $p(q)$ is that of $x^i$ in $q$, times the coefficient $c$ of $x$ in $p$,
// plus terms from lower coefficients of $q$, so each new coefficient makes one composition's
// coefficient vanish.
pub fn mod_compositional_inverse_truncated_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    len: u64,
    m: T,
) -> Option<UnsignedPolynomial<T>> {
    if m == T::ONE {
        return Some(UnsignedPolynomial::ZERO);
    }
    let xs = p.coefficients_asc();
    if xs.len() < 2 || xs[0] != T::ZERO || xs[1] == T::ZERO {
        return None;
    }
    let c_inverse = xs[1].mod_inverse(m)?;
    let n = usize::exact_from(len);
    let mut q = vec![T::ZERO; n];
    if n > 1 {
        q[1] = c_inverse;
    }
    for i in 2..n {
        let composed = mod_compose_truncated_naive(
            p,
            &UnsignedPolynomial::from_coefficients_asc(q.clone()),
            u64::exact_from(i + 1),
            m,
        );
        let ci = composed
            .coefficients_asc()
            .get(i)
            .copied()
            .unwrap_or(T::ZERO);
        q[i] = ci.mod_neg(m).mod_mul(c_inverse, m);
    }
    Some(UnsignedPolynomial::from_coefficients_asc(q))
}

fn run<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    len: u64,
    m: T,
    kernel: impl Fn(&ModRing<T>, &[T], usize) -> Option<Vec<T>>,
) -> Option<UnsignedPolynomial<T>> {
    let xs = p.coefficients_asc();
    let n = usize::exact_from(len);
    if m == T::ONE
        || n < 3
        || xs.len() < 2
        || xs[0] != T::ZERO
        || xs[1] == T::ZERO
        || xs[1].mod_inverse(m).is_none()
    {
        return mod_compositional_inverse_truncated_naive(p, len, m);
    }
    kernel(&ModRing { m }, xs, n).map(UnsignedPolynomial::from_coefficients_asc)
}

// `None` also when one of Lagrange's divisions is not possible modulo `m`.
pub fn mod_compositional_inverse_truncated_lagrange<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    len: u64,
    m: T,
) -> Option<UnsignedPolynomial<T>> {
    run(p, len, m, |r, xs, n| {
        compositional_inverse_truncated_lagrange(r, xs, n)
    })
}

// `None` also when one of Lagrange's divisions is not possible modulo `m`.
pub fn mod_compositional_inverse_truncated_lagrange_fast<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    len: u64,
    m: T,
) -> Option<UnsignedPolynomial<T>> {
    run(p, len, m, |r, xs, n| {
        compositional_inverse_truncated_lagrange_fast(r, xs, n)
    })
}

pub fn mod_compositional_inverse_truncated_newton<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    len: u64,
    m: T,
) -> Option<UnsignedPolynomial<T>> {
    run(p, len, m, |r, xs, n| {
        Some(compositional_inverse_truncated_newton(r, xs, n))
    })
}
