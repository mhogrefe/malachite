// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingInto;
use crate::test_util::common::rle_encode;

pub fn factor_naive<T: PrimitiveUnsigned>(mut n: T) -> Vec<(T, u8)> {
    assert_ne!(n, T::ZERO);
    if n == T::ONE {
        return Vec::new();
    }
    let mut factors = Vec::new();
    'outer: loop {
        let limit = n.floor_sqrt();
        for p in T::primes().take_while(|&p| p <= limit) {
            let (q, r) = n.div_mod(p);
            if r == T::ZERO {
                factors.push(p);
                n = q;
                continue 'outer;
            }
        }
        factors.push(n);
        break;
    }
    rle_encode(factors.into_iter())
        .into_iter()
        .map(|(p, e)| {
            let e: u8 = e.wrapping_into();
            (p, e)
        })
        .collect()
}
