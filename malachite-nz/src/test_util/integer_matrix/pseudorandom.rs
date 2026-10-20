// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{Parity, PowerOf2};
use malachite_base::num::basic::traits::Zero;

// A matrix whose elements are built from a fixed pseudorandom sequence: each has a pseudorandom
// number of significant bits, at most `bits`, and is negative about half of the time if `signed`.
pub fn pseudorandom_integer_matrix(
    height: u64,
    width: u64,
    bits: u64,
    signed: bool,
    seed: u64,
) -> IntegerMatrix {
    let mut state = seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1;
    let mut next = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    let mut elements = Vec::new();
    for _ in 0..height * width {
        let len = next() % (bits + 1);
        let mut x = Integer::ZERO;
        let mut remaining = len;
        while remaining != 0 {
            let chunk = remaining.min(32);
            x <<= chunk;
            x += Integer::from(next() >> (64 - chunk));
            remaining -= chunk;
        }
        if len != 0 {
            // Make the element have exactly `len` significant bits.
            x |= Integer::power_of_2(len - 1);
        }
        let state = next();
        let x = if signed && (state >> 7).odd() { -x } else { x };
        elements.push(x);
    }
    IntegerMatrix::from_owned_elements(height, width, elements)
}
