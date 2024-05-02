// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::{Natural, SerdeNatural};
use alloc::string::String;
use core::convert::TryFrom;
use malachite_base::num::conversion::traits::FromStringBase;

impl From<Natural> for SerdeNatural {
    #[inline]
    fn from(x: Natural) -> SerdeNatural {
        SerdeNatural(format!("{x:#x}"))
    }
}

impl TryFrom<SerdeNatural> for Natural {
    type Error = String;

    #[inline]
    fn try_from(s: SerdeNatural) -> Result<Natural, String> {
        if s.0.starts_with("0x") {
            Natural::from_string_base(16, &s.0[2..])
                .ok_or_else(|| format!("Unrecognized digits in {}", s.0))
        } else {
            Err(format!("String '{}' does not start with '0x'", s.0))
        }
    }
}
