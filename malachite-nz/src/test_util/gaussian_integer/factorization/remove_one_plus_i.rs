// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use crate::integer::Integer;
use malachite_base::num::arithmetic::traits::{DivRem, ModPowerOf2, MulIAssign, Parity};
use malachite_base::num::basic::traits::{One, Zero};

// (1 + i)^k, as (2i)^(k/2) times 1 + i if k is odd.
pub fn gaussian_integer_one_plus_i_pow(k: u64) -> GaussianInteger {
    let mut p = GaussianInteger::ONE << (k >> 1);
    for _ in 0..(k >> 1).mod_power_of_2(2) {
        p.mul_i_assign();
    }
    if k.odd() {
        p *= GaussianInteger {
            real: Integer::ONE,
            imaginary: Integer::ONE,
        };
    }
    p
}

// Divides by 1 + i, using the nearest-quotient division, for as long as the remainder is zero.
pub fn gaussian_integer_remove_one_plus_i_naive(x: &GaussianInteger) -> (GaussianInteger, u64) {
    if *x == 0u32 {
        return (GaussianInteger::ZERO, 0);
    }
    let one_plus_i = GaussianInteger {
        real: Integer::ONE,
        imaginary: Integer::ONE,
    };
    let mut x = x.clone();
    let mut k = 0;
    loop {
        let (q, r) = (&x).div_rem(&one_plus_i);
        if r != 0u32 {
            return (x, k);
        }
        x = q;
        k += 1;
    }
}
