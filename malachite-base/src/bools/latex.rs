// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::strings::latex::ToLatex;
use core::fmt::{Formatter, Result};

impl ToLatex for bool {
    /// Writes a [`bool`] as a LaTeX math-mode fragment.
    ///
    /// `true` becomes `\text{T}` and `false` becomes `\text{F}`. The `\text` keeps them upright,
    /// rather than letting them be typeset as italic variables.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::latex::ToLatex;
    ///
    /// assert_eq!(true.to_latex().to_string(), r"\text{T}");
    /// assert_eq!(false.to_latex().to_string(), r"\text{F}");
    /// ```
    ///
    /// | value   | fragment   | renders as |
    /// |---------|------------|------------|
    /// | `true`  | `\text{T}` | $\text{T}$ |
    /// | `false` | `\text{F}` | $\text{F}$ |
    #[inline]
    fn fmt_latex(&self, f: &mut Formatter) -> Result {
        f.write_str(if *self { "\\text{T}" } else { "\\text{F}" })
    }
}
