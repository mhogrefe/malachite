// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2008, 2009 William Hart
//
//      Copyright © 2014 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::arithmetic::square_truncated::tiny::square_truncated_to_out_tiny;
use crate::platform::{SignedDoubleLimb, SignedLimb};

// Sets `out` to the coefficients of the square of the polynomial with coefficients `xs`, which is
// nonempty, using single-word arithmetic. Every element of `xs` must be a small `fmpz`, and every
// coefficient of the square, and every partial sum of the terms that make it up, must fit in
// `SMALL_FMPZ_BITCOUNT_MAX` bits. `out` must have length `2 * xs.len() - 1`, so this is the
// truncated square with nothing truncated.
//
// This is equivalent to `_fmpz_poly_sqr_tiny1` from `fmpz_poly/sqr.c`, FLINT 3.6.0.
crate_test_fn! {
#[inline]
square_to_out_tiny_1(out: &mut [Integer], xs: &[Integer]) {
    assert_eq!(out.len(), (xs.len() << 1) - 1);
    square_truncated_to_out_tiny::<SignedLimb>(out, xs);
}}

// Sets `out` to the coefficients of the square of the polynomial with coefficients `xs`, which is
// nonempty, using double-word arithmetic. Every element of `xs` must be a small `fmpz`, and every
// coefficient of the square, and every partial sum of the terms that make it up, must fit in `2 *
// Limb::WIDTH - 1` bits. `out` must have length `2 * xs.len() - 1`.
//
// This is equivalent to `_fmpz_poly_sqr_tiny2` from `fmpz_poly/sqr.c`, FLINT 3.6.0.
crate_test_fn! {
#[inline]
square_to_out_tiny_2(out: &mut [Integer], xs: &[Integer]) {
    assert_eq!(out.len(), (xs.len() << 1) - 1);
    square_truncated_to_out_tiny::<SignedDoubleLimb>(out, xs);
}}
