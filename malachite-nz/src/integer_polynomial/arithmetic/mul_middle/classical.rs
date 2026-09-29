// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 William Hart
//
//      Copyright © 2026 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::arithmetic::mul_truncated::classical::product_coefficient;
use crate::integer_polynomial::arithmetic::scalar_mul::integers_mul_scalar_to_out;
use crate::integer_polynomial::arithmetic::square_truncated::classical::square_coefficient;
use core::ptr;

// Sets `out` to the coefficients of $x^i$ for `nlo` $\leq i <$ `nhi` of the product of the
// polynomials with coefficients `xs` and `ys`, both nonempty. `nlo < nhi <= xs.len() + ys.len() -
// 1` must hold, and `out` must have length `nhi - nlo`.
//
// # Worst-case complexity
// $T(n, m) = O(n^2 m \log m \log\log m)$
//
// $M(n, m) = O(n(m + \log n))$
//
// where $T$ is time, $M$ is additional memory, $n$ is `max(xs.len(), ys.len())`, and $m$ is the
// largest number of significant bits of any element of `xs` or `ys`.
//
// This is equivalent to `_fmpz_poly_mulmid_classical` from `fmpz_poly/mulmid_classical.c`, FLINT
// 3.6.0.
crate_test_fn! {mul_middle_to_out_classical(
    out: &mut [Integer],
    xs: &[Integer],
    ys: &[Integer],
    nlo: usize,
    nhi: usize,
) {
    assert_ne!(xs.len(), 0);
    assert_ne!(ys.len(), 0);
    assert!(nlo < nhi);
    assert!(nhi < xs.len() + ys.len());
    if xs.len() == 1 {
        integers_mul_scalar_to_out(out, &ys[nlo..nhi], &xs[0]);
    } else if ys.len() == 1 {
        integers_mul_scalar_to_out(out, &xs[nlo..nhi], &ys[0]);
    } else if ptr::eq(xs, ys) {
        for (o, i) in out.iter_mut().zip(nlo..nhi) {
            *o = square_coefficient(xs, i);
        }
    } else {
        for (o, i) in out.iter_mut().zip(nlo..nhi) {
            *o = product_coefficient(xs, ys, i);
        }
    }
}}
