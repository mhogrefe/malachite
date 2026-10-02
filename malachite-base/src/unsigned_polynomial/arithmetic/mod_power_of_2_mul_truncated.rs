// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul::{
    MOD_POWER_OF_2_MUL_KARATSUBA_THRESHOLD, add_wrapping_assign, mask_coefficients,
    mul_karatsuba_wrapping,
};
use alloc::vec;
use core::cmp::min;

// Sets `out` to the first `out.len()` coefficients of the product of the polynomials with
// coefficients `xs` and `ys`, modulo $2^\text{W}$, by schoolbook multiplication.
pub(crate) fn mul_truncated_classical_wrapping<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
) {
    let len = out.len();
    out.fill(T::ZERO);
    for (i, &x) in xs.iter().take(len).enumerate() {
        if x != T::ZERO {
            for (o, &y) in out[i..].iter_mut().zip(ys) {
                o.wrapping_add_assign(x.wrapping_mul(y));
            }
        }
    }
}

// Sets `out` to the first `out.len()` coefficients of the product of the polynomials with
// coefficients `xs` and `ys`, both nonempty, modulo $2^\text{W}$. With $n$ equal to `out.len()` and
// $h = \lceil n/2 \rceil$, write x = x_0 + x^h x_1 and y = y_0 + x^h y_1; since $2h \geq n$, xy mod
// x^n is x_0 y_0 + x^h (x_1 y_0 + x_0 y_1) mod x^n. The first product is a full product of
// half-length factors, computed by Karatsuba multiplication, and the other two are truncated
// products of half the length, computed recursively.
pub(crate) fn mul_truncated_karatsuba_wrapping<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
) {
    let len = out.len();
    let xs = &xs[..min(xs.len(), len)];
    let ys = &ys[..min(ys.len(), len)];
    let (xs, ys) = if xs.len() >= ys.len() {
        (xs, ys)
    } else {
        (ys, xs)
    };
    let full_len = xs.len() + ys.len() - 1;
    if full_len <= len {
        mul_karatsuba_wrapping(&mut out[..full_len], xs, ys);
        out[full_len..].fill(T::ZERO);
        return;
    }
    if ys.len() < MOD_POWER_OF_2_MUL_KARATSUBA_THRESHOLD {
        mul_truncated_classical_wrapping(out, xs, ys);
        return;
    }
    let h = len.div_ceil(2);
    let x0 = &xs[..min(h, xs.len())];
    let y0 = &ys[..min(h, ys.len())];
    // The product of x_0 and y_0 has at most 2h - 1 <= `len` coefficients, so this is a full
    // product.
    mul_truncated_karatsuba_wrapping(out, x0, y0);
    let mut cross = vec![T::ZERO; len - h];
    if xs.len() > h {
        mul_truncated_karatsuba_wrapping(&mut cross, &xs[h..], y0);
        add_wrapping_assign(&mut out[h..], &cross);
    }
    if ys.len() > h {
        mul_truncated_karatsuba_wrapping(&mut cross, x0, &ys[h..]);
        add_wrapping_assign(&mut out[h..], &cross);
    }
}

fn assert_lengths<T>(out: &[T], xs: &[T], ys: &[T]) {
    assert!(!out.is_empty());
    assert!(!xs.is_empty());
    assert!(!ys.is_empty());
}

// Sets `out` to the first `out.len()` coefficients of the product of the polynomials with
// coefficients `xs` and `ys`, all three nonempty and the inputs reduced modulo $2^k$, where $k$ is
// `pow`, by schoolbook multiplication. `pow` must be no greater than `T::WIDTH`.
crate_test_fn! {
#[allow(dead_code)]
mod_power_of_2_mul_truncated_to_out_classical<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    pow: u64,
) {
    assert_lengths(out, xs, ys);
    assert!(pow <= T::WIDTH);
    mul_truncated_classical_wrapping(out, xs, ys);
    mask_coefficients(out, pow);
}}

// Sets `out` to the first `out.len()` coefficients of the product of the polynomials with
// coefficients `xs` and `ys`, all three nonempty and the inputs reduced modulo $2^k$, where $k$ is
// `pow`, by Karatsuba multiplication. `pow` must be no greater than `T::WIDTH`.
crate_test_fn! {
#[allow(dead_code)]
mod_power_of_2_mul_truncated_to_out_karatsuba<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    pow: u64,
) {
    assert_lengths(out, xs, ys);
    assert!(pow <= T::WIDTH);
    mul_truncated_karatsuba_wrapping(out, xs, ys);
    mask_coefficients(out, pow);
}}

/// Sets `out` to the first `out.len()` coefficients of the product of the polynomials with
/// coefficients `xs` and `ys`, all three nonempty and the inputs reduced modulo $2^k$, where $k$ is
/// `pow`. `pow` must be no greater than `T::WIDTH`.
///
/// This is not part of the public API; it is public so that `malachite-nz` can multiply
/// `NaturalPolynomial`s with word-sized coefficients modulo $2^k$.
#[doc(hidden)]
pub fn mod_power_of_2_mul_truncated_to_out<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    pow: u64,
) {
    assert_lengths(out, xs, ys);
    assert!(pow <= T::WIDTH);
    mul_truncated_karatsuba_wrapping(out, xs, ys);
    mask_coefficients(out, pow);
}
