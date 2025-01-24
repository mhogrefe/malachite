// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{DivAssignMod, UnsignedAbs};
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::string::to_string::digit_to_display_byte_lower;
use crate::num::conversion::traits::WrappingFrom;

pub fn to_string_base_unsigned_naive<T: PrimitiveUnsigned>(mut x: T, base: u8) -> String
where
    u8: WrappingFrom<T>,
{
    assert!((2..=36).contains(&base), "base out of range");
    if x == T::ZERO {
        "0".to_string()
    } else {
        let base = T::from(base);
        let mut cs = Vec::new();
        while x != T::ZERO {
            cs.push(char::from(
                digit_to_display_byte_lower(u8::wrapping_from(x.div_assign_mod(base))).unwrap(),
            ));
        }
        cs.into_iter().rev().collect()
    }
}

pub fn to_string_base_signed_naive<T: PrimitiveSigned>(x: T, base: u8) -> String
where
    u8: WrappingFrom<<T as UnsignedAbs>::Output>,
    <T as UnsignedAbs>::Output: PrimitiveUnsigned,
{
    assert!((2..=36).contains(&base), "base out of range");
    if x == T::ZERO {
        "0".to_string()
    } else {
        let base = <T as UnsignedAbs>::Output::from(base);
        let mut cs = Vec::new();
        let mut abs_x = x.unsigned_abs();
        while abs_x != <T as UnsignedAbs>::Output::ZERO {
            cs.push(char::from(
                digit_to_display_byte_lower(u8::wrapping_from(abs_x.div_assign_mod(base))).unwrap(),
            ));
        }
        if x < T::ZERO {
            cs.push('-');
        }
        cs.into_iter().rev().collect()
    }
}
