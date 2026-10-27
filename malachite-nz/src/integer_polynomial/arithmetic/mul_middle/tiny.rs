// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2008, 2009 William Hart
//
//      Copyright © 2010 Sebastian Pancratz
//
//      Copyright © 2026 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::coefficient::{PolynomialCoefficient, small_value};
use crate::integer_polynomial::arithmetic::mul_truncated::tiny::TINY_STACK_LEN;
use crate::platform::{SignedDoubleLimb, SignedLimb};
use alloc::vec;
use core::cmp::min;
use core::ptr;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::conversion::traits::ExactFrom;

// Sets `out` to the coefficients of $x^i$ for `nlo` $\leq i <$ `nhi` of the product of the
// polynomials with coefficients `xs` and `ys`, both nonempty, using arithmetic in `T`. Every
// element of `xs` and `ys` must be a small `fmpz`, and every coefficient of the product, and every
// partial sum of the terms that make it up, must fit in `T`. `nlo < nhi <= xs.len() + ys.len() - 1`
// must hold, and `out` must have length `nhi - nlo`.
//
// # Worst-case complexity
// $T(n) = O(n^2)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// This is equivalent to `_fmpz_poly_mulmid_tiny1` from `fmpz_poly/mulmid.c`, FLINT 3.6.0, when `T`
// is `SignedLimb`, and to `_fmpz_poly_mulmid_tiny2` when `T` is `SignedDoubleLimb`.
pub(crate) fn mul_middle_to_out_tiny<
    T: PrimitiveSigned + From<SignedLimb>,
    C: PolynomialCoefficient + ExactFrom<T>,
>(
    out: &mut [C],
    xs: &[C],
    ys: &[C],
    nlo: usize,
    nhi: usize,
) {
    let len1 = xs.len();
    let len2 = ys.len();
    let square = ptr::eq(xs, ys);
    // Short factors, the common case, keep their values on the stack, as FLINT's do.
    if len1 <= TINY_STACK_LEN && len2 <= TINY_STACK_LEN {
        let mut xs_values = [T::ZERO; TINY_STACK_LEN];
        let mut ys_values = [T::ZERO; TINY_STACK_LEN];
        assign_small_values(&mut xs_values[..len1], xs);
        let ys_values = if square {
            None
        } else {
            assign_small_values(&mut ys_values[..len2], ys);
            Some(&ys_values[..len2])
        };
        mul_middle_to_out_tiny_with(out, &xs_values[..len1], ys_values, nlo, nhi);
    } else {
        let mut xs_values = vec![T::ZERO; len1];
        assign_small_values(&mut xs_values, xs);
        let ys_values = if square {
            None
        } else {
            let mut ys_values = vec![T::ZERO; len2];
            assign_small_values(&mut ys_values, ys);
            Some(ys_values)
        };
        mul_middle_to_out_tiny_with(out, &xs_values, ys_values.as_deref(), nlo, nhi);
    }
}

// Sets `values` to the values of `xs`, which it must have the length of.
fn assign_small_values<T: From<SignedLimb>, C: PolynomialCoefficient>(values: &mut [T], xs: &[C]) {
    for (v, x) in values.iter_mut().zip(xs) {
        *v = T::from(small_value(x).unwrap());
    }
}

// `mul_middle_to_out_tiny` on the values `xs` and `ys` of the factors' coefficients, where `ys` is
// `None` when the factors are the same.
fn mul_middle_to_out_tiny_with<T: PrimitiveSigned, C: PolynomialCoefficient + ExactFrom<T>>(
    out: &mut [C],
    xs: &[T],
    ys: Option<&[T]>,
    nlo: usize,
    nhi: usize,
) {
    let len1 = xs.len();
    if let Some(ys) = ys {
        let len2 = ys.len();
        for (o, i) in out.iter_mut().zip(nlo..nhi) {
            let top1 = min(len1 - 1, i);
            let top2 = min(len2 - 1, i);
            let n = top1 + top2 + 1 - i;
            let mut s = T::ZERO;
            for (&x, &y) in xs[i - top2..i - top2 + n]
                .iter()
                .zip(ys[i - top1..i - top1 + n].iter().rev())
            {
                s += x * y;
            }
            *o = C::exact_from(s);
        }
    } else {
        for (o, i) in out.iter_mut().zip(nlo..nhi) {
            let start = (i + 1).saturating_sub(len1);
            let stop = min(len1, (i + 1) >> 1);
            let mut s = T::ZERO;
            for (&x, &y) in xs[start..stop]
                .iter()
                .zip(xs[i + 1 - stop..=i - start].iter().rev())
            {
                s += x * y;
            }
            s <<= 1u64;
            if i.even() {
                s += xs[i >> 1] * xs[i >> 1];
            }
            *o = C::exact_from(s);
        }
    }
}

crate_test_fn! {
#[inline]
mul_middle_to_out_tiny_1<C: PolynomialCoefficient>(
    out: &mut [C],
    xs: &[C],
    ys: &[C],
    nlo: usize,
    nhi: usize,
) {
    mul_middle_to_out_tiny::<SignedLimb, C>(out, xs, ys, nlo, nhi);
}}

// `mul_middle_to_out_tiny` with double-word arithmetic: every coefficient of the product, and every
// partial sum of the terms that make it up, must fit in `2 * Limb::WIDTH - 1` bits.
//
// This is equivalent to `_fmpz_poly_mulmid_tiny2` from `fmpz_poly/mulmid.c`, FLINT 3.6.0.
crate_test_fn! {
#[inline]
mul_middle_to_out_tiny_2<C: PolynomialCoefficient>(
    out: &mut [C],
    xs: &[C],
    ys: &[C],
    nlo: usize,
    nhi: usize,
) {
    mul_middle_to_out_tiny::<SignedDoubleLimb, C>(out, xs, ys, nlo, nhi);
}}
