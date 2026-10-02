// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul::{
    MOD_POWER_OF_2_SQUARE_KARATSUBA_THRESHOLD, add_wrapping_assign, karatsuba_wrapping_scratch_len,
    mask_coefficients, sub_wrapping_assign,
};
use alloc::vec;

// Sets `out` to the square of the polynomial with coefficients `xs`, modulo $2^\text{W}$, by
// schoolbook multiplication: each product of two different coefficients is computed once and
// doubled. `out` must have length `2 * xs.len() - 1`.
pub(crate) fn square_classical_wrapping<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T]) {
    out.fill(T::ZERO);
    for (i, &x) in xs.iter().enumerate() {
        if x != T::ZERO {
            for (o, &y) in out[(i << 1) + 1..].iter_mut().zip(&xs[i + 1..]) {
                o.wrapping_add_assign(x.wrapping_mul(y));
            }
        }
    }
    for o in out.iter_mut() {
        *o = o.wrapping_add(*o);
    }
    for (i, &x) in xs.iter().enumerate() {
        out[i << 1].wrapping_add_assign(x.wrapping_mul(x));
    }
}

// Sets `out` to the square of the polynomial with coefficients `xs`, of nonzero length $n$, modulo
// $2^\text{W}$, by Karatsuba multiplication, falling back to schoolbook multiplication below the
// threshold. `out` must have length $2n - 1$, and `scratch` at least
// `karatsuba_wrapping_scratch_len(n)`.
fn square_karatsuba_scratch_wrapping<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    scratch: &mut [T],
) {
    let n = xs.len();
    if n < MOD_POWER_OF_2_SQUARE_KARATSUBA_THRESHOLD {
        square_classical_wrapping(out, xs);
        return;
    }
    // Write x = x_0 + x^h x_1. Then x^2 = x_0^2 + x^h ((x_0 + x_1)^2 - x_0^2 - x_1^2) + x^{2h}
    // x_1^2.
    let h = n >> 1;
    let c = n - h;
    let two_h = h << 1;
    let (x0, x1) = xs.split_at(h);
    let (sum, scratch) = scratch.split_at_mut(c);
    let (middle, scratch) = scratch.split_at_mut((c << 1) - 1);
    {
        let (low, high) = out.split_at_mut(two_h);
        square_karatsuba_scratch_wrapping(&mut low[..two_h - 1], x0, scratch);
        low[two_h - 1] = T::ZERO;
        square_karatsuba_scratch_wrapping(high, x1, scratch);
    }
    sum.copy_from_slice(x1);
    add_wrapping_assign(sum, x0);
    square_karatsuba_scratch_wrapping(middle, sum, scratch);
    sub_wrapping_assign(middle, &out[..two_h - 1]);
    sub_wrapping_assign(middle, &out[two_h..]);
    add_wrapping_assign(&mut out[h..], middle);
}

// Sets `out` to the square of the polynomial with coefficients `xs`, which is nonempty, modulo
// $2^\text{W}$, by Karatsuba multiplication, falling back to schoolbook multiplication for short
// polynomials. `out` must have length `2 * xs.len() - 1`.
pub(crate) fn square_karatsuba_wrapping<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T]) {
    if xs.len() < MOD_POWER_OF_2_SQUARE_KARATSUBA_THRESHOLD {
        square_classical_wrapping(out, xs);
        return;
    }
    let mut scratch = vec![T::ZERO; karatsuba_wrapping_scratch_len(xs.len())];
    square_karatsuba_scratch_wrapping(out, xs, &mut scratch);
}

fn assert_lengths<T>(out: &[T], xs: &[T]) {
    assert!(!xs.is_empty());
    assert_eq!(out.len(), (xs.len() << 1) - 1);
}

// Sets `out` to the square of the polynomial with coefficients `xs`, nonempty and reduced modulo
// $2^k$, where $k$ is `pow`, by schoolbook multiplication. `out` must have length `2 * xs.len() -
// 1`, and `pow` must be no greater than `T::WIDTH`.
crate_test_fn! {
#[allow(dead_code)]
mod_power_of_2_square_to_out_classical<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    pow: u64,
) {
    assert_lengths(out, xs);
    assert!(pow <= T::WIDTH);
    square_classical_wrapping(out, xs);
    mask_coefficients(out, pow);
}}

// Sets `out` to the square of the polynomial with coefficients `xs`, nonempty and reduced modulo
// $2^k$, where $k$ is `pow`, by Karatsuba multiplication. `out` must have length `2 * xs.len() -
// 1`, and `pow` must be no greater than `T::WIDTH`.
crate_test_fn! {
#[allow(dead_code)]
mod_power_of_2_square_to_out_karatsuba<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    pow: u64,
) {
    assert_lengths(out, xs);
    assert!(pow <= T::WIDTH);
    square_karatsuba_wrapping(out, xs);
    mask_coefficients(out, pow);
}}

/// Sets `out` to the square of the polynomial with coefficients `xs`, nonempty and reduced modulo
/// $2^k$, where $k$ is `pow`. `out` must have length `2 * xs.len() - 1`, and `pow` must be no
/// greater than `T::WIDTH`.
///
/// This is not part of the public API; it is public so that `malachite-nz` can square
/// `NaturalPolynomial`s with word-sized coefficients modulo $2^k$.
#[doc(hidden)]
pub fn mod_power_of_2_square_to_out<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T], pow: u64) {
    assert_lengths(out, xs);
    assert!(pow <= T::WIDTH);
    square_karatsuba_wrapping(out, xs);
    mask_coefficients(out, pow);
}
