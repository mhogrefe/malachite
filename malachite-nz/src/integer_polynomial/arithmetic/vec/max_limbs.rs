// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::coefficient::PolynomialCoefficient;

// Returns the largest number of limbs in the absolute value of any element of `xs`, or 0 if `xs` is
// empty.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `_fmpz_vec_max_limbs` from `fmpz_vec/max_limbs.c`, FLINT 3.6.0.
crate_test_fn! {vec_max_limbs<C: PolynomialCoefficient>(xs: &[C]) -> u64 {
    let mut max_limbs = 0;
    for x in xs {
        let limbs = x.unsigned_abs_ref().limb_count();
        if limbs > max_limbs {
            max_limbs = limbs;
        }
    }
    max_limbs
}}
