// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::ZERO;
use crate::integer_polynomial::arithmetic::vec::{vec_add, vec_sub_assign};
use alloc::vec;
use core::borrow::Borrow;
use core::mem::take;
use malachite_base::num::arithmetic::traits::{CeilingLogBase2, PowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::LowMask;

// This is the "odd/even" Karatsuba algorithm. Let $f(x) = f_1(x^2) + xf_2(x^2)$ and $g(x) =
// g_1(x^2) + xg_2(x^2)$. Then
//
// $$
// f(x)g(x) = f_1(x^2)g_1(x^2) + x^2f_2(x^2)g_2(x^2)
//     + x((f_1(x^2) + f_2(x^2))(g_1(x^2) + g_2(x^2)) - f_1(x^2)g_1(x^2) - f_2(x^2)g_2(x^2)),
// $$
//
// so only three multiplications are performed. The coefficients are kept in bit-reversed order:
// with the length padded to $2^k$, the coefficient of $x^i$ is stored at the index whose $k$-bit
// binary representation is that of $i$ reversed. Then the first half of the coefficients of $f$ are
// those of $f_1$, and the second half those of $f_2$, all the way down the recursion. The only
// tricky part is multiplying by $x$, which has to undo the bit reversal to shift by one term.
//
// There is no crossover to a basecase, so this is only efficient when the coefficients are large.

// The low `bits` bits of `n`, reversed. `bits` must be positive.
//
// This is equivalent to `n_revbin` from `ulong_extras/revbin.c`, FLINT 3.6.0, where `b` is
// positive.
const fn revbin(n: usize, bits: u64) -> usize {
    debug_assert!(bits != 0);
    n.reverse_bits() >> (usize::WIDTH - bits)
}

// Places references to the elements of `xs` in `out`, which has length $2^b$ and is otherwise
// filled with references to zero, in bit-reversed order.
//
// This is equivalent to `revbin1` from `fmpz_poly/mul_karatsuba.c`, FLINT 3.6.0, which copies the
// `fmpz`s shallowly.
pub(crate) fn revbin_in<'a>(out: &mut [&'a Integer], xs: &'a [Integer], bits: u64) {
    for (i, x) in xs.iter().enumerate() {
        out[revbin(i, bits)] = x;
    }
}

// Moves the first `out.len()` coefficients of `xs`, which has length $2^b$ and is in bit-reversed
// order, into `out`.
//
// This is equivalent to `revbin2` from `fmpz_poly/mul_karatsuba.c`, FLINT 3.6.0.
pub(crate) fn revbin_out(out: &mut [Integer], xs: &mut [Integer], bits: u64) {
    for (i, o) in out.iter_mut().enumerate() {
        *o = take(&mut xs[revbin(i, bits)]);
    }
}

// Adds $x$ times `ys` to `xs`, where both are in bit-reversed order with $2^b$ coefficients.
//
// This is equivalent to `_fmpz_vec_add_rev` from `fmpz_poly/mul_karatsuba.c`, FLINT 3.6.0.
pub(crate) fn add_shifted_rev(xs: &mut [Integer], ys: &[Integer], bits: u64) {
    for (i, y) in ys[..usize::low_mask(bits)].iter().enumerate() {
        xs[revbin(revbin(i, bits) + 1, bits)] += y;
    }
}

// Karatsuba multiplication of polynomials in bit-reversed order: `xs` and `ys` have length $2^b$,
// `out` has length $2^{b+1}$, and `temp` has length at least $2^{b+1}$. The inputs are the
// coefficients themselves or references to them; the sums of halves that the recursion creates are
// owned.
//
// This is equivalent to `_fmpz_poly_mul_kara_recursive` from `fmpz_poly/mul_karatsuba.c`, FLINT
// 3.6.0.
fn mul_karatsuba_recursive<T: Borrow<Integer>>(
    out: &mut [Integer],
    xs: &[T],
    ys: &[T],
    temp: &mut [Integer],
    bits: u64,
) {
    let length = usize::power_of_2(bits);
    let m = length >> 1;
    if length == 1 {
        out[0] = xs[0].borrow() * ys[0].borrow();
        out[1] = Integer::ZERO;
        return;
    }
    let (sums, temp) = temp.split_at_mut(length);
    vec_add(&mut sums[..m], &xs[..m], &xs[m..length]);
    vec_add(&mut sums[m..], &ys[..m], &ys[m..length]);
    let (out_lo, out_hi) = out.split_at_mut(length);
    mul_karatsuba_recursive(out_lo, &xs[..m], &ys[..m], temp, bits - 1);
    mul_karatsuba_recursive(out_hi, &sums[..m], &sums[m..], temp, bits - 1);
    mul_karatsuba_recursive(sums, &xs[m..], &ys[m..], temp, bits - 1);
    vec_sub_assign(&mut out_hi[..length], out_lo);
    vec_sub_assign(&mut out_hi[..length], sums);
    add_shifted_rev(out_lo, sums, bits);
}

// Sets `out` to the coefficients of the product of the polynomials with coefficients `xs` and `ys`,
// where `xs.len() >= ys.len() >= 1`. `out` must have length `xs.len() + ys.len() - 1`.
//
// # Worst-case complexity
// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
//
// $M(n, m) = O(n(m + \log n))$
//
// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, and $m$ is the largest number of
// significant bits of any element of `xs` or `ys`.
//
// This is equivalent to `_fmpz_poly_mul_karatsuba` from `fmpz_poly/mul_karatsuba.c`, FLINT 3.6.0.
crate_test_fn! {mul_to_out_karatsuba(out: &mut [Integer], xs: &[Integer], ys: &[Integer]) {
    let len1 = xs.len();
    assert!(len1 >= ys.len());
    assert_ne!(ys.len(), 0);
    if len1 == 1 {
        out[0] = &xs[0] * &ys[0];
        return;
    }
    let loglen = u64::exact_from(len1).ceiling_log_base_2();
    let length = usize::power_of_2(loglen);
    let mut revs = vec![&ZERO; length << 1];
    let (rev1, rev2) = revs.split_at_mut(length);
    let mut scratch = vec![Integer::ZERO; length << 2];
    let (rev_out, temp) = scratch.split_at_mut(length << 1);
    revbin_in(rev1, xs, loglen);
    revbin_in(rev2, ys, loglen);
    mul_karatsuba_recursive(rev_out, &*rev1, &*rev2, temp, loglen);
    revbin_out(out, rev_out, loglen + 1);
}}
