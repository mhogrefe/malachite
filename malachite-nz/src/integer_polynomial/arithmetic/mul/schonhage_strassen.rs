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

use crate::integer_polynomial::arithmetic::coefficient::PolynomialCoefficient;
use crate::integer_polynomial::arithmetic::mul_middle::schonhage_strassen::*;

// Sets `out` to the coefficients of the product of the polynomials with coefficients `xs` and `ys`,
// both nonempty, by Schönhage–Strassen multiplication. `out` must have length `xs.len() +
// ys.len() - 1`.
//
// This is equivalent to `_fmpz_poly_mul_SS` from `fmpz_poly/mul_SS.c`, FLINT 3.6.0.
crate_test_fn! {
#[inline]
mul_to_out_schonhage_strassen<C: PolynomialCoefficient>(out: &mut [C], xs: &[C], ys: &[C]) {
    mul_middle_to_out_schonhage_strassen(out, xs, ys, 0, xs.len() + ys.len() - 1);
}}
