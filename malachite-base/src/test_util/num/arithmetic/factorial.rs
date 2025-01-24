// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::logic::traits::NotAssign;

pub fn checked_factorial_naive<T: PrimitiveUnsigned>(n: u64) -> Option<T> {
    let mut f = T::ONE;
    let mut n = T::try_from(n).ok()?;
    while n != T::ZERO {
        f = f.checked_mul(n)?;
        n -= T::ONE;
    }
    Some(f)
}

pub fn checked_double_factorial_naive<T: PrimitiveUnsigned>(n: u64) -> Option<T> {
    let mut f = T::ONE;
    let mut n = T::try_from(n).ok()?;
    while n != T::ZERO {
        f = f.checked_mul(n)?;
        n.saturating_sub_assign(T::TWO);
    }
    Some(f)
}

pub fn checked_subfactorial_naive<T: PrimitiveUnsigned>(n: u64) -> Option<T> {
    let mut f = T::ONE;
    let mut b = true;
    for i in 1..=n {
        f = f.checked_mul(T::try_from(i).ok()?)?;
        if b {
            f -= T::ONE;
        } else {
            f = f.checked_add(T::ONE)?;
        }
        b.not_assign();
    }
    Some(f)
}
