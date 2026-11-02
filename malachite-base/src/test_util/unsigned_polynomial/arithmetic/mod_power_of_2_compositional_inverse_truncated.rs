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
use crate::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_compose_truncated::*;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_compositional_inverse_truncated::*;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_compositional_inverse_truncated::*;
use core::marker::PhantomData;

// The compositional inverse modulo $2^k$ of `p` to `len` terms, one coefficient at a time, as in
// `mod_compositional_inverse_truncated_naive`.
pub fn mod_power_of_2_compositional_inverse_truncated_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    len: u64,
    pow: u64,
) -> Option<UnsignedPolynomial<T>> {
    if pow == 0 {
        return Some(UnsignedPolynomial::ZERO);
    }
    let xs = p.coefficients_asc();
    if xs.len() < 2 || xs[0] != T::ZERO || xs[1].even() {
        return None;
    }
    let c_inverse = xs[1].mod_power_of_2_inverse(pow)?;
    let n = usize::exact_from(len);
    let mut q = vec![T::ZERO; n];
    if n > 1 {
        q[1] = c_inverse;
    }
    for i in 2..n {
        let composed = mod_power_of_2_compose_truncated_naive(
            p,
            &UnsignedPolynomial::from_coefficients_asc(q.clone()),
            u64::exact_from(i + 1),
            pow,
        );
        let ci = composed
            .coefficients_asc()
            .get(i)
            .copied()
            .unwrap_or(T::ZERO);
        q[i] = ci
            .mod_power_of_2_neg(pow)
            .mod_power_of_2_mul(c_inverse, pow);
    }
    Some(UnsignedPolynomial::from_coefficients_asc(q))
}

fn run<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    len: u64,
    pow: u64,
    kernel: impl Fn(&ModPowerOf2Ring<T>, &[T], usize) -> Option<Vec<T>>,
) -> Option<UnsignedPolynomial<T>> {
    let xs = p.coefficients_asc();
    let n = usize::exact_from(len);
    if pow == 0 || n < 3 || xs.len() < 2 || xs[0] != T::ZERO || xs[1].even() {
        return mod_power_of_2_compositional_inverse_truncated_naive(p, len, pow);
    }
    kernel(
        &ModPowerOf2Ring {
            pow,
            phantom: PhantomData,
        },
        xs,
        n,
    )
    .map(UnsignedPolynomial::from_coefficients_asc)
}

// `None` also when one of Lagrange's divisions is not possible modulo $2^k$.
pub fn mod_power_of_2_compositional_inverse_truncated_lagrange<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    len: u64,
    pow: u64,
) -> Option<UnsignedPolynomial<T>> {
    run(p, len, pow, |r, xs, n| {
        compositional_inverse_truncated_lagrange(r, xs, n)
    })
}

// `None` also when one of Lagrange's divisions is not possible modulo $2^k$.
pub fn mod_power_of_2_compositional_inverse_truncated_lagrange_fast<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    len: u64,
    pow: u64,
) -> Option<UnsignedPolynomial<T>> {
    run(p, len, pow, |r, xs, n| {
        compositional_inverse_truncated_lagrange_fast(r, xs, n)
    })
}

pub fn mod_power_of_2_compositional_inverse_truncated_newton<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    len: u64,
    pow: u64,
) -> Option<UnsignedPolynomial<T>> {
    run(p, len, pow, |r, xs, n| {
        Some(compositional_inverse_truncated_newton(r, xs, n))
    })
}
