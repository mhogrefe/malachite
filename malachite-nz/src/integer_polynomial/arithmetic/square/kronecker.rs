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

use crate::integer_polynomial::arithmetic::coefficient::PolynomialCoefficient;
use crate::integer_polynomial::arithmetic::mul::kronecker::mul_to_out_kronecker;

// Sets `out` to the coefficients of the square of the polynomial with coefficients `xs`, which is
// nonempty, by Kronecker substitution. `out` must have length `2 * xs.len() - 1`.
//
// This is equivalent to `_fmpz_poly_sqr_KS` from `fmpz_poly/sqr_KS.c`, FLINT 3.6.0.
crate_test_fn! {
#[inline]
square_to_out_kronecker<C: PolynomialCoefficient>(out: &mut [C], xs: &[C]) {
    mul_to_out_kronecker(out, xs, xs);
}}
