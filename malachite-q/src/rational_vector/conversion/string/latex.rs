// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_vector::RationalVector;
use core::fmt::{Formatter, Result};
use malachite_base::strings::latex::ToLatex;

impl ToLatex for RationalVector {
    /// Writes a [`RationalVector`] as a LaTeX math-mode fragment.
    ///
    /// The elements' fragments are separated by commas and enclosed in parentheses that grow to fit
    /// their contents, as a vector's coordinates are written by hand. The 0-dimensional vector is
    /// `\left(\right)`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the bits of the elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::strings::latex::ToLatex;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector {
    ///     elements: vec![Rational::from(1u32), Rational::from(2u32), Rational::from(3u32)],
    /// };
    /// assert_eq!(v.to_latex_string(), "\\left(1, 2, 3\\right)");
    /// assert_eq!(
    ///     RationalVector::from_str("(1/2)").unwrap().to_latex_string(),
    ///     "\\left(\\frac{1}{2}\\right)"
    /// );
    /// assert_eq!(
    ///     RationalVector {
    ///         elements: vec![Rational::from(5u32)]
    ///     }
    ///     .to_latex_string(),
    ///     "\\left(5\\right)"
    /// );
    /// assert_eq!(
    ///     RationalVector {
    ///         elements: Vec::new()
    ///     }
    ///     .to_latex_string(),
    ///     "\\left(\\right)"
    /// );
    /// ```
    ///
    /// The value column holds each vector as [`Display`](core::fmt::Display) writes it.
    ///
    /// | value       | fragment                   | renders as                 |
    /// |-------------|----------------------------|----------------------------|
    /// | `(1/2)`     | `\left(\frac{1}{2}\right)` | $\left(\frac{1}{2}\right)$ |
    /// | `(1, 2, 3)` | `\left(1, 2, 3\right)`     | $\left(1, 2, 3\right)$     |
    /// | `(5)`       | `\left(5\right)`           | $\left(5\right)$           |
    /// | `()`        | `\left(\right)`            | $\left(\right)$            |
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
