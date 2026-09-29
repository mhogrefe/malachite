// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 William Hart Copyright © 2011 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::platform::Limb;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;

// Returns the number of significant bits of the largest absolute value in `xs`, and whether any
// element of `xs` is negative. The top limbs of the elements with the most limbs are or-ed
// together, which finds the largest bit length without computing any element's.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `_fmpz_vec_max_bits` from `fmpz_vec/max_bits.c`, FLINT 3.6.0, which returns
// the two results combined, as a count that is negated when some element is negative.
crate_test_fn! {vec_max_bits(xs: &[Integer]) -> (u64, bool) {
    let mut negative = false;
    let mut max_limbs = 0;
    let mut max_limb: Limb = 0;
    for x in xs {
        if !x.sign {
            negative = true;
        }
        let limbs = x.abs.as_limbs_asc();
        let len = limbs.len();
        if len == 0 {
            continue;
        }
        if len == max_limbs {
            max_limb |= limbs[len - 1];
        } else if len > max_limbs {
            max_limb = limbs[len - 1];
            max_limbs = len;
        }
    }
    let bits = if max_limbs == 0 {
        0
    } else {
        (u64::exact_from(max_limbs - 1) << Limb::LOG_WIDTH) + max_limb.significant_bits()
    };
    (bits, negative)
}}
