// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rounding_modes::RoundingMode::{self, *};
use crate::strings::latex::ToLatex;
use core::fmt::{Formatter, Result};

impl ToLatex for RoundingMode {
    /// Writes a [`RoundingMode`] as a LaTeX math-mode fragment.
    ///
    /// The fragment is the mode's name in small capitals-style uppercase, wrapped in `\text` to
    /// keep it upright: `\text{FLOOR}`, and so on. There is no conventional mathematical symbol for
    /// a rounding mode, so the name is spelled out.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_base::strings::latex::ToLatex;
    ///
    /// assert_eq!(Down.to_latex().to_string(), r"\text{DOWN}");
    /// assert_eq!(Up.to_latex().to_string(), r"\text{UP}");
    /// assert_eq!(Floor.to_latex().to_string(), r"\text{FLOOR}");
    /// assert_eq!(Ceiling.to_latex().to_string(), r"\text{CEILING}");
    /// assert_eq!(Nearest.to_latex().to_string(), r"\text{NEAREST}");
    /// assert_eq!(Exact.to_latex().to_string(), r"\text{EXACT}");
    /// ```
    ///
    /// | value     | fragment         | renders as       |
    /// |-----------|------------------|------------------|
    /// | `Down`    | `\text{DOWN}`    | $\text{DOWN}$    |
    /// | `Up`      | `\text{UP}`      | $\text{UP}$      |
    /// | `Floor`   | `\text{FLOOR}`   | $\text{FLOOR}$   |
    /// | `Ceiling` | `\text{CEILING}` | $\text{CEILING}$ |
    /// | `Nearest` | `\text{NEAREST}` | $\text{NEAREST}$ |
    /// | `Exact`   | `\text{EXACT}`   | $\text{EXACT}$   |
    #[inline]
    fn fmt_latex(&self, f: &mut Formatter) -> Result {
        f.write_str(match self {
            Down => "\\text{DOWN}",
            Up => "\\text{UP}",
            Floor => "\\text{FLOOR}",
            Ceiling => "\\text{CEILING}",
            Nearest => "\\text{NEAREST}",
            Exact => "\\text{EXACT}",
        })
    }
}
