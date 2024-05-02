// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::exhaustive::primitive_int_increasing_inclusive_range;

pub struct PrimesNaive<T: PrimitiveUnsigned>(T);

impl<T: PrimitiveUnsigned> Iterator for PrimesNaive<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.0 == T::ONE {
            self.0 = T::TWO;
            return Some(self.0);
        } else if self.0 == T::TWO {
            self.0 = T::from(3u8);
            return Some(self.0);
        }
        'outer: loop {
            self.0 = self.0.checked_add(T::TWO)?;
            let a = T::from(3u8);
            let b = self.0.floor_sqrt();
            if a <= b {
                for f in primitive_int_increasing_inclusive_range(a, b) {
                    if self.0.divisible_by(f) {
                        continue 'outer;
                    }
                }
            }
            return Some(self.0);
        }
    }
}

pub const fn primes_naive<T: PrimitiveUnsigned>() -> PrimesNaive<T> {
    // 1 will never actually be generated
    PrimesNaive(T::ONE)
}
