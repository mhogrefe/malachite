// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::strings::typst::ToTypst;
use crate::unsigned_matrix::{UnsignedMatrix, write_rows};
use core::fmt::{Display, Formatter, Result};

impl<T: PrimitiveUnsigned> ToTypst for UnsignedMatrix<T> {
    /// Writes an [`UnsignedMatrix`] as a Typst math-mode fragment.
    ///
    /// The matrix is a call to Typst's `mat` function, with `, ` between the elements of a row and
    /// `; ` between rows, which Typst draws in parentheses. A matrix with no rows or no columns is
    /// an empty `mat()` subscripted with its dimensions, such as `mat()_(4 times 0)`, matching the
    /// way [`Display`](core::fmt::Display) writes it.
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
    /// use malachite_base::strings::typst::ToTypst;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let m = UnsignedMatrix::<u64>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(m.to_typst_string(), "mat(1, 2; 3, 4)");
    ///
    /// let m = UnsignedMatrix::<u64>::from_str("[10, 5]").unwrap();
    /// assert_eq!(m.to_typst_string(), "mat(10, 5)");
    ///
    /// assert_eq!(
    ///     UnsignedMatrix::<u64>::from_str("[]_{4,0}")
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
        write_rows(self, f, ", ", "; ", Display::fmt)?;
        f.write_str(")")
    }
}
