// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::traits::Zero;

// A matrix whose elements are built from a fixed pseudorandom sequence: each has a pseudorandom
// number of significant bits, at most `bits`.
pub fn pseudorandom_natural_matrix(height: u64, width: u64, bits: u64, seed: u64) -> NaturalMatrix {
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
        let mut x = Natural::ZERO;
        let mut remaining = len;
        while remaining != 0 {
            let chunk = remaining.min(32);
            x <<= chunk;
            x += Natural::from(next() >> (64 - chunk));
            remaining -= chunk;
        }
        if len != 0 {
            // Make the element have exactly `len` significant bits.
            x |= Natural::power_of_2(len - 1);
        }
        elements.push(x);
    }
    NaturalMatrix::from_owned_elements(height, width, elements)
}
