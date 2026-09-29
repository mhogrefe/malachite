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

use crate::integer::Integer;
use crate::integer_polynomial::arithmetic::mul_truncated::classical::mul_truncated_to_out_classical;

// Sets `out` to the coefficients of the product of the polynomials with coefficients `xs` and `ys`,
// both nonempty. `out` must have length `xs.len() + ys.len() - 1`, so this is the truncated product
// with nothing truncated.
//
// This is equivalent to `_fmpz_poly_mul_classical` from `fmpz_poly/mul_classical.c`, FLINT 3.6.0.
crate_test_fn! {
#[inline]
mul_to_out_classical(out: &mut [Integer], xs: &[Integer], ys: &[Integer]) {
    assert_eq!(out.len(), xs.len() + ys.len() - 1);
    mul_truncated_to_out_classical(out, xs, ys);
}}
