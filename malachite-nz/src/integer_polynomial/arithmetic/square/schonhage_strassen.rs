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

// Sets `out` to the coefficients of the square of the polynomial with coefficients `xs`, which is
// nonempty, by Schönhage–Strassen multiplication. `out` must have length `2 * xs.len() - 1`.
//
// This is equivalent to `_fmpz_poly_mul_SS` from `fmpz_poly/mul_SS.c`, FLINT 3.6.0, where both
// factors are the same, as `_fmpz_poly_sqr` calls it.
crate_test_fn! {
#[inline]
square_to_out_schonhage_strassen(out: &mut [Integer], xs: &[Integer]) {
    mul_middle_to_out_schonhage_strassen(out, xs, xs, 0, (xs.len() << 1) - 1);
}}
