// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::strings::latex::ToLatex;
use crate::vector::Vector;
use core::fmt::{Formatter, Result};

impl<T: ToLatex> ToLatex for Vector<T> {
    /// Writes a [`Vector`] as a LaTeX math-mode fragment.
    ///
    /// The elements' fragments are separated by commas and enclosed in parentheses that grow to fit
    /// their contents, as a vector's coordinates are written by hand. The 0-dimensional vector is
    /// `\left(\right)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::latex::ToLatex;
    /// use malachite_base::vector::Vector;
    ///
    /// assert_eq!(
    ///     Vector {
    ///         elements: vec![1u32, 2, 3]
    ///     }
    ///     .to_latex_string(),
    ///     "\\left(1, 2, 3\\right)"
    /// );
    /// assert_eq!(
    ///     Vector {
    ///         elements: vec![-5i32]
    ///     }
    ///     .to_latex_string(),
    ///     "\\left(-5\\right)"
    /// );
    /// assert_eq!(
    ///     Vector::<u32> {
    ///         elements: Vec::new()
    ///     }
    ///     .to_latex_string(),
    ///     "\\left(\\right)"
    /// );
    /// ```
    ///
    /// The value column holds each vector as [`Display`](core::fmt::Display) writes it.
    ///
    /// | value       | fragment               | renders as             |
    /// |-------------|------------------------|------------------------|
    /// | `(1, 2, 3)` | `\left(1, 2, 3\right)` | $\left(1, 2, 3\right)$ |
    /// | `(-5)`      | `\left(-5\right)`      | $\left(-5\right)$      |
    /// | `()`        | `\left(\right)`        | $\left(\right)$        |
    fn fmt_latex(&self, f: &mut Formatter) -> Result {
        f.write_str("\\left(")?;
        for (i, x) in self.elements.iter().enumerate() {
            if i != 0 {
                f.write_str(", ")?;
            }
            x.fmt_latex(f)?;
        }
        f.write_str("\\right)")
    }
}
