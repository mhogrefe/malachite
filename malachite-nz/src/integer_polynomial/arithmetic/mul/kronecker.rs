// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2008, 2009 William Hart
//
//      Copyright © 2010, 2012 Sebastian Pancratz
//
//      Copyright © 2026 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::arithmetic::mul_middle::kronecker::mul_middle_to_out_kronecker;

// Sets `out` to the coefficients of the product of the polynomials with coefficients `xs` and `ys`,
// both nonempty, by Kronecker substitution. `out` must have length `xs.len() + ys.len() - 1`.
//
// This is equivalent to `_fmpz_poly_mul_KS` from `fmpz_poly/mul_KS.c`, FLINT 3.6.0.
crate_test_fn! {
#[inline]
mul_to_out_kronecker(out: &mut [Integer], xs: &[Integer], ys: &[Integer]) {
    mul_middle_to_out_kronecker(out, xs, ys, 0, xs.len() + ys.len() - 1);
}}
