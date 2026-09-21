// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::slices::typst::fmt_typst_slice;
use crate::strings::typst::ToTypst;
use alloc::vec::Vec;
use core::fmt::{Formatter, Result};

impl<T: ToTypst> ToTypst for Vec<T> {
    /// Writes a [`Vec`] as a Typst math-mode fragment.
    ///
    /// This is the same as the slice implementation.
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
    /// assert_eq!(Vec::<u8>::new().to_typst().to_string(), "[]");
    /// assert_eq!(vec![5u8].to_typst().to_string(), "[5]");
    /// assert_eq!(vec![1u8, 2, 3].to_typst().to_string(), "[1, 2, 3]");
    /// assert_eq!(
    ///     vec![vec![1u8], vec![2, 3]].to_typst().to_string(),
    ///     "[[1], [2, 3]]"
    /// );
    /// ```
    ///
    /// | value                         | fragment        |
    /// |-------------------------------|-----------------|
    /// | `Vec::<u8>::new()`            | `[]`            |
    /// | `vec![5u8]`                   | `[5]`           |
    /// | `vec![1u8, 2, 3]`             | `[1, 2, 3]`     |
    /// | `vec![vec![1u8], vec![2, 3]]` | `[[1], [2, 3]]` |
    #[inline]
    fn fmt_typst(&self, f: &mut Formatter) -> Result {
        fmt_typst_slice(self, f)
    }
}
