// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::slices::latex::fmt_latex_slice;
use crate::strings::latex::ToLatex;
use alloc::vec::Vec;
use core::fmt::{Formatter, Result};

impl<T: ToLatex> ToLatex for Vec<T> {
    /// Writes a [`Vec`] as a LaTeX math-mode fragment.
    ///
    /// This is the same as the slice implementation.
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
    /// assert_eq!(Vec::<u8>::new().to_latex().to_string(), r"\left[\right]");
    /// assert_eq!(vec![5u8].to_latex().to_string(), r"\left[5\right]");
    /// assert_eq!(
    ///     vec![1u8, 2, 3].to_latex().to_string(),
    ///     r"\left[1, 2, 3\right]"
    /// );
    /// assert_eq!(
    ///     vec![vec![1u8], vec![2, 3]].to_latex().to_string(),
    ///     r"\left[\left[1\right], \left[2, 3\right]\right]"
    /// );
    /// ```
    ///
    /// | value                         | fragment                                         | renders as                                       |
    /// |-------------------------------|--------------------------------------------------|--------------------------------------------------|
    /// | `Vec::<u8>::new()`            | `\left[\right]`                                  | $\left[\right]$                                  |
    /// | `vec![5u8]`                   | `\left[5\right]`                                 | $\left[5\right]$                                 |
    /// | `vec![1u8, 2, 3]`             | `\left[1, 2, 3\right]`                           | $\left[1, 2, 3\right]$                           |
    /// | `vec![vec![1u8], vec![2, 3]]` | `\left[\left[1\right], \left[2, 3\right]\right]` | $\left[\left[1\right], \left[2, 3\right]\right]$ |
    #[cfg_attr(dylint_lib = "malachite_lints", expect(long_lines))]
    #[inline]
    fn fmt_latex(&self, f: &mut Formatter) -> Result {
        fmt_latex_slice(self, f)
    }
}
