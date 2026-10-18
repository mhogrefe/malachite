// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::strings::latex::ToLatex;
use crate::unsigned_matrix::{UnsignedMatrix, write_rows};
use core::fmt::{Display, Formatter, Result};

impl<T: PrimitiveUnsigned> ToLatex for UnsignedMatrix<T> {
    /// Writes an [`UnsignedMatrix`] as a LaTeX math-mode fragment.
    ///
    /// The matrix is a `pmatrix` environment (from the `amsmath` package), with `&` between the
    /// elements of a row and `\\` between rows. A matrix with no rows or no columns is an empty
    /// `pmatrix` subscripted with its dimensions, such as `\begin{pmatrix}\end{pmatrix}_{4\times
    /// 0}`, matching the way [`Display`] writes it.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::strings::latex::ToLatex;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let m = UnsignedMatrix::<u64>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     m.to_latex_string(),
    ///     "\\begin{pmatrix}1&2\\\\3&4\\end{pmatrix}"
    /// );
    ///
    /// let m = UnsignedMatrix::<u64>::from_str("[10, 5]").unwrap();
    /// assert_eq!(m.to_latex_string(), "\\begin{pmatrix}10&5\\end{pmatrix}");
    ///
    /// assert_eq!(
    ///     UnsignedMatrix::<u64>::from_str("[]_{4,0}")
    ///         .unwrap()
    ///         .to_latex_string(),
    ///     "\\begin{pmatrix}\\end{pmatrix}_{4\\times 0}"
    /// );
    /// ```
    ///
    /// The value column holds each matrix as [`Display`] writes it, and the other column shows the
    /// fragments from the examples above, rendered.
    ///
    /// | value          | renders as                                   |
    /// |----------------|----------------------------------------------|
    /// | `[1, 2; 3, 4]` | $\begin{pmatrix}1&2\\\\3&4\end{pmatrix}$     |
    /// | `[10, 5]`      | $\begin{pmatrix}10&5\end{pmatrix}$           |
    /// | `[]_{4,0}`     | $\begin{pmatrix}\end{pmatrix}_{4\times 0}$   |
    fn fmt_latex(&self, f: &mut Formatter) -> Result {
        f.write_str("\\begin{pmatrix}")?;
        if self.elements.is_empty() {
            return write!(
                f,
                "\\end{{pmatrix}}_{{{}\\times {}}}",
                self.height, self.width
            );
        }
        write_rows(self, f, "&", "\\\\", Display::fmt)?;
        f.write_str("\\end{pmatrix}")
    }
}
