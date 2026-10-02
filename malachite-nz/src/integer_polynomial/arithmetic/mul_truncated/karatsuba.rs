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

use crate::integer_polynomial::arithmetic::coefficient::PolynomialCoefficient;
use crate::integer_polynomial::arithmetic::mul::karatsuba::mul_to_out_karatsuba;
use crate::integer_polynomial::arithmetic::mul_truncated::classical::mul_truncated_to_out_classical;
use crate::integer_polynomial::arithmetic::vec::{vec_add, vec_add_assign, vec_sub_assign};
use alloc::borrow::Cow;
use alloc::vec;
use alloc::vec::Vec;
use core::mem::take;
use malachite_base::num::arithmetic::traits::{CeilingLogBase2, Parity, PowerOf2};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::split_into_chunks_mut;

// Multiplication using truncated Karatsuba. Below length 7, classical truncated multiplication is
// always theoretically faster, so it is the basecase. Above that, the ordinary (left/right)
// Karatsuba identity is used, with one full Karatsuba multiplication and two truncated Karatsuba
// multiplications, done recursively.
//
// This is equivalent to `_fmpz_poly_mullow_kara_recursive` from `fmpz_poly/mullow_karatsuba_n.c`,
// FLINT 3.6.0.
fn mul_truncated_karatsuba_recursive<C: PolynomialCoefficient>(
    out: &mut [C],
    xs: &[C],
    ys: &[C],
    temp: &mut [C],
    len: usize,
) {
    let m1 = len >> 1;
    let m2 = len - m1;
    let odd = len.odd();
    if len <= 6 {
        mul_truncated_to_out_classical(&mut out[..len], &xs[..len], &ys[..len]);
        return;
    }
    let two_m1 = m1 << 1;
    vec_add(&mut temp[m2..m2 + m1], &xs[..m1], &xs[m1..two_m1]);
    if odd {
        temp[m2 + m1] = xs[two_m1].clone();
    }
    vec_add(
        &mut temp[m2 << 1..(m2 << 1) + m1],
        &ys[..m1],
        &ys[m1..two_m1],
    );
    if odd {
        temp[(m2 << 1) + m1] = ys[two_m1].clone();
    }
    mul_to_out_karatsuba(&mut out[..two_m1 - 1], &xs[..m1], &ys[..m1]);
    out[two_m1 - 1] = C::ZERO;
    split_into_chunks_mut!(temp, m2, [low, sums_1, sums_2], rest);
    mul_truncated_karatsuba_recursive(low, sums_1, sums_2, rest, m2);
    let (high, rest) = temp[m2..].split_at_mut(m2);
    mul_truncated_karatsuba_recursive(high, &xs[m1..], &ys[m1..], rest, m2);
    combine_truncated_karatsuba(out, temp, m1, m2);
}

// The last step of the truncated Karatsuba recursions, with `len = m1 + m2` and `m2` either `m1` or
// `m1 + 1`. On entry, `out[..2 * m1]` holds the product of the low halves, `temp[..m2]` the
// truncated product of the sums of the halves, and `temp[m2..2 * m2]` the truncated product of the
// high halves. On exit, `out[..len]` holds the truncated product; `temp` is left as scratch.
//
// # Worst-case complexity
// $T(n, m) = O(nm)$
//
// $M(m) = O(m)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `m2`, and $m$ is the largest number of
// significant bits of any element of `out` or `temp`.
pub(crate) fn combine_truncated_karatsuba<C: PolynomialCoefficient>(
    out: &mut [C],
    temp: &mut [C],
    m1: usize,
    m2: usize,
) {
    vec_sub_assign(&mut temp[..m2], &out[..m2]);
    let (low, high) = temp.split_at_mut(m2);
    vec_sub_assign(low, &high[..m2]);
    if m2 != m1 {
        out[m1 << 1] = take(&mut high[0]);
    }
    vec_add_assign(&mut out[m1..m1 + m2], low);
}

// Sets `out` to the first `out.len()` coefficients of the product of the polynomials with
// coefficients `xs` and `ys`, both of which have length at least `out.len()`, which must be
// positive.
//
// # Worst-case complexity
// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
//
// $M(n, m) = O(n(m + \log n))$
//
// where $T$ is time, $M$ is additional memory, $n$ is `out.len()`, and $m$ is the largest number of
// significant bits of any element of `xs` or `ys`.
//
// This is equivalent to `_fmpz_poly_mullow_karatsuba_n` from `fmpz_poly/mullow_karatsuba_n.c`,
// FLINT 3.6.0, where `n` is `out.len()`.
crate_test_fn! {mul_truncated_to_out_karatsuba_n<C: PolynomialCoefficient>(
    out: &mut [C],
    xs: &[C],
    ys: &[C],
) {
    let n = out.len();
    assert_ne!(n, 0);
    assert!(xs.len() >= n);
    assert!(ys.len() >= n);
    if n == 1 {
        out[0] = xs[0].mul_ref(&ys[0]);
        return;
    }
    let len = usize::power_of_2(u64::exact_from(n).ceiling_log_base_2());
    let mut temp = vec![C::ZERO; 3 * len];
    mul_truncated_karatsuba_recursive(out, xs, ys, &mut temp, n);
}}

// The first `n` elements of `xs`, padded with zeros if there are fewer.
//
// # Worst-case complexity
// $T(n, m) = O(nm)$
//
// $M(n, m) = O(nm)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `n`, and $m$ is the largest number of
// significant bits of any element of `xs`.
pub(crate) fn padded<C: PolynomialCoefficient>(xs: &[C], n: usize) -> Cow<'_, [C]> {
    if xs.len() >= n {
        Cow::Borrowed(&xs[..n])
    } else {
        let mut padded = Vec::with_capacity(n);
        padded.extend_from_slice(xs);
        padded.resize(n, C::ZERO);
        Cow::Owned(padded)
    }
}

// Sets `out` to the first `out.len()` coefficients of the product of the polynomials with
// coefficients `xs` and `ys`, both nonempty. `out.len()` must be positive and at most `xs.len() +
// ys.len() - 1`.
//
// # Worst-case complexity
// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
//
// $M(n, m) = O(n(m + \log n))$
//
// where $T$ is time, $M$ is additional memory, $n$ is `out.len()`, and $m$ is the largest number of
// significant bits of any element of `xs` or `ys`.
//
// This is equivalent to `_fmpz_poly_mullow_karatsuba` from `fmpz_poly/mullow_karatsuba_n.c`, FLINT
// 3.6.0, where `n` is `out.len()`.
crate_test_fn! {mul_truncated_to_out_karatsuba<C: PolynomialCoefficient>(
    out: &mut [C],
    xs: &[C],
    ys: &[C],
) {
    let n = out.len();
    mul_truncated_to_out_karatsuba_n(out, &padded(xs, n), &padded(ys, n));
}}
