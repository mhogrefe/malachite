// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::num::arithmetic::traits::Parity;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_polynomial::arithmetic::mod_mul::{
    MOD_SQUARE_KARATSUBA_THRESHOLD, ModData, accumulate, column_sum, mod_add_assign_slice,
    mod_karatsuba_scratch_len, mod_sub_assign_slice,
};
use alloc::vec;

// Doubles the three-word accumulator `(a2, a1, a0)`, which must be less than $2^{3\text{W} - 1}$;
// sums from `column_sum` are.
#[inline]
pub(crate) fn double<T: PrimitiveUnsigned>(acc: &mut (T, T, T)) {
    let (a2, a1, a0) = *acc;
    let top = T::WIDTH - 1;
    *acc = ((a2 << 1) | (a1 >> top), (a1 << 1) | (a0 >> top), a0 << 1);
}

// Sets `out[k]`, for each `k` less than `out.len()`, to coefficient `k` of the square of the
// polynomial with coefficients `xs`, reduced modulo $m$, by schoolbook multiplication: each product
// of two different coefficients is accumulated once and doubled.
pub(crate) fn mod_square_classical_prefix<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    d: &ModData<T>,
) {
    let n = xs.len();
    for (k, o) in out.iter_mut().enumerate() {
        let start = k.saturating_sub(n - 1);
        // Pairs (i, k - i) with i < k - i.
        let stop = (k + 1) >> 1;
        let mut acc = if start < stop {
            column_sum(&xs[start..stop], &xs[k + 1 - stop..=k - start], d)
        } else {
            (T::ZERO, T::ZERO, T::ZERO)
        };
        double(&mut acc);
        if k.even() && (k >> 1) < n {
            let x = xs[k >> 1];
            accumulate(&mut acc, x, x);
        }
        *o = d.reduce_sum(acc);
    }
}

// Sets `out` to the square of the polynomial with coefficients `xs`, of nonzero length $n$, modulo
// $m$, by Karatsuba multiplication, falling back to schoolbook multiplication below the threshold.
// `out` must have length $2n - 1$.
fn mod_square_karatsuba_scratch<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    d: &ModData<T>,
    scratch: &mut [T],
) {
    let n = xs.len();
    if n < MOD_SQUARE_KARATSUBA_THRESHOLD {
        mod_square_classical_prefix(out, xs, d);
        return;
    }
    // Write x = x_0 + x^h x_1. Then x^2 = x_0^2 + x^h ((x_0 + x_1)^2 - x_0^2 - x_1^2) + x^{2h}
    // x_1^2.
    let m = d.m;
    let h = n >> 1;
    let c = n - h;
    let two_h = h << 1;
    let (x0, x1) = xs.split_at(h);
    let (sum, scratch) = scratch.split_at_mut(c);
    let (middle, scratch) = scratch.split_at_mut((c << 1) - 1);
    {
        let (low, high) = out.split_at_mut(two_h);
        mod_square_karatsuba_scratch(&mut low[..two_h - 1], x0, d, scratch);
        low[two_h - 1] = T::ZERO;
        mod_square_karatsuba_scratch(high, x1, d, scratch);
    }
    sum.copy_from_slice(x1);
    mod_add_assign_slice(sum, x0, m);
    mod_square_karatsuba_scratch(middle, sum, d, scratch);
    mod_sub_assign_slice(middle, &out[..two_h - 1], m);
    mod_sub_assign_slice(middle, &out[two_h..], m);
    mod_add_assign_slice(&mut out[h..], middle, m);
}

// Sets `out` to the square of the polynomial with coefficients `xs`, which is nonempty, modulo $m$,
// by Karatsuba multiplication, falling back to schoolbook multiplication for short polynomials.
pub(crate) fn mod_square_karatsuba<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T], d: &ModData<T>) {
    if xs.len() < MOD_SQUARE_KARATSUBA_THRESHOLD {
        mod_square_classical_prefix(out, xs, d);
        return;
    }
    let mut scratch =
        vec![T::ZERO; mod_karatsuba_scratch_len(xs.len(), MOD_SQUARE_KARATSUBA_THRESHOLD)];
    mod_square_karatsuba_scratch(out, xs, d, &mut scratch);
}

fn assert_lengths<T>(out: &[T], xs: &[T]) {
    assert!(!xs.is_empty());
    assert_eq!(out.len(), (xs.len() << 1) - 1);
}

// Sets `out` to the square of the polynomial with coefficients `xs`, nonempty and reduced modulo
// `m`, modulo `m`, by schoolbook multiplication. `out` must have length `2 * xs.len() - 1`.
crate_test_fn! {
#[allow(dead_code)]
mod_square_to_out_classical<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T], m: T) {
    assert_lengths(out, xs);
    mod_square_classical_prefix(out, xs, &ModData::new(m, xs.len()));
}}

// Sets `out` to the square of the polynomial with coefficients `xs`, nonempty and reduced modulo
// `m`, modulo `m`, by Karatsuba multiplication. `out` must have length `2 * xs.len() - 1`.
crate_test_fn! {
#[allow(dead_code)]
mod_square_to_out_karatsuba<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T], m: T) {
    assert_lengths(out, xs);
    mod_square_karatsuba(out, xs, &ModData::new(m, xs.len()));
}}

/// Sets `out` to the square of the polynomial with coefficients `xs`, nonempty and reduced modulo
/// `m`, modulo `m`. `out` must have length `2 * xs.len() - 1`, and `m` must be positive.
///
/// This is not part of the public API; it is public so that `malachite-nz` can square
/// `NaturalPolynomial`s modulo a word.
#[doc(hidden)]
pub fn mod_square_to_out<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T], m: T) {
    assert_lengths(out, xs);
    mod_square_karatsuba(out, xs, &ModData::new(m, xs.len()));
}
