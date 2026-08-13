// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::arithmetic::traits::{Mod, Square};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::integer::Integer;

// The Dedekind sum by its definition, term by term over 0 < i < k. This is fmpq_dedekind_sum_naive
// from fmpq/dedekind_sum.c, FLINT 3.6.0. It takes Theta(k) iterations, so it is only usable as a
// reference for small k.
pub fn dedekind_sum_naive(h: &Integer, k: &Integer) -> Rational {
    if *k == 0u32 {
        return Rational::ZERO;
    }
    let mut num = Integer::ZERO;
    let mut i = Integer::ONE;
    while i < *k {
        let j = h * &i;
        let r2 = j.mod_op(k);
        if r2 != 0u32 {
            let a = (((&i).mod_op(k)) << 1u32) - k;
            let b = (r2 << 1u32) - k;
            num += a * b;
        }
        i += Integer::ONE;
    }
    Rational::from_integers(num, k.square() << 2u32)
}
