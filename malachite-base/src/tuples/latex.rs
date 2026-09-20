// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::strings::latex::ToLatex;
use core::fmt::{Formatter, Result};

impl ToLatex for () {
    /// Writes the unit type as a LaTeX math-mode fragment.
    ///
    /// The fragment is `()`, an empty pair of parentheses.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::latex::ToLatex;
    ///
    /// assert_eq!(().to_latex().to_string(), "()");
    /// ```
    ///
    /// | value | fragment | renders as |
    /// |-------|----------|------------|
    /// | `()`  | `()`     | $()$       |
    #[inline]
    fn fmt_latex(&self, f: &mut Formatter) -> Result {
        f.write_str("()")
    }
}
