// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_polynomial::arithmetic::mod_mul::{
    MOD_MUL_KARATSUBA_THRESHOLD, ModData, column_sum, mod_add_assign_slice, mod_mul_karatsuba,
};
use alloc::vec;
use core::cmp::min;

// Sets `out` to the first `out.len()` coefficients of the product of the polynomials with
// coefficients `xs` and `ys`, both nonempty, reduced modulo $m$, by schoolbook multiplication, one
// coefficient at a time.
pub(crate) fn mod_mul_truncated_classical<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    d: &ModData<T>,
) {
    let n = xs.len();
    let m = ys.len();
    for (k, o) in out.iter_mut().enumerate() {
        if k > n + m - 2 {
            *o = T::ZERO;
            continue;
        }
        let start = k.saturating_sub(m - 1);
        let stop = min(k, n - 1);
        let acc = column_sum(&xs[start..=stop], &ys[k - stop..=k - start], d);
        *o = d.reduce_sum(acc);
    }
}

// Sets `out` to the first `out.len()` coefficients of the product of the polynomials with
// coefficients `xs` and `ys`, both nonempty, modulo $m$. With $n$ equal to `out.len()` and $h =
// \lceil n/2 \rceil$, write x = x_0 + x^h x_1 and y = y_0 + x^h y_1; since $2h \geq n$, xy mod x^n
// is x_0 y_0 + x^h (x_1 y_0 + x_0 y_1) mod x^n. The first product is a full product of half-length
// factors, computed by Karatsuba multiplication, and the other two are truncated products of half
// the length, computed recursively.
pub(crate) fn mod_mul_truncated_karatsuba<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    d: &ModData<T>,
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
        mod_mul_karatsuba(&mut out[..full_len], xs, ys, d);
        out[full_len..].fill(T::ZERO);
        return;
    }
    if ys.len() < MOD_MUL_KARATSUBA_THRESHOLD {
        mod_mul_truncated_classical(out, xs, ys, d);
        return;
    }
    let h = len.div_ceil(2);
    let x0 = &xs[..min(h, xs.len())];
    let y0 = &ys[..min(h, ys.len())];
    // The product of x_0 and y_0 has at most 2h - 1 <= `len` coefficients, so this is a full
    // product.
    mod_mul_truncated_karatsuba(out, x0, y0, d);
    let mut cross = vec![T::ZERO; len - h];
    if xs.len() > h {
        mod_mul_truncated_karatsuba(&mut cross, &xs[h..], y0, d);
        mod_add_assign_slice(&mut out[h..], &cross, d.m);
    }
    if ys.len() > h {
        mod_mul_truncated_karatsuba(&mut cross, x0, &ys[h..], d);
        mod_add_assign_slice(&mut out[h..], &cross, d.m);
    }
}

fn assert_lengths<T>(out: &[T], xs: &[T], ys: &[T]) {
    assert!(!out.is_empty());
    assert!(!xs.is_empty());
    assert!(!ys.is_empty());
}

// Sets `out` to the first `out.len()` coefficients of the product of the polynomials with
// coefficients `xs` and `ys`, all three nonempty and the inputs reduced modulo `m`, modulo `m`, by
// schoolbook multiplication.
crate_test_fn! {
#[allow(dead_code)]
mod_mul_truncated_to_out_classical<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    m: T,
) {
    assert_lengths(out, xs, ys);
    let terms = xs.len().min(ys.len()).min(out.len());
    mod_mul_truncated_classical(out, xs, ys, &ModData::new(m, terms));
}}

// Sets `out` to the first `out.len()` coefficients of the product of the polynomials with
// coefficients `xs` and `ys`, all three nonempty and the inputs reduced modulo `m`, modulo `m`, by
// Karatsuba multiplication.
crate_test_fn! {
#[allow(dead_code)]
mod_mul_truncated_to_out_karatsuba<T: PrimitiveUnsigned>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    m: T,
) {
    assert_lengths(out, xs, ys);
    let terms = xs.len().min(ys.len()).min(out.len());
    mod_mul_truncated_karatsuba(out, xs, ys, &ModData::new(m, terms));
}}

/// Sets `out` to the first `out.len()` coefficients of the product of the polynomials with
/// coefficients `xs` and `ys`, all three nonempty and the inputs reduced modulo `m`, modulo `m`.
/// `m` must be positive.
///
/// This is not part of the public API; it is public so that `malachite-nz` can multiply
/// `NaturalPolynomial`s modulo a word.
#[doc(hidden)]
pub fn mod_mul_truncated_to_out<T: PrimitiveUnsigned>(out: &mut [T], xs: &[T], ys: &[T], m: T) {
    assert_lengths(out, xs, ys);
    mod_mul_truncated_karatsuba(
        out,
        xs,
        ys,
        &ModData::new(m, xs.len().min(ys.len()).min(out.len())),
    );
}
