// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{ExtendedGcd, ModInverse};
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;

fn mod_inverse_simple_helper(x: Natural, m: Natural) -> Option<Natural> {
    let (gcd, _, inverse) = (&m).extended_gcd(x);
    if gcd == 1u32 {
        Some(Natural::exact_from(if inverse < 0u32 {
            inverse + Integer::from(m)
        } else {
            inverse
        }))
    } else {
        None
    }
}

pub fn mod_inverse_simple(n: Natural, m: Natural) -> Option<Natural> {
    assert_ne!(n, 0u32);
    assert!(n < m);
    match (n, m) {
        (x @ Natural::ONE, _) => Some(x),
        (Natural(Small(x)), Natural(Small(y))) => x.mod_inverse(y).map(Natural::from),
        (a, b) => mod_inverse_simple_helper(a, b),
    }
}
