// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::basic::traits::Zero;

pub fn from_string_base_naive(small_base: u8, s: &str) -> Option<Natural> {
    let mut x = Natural::ZERO;
    let base = Natural::from(small_base);
    for c in s.chars() {
        x *= &base;
        x += Natural::from(c.to_digit(u32::from(small_base))?);
    }
    Some(x)
}
