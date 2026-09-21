// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::foer_sequences::FoerSequence;
use crate::strings::typst::ToTypst;
use core::fmt::{Formatter, Result};

impl<T: ToTypst + Eq> ToTypst for FoerSequence<T> {
    /// Writes a [`FoerSequence`] as a Typst math-mode fragment.
    ///
    /// The elements' fragments are separated by commas and wrapped in square brackets, as a
    /// sequence's are, and the repeating part, if there is one, is written under a vinculum: the
    /// overline that marks a repeating decimal. A sequence with nothing but a repeating part is all
    /// vinculum, and one with none is an ordinary bracketed list.
    ///
    /// Typst grows a matched pair of delimiters to fit what is between them, so the brackets fit an
    /// element that is taller than one line without being asked to.
    ///
    /// Inside the vinculum the elements are separated by Typst's `comma` symbol rather than by a
    /// literal comma. `overline` takes a single body, and a literal comma there would be read as
    /// the separator between two arguments and rejected; `comma` renders the same.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n + \sum_{i=0}^{n-1}T^\prime(i))$
    ///
    /// $M(n) = O(\max_{i=0}^{n-1}M^\prime(i))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.component_len()`, $i$ is an
    /// element's index, and $T^\prime$ and $M^\prime$ are the time and memory functions of
    /// `fmt_typst` for `T`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::foer_sequences::FoerSequence;
    /// use malachite_base::strings::typst::ToTypst;
    ///
    /// let empty = FoerSequence::<u8>::from_vecs(vec![], vec![]);
    /// assert_eq!(empty.to_typst().to_string(), "[]");
    ///
    /// let finite = FoerSequence::<u8>::from_vecs(vec![1, 2], vec![]);
    /// assert_eq!(finite.to_typst().to_string(), "[1, 2]");
    ///
    /// let repeating = FoerSequence::<u8>::from_vecs(vec![], vec![3, 4]);
    /// assert_eq!(repeating.to_typst().to_string(), "[overline(3 comma 4)]");
    ///
    /// let both = FoerSequence::<u8>::from_vecs(vec![1, 2], vec![3, 4]);
    /// assert_eq!(both.to_typst().to_string(), "[1, 2, overline(3 comma 4)]");
    /// ```
    ///
    /// | value            | fragment                      |
    /// |------------------|-------------------------------|
    /// | `[]`             | `[]`                          |
    /// | `[1, 2]`         | `[1, 2]`                      |
    /// | `[[3, 4]]`       | `[overline(3 comma 4)]`       |
    /// | `[1, 2, [3, 4]]` | `[1, 2, overline(3 comma 4)]` |
    fn fmt_typst(&self, f: &mut Formatter) -> Result {
        f.write_str("[")?;
        for (i, x) in self.non_repeating.iter().enumerate() {
            if i != 0 {
                f.write_str(", ")?;
            }
            x.fmt_typst(f)?;
        }
        if !self.repeating.is_empty() {
            if !self.non_repeating.is_empty() {
                f.write_str(", ")?;
            }
            f.write_str("overline(")?;
            for (i, x) in self.repeating.iter().enumerate() {
                if i != 0 {
                    f.write_str(" comma ")?;
                }
                x.fmt_typst(f)?;
            }
            f.write_str(")")?;
        }
        f.write_str("]")
    }
}
