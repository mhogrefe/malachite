// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::strings::latex::ToLatex;
use core::fmt::{Formatter, Result};

impl<T: ToLatex> ToLatex for Option<T> {
    /// Writes an [`Option`] as a LaTeX math-mode fragment.
    ///
    /// [`None`] becomes $\bot$, and [`Some`] wraps its value in square brackets. The brackets are
    /// not decoration: without them `Some(None)` and [`None`] would both be $\bot$, and distinct
    /// values would have the same fragment. Braces would have served as well, but this crate's
    /// documentation already spells the Iverson bracket with them.
    ///
    /// The brackets are written with `\left` and `\right`, so that they grow to fit a value that is
    /// taller than one line, such as a fraction or a nested [`Option`].
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of `fmt_latex` for `T`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::latex::ToLatex;
    ///
    /// assert_eq!(None::<u8>.to_latex().to_string(), r"\bot");
    /// assert_eq!(Some(5u8).to_latex().to_string(), r"\left[5\right]");
    /// assert_eq!(Some("hi").to_latex().to_string(), r"\left[\text{hi}\right]");
    ///
    /// // The brackets keep nested `Option`s apart.
    /// assert_eq!(
    ///     Some(None::<u8>).to_latex().to_string(),
    ///     r"\left[\bot\right]"
    /// );
    /// assert_eq!(
    ///     Some(Some(5u8)).to_latex().to_string(),
    ///     r"\left[\left[5\right]\right]"
    /// );
    /// ```
    ///
    /// | value              | fragment                      | renders as                    |
    /// |--------------------|-------------------------------|-------------------------------|
    /// | `None::<u8>`       | `\bot`                        | $\bot$                        |
    /// | `Some(5u8)`        | `\left[5\right]`              | $\left[5\right]$              |
    /// | `Some("hi")`       | `\left[\text{hi}\right]`      | $\left[\text{hi}\right]$      |
    /// | `Some(None::<u8>)` | `\left[\bot\right]`           | $\left[\bot\right]$           |
    /// | `Some(Some(5u8))`  | `\left[\left[5\right]\right]` | $\left[\left[5\right]\right]$ |
    #[inline]
    fn fmt_latex(&self, f: &mut Formatter) -> Result {
        match self {
            None => f.write_str("\\bot"),
            Some(x) => {
                f.write_str("\\left[")?;
                x.fmt_latex(f)?;
                f.write_str("\\right]")
            }
        }
    }
}
