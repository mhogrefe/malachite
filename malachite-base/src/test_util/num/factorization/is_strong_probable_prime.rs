// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::logic::traits::{BitAccess, SignificantBits};
use crate::num::modular::traits::{ModMulAssign, ModSquareAssign};

// Computes a^exp mod n by plain square-and-multiply.
fn mod_pow_naive(a: u128, exp: u128, n: u128) -> u128 {
    let mut y = 1 % n;
    for bit in (0..exp.significant_bits()).rev() {
        y.mod_square_assign(n);
        if exp.get_bit(bit) {
            y.mod_mul_assign(a, n);
        }
    }
    y
}

/// Whether n is a strong probable prime to base a: with a reduced mod n, either a is 0, 1, or n -
/// 1, or, writing n - 1 = d * 2^s with d odd, a^d = 1 or a^(d * 2^r) = -1 mod n for some r < s.
pub fn is_strong_probable_prime_naive(n: u128, a: u128) -> bool {
    if n <= 1 {
        return false;
    }
    let a = a % n;
    let nm1 = n - 1;
    if a == 0 || a == 1 || a == nm1 {
        return true;
    }
    let s = nm1.trailing_zeros();
    let d = nm1 >> s;
    let mut y = mod_pow_naive(a, d, n);
    if y == 1 || y == nm1 {
        return true;
    }
    for _ in 1..s.max(1) {
        y.mod_square_assign(n);
        if y == nm1 {
            return true;
        }
    }
    false
}
