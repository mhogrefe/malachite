// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::strings::typst::ToTypst;
use core::fmt::{Formatter, Result};

// Writes a sequence of values as one delimited, comma-separated Typst math-mode fragment.
pub(crate) fn fmt_typst_sequence<'a, T: ToTypst + 'a>(
    xs: impl Iterator<Item = &'a T>,
    open: &str,
    close: &str,
    f: &mut Formatter,
) -> Result {
    f.write_str(open)?;
    for (i, x) in xs.enumerate() {
        if i != 0 {
            f.write_str(", ")?;
        }
        x.fmt_typst(f)?;
    }
    f.write_str(close)
}

// Writes a sequence of values as one bracketed, comma-separated Typst math-mode fragment.
pub(crate) fn fmt_typst_slice<T: ToTypst>(xs: &[T], f: &mut Formatter) -> Result {
    fmt_typst_sequence(xs.iter(), "[", "]", f)
}

impl<T: ToTypst> ToTypst for &[T] {
    /// Writes a slice as a Typst math-mode fragment.
    ///
    /// The elements' fragments are separated by commas and wrapped in square brackets, so that a
    /// slice's fragment is built out of its elements' own.
    ///
    /// The brackets are not decoration: without them `[1, 2]` and `[[1], [2]]` would both be `1,
    /// 2`, and distinct values would have the same fragment. Typst grows a matched pair of
    /// delimiters to fit what is between them, so they fit an element that is taller than one line,
    /// such as a fraction or a nested slice, without being asked to.
    ///
    /// An empty slice becomes `[]` rather than nothing at all.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n + \sum_{i=0}^{n-1}T^\prime(i))$
    ///
    /// $M(n) = O(\max_{i=0}^{n-1}M^\prime(i))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $i$ is an element's index,
    /// and $T^\prime$ and $M^\prime$ are the time and memory functions of `fmt_typst` for `T`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::typst::ToTypst;
    ///
    /// assert_eq!([0u8; 0].as_slice().to_typst().to_string(), "[]");
    /// assert_eq!([5u8].as_slice().to_typst().to_string(), "[5]");
    /// assert_eq!([1u8, 2, 3].as_slice().to_typst().to_string(), "[1, 2, 3]");
    /// assert_eq!(
    ///     ["hi", "yo"].as_slice().to_typst().to_string(),
    ///     r#"["hi", "yo"]"#
    /// );
    /// ```
    ///
    /// | value          | fragment       |
    /// |----------------|----------------|
    /// | `[0u8; 0]`     | `[]`           |
    /// | `[5u8]`        | `[5]`          |
    /// | `[1u8, 2, 3]`  | `[1, 2, 3]`    |
    /// | `["hi", "yo"]` | `["hi", "yo"]` |
    #[inline]
    fn fmt_typst(&self, f: &mut Formatter) -> Result {
        fmt_typst_slice(self, f)
    }
}
