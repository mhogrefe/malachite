// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::strings::latex::ToLatex;
use core::fmt::{Formatter, Result};

// Writes a sequence of values as one delimited, comma-separated LaTeX math-mode fragment.
pub(crate) fn fmt_latex_sequence<'a, T: ToLatex + 'a>(
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
        x.fmt_latex(f)?;
    }
    f.write_str(close)
}

// Writes a sequence of values as one bracketed, comma-separated LaTeX math-mode fragment.
pub(crate) fn fmt_latex_slice<T: ToLatex>(xs: &[T], f: &mut Formatter) -> Result {
    fmt_latex_sequence(xs.iter(), "\\left[", "\\right]", f)
}

impl<T: ToLatex> ToLatex for &[T] {
    /// Writes a slice as a LaTeX math-mode fragment.
    ///
    /// The elements' fragments are separated by commas and wrapped in square brackets, so that a
    /// slice's fragment is built out of its elements' own.
    ///
    /// The brackets are not decoration: without them `[1, 2]` and `[[1], [2]]` would both be `1,
    /// 2`, and distinct values would have the same fragment. They are written with `\left` and
    /// `\right`, so that they grow to fit an element that is taller than one line, such as a
    /// fraction or a nested slice.
    ///
    /// An empty slice becomes `\left[\right]` rather than nothing at all.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n + \sum_{i=0}^{n-1}T^\prime(i))$
    ///
    /// $M(n) = O(\max_{i=0}^{n-1}M^\prime(i))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $i$ is an element's index,
    /// and $T^\prime$ and $M^\prime$ are the time and memory functions of `fmt_latex` for `T`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::latex::ToLatex;
    ///
    /// assert_eq!([0u8; 0].as_slice().to_latex().to_string(), r"\left[\right]");
    /// assert_eq!([5u8].as_slice().to_latex().to_string(), r"\left[5\right]");
    /// assert_eq!(
    ///     [1u8, 2, 3].as_slice().to_latex().to_string(),
    ///     r"\left[1, 2, 3\right]"
    /// );
    /// assert_eq!(
    ///     ["a", "b"].as_slice().to_latex().to_string(),
    ///     r"\left[\text{a}, \text{b}\right]"
    /// );
    /// ```
    ///
    /// | value         | fragment                          | renders as                        |
    /// |---------------|-----------------------------------|-----------------------------------|
    /// | `[0u8; 0]`    | `\left[\right]`                   | $\left[\right]$                   |
    /// | `[5u8]`       | `\left[5\right]`                  | $\left[5\right]$                  |
    /// | `[1u8, 2, 3]` | `\left[1, 2, 3\right]`            | $\left[1, 2, 3\right]$            |
    /// | `["a", "b"]`  | `\left[\text{a}, \text{b}\right]` | $\left[\text{a}, \text{b}\right]$ |
    #[inline]
    fn fmt_latex(&self, f: &mut Formatter) -> Result {
        fmt_latex_slice(self, f)
    }
}

impl<T: ToLatex, const N: usize> ToLatex for [T; N] {
    /// Writes an array as a LaTeX math-mode fragment.
    ///
    /// This is the same as the slice implementation.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n + \sum_{i=0}^{n-1}T^\prime(i))$
    ///
    /// $M(n) = O(\max_{i=0}^{n-1}M^\prime(i))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `N`, $i$ is an element's index, and
    /// $T^\prime$ and $M^\prime$ are the time and memory functions of `fmt_latex` for `T`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::latex::ToLatex;
    ///
    /// assert_eq!([0u8; 0].to_latex().to_string(), r"\left[\right]");
    /// assert_eq!([5u8].to_latex().to_string(), r"\left[5\right]");
    /// assert_eq!([1u8, 2, 3].to_latex().to_string(), r"\left[1, 2, 3\right]");
    /// assert_eq!(
    ///     [[1u8, 2], [3, 4]].to_latex().to_string(),
    ///     r"\left[\left[1, 2\right], \left[3, 4\right]\right]"
    /// );
    /// ```
    ///
    /// | value                | fragment                                            | renders as                                          |
    /// |----------------------|-----------------------------------------------------|-----------------------------------------------------|
    /// | `[0u8; 0]`           | `\left[\right]`                                     | $\left[\right]$                                     |
    /// | `[5u8]`              | `\left[5\right]`                                    | $\left[5\right]$                                    |
    /// | `[1u8, 2, 3]`        | `\left[1, 2, 3\right]`                              | $\left[1, 2, 3\right]$                              |
    /// | `[[1u8, 2], [3, 4]]` | `\left[\left[1, 2\right], \left[3, 4\right]\right]` | $\left[\left[1, 2\right], \left[3, 4\right]\right]$ |
    #[cfg_attr(dylint_lib = "malachite_lints", expect(long_lines))]
    #[inline]
    fn fmt_latex(&self, f: &mut Formatter) -> Result {
        fmt_latex_slice(self, f)
    }
}
