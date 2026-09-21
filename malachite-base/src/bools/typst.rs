// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::strings::typst::ToTypst;
use core::fmt::{Formatter, Result};

impl ToTypst for bool {
    /// Writes a [`bool`] as a Typst math-mode fragment.
    ///
    /// `true` becomes `"T"` and `false` becomes `"F"`. The quotation marks make them a string,
    /// which Typst typesets upright, rather than letting them be typeset as italic variables.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::typst::ToTypst;
    ///
    /// assert_eq!(true.to_typst_string(), r#""T""#);
    /// assert_eq!(false.to_typst_string(), r#""F""#);
    /// ```
    ///
    /// | value   | fragment |
    /// |---------|----------|
    /// | `true`  | `"T"`    |
    /// | `false` | `"F"`    |
    #[inline]
    fn fmt_typst(&self, f: &mut Formatter) -> Result {
        f.write_str(if *self { "\"T\"" } else { "\"F\"" })
    }
}
