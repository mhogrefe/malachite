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
use crate::integer_vector::arithmetic::scalar_mul::integers_mul_scalar_to_out;
use core::cmp::max;
use malachite_base::num::arithmetic::traits::AddMulAssign;
use malachite_base::num::basic::traits::Zero;

// Sets `out` to the coefficients of the product of the polynomials with coefficients `xs` and `ys`,
// both nonempty, except that the first `start` coefficients are set to zero; the coefficients of
// $x^i$ for $i < `start`$ are not computed. `out` must have length `xs.len() + ys.len() - 1`.
//
// # Worst-case complexity
// $T(n, m) = O(n^2 m \log m \log\log m)$
//
// $M(n, m) = O(n(m + \log n))$
//
// where $T$ is time, $M$ is additional memory, $n$ is `max(xs.len(), ys.len())`, and $m$ is the
// largest number of significant bits of any element of `xs` or `ys`.
//
// This is equivalent to `_fmpz_poly_mulhigh_classical` from `fmpz_poly/mulhigh_classical.c`, FLINT
// 3.6.0.
crate_test_fn! {mul_high_to_out_classical(
    out: &mut [Integer],
    xs: &[Integer],
    ys: &[Integer],
    start: usize,
) {
    let len1 = xs.len();
    let len2 = ys.len();
    out[..start].fill(Integer::ZERO);
    if len1 == 1 && len2 == 1 {
        // Special case if the length of both inputs is 1
        if start == 0 {
            out[0] = &xs[0] * &ys[0];
        }
    } else {
        // Set out[i] = xs[i] * ys[0]
        if start < len1 {
            integers_mul_scalar_to_out(&mut out[start..len1], &xs[start..], &ys[0]);
        }
        // Set out[i + len1 - 1] = xs[len1 - 1] * ys[i]
        let m = max(len1 - 1, start);
        integers_mul_scalar_to_out(&mut out[m..], &ys[m + 1 - len1..], &xs[len1 - 1]);
        // out[i + j] += xs[i] * ys[j]
        let m = max(start, len2 - 1);
        for i in m + 1 - len2..len1 - 1 {
            let n = max(i + 1, start);
            for (o, y) in out[n..].iter_mut().zip(&ys[n - i..]) {
                o.add_mul_assign(y, &xs[i]);
            }
        }
    }
}}
