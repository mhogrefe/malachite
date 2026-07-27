// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::float::SerdeFloat;
use crate::{ComparableFloatRef, Float};
use alloc::format;
use alloc::string::String;
use malachite_base::num::conversion::traits::FromStringBase;

impl From<Float> for SerdeFloat {
    #[inline]
    fn from(x: Float) -> Self {
        // `ComparableFloat`'s `Display` rather than `Float`'s, because it writes the precision
        // after a `#`; the digits alone do not determine one, so `Float`'s own output would not
        // round-trip.
        Self(format!("{:#x}", ComparableFloatRef(&x)))
    }
}

impl TryFrom<SerdeFloat> for Float {
    type Error = String;

    #[inline]
    fn try_from(s: SerdeFloat) -> Result<Self, String> {
        Self::from_string_base(16, &s.0).ok_or_else(|| format!("Unrecognized digits in {}", s.0))
    }
}
