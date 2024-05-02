// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::basic::traits::Zero;

pub fn natural_sum_alt<I: Iterator<Item = Natural>>(xs: I) -> Natural {
    let mut stack = Vec::new();
    for (i, x) in xs.enumerate().map(|(i, x)| (i + 1, x)) {
        let mut s = x;
        for _ in 0..i.trailing_zeros() {
            s += stack.pop().unwrap();
        }
        stack.push(s);
    }
    let mut s = Natural::ZERO;
    for x in stack {
        s += x;
    }
    s
}
