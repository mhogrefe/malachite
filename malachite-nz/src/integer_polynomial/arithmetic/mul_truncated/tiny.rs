// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2008, 2009 William Hart
//
//      Copyright © 2010 Sebastian Pancratz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::arithmetic::vec::small_values;
use crate::platform::{SignedDoubleLimb, SignedLimb};
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::basic::signeds::PrimitiveSigned;

// Sets `out` to the first `out.len()` coefficients of the product of the polynomials with
// coefficients `xs` and `ys`, both nonempty, using arithmetic in `T`. Every element of `xs` and
// `ys` must be a small `fmpz`, and every coefficient of the product, and every partial sum of the
// terms that make it up, must fit in `T`. `out.len()` must be positive and at most `xs.len() +
// ys.len() - 1`.
//
// # Worst-case complexity
// $T(n) = O(n^2)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `out.len()`.
//
// This is equivalent to `_fmpz_poly_mullow_tiny1` from `fmpz_poly/mullow.c`, FLINT 3.6.0, when `T`
// is `SignedLimb`, and to `_fmpz_poly_mullow_tiny2` when `T` is `SignedDoubleLimb`, where `n` is
// `out.len()`.
pub(crate) fn mul_truncated_to_out_tiny<T: PrimitiveSigned + From<SignedLimb>>(
    out: &mut [Integer],
    xs: &[Integer],
    ys: &[Integer],
) where
    Integer: From<T>,
{
    let n = out.len();
    let ys: Vec<T> = small_values(&ys[..ys.len().min(n)]);
    let mut res = vec![T::ZERO; n];
    for (i, x) in small_values::<T>(&xs[..xs.len().min(n)])
        .into_iter()
        .enumerate()
    {
        if x != T::ZERO {
            for (r, &y) in res[i..].iter_mut().zip(&ys) {
                if y != T::ZERO {
                    *r += x * y;
                }
            }
        }
    }
    for (o, r) in out.iter_mut().zip(res) {
        *o = Integer::from(r);
    }
}

// `mul_truncated_to_out_tiny` with single-word arithmetic: every coefficient of the product, and
// every partial sum of the terms that make it up, must fit in `SMALL_FMPZ_BITCOUNT_MAX` bits.
//
// This is equivalent to `_fmpz_poly_mullow_tiny1` from `fmpz_poly/mullow.c`, FLINT 3.6.0.
crate_test_fn! {
#[inline]
mul_truncated_to_out_tiny_1(out: &mut [Integer], xs: &[Integer], ys: &[Integer]) {
    mul_truncated_to_out_tiny::<SignedLimb>(out, xs, ys);
}}

// `mul_truncated_to_out_tiny` with double-word arithmetic: every coefficient of the product, and
// every partial sum of the terms that make it up, must fit in `2 * Limb::WIDTH - 1` bits.
//
// This is equivalent to `_fmpz_poly_mullow_tiny2` from `fmpz_poly/mullow.c`, FLINT 3.6.0.
crate_test_fn! {
#[inline]
mul_truncated_to_out_tiny_2(out: &mut [Integer], xs: &[Integer], ys: &[Integer]) {
    mul_truncated_to_out_tiny::<SignedDoubleLimb>(out, xs, ys);
}}
