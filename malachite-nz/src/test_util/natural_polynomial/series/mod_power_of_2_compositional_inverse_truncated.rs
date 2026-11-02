// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::series::mod_power_of_2_compositional_inverse_truncated::*;
use crate::test_util::natural_polynomial::series::mod_power_of_2_compose_truncated::*;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::modular::traits::{ModPowerOf2Inverse, ModPowerOf2Mul, ModPowerOf2Neg};
use malachite_base::polynomial::Polynomial;
use malachite_base::unsigned_polynomial::series::mod_compositional_inverse_truncated::*;

// The compositional inverse modulo $2^k$ of `p` to `len` terms, one coefficient at a time: the
// coefficient of $x^i$ in $p(q)$ is that of $x^i$ in $q$, times the coefficient $c$ of $x$ in $p$,
// plus terms from lower coefficients of $q$, so each new coefficient makes one composition's
// coefficient vanish.
pub fn mod_power_of_2_compositional_inverse_truncated_naive(
    p: &NaturalPolynomial,
    len: u64,
    pow: u64,
) -> Option<NaturalPolynomial> {
    if pow == 0 {
        return Some(NaturalPolynomial::ZERO);
    }
    let xs = p.coefficients_asc();
    if xs.len() < 2 || xs[0] != 0u32 || xs[1].even() {
        return None;
    }
    let c_inverse = (&xs[1]).mod_power_of_2_inverse(pow)?;
    let n = usize::exact_from(len);
    let mut q = vec![Natural::ZERO; n];
    if n > 1 {
        q[1] = c_inverse.clone();
    }
    for i in 2..n {
        let composed = mod_power_of_2_compose_truncated_naive(
            p,
            &NaturalPolynomial::from_coefficients_asc(q.clone()),
            u64::exact_from(i + 1),
            pow,
        );
        let ci = composed
            .coefficients_asc()
            .get(i)
            .cloned()
            .unwrap_or(Natural::ZERO);
        q[i] = ci
            .mod_power_of_2_neg(pow)
            .mod_power_of_2_mul(&c_inverse, pow);
    }
    Some(NaturalPolynomial::from_coefficients_asc(q))
}

// Runs `kernel` with `Natural` coefficients, whatever the size of the modulus.
fn run(
    p: &NaturalPolynomial,
    len: u64,
    pow: u64,
    kernel: impl Fn(&NaturalModPowerOf2Ring, &[Natural], usize) -> Option<Vec<Natural>>,
) -> Option<NaturalPolynomial> {
    let xs = p.coefficients_asc();
    let n = usize::exact_from(len);
    if pow == 0 || n < 3 || xs.len() < 2 || xs[0] != 0u32 || xs[1].even() {
        return mod_power_of_2_compositional_inverse_truncated_naive(p, len, pow);
    }
    kernel(&NaturalModPowerOf2Ring { pow }, xs, n).map(NaturalPolynomial::from_coefficients_asc)
}

// `None` also when one of Lagrange's divisions is not possible modulo $2^k$.
pub fn mod_power_of_2_compositional_inverse_truncated_lagrange(
    p: &NaturalPolynomial,
    len: u64,
    pow: u64,
) -> Option<NaturalPolynomial> {
    run(p, len, pow, |r, xs, n| {
        compositional_inverse_truncated_lagrange(r, xs, n)
    })
}

// `None` also when one of Lagrange's divisions is not possible modulo $2^k$.
pub fn mod_power_of_2_compositional_inverse_truncated_lagrange_fast(
    p: &NaturalPolynomial,
    len: u64,
    pow: u64,
) -> Option<NaturalPolynomial> {
    run(p, len, pow, |r, xs, n| {
        compositional_inverse_truncated_lagrange_fast(r, xs, n)
    })
}

pub fn mod_power_of_2_compositional_inverse_truncated_newton(
    p: &NaturalPolynomial,
    len: u64,
    pow: u64,
) -> Option<NaturalPolynomial> {
    run(p, len, pow, |r, xs, n| {
        Some(compositional_inverse_truncated_newton(r, xs, n))
    })
}
