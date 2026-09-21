// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rounding_modes::RoundingMode::{self, *};
use crate::strings::typst::ToTypst;
use core::fmt::{Formatter, Result};

impl ToTypst for RoundingMode {
    /// Writes a [`RoundingMode`] as a Typst math-mode fragment.
    ///
    /// Each mode becomes its own name, in capitals, inside a string, which Typst typesets upright.
    /// There is no set of symbols for these that a reader would recognize, so the names stand.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_base::strings::typst::ToTypst;
    ///
    /// assert_eq!(Down.to_typst_string(), r#""DOWN""#);
    /// assert_eq!(Up.to_typst_string(), r#""UP""#);
    /// assert_eq!(Floor.to_typst_string(), r#""FLOOR""#);
    /// assert_eq!(Ceiling.to_typst_string(), r#""CEILING""#);
    /// assert_eq!(Nearest.to_typst_string(), r#""NEAREST""#);
    /// assert_eq!(Exact.to_typst_string(), r#""EXACT""#);
    /// ```
    ///
    /// | value     | fragment    |
    /// |-----------|-------------|
    /// | `Down`    | `"DOWN"`    |
    /// | `Up`      | `"UP"`      |
    /// | `Floor`   | `"FLOOR"`   |
    /// | `Ceiling` | `"CEILING"` |
    /// | `Nearest` | `"NEAREST"` |
    /// | `Exact`   | `"EXACT"`   |
    #[inline]
    fn fmt_typst(&self, f: &mut Formatter) -> Result {
        f.write_str(match self {
            Down => "\"DOWN\"",
            Up => "\"UP\"",
            Floor => "\"FLOOR\"",
            Ceiling => "\"CEILING\"",
            Nearest => "\"NEAREST\"",
            Exact => "\"EXACT\"",
        })
    }
}
