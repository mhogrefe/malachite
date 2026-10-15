// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_matrix::{NaturalMatrix, write_rows};
use core::fmt::{Formatter, Result};
use malachite_base::strings::typst::ToTypst;

impl ToTypst for NaturalMatrix {
    /// Writes a [`NaturalMatrix`] as a Typst math-mode fragment.
    ///
    /// The matrix is a call to Typst's `mat` function, with `, ` between the elements of a row and
    /// `; ` between rows, which Typst draws in parentheses. A matrix with no rows or no columns is
    /// an empty `mat()` subscripted with its dimensions, such as `mat()_(4 times 0)`, matching the
    /// way [`Display`](core::fmt::Display) writes it.
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
    /// use malachite_base::strings::typst::ToTypst;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let m = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(m.to_typst_string(), "mat(1, 2; 3, 4)");
    ///
    /// let m = NaturalMatrix::from_str("[10, 5]").unwrap();
    /// assert_eq!(m.to_typst_string(), "mat(10, 5)");
    ///
    /// assert_eq!(
    ///     NaturalMatrix::from_str("[]_{4,0}")
    ///         .unwrap()
    ///         .to_typst_string(),
    ///     "mat()_(4 times 0)"
    /// );
    /// ```
    ///
    /// The value column holds each matrix as [`Display`](core::fmt::Display) writes it.
    ///
    /// | value          | fragment            |
    /// |----------------|---------------------|
    /// | `[1, 2; 3, 4]` | `mat(1, 2; 3, 4)`   |
    /// | `[10, 5]`      | `mat(10, 5)`        |
    /// | `[]_{4,0}`     | `mat()_(4 times 0)` |
    fn fmt_typst(&self, f: &mut Formatter) -> Result {
        if self.elements.is_empty() {
            return write!(f, "mat()_({} times {})", self.height, self.width);
        }
        f.write_str("mat(")?;
        write_rows(self, f, ", ", "; ", ToTypst::fmt_typst)?;
        f.write_str(")")
    }
}
