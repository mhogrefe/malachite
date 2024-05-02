// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::platform::Limb;
use malachite_base::num::conversion::string::to_string::digit_to_display_byte_lower;
use malachite_base::num::conversion::traits::WrappingFrom;

pub fn to_string_base_naive(x: &Natural, base: u8) -> String {
    assert!((2..=36).contains(&base), "base out of range");
    let base = Limb::from(base);
    if *x == 0 {
        "0".to_string()
    } else {
        let mut x = x.clone();
        let mut cs = Vec::new();
        while x != 0 {
            cs.push(char::from(
                digit_to_display_byte_lower(u8::wrapping_from(x.div_assign_mod_limb(base)))
                    .unwrap(),
            ));
        }
        cs.into_iter().rev().collect()
    }
}
