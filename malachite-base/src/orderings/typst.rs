// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::strings::typst::ToTypst;
use core::cmp::Ordering::{self, *};
use core::fmt::{Formatter, Result};

impl ToTypst for Ordering {
    /// Writes an [`Ordering`] as a Typst math-mode fragment.
    ///
    /// The three orderings become the three relations they stand for: `<`, `=`, and `>`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::typst::ToTypst;
    /// use std::cmp::Ordering::*;
    ///
    /// assert_eq!(Less.to_typst().to_string(), "<");
    /// assert_eq!(Equal.to_typst().to_string(), "=");
    /// assert_eq!(Greater.to_typst().to_string(), ">");
    /// ```
    ///
    /// | value     | fragment |
    /// |-----------|----------|
    /// | `Less`    | `<`      |
    /// | `Equal`   | `=`      |
    /// | `Greater` | `>`      |
    #[inline]
    fn fmt_typst(&self, f: &mut Formatter) -> Result {
        f.write_str(match self {
            Less => "<",
            Equal => "=",
            Greater => ">",
        })
    }
}
