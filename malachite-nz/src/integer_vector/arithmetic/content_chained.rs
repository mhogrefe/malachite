// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::integer::Integer;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{Gcd, GcdAssign};
use malachite_base::num::basic::traits::One;

// Returns the GCD of `x` and the absolute values of the elements of `xs`.
//
// This is equivalent to `_fmpz_vec_content_chained` from `fmpz_vec/content_chained.c`, FLINT 3.6.0.
#[doc(hidden)]
pub fn integers_content_chained(xs: &[Integer], x: &Natural) -> Natural {
    // Zeros make no difference, so they are skipped at both ends.
    let Some(start) = xs.iter().position(|c| *c != 0u32) else {
        return x.clone();
    };
    let end = xs.iter().rposition(|c| *c != 0u32).unwrap();
    let mut xs = &xs[start..=end];
    // An element of absolute value 1 makes the GCD 1. Only the ends are checked, since for the
    // coefficients of a polynomial they are the likeliest to be ±1.
    if let [first, .., last] = xs
        && (*first.unsigned_abs_ref() == 1u32 || *last.unsigned_abs_ref() == 1u32)
    {
        return Natural::ONE;
    }
    // The elements are taken in pairs from both ends, stopping as soon as the GCD reaches 1.
    let mut gcd = x.clone();
    while gcd != 1u32
        && let [first, middle @ .., last] = xs
    {
        gcd.gcd_assign(first.unsigned_abs_ref().gcd(last.unsigned_abs_ref()));
        xs = middle;
    }
    if gcd != 1u32
        && let [middle] = xs
    {
        gcd.gcd_assign(middle.unsigned_abs_ref());
    }
    gcd
}
