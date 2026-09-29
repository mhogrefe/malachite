// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2008, 2009 William Hart
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
use crate::integer_polynomial::arithmetic::square_truncated::classical::*;

// Sets `out` to the coefficients of the square of the polynomial with coefficients `xs`, which is
// nonempty. `out` must have length `2 * xs.len() - 1`, so this is the truncated square with nothing
// truncated.
//
// This is equivalent to `_fmpz_poly_sqr_classical` from `fmpz_poly/sqr_classical.c`, FLINT 3.6.0.
crate_test_fn! {
#[inline]
square_to_out_classical(out: &mut [Integer], xs: &[Integer]) {
    assert_eq!(out.len(), (xs.len() << 1) - 1);
    square_truncated_to_out_classical(out, xs);
}}
