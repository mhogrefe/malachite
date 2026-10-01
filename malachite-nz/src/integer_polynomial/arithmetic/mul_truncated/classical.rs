// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2008, 2009 William Hart
//
//      Copyright © 2024 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::coefficient::PolynomialCoefficient;
use crate::integer_polynomial::arithmetic::vec::dot_general::vec_dot_general;
use core::cmp::min;

// The coefficient of $x^i$ in the product of the polynomials with coefficients `xs` and `ys`, both
// nonempty, where `i < xs.len() + ys.len() - 1`: the dot product of the coefficients of $x^j$ in
// the first and of $x^{i-j}$ in the second, over the $j$ for which both exist.
//
// # Worst-case complexity
// $T(n, m) = O(nm \log m \log\log m)$
//
// $M(m) = O(m \log m)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `min(xs.len(), ys.len())`, and $m$ is the
// largest number of significant bits of any element of `xs` or `ys`.
pub(crate) fn product_coefficient<C: PolynomialCoefficient>(xs: &[C], ys: &[C], i: usize) -> C {
    let top1 = min(xs.len() - 1, i);
    let top2 = min(ys.len() - 1, i);
    let n = top1 + top2 + 1 - i;
    vec_dot_general(
        None,
        false,
        &xs[i - top2..i - top2 + n],
        &ys[i - top1..i - top1 + n],
        true,
    )
}

// Sets `out` to the first `out.len()` coefficients of the product of the polynomials with
// coefficients `xs` and `ys`, both nonempty. `out.len()` must be positive and at most `xs.len() +
// ys.len() - 1`.
//
// # Worst-case complexity
// $T(n, m) = O(n^2 m \log m \log\log m)$
//
// $M(n, m) = O(n(m + \log n))$
//
// where $T$ is time, $M$ is additional memory, $n$ is `out.len()`, and $m$ is the largest number of
// significant bits of any element of `xs` or `ys`.
//
// This is equivalent to `_fmpz_poly_mullow_classical` from `fmpz_poly/mullow_classical.c`, FLINT
// 3.6.0, where `n` is `out.len()`.
crate_test_fn! {mul_truncated_to_out_classical<C: PolynomialCoefficient>(
    out: &mut [C],
    xs: &[C],
    ys: &[C],
) {
    let n = out.len();
    let xs = &xs[..min(xs.len(), n)];
    let ys = &ys[..min(ys.len(), n)];
    if xs.len() == 1 {
        C::vec_mul_scalar_to_out(out, ys, &xs[0]);
    } else if ys.len() == 1 {
        C::vec_mul_scalar_to_out(out, xs, &ys[0]);
    } else {
        for (i, o) in out.iter_mut().enumerate() {
            *o = product_coefficient(xs, ys, i);
        }
    }
}}
