// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2008-2011 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::arithmetic::mul_middle::schonhage_strassen::*;

// Sets `out` to the lowest `out.len()` coefficients of the product of the polynomials with
// coefficients `xs` and `ys`, both nonempty, by Schönhage–Strassen multiplication. `out.len()`
// must be positive and less than `xs.len() + ys.len()`.
//
// This is equivalent to `_fmpz_poly_mullow_SS` from `fmpz_poly/mullow_SS.c`, FLINT 3.6.0.
crate_test_fn! {
#[inline]
mul_truncated_to_out_schonhage_strassen(out: &mut [Integer], xs: &[Integer], ys: &[Integer]) {
    mul_middle_to_out_schonhage_strassen(out, xs, ys, 0, out.len());
}}
