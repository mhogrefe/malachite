// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul::{
    MOD_POWER_OF_2_SQUARE_KARATSUBA_THRESHOLD, add_wrapping_assign, mask_coefficients,
};
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul_truncated::*;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_square::square_karatsuba_wrapping;
use alloc::vec;
use core::cmp::min;

// Sets `out` to the first `out.len()` coefficients of the square of the polynomial with
// coefficients `xs`, modulo $2^\text{W}$, by schoolbook multiplication: each product of two
// different coefficients is computed once and doubled.
pub(crate) fn square_truncated_classical_wrapping<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T]) {
    let len = out.len();
    out.fill(T::ZERO);
    for (i, &x) in xs.iter().enumerate() {
        if (i << 1) + 1 >= len {
            break;
        }
        if x != T::ZERO {
            for (o, &y) in out[(i << 1) + 1..].iter_mut().zip(&xs[i + 1..]) {
                o.wrapping_add_assign(x.wrapping_mul(y));
            }
        }
    }
    for o in out.iter_mut() {
        *o = o.wrapping_add(*o);
    }
    for (o, &x) in out.iter_mut().step_by(2).zip(xs) {
        o.wrapping_add_assign(x.wrapping_mul(x));
    }
}

// Sets `out` to the first `out.len()` coefficients of the square of the polynomial with
// coefficients `xs`, which is nonempty, modulo $2^\text{W}$. With $n$ equal to `out.len()` and $h =
// \lceil n/2 \rceil$, write x = x_0 + x^h x_1; since $2h \geq n$, x^2 mod x^n is x_0^2 + 2 x^h x_0
// x_1 mod x^n. The square is a full square of a half-length polynomial, computed by Karatsuba
// multiplication, and the cross product is a truncated product of half the length.
pub(crate) fn square_truncated_karatsuba_wrapping<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T]) {
    let len = out.len();
    let xs = &xs[..min(xs.len(), len)];
    let n = xs.len();
    let full_len = (n << 1) - 1;
    if full_len <= len {
        square_karatsuba_wrapping(&mut out[..full_len], xs);
        out[full_len..].fill(T::ZERO);
        return;
    }
    if n < MOD_POWER_OF_2_SQUARE_KARATSUBA_THRESHOLD {
        square_truncated_classical_wrapping(out, xs);
        return;
    }
    let h = len.div_ceil(2);
    let x0 = &xs[..min(h, n)];
    // The square of x_0 has at most 2h - 1 <= `len` coefficients, so this is a full square.
    square_truncated_karatsuba_wrapping(out, x0);
    if n > h {
        let mut cross = vec![T::ZERO; len - h];
        mul_truncated_karatsuba_wrapping(&mut cross, x0, &xs[h..]);
        add_wrapping_assign(&mut out[h..], &cross);
        add_wrapping_assign(&mut out[h..], &cross);
    }
}

fn assert_lengths<T>(out: &[T], xs: &[T]) {
    assert!(!out.is_empty());
    assert!(!xs.is_empty());
}

// Sets `out` to the first `out.len()` coefficients of the square of the polynomial with
// coefficients `xs`, both nonempty and the input reduced modulo $2^k$, where $k$ is `pow`, by
// schoolbook multiplication. `pow` must be no greater than `T::WIDTH`.
crate_test_fn! {
#[allow(dead_code)]
mod_power_of_2_square_truncated_to_out_classical<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    pow: u64,
) {
    assert_lengths(out, xs);
    assert!(pow <= T::WIDTH);
    square_truncated_classical_wrapping(out, xs);
    mask_coefficients(out, pow);
}}

// Sets `out` to the first `out.len()` coefficients of the square of the polynomial with
// coefficients `xs`, both nonempty and the input reduced modulo $2^k$, where $k$ is `pow`, by
// Karatsuba multiplication. `pow` must be no greater than `T::WIDTH`.
crate_test_fn! {
#[allow(dead_code)]
mod_power_of_2_square_truncated_to_out_karatsuba<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    pow: u64,
) {
    assert_lengths(out, xs);
    assert!(pow <= T::WIDTH);
    square_truncated_karatsuba_wrapping(out, xs);
    mask_coefficients(out, pow);
}}

/// Sets `out` to the first `out.len()` coefficients of the square of the polynomial with
/// coefficients `xs`, both nonempty and the input reduced modulo $2^k$, where $k$ is `pow`. `pow`
/// must be no greater than `T::WIDTH`.
///
/// This is not part of the public API; it is public so that `malachite-nz` can square
/// `NaturalPolynomial`s with word-sized coefficients modulo $2^k$.
#[doc(hidden)]
pub fn mod_power_of_2_square_truncated_to_out<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    pow: u64,
) {
    assert_lengths(out, xs);
    assert!(pow <= T::WIDTH);
    square_truncated_karatsuba_wrapping(out, xs);
    mask_coefficients(out, pow);
}
