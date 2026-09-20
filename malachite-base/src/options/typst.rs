// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::strings::typst::ToTypst;
use core::fmt::{Formatter, Result};

impl<T: ToTypst> ToTypst for Option<T> {
    /// Writes an [`Option`] as a Typst math-mode fragment.
    ///
    /// [`None`] becomes `bot`, and [`Some`] wraps its value in square brackets. The brackets are
    /// not decoration: without them `Some(None)` and [`None`] would both be `bot`, and distinct
    /// values would have the same fragment.
    ///
    /// Typst grows a matched pair of delimiters to fit what is between them, so the brackets fit a
    /// value that is taller than one line, such as a fraction or a nested [`Option`], without being
    /// asked to.
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of `fmt_typst` for `T`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::typst::ToTypst;
    ///
    /// assert_eq!(None::<u8>.to_typst().to_string(), "bot");
    /// assert_eq!(Some(5u8).to_typst().to_string(), "[5]");
    /// assert_eq!(Some("hi").to_typst().to_string(), r#"["hi"]"#);
    ///
    /// // The brackets keep nested `Option`s apart.
    /// assert_eq!(Some(None::<u8>).to_typst().to_string(), "[bot]");
    /// assert_eq!(Some(Some(5u8)).to_typst().to_string(), "[[5]]");
    /// ```
    ///
    /// | value              | fragment  |
    /// |--------------------|-----------|
    /// | `None::<u8>`       | `bot`     |
    /// | `Some(5u8)`        | `[5]`     |
    /// | `Some("hi")`       | `["hi"]`  |
    /// | `Some(None::<u8>)` | `[bot]`   |
    /// | `Some(Some(5u8))`  | `[[5]]`   |
    #[inline]
    fn fmt_typst(&self, f: &mut Formatter) -> Result {
        match self {
            None => f.write_str("bot"),
            Some(x) => {
                f.write_str("[")?;
                x.fmt_typst(f)?;
                f.write_str("]")
            }
        }
    }
}
