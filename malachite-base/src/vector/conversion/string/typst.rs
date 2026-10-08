// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::strings::typst::ToTypst;
use crate::vector::Vector;
use core::fmt::{Formatter, Result};

impl<T: ToTypst> ToTypst for Vector<T> {
    /// Writes a [`Vector`] as a Typst math-mode fragment.
    ///
    /// The elements' fragments are separated by commas and enclosed in parentheses, as a vector's
    /// coordinates are written by hand; Typst grows the parentheses to fit their contents. The
    /// 0-dimensional vector is `()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::typst::ToTypst;
    /// use malachite_base::vector::Vector;
    ///
    /// assert_eq!(
    ///     Vector {
    ///         elements: vec![1u32, 2, 3]
    ///     }
    ///     .to_typst_string(),
    ///     "(1, 2, 3)"
    /// );
    /// assert_eq!(
    ///     Vector {
    ///         elements: vec![-5i32]
    ///     }
    ///     .to_typst_string(),
    ///     "(-5)"
    /// );
    /// assert_eq!(
    ///     Vector::<u32> {
    ///         elements: Vec::new()
    ///     }
    ///     .to_typst_string(),
    ///     "()"
    /// );
    /// ```
    ///
    /// The value column holds each vector as [`Display`](core::fmt::Display) writes it.
    ///
    /// | value       | fragment    |
    /// |-------------|-------------|
    /// | `(1, 2, 3)` | `(1, 2, 3)` |
    /// | `(-5)`      | `(-5)`      |
    /// | `()`        | `()`        |
    fn fmt_typst(&self, f: &mut Formatter) -> Result {
        f.write_str("(")?;
        for (i, x) in self.elements.iter().enumerate() {
            if i != 0 {
                f.write_str(", ")?;
            }
            x.fmt_typst(f)?;
        }
        f.write_str(")")
    }
}
