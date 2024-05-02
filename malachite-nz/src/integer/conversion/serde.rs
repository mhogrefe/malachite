// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::{Integer, SerdeInteger};
use crate::natural::Natural;
use alloc::string::String;
use core::convert::TryFrom;
use malachite_base::num::conversion::traits::FromStringBase;

impl From<Integer> for SerdeInteger {
    #[inline]
    fn from(x: Integer) -> SerdeInteger {
        SerdeInteger(format!("{x:#x}"))
    }
}

impl TryFrom<SerdeInteger> for Integer {
    type Error = String;

    #[inline]
    fn try_from(s: SerdeInteger) -> Result<Integer, String> {
        if s.0.starts_with('-') {
            if s.0.starts_with("-0x") {
                Ok(Integer::from_sign_and_abs(
                    false,
                    Natural::from_string_base(16, &s.0[3..])
                        .ok_or_else(|| format!("Unrecognized digits in {}", s.0))?,
                ))
            } else {
                Err(format!(
                    "String '{}' starts with '-' but not with '-0x'",
                    s.0
                ))
            }
        } else if s.0.starts_with("0x") {
            Ok(Integer::from(
                Natural::from_string_base(16, &s.0[2..])
                    .ok_or_else(|| format!("Unrecognized digits in {}", s.0))?,
            ))
        } else {
            Err(format!(
                "String '{}' does not start with '0x' or '-0x'",
                s.0
            ))
        }
    }
}
