// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::strings::typst::ToTypst;
use core::fmt::{Formatter, Result};

impl ToTypst for () {
    /// Writes the unit type as a Typst math-mode fragment.
    ///
    /// The fragment is `()`, which is how the unit value is written in Rust as well.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::typst::ToTypst;
    ///
    /// assert_eq!(().to_typst().to_string(), "()");
    /// ```
    ///
    /// | value | fragment |
    /// |-------|----------|
    /// | `()`  | `()`     |
    #[inline]
    fn fmt_typst(&self, f: &mut Formatter) -> Result {
        f.write_str("()")
    }
}
