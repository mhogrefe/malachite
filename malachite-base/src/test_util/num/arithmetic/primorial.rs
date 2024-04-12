// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;

pub fn checked_primorial_naive<T: PrimitiveUnsigned>(n: u64) -> Option<T> {
    let n = T::try_from(n).ok()?;
    let mut f = T::ONE;
    for p in T::primes().take_while(|&p| p <= n) {
        f = f.checked_mul(p)?;
    }
    Some(f)
}

pub fn checked_product_of_first_n_primes_naive<T: PrimitiveUnsigned>(n: u64) -> Option<T> {
    let mut f = T::ONE;
    for p in T::primes().take(usize::try_from(n).ok()?) {
        f = f.checked_mul(p)?;
    }
    Some(f)
}
