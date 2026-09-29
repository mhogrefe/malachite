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

use crate::integer::Integer;
use crate::integer_polynomial::arithmetic::vec::small_values;
use crate::platform::{SignedDoubleLimb, SignedLimb};
use alloc::vec::Vec;
use core::cmp::min;
use core::ptr;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::signeds::PrimitiveSigned;

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
pub(crate) fn mul_middle_to_out_tiny<T: PrimitiveSigned + From<SignedLimb>>(
    out: &mut [Integer],
    xs: &[Integer],
    ys: &[Integer],
    nlo: usize,
    nhi: usize,
) where
    Integer: From<T>,
{
    let len1 = xs.len();
    let len2 = ys.len();
    if ptr::eq(xs, ys) {
        let xs: Vec<T> = small_values(xs);
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
            *o = Integer::from(s);
        }
    } else {
        let xs: Vec<T> = small_values(xs);
        let ys: Vec<T> = small_values(ys);
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
            *o = Integer::from(s);
        }
    }
}

// `mul_middle_to_out_tiny` with single-word arithmetic: every coefficient of the product, and every
// partial sum of the terms that make it up, must fit in `SMALL_FMPZ_BITCOUNT_MAX` bits.
//
// This is equivalent to `_fmpz_poly_mulmid_tiny1` from `fmpz_poly/mulmid.c`, FLINT 3.6.0.
crate_test_fn! {
#[inline]
mul_middle_to_out_tiny_1(
    out: &mut [Integer],
    xs: &[Integer],
    ys: &[Integer],
    nlo: usize,
    nhi: usize,
) {
    mul_middle_to_out_tiny::<SignedLimb>(out, xs, ys, nlo, nhi);
}}

// `mul_middle_to_out_tiny` with double-word arithmetic: every coefficient of the product, and every
// partial sum of the terms that make it up, must fit in `2 * Limb::WIDTH - 1` bits.
//
// This is equivalent to `_fmpz_poly_mulmid_tiny2` from `fmpz_poly/mulmid.c`, FLINT 3.6.0.
crate_test_fn! {
#[inline]
mul_middle_to_out_tiny_2(
    out: &mut [Integer],
    xs: &[Integer],
    ys: &[Integer],
    nlo: usize,
    nhi: usize,
) {
    mul_middle_to_out_tiny::<SignedDoubleLimb>(out, xs, ys, nlo, nhi);
}}
