// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_polynomial::arithmetic::mod_mul::{
    MOD_SQUARE_KARATSUBA_THRESHOLD, ModData, mod_add_assign_slice,
};
use crate::unsigned_polynomial::arithmetic::mod_mul_truncated::mod_mul_truncated_karatsuba;
use crate::unsigned_polynomial::arithmetic::mod_square::{
    mod_square_classical_prefix, mod_square_karatsuba,
};
use alloc::vec;
use core::cmp::min;

// Sets `out` to the first `out.len()` coefficients of the square of the polynomial with
// coefficients `xs`, which is nonempty, modulo $m$. With $n$ equal to `out.len()` and $h = \lceil
// n/2 \rceil$, write x = x_0 + x^h x_1; since $2h \geq n$, x^2 mod x^n is x_0^2 + 2 x^h x_0 x_1 mod
// x^n. The square is a full square of a half-length polynomial, computed by Karatsuba
// multiplication, and the cross product is a truncated product of half the length.
pub(crate) fn mod_square_truncated_karatsuba<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    d: &ModData<T>,
) {
    let len = out.len();
    let xs = &xs[..min(xs.len(), len)];
    let n = xs.len();
    let full_len = (n << 1) - 1;
    if full_len <= len {
        mod_square_karatsuba(&mut out[..full_len], xs, d);
        out[full_len..].fill(T::ZERO);
        return;
    }
    if n < MOD_SQUARE_KARATSUBA_THRESHOLD {
        mod_square_classical_prefix(out, xs, d);
        return;
    }
    let h = len.div_ceil(2);
    let x0 = &xs[..min(h, n)];
    // The square of x_0 has at most 2h - 1 <= `len` coefficients, so this is a full square.
    mod_square_truncated_karatsuba(out, x0, d);
    if n > h {
        let mut cross = vec![T::ZERO; len - h];
        mod_mul_truncated_karatsuba(&mut cross, x0, &xs[h..], d);
        mod_add_assign_slice(&mut out[h..], &cross, d.m);
        mod_add_assign_slice(&mut out[h..], &cross, d.m);
    }
}

fn assert_lengths<T>(out: &[T], xs: &[T]) {
    assert!(!out.is_empty());
    assert!(!xs.is_empty());
}

// Sets `out` to the first `out.len()` coefficients of the square of the polynomial with
// coefficients `xs`, both nonempty and the input reduced modulo `m`, modulo `m`, by schoolbook
// multiplication.
crate_test_fn! {
#[allow(dead_code)]
mod_square_truncated_to_out_classical<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T], m: T) {
    assert_lengths(out, xs);
    mod_square_classical_prefix(out, xs, &ModData::new(m, xs.len().min(out.len())));
}}

// Sets `out` to the first `out.len()` coefficients of the square of the polynomial with
// coefficients `xs`, both nonempty and the input reduced modulo `m`, modulo `m`, by Karatsuba
// multiplication.
crate_test_fn! {
#[allow(dead_code)]
mod_square_truncated_to_out_karatsuba<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T], m: T) {
    assert_lengths(out, xs);
    mod_square_truncated_karatsuba(out, xs, &ModData::new(m, xs.len().min(out.len())));
}}

/// Sets `out` to the first `out.len()` coefficients of the square of the polynomial with
/// coefficients `xs`, both nonempty and the input reduced modulo `m`, modulo `m`. `m` must be
/// positive.
///
/// This is not part of the public API; it is public so that `malachite-nz` can square
/// `NaturalPolynomial`s modulo a word.
#[doc(hidden)]
pub fn mod_square_truncated_to_out<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T], m: T) {
    assert_lengths(out, xs);
    mod_square_truncated_karatsuba(out, xs, &ModData::new(m, xs.len().min(out.len())));
}
