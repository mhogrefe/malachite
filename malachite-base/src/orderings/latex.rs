// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::strings::latex::ToLatex;
use core::cmp::Ordering::{self, *};
use core::fmt::{Formatter, Result};

impl ToLatex for Ordering {
    /// Writes an [`Ordering`] as a LaTeX math-mode fragment.
    ///
    /// The fragment is the relation symbol the [`Ordering`] stands for: `<`, `=`, or `>`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::strings::latex::ToLatex;
    ///
    /// assert_eq!(Less.to_latex_string(), "<");
    /// assert_eq!(Equal.to_latex_string(), "=");
    /// assert_eq!(Greater.to_latex_string(), ">");
    /// ```
    ///
    /// | value     | fragment | renders as |
    /// |-----------|----------|------------|
    /// | `Less`    | `<`      | $<$        |
    /// | `Equal`   | `=`      | $=$        |
    /// | `Greater` | `>`      | $>$        |
    #[inline]
    fn fmt_latex(&self, f: &mut Formatter) -> Result {
        f.write_str(match self {
            Less => "<",
            Equal => "=",
            Greater => ">",
        })
    }
}
