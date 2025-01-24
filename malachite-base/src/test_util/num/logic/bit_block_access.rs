// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::traits::Zero;
use crate::num::logic::traits::BitAccess;

pub fn get_bits_naive<T: BitAccess, U: BitAccess + Zero>(n: &T, start: u64, end: u64) -> U {
    let mut result = U::ZERO;
    for i in start..end {
        if n.get_bit(i) {
            result.set_bit(i - start);
        }
    }
    result
}

pub fn assign_bits_naive<T: BitAccess, U: BitAccess>(n: &mut T, start: u64, end: u64, bits: &U) {
    for i in start..end {
        n.assign_bit(i, bits.get_bit(i - start));
    }
}
