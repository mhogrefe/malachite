// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2008, 2009 William Hart
//
//      Copyright © 2010, 2011 Sebastian Pancratz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::arithmetic::mul_truncated::kronecker::mul_truncated_to_out_kronecker;

// Sets `out` to the first `out.len()` coefficients of the square of the polynomial with
// coefficients `xs`, which is nonempty, by Kronecker substitution. `out.len()` must be positive and
// at most `2 * xs.len() - 1`.
//
// This is equivalent to `_fmpz_poly_sqrlow_KS` from `fmpz_poly/sqrlow_KS.c`, FLINT 3.6.0, where `n`
// is `out.len()`.
crate_test_fn! {
#[inline]
square_truncated_to_out_kronecker(out: &mut [Integer], xs: &[Integer]) {
    mul_truncated_to_out_kronecker(out, xs, xs);
}}
