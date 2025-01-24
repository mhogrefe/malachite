// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;

pub fn naive_mod_pow<T: PrimitiveUnsigned>(x: T, exp: u64, m: T) -> T {
    if m == T::ONE {
        return T::ZERO;
    }
    let data = T::precompute_mod_mul_data(&m);
    let mut out = T::ONE;
    for _ in 0..exp {
        out.mod_mul_precomputed_assign(x, m, &data);
    }
    out
}
