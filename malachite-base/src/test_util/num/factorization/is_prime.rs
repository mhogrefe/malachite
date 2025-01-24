// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use std::cmp::Ordering::*;

pub fn is_prime_naive<T: PrimitiveUnsigned>(n: T) -> bool {
    match n.cmp(&T::TWO) {
        Less => false,
        Equal => true,
        Greater => {
            if n.even() {
                return false;
            }
            let limit = n.floor_sqrt();
            let mut i = T::from(3u8);
            while i <= limit {
                if n.divisible_by(i) {
                    return false;
                }
                i += T::TWO;
            }
            true
        }
    }
}
