// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::big_exponent_float::{BigExponentFloat, SerdeBigExponentFloat};
use alloc::format;
use alloc::string::String;
use malachite_base::num::conversion::traits::FromStringBase;

impl From<BigExponentFloat> for SerdeBigExponentFloat {
    // A `BigExponentFloat` is serialized as its exact hexadecimal string, which round-trips.
    #[inline]
    fn from(x: BigExponentFloat) -> Self {
        Self(format!("{x:#x}"))
    }
}

impl TryFrom<SerdeBigExponentFloat> for BigExponentFloat {
    type Error = String;

    #[inline]
    fn try_from(s: SerdeBigExponentFloat) -> Result<Self, String> {
        Self::from_string_base(16, &s.0).ok_or_else(|| format!("Unrecognized digits in {}", s.0))
    }
}
