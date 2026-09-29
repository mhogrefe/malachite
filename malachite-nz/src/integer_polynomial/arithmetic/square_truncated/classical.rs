// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Sebastian Pancratz
//
//      Copyright © 2024 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::arithmetic::vec::dot_general::vec_dot_general;
use core::cmp::min;
use malachite_base::num::arithmetic::traits::{AddMulAssign, Parity};

// The coefficient of $x^i$ in the square of the polynomial with coefficients `xs`, which is
// nonempty, where `i < 2 * xs.len() - 1`: twice the dot product of the coefficients of $x^j$ and
// $x^{i-j}$ over the $j < i / 2$ for which both exist, plus the square of the coefficient of
// $x^{i/2}$ if `i` is even.
//
// # Worst-case complexity
// $T(n, m) = O(nm \log m \log\log m)$
//
// $M(m) = O(m \log m)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, and $m$ is the largest number of
// significant bits of any element of `xs`.
pub(crate) fn square_coefficient(xs: &[Integer], i: usize) -> Integer {
    let start = (i + 1).saturating_sub(xs.len());
    let stop = min(xs.len(), (i + 1) >> 1);
    let mut c = vec_dot_general(
        None,
        false,
        &xs[start..stop],
        &xs[i + 1 - stop..=i - start],
        true,
    );
    c <<= 1u32;
    if i.even() {
        c.add_mul_assign(&xs[i >> 1], &xs[i >> 1]);
    }
    c
}

// Sets `out` to the first `out.len()` coefficients of the square of the polynomial with
// coefficients `xs`, which is nonempty. `out.len()` must be positive and at most `2 * xs.len() -
// 1`.
//
// # Worst-case complexity
// $T(n, m) = O(n^2 m \log m \log\log m)$
//
// $M(n, m) = O(n(m + \log n))$
//
// where $T$ is time, $M$ is additional memory, $n$ is `out.len()`, and $m$ is the largest number of
// significant bits of any element of `xs`.
//
// This is equivalent to `_fmpz_poly_sqrlow_classical` from `fmpz_poly/sqrlow_classical.c`, FLINT
// 3.6.0, where `n` is `out.len()`.
crate_test_fn! {square_truncated_to_out_classical(out: &mut [Integer], xs: &[Integer]) {
    let xs = &xs[..min(xs.len(), out.len())];
    for (i, o) in out.iter_mut().enumerate() {
        *o = square_coefficient(xs, i);
    }
}}
