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
    /// See [here](super::latex#fmt_latex).
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
