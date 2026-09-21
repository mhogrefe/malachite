// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use core::fmt::{Display, Formatter, Result};
use malachite_base::strings::latex::ToLatex;

impl ToLatex for Integer {
    /// Writes a [`Integer`] as a LaTeX math-mode fragment.
    ///
    /// The fragment is the number's decimal digits, preceded by a minus sign if it is negative,
    /// which is what LaTeX math mode already writes a number as, and what [`Display`] gives.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::latex::ToLatex;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(0).to_latex_string(), "0");
    /// assert_eq!(Integer::from(123).to_latex_string(), "123");
    /// assert_eq!(Integer::from(-123).to_latex_string(), "-123");
    /// ```
    ///
    /// | value                 | fragment | renders as |
    /// |-----------------------|----------|------------|
    /// | `Integer::from(123)`  | `123`    | $123$      |
    /// | `Integer::from(-123)` | `-123`   | $-123$     |
    #[inline]
    fn fmt_latex(&self, f: &mut Formatter) -> Result {
        Display::fmt(self, f)
    }
}
