// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{DivMod, ExtendedGcd, ModDiv, ModMul};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;

fn mod_div_simple_helper(b: Natural, c: Natural, m: Natural) -> Option<Natural> {
    let (gcd, _, s) = (&m).extended_gcd(c);
    let (q, r) = b.div_mod(gcd);
    if r != 0u32 {
        return None;
    }
    let s = Natural::exact_from(if s < 0u32 { s + Integer::from(&m) } else { s });
    Some(q.mod_mul(s, m))
}

// A simple reference implementation of `ModDiv`, using the public extended GCD. It goes through the
// same underlying cofactor computation as the implementation in `mod_div.rs`, so the returned
// quotients agree exactly.
pub fn mod_div_simple(b: Natural, c: Natural, m: Natural) -> Option<Natural> {
    assert!(b < m);
    assert!(c < m);
    if c == 0u32 {
        return if b == 0u32 { Some(Natural::ZERO) } else { None };
    }
    if b == 0u32 {
        return Some(Natural::ZERO);
    }
    match (b, c, m) {
        (Natural(Small(b)), Natural(Small(c)), Natural(Small(m))) => {
            b.mod_div(c, m).map(Natural::from)
        }
        (b, c, m) => mod_div_simple_helper(b, c, m),
    }
}
