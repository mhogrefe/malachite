// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::strings::latex::ToLatex;
use core::fmt::{Display, Formatter, Result};

macro_rules! impl_to_latex {
    ($t:ident) => {
        impl ToLatex for $t {
            /// Writes a primitive integer as a LaTeX math-mode fragment.
            ///
            /// The fragment is identical to the [`Display`] output. LaTeX typesets decimal digits,
            /// and a leading `-` before them, correctly in math mode, so nothing needs escaping or
            /// marking up.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::latex#fmt_latex).
            #[inline]
            fn fmt_latex(&self, f: &mut Formatter) -> Result {
                Display::fmt(self, f)
            }
        }
    };
}
apply_to_primitive_ints!(impl_to_latex);
