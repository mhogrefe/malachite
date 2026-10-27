// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2008, 2009 William Hart
//
//      Copyright © 2010, 2011 Sebastian Pancratz
//
//      Copyright © 2014 Fredrik Johansson
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
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::conversion::traits::ExactFrom;

// Sets `out` to the first `out.len()` coefficients of the square of the polynomial with
// coefficients `xs`, which is nonempty, using arithmetic in `T`. Every element of `xs` must be a
// small `fmpz`, and every coefficient of the square, and every partial sum of the terms that make
// it up, must fit in `T`. `out.len()` must be positive and at most `2 * xs.len() - 1`.
//
// # Worst-case complexity
// $T(n) = O(n^2)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `out.len()`.
//
// This is equivalent to `_fmpz_poly_sqrlow_tiny1` from `fmpz_poly/sqrlow.c`, FLINT 3.6.0, when `T`
// is `SignedLimb`, and to `_fmpz_poly_sqrlow_tiny2` when `T` is `SignedDoubleLimb`, where `n` is
// `out.len()`.
pub(crate) fn square_truncated_to_out_tiny<
    T: PrimitiveSigned + From<SignedLimb>,
    C: PolynomialCoefficient + ExactFrom<T>,
>(
    out: &mut [C],
    xs: &[C],
) {
    let n = out.len();
    let xs = &xs[..xs.len().min(n)];
    // Short squares, the common case, sum on the stack, as FLINT's do.
    if n <= TINY_STACK_LEN {
        let mut xs_values = [T::ZERO; TINY_STACK_LEN];
        let mut res = [T::ZERO; TINY_STACK_LEN];
        square_truncated_to_out_tiny_with(out, xs, &mut xs_values[..xs.len()], &mut res[..n]);
    } else {
        let mut buffer = vec![T::ZERO; xs.len() + n];
        let (xs_values, res) = buffer.split_at_mut(xs.len());
        square_truncated_to_out_tiny_with(out, xs, xs_values, res);
    }
}

// `square_truncated_to_out_tiny`, with `xs_values` to hold the values of `xs`, which it must have
// the length of, and `res` to sum in, which must be zero and have the length of `out`.
fn square_truncated_to_out_tiny_with<
    T: PrimitiveSigned + From<SignedLimb>,
    C: PolynomialCoefficient + ExactFrom<T>,
>(
    out: &mut [C],
    xs: &[C],
    xs_values: &mut [T],
    res: &mut [T],
) {
    let n = out.len();
    for (v, x) in xs_values.iter_mut().zip(xs) {
        *v = T::from(small_value(x).unwrap());
    }
    let xs = &*xs_values;
    for (i, &x) in xs.iter().enumerate() {
        if x != T::ZERO {
            if i << 1 < n {
                res[i << 1] += x * x;
            }
            // This does not overflow, since `x` is small.
            let c = x << 1u64;
            for (r, &y) in res.iter_mut().skip((i << 1) + 1).zip(&xs[i + 1..]) {
                if y != T::ZERO {
                    *r += c * y;
                }
            }
        }
    }
    for (o, &r) in out.iter_mut().zip(&*res) {
        *o = C::exact_from(r);
    }
}

// `square_truncated_to_out_tiny` with single-word arithmetic: every coefficient of the square, and
// every partial sum of the terms that make it up, must fit in `SMALL_FMPZ_BITCOUNT_MAX` bits.
//
// This is equivalent to `_fmpz_poly_sqrlow_tiny1` from `fmpz_poly/sqrlow.c`, FLINT 3.6.0.
crate_test_fn! {
#[inline]
square_truncated_to_out_tiny_1<C: PolynomialCoefficient>(out: &mut [C], xs: &[C]) {
    square_truncated_to_out_tiny::<SignedLimb, C>(out, xs);
}}

// `square_truncated_to_out_tiny` with double-word arithmetic: every coefficient of the square, and
// every partial sum of the terms that make it up, must fit in `2 * Limb::WIDTH - 1` bits.
//
// This is equivalent to `_fmpz_poly_sqrlow_tiny2` from `fmpz_poly/sqrlow.c`, FLINT 3.6.0.
crate_test_fn! {
#[inline]
square_truncated_to_out_tiny_2<C: PolynomialCoefficient>(out: &mut [C], xs: &[C]) {
    square_truncated_to_out_tiny::<SignedDoubleLimb, C>(out, xs);
}}
