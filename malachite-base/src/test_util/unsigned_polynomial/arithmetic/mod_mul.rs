// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_mul::*;
use alloc::vec;
use alloc::vec::Vec;

// Multiplies the polynomials with coefficients `xs` and `ys`, both nonempty and reduced modulo $m$,
// by schoolbook multiplication, reducing each product and each sum modulo $m$.
pub fn mod_mul_naive<T: PrimitiveUnsigned>(xs: &[T], ys: &[T], m: T) -> Vec<T> {
    let mut out = vec![T::ZERO; xs.len() + ys.len() - 1];
    for (i, &x) in xs.iter().enumerate() {
        for (o, &y) in out[i..].iter_mut().zip(ys) {
            *o = o.mod_add(x.mod_mul(y, m), m);
        }
    }
    out
}

// `len` values below $m$, for exercising the multiplication algorithms at chosen lengths. They come
// from a simple linear congruential generator seeded with `seed`.
pub fn mod_generated_coefficients<T: PrimitiveUnsigned>(len: usize, m: T, seed: u64) -> Vec<T> {
    mod_power_of_2_generated_coefficients::<T>(len, T::WIDTH, seed)
        .into_iter()
        .map(|x| x % m)
        .collect()
}

// Moduli for exercising the multiplication algorithms: small ones, and ones near the top of the
// type's range.
pub fn test_moduli<T: PrimitiveUnsigned>() -> Vec<T> {
    let half = T::MAX >> 1;
    vec![
        T::ONE,
        T::TWO,
        T::exact_from(3),
        T::exact_from(7),
        half,
        half + T::TWO,
        T::MAX - T::ONE,
        T::MAX,
    ]
}
