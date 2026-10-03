// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 Sebastian Pancratz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::coefficient::PolynomialCoefficient;
use crate::integer_polynomial::arithmetic::mul::mul_greater_to_out;
use crate::integer_polynomial::arithmetic::square::square_to_out;
use alloc::vec;

// Sets `out` to the coefficients of the `e`th power of the polynomial with coefficients `xs`, which
// is nonempty, where `e` is less than 5. `out` must have length `e * (xs.len() - 1) + 1`.
//
// This is equivalent to `_fmpz_poly_pow_small` from `fmpz_poly/pow_small.c`, FLINT 3.6.0.
crate_test_fn! {pow_to_out_small<C: PolynomialCoefficient>(out: &mut [C], xs: &[C], e: u64) {
    match e {
        0 => out[0] = C::ONE,
        1 => out.clone_from_slice(xs),
        2 => square_to_out(out, xs),
        3 => {
            let alloc = (xs.len() << 1) - 1;
            let mut t = vec![C::ZERO; alloc];
            square_to_out(&mut t, xs);
            mul_greater_to_out(out, &t, xs);
        }
        4 => {
            let alloc = (xs.len() << 1) - 1;
            let mut t = vec![C::ZERO; alloc];
            square_to_out(&mut t, xs);
            square_to_out(out, &t);
        }
        _ => panic!("e must be less than 5, but is {e}"),
    }
}}
