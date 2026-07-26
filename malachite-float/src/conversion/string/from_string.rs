// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// The inverse of `to_string.rs`: reads back what `Float` and `ComparableFloat` write.
//
// A `ComparableFloat` writes a precision after a `#`, since the digits alone do not determine one
// (`0xff.0#8` has three hex digits, or twelve bits, but a precision of 8). That suffix is what
// makes the round trip exact, in every base. Without it the precision is inferred from the digits,
// as `from_sci_string.rs` describes.

use crate::conversion::string::from_sci_string::float_from_string_base;
use crate::{ComparableFloat, Float};
use core::str::FromStr;
use malachite_base::num::conversion::traits::FromStringBase;

impl FromStringBase for Float {
    /// Converts a string, in a specified base, to a [`Float`].
    #[inline]
    fn from_string_base(base: u8, s: &str) -> Option<Self> {
        float_from_string_base(base, s)
    }
}

impl FromStr for Float {
    type Err = ();

    /// Converts a string to a [`Float`].
    #[inline]
    fn from_str(s: &str) -> Result<Self, ()> {
        float_from_string_base(10, s).ok_or(())
    }
}

impl FromStr for ComparableFloat {
    type Err = ();

    /// Converts a string to a [`ComparableFloat`].
    #[inline]
    fn from_str(s: &str) -> Result<Self, ()> {
        float_from_string_base(10, s).map(ComparableFloat).ok_or(())
    }
}
