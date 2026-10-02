// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::num::arithmetic::traits::ModPowerOf2;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use alloc::vec;
use alloc::vec::Vec;

// Multiplies the polynomials with coefficients `xs` and `ys`, both nonempty and reduced modulo
// $2^k$, by schoolbook multiplication, reducing each product and each sum modulo $2^k$.
pub fn mod_power_of_2_mul_naive<T: PrimitiveUnsigned>(xs: &[T], ys: &[T], pow: u64) -> Vec<T> {
    let mut out = vec![T::ZERO; xs.len() + ys.len() - 1];
    for (i, &x) in xs.iter().enumerate() {
        for (o, &y) in out[i..].iter_mut().zip(ys) {
            *o = o.mod_power_of_2_add(x.mod_power_of_2_mul(y, pow), pow);
        }
    }
    out
}

// `len` values below $2^k$, for exercising the multiplication algorithms at chosen lengths. They
// come from a simple linear congruential generator seeded with `seed`.
pub fn mod_power_of_2_generated_coefficients<T: PrimitiveUnsigned>(
    len: usize,
    pow: u64,
    seed: u64,
) -> Vec<T> {
    let mut state = seed;
    (0..len)
        .map(|_| {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let high = state;
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let x = (u128::from(high) << 64) | u128::from(state);
            T::wrapping_from(x.mod_power_of_2(pow))
        })
        .collect()
}
