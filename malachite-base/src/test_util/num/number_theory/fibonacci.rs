// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;

pub fn checked_fibonacci_naive<T: PrimitiveUnsigned>(n: u64) -> Option<T> {
    if n == 0 {
        return Some(T::ZERO);
    }
    let (mut a, mut b) = (T::ZERO, T::ONE);
    for _ in 1..n {
        let c = a.checked_add(b)?;
        a = b;
        b = c;
    }
    Some(b)
}

pub fn checked_fibonacci_pair_naive<T: PrimitiveUnsigned>(n: u64) -> Option<(T, T)> {
    if n == 0 {
        // F(-1) = 1
        return Some((T::ZERO, T::ONE));
    }
    let (mut a, mut b) = (T::ZERO, T::ONE);
    for _ in 1..n {
        let c = a.checked_add(b)?;
        a = b;
        b = c;
    }
    Some((b, a))
}

pub fn checked_lucas_number_naive<T: PrimitiveUnsigned>(n: u64) -> Option<T> {
    if n == 0 {
        return Some(T::TWO);
    }
    let (mut a, mut b) = (T::TWO, T::ONE);
    for _ in 1..n {
        let c = a.checked_add(b)?;
        a = b;
        b = c;
    }
    Some(b)
}

pub fn checked_lucas_number_pair_naive<T: PrimitiveUnsigned>(n: u64) -> Option<(T, T)> {
    if n == 0 {
        // L(-1) = -1, which cannot be represented
        return None;
    }
    let (mut a, mut b) = (T::TWO, T::ONE);
    for _ in 1..n {
        let c = a.checked_add(b)?;
        a = b;
        b = c;
    }
    Some((b, a))
}
