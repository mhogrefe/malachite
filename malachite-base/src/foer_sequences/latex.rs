// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::foer_sequences::FoerSequence;
use crate::strings::latex::ToLatex;
use core::fmt::{Formatter, Result};

impl<T: ToLatex + Eq> ToLatex for FoerSequence<T> {
    /// Writes a [`FoerSequence`] as a LaTeX math-mode fragment.
    ///
    /// The elements' fragments are separated by commas and wrapped in square brackets, as a
    /// sequence's are, and the repeating part, if there is one, is written under a vinculum: the
    /// overline that marks a repeating decimal. A sequence with nothing but a repeating part is all
    /// vinculum, and one with none is an ordinary bracketed list.
    ///
    /// The brackets are written with `\left` and `\right`, so that they grow to fit an element that
    /// is taller than one line.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n + \sum_{i=0}^{n-1}T^\prime(i))$
    ///
    /// $M(n) = O(\max_{i=0}^{n-1}M^\prime(i))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.component_len()`, $i$ is an
    /// element's index, and $T^\prime$ and $M^\prime$ are the time and memory functions of
    /// `fmt_latex` for `T`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::foer_sequences::FoerSequence;
    /// use malachite_base::strings::latex::ToLatex;
    ///
    /// let empty = FoerSequence::<u8>::from_vecs(vec![], vec![]);
    /// assert_eq!(empty.to_latex_string(), r"\left[\right]");
    ///
    /// let finite = FoerSequence::<u8>::from_vecs(vec![1, 2], vec![]);
    /// assert_eq!(finite.to_latex_string(), r"\left[1, 2\right]");
    ///
    /// let repeating = FoerSequence::<u8>::from_vecs(vec![], vec![3, 4]);
    /// assert_eq!(repeating.to_latex_string(), r"\left[\overline{3, 4}\right]");
    ///
    /// let both = FoerSequence::<u8>::from_vecs(vec![1, 2], vec![3, 4]);
    /// assert_eq!(
    ///     both.to_latex_string(),
    ///     r"\left[1, 2, \overline{3, 4}\right]"
    /// );
    /// ```
    ///
    /// | value            | fragment                             | renders as                           |
    /// |------------------|--------------------------------------|--------------------------------------|
    /// | `[]`             | `\left[\right]`                      | $\left[\right]$                      |
    /// | `[1, 2]`         | `\left[1, 2\right]`                  | $\left[1, 2\right]$                  |
    /// | `[[3, 4]]`       | `\left[\overline{3, 4}\right]`       | $\left[\overline{3, 4}\right]$       |
    /// | `[1, 2, [3, 4]]` | `\left[1, 2, \overline{3, 4}\right]` | $\left[1, 2, \overline{3, 4}\right]$ |
    #[cfg_attr(dylint_lib = "malachite_lints", expect(long_lines))]
    fn fmt_latex(&self, f: &mut Formatter) -> Result {
        f.write_str("\\left[")?;
        for (i, x) in self.non_repeating.iter().enumerate() {
            if i != 0 {
                f.write_str(", ")?;
            }
            x.fmt_latex(f)?;
        }
        if !self.repeating.is_empty() {
            if !self.non_repeating.is_empty() {
                f.write_str(", ")?;
            }
            f.write_str("\\overline{")?;
            for (i, x) in self.repeating.iter().enumerate() {
                if i != 0 {
                    f.write_str(", ")?;
                }
                x.fmt_latex(f)?;
            }
            f.write_str("}")?;
        }
        f.write_str("\\right]")
    }
}
