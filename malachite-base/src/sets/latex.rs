// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::slices::latex::fmt_latex_sequence;
use crate::strings::latex::ToLatex;
#[cfg(not(feature = "std"))]
use alloc::collections::BTreeSet;
use alloc::vec::Vec;
use core::fmt::{Formatter, Result};
use core::hash::Hash;
#[cfg(not(feature = "std"))]
use hashbrown::HashSet;
#[cfg(feature = "std")]
use std::collections::{BTreeSet, HashSet};

impl<T: ToLatex> ToLatex for BTreeSet<T> {
    /// Writes a [`BTreeSet`] as a LaTeX math-mode fragment.
    ///
    /// The elements' fragments are separated by commas and wrapped in braces, as a set is written
    /// in mathematics. The elements come in the set's own order, which is ascending.
    ///
    /// The braces are written with `\left` and `\right`, so that they grow to fit an element that
    /// is taller than one line. An empty set becomes `\left\{\right\}` rather than nothing at all.
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
    /// use std::collections::BTreeSet;
    ///
    /// let empty = BTreeSet::<u8>::new();
    /// assert_eq!(empty.to_latex_string(), r"\left\{\right\}");
    ///
    /// let xs = BTreeSet::from([3u8, 1, 2]);
    /// assert_eq!(xs.to_latex_string(), r"\left\{1, 2, 3\right\}");
    /// ```
    ///
    /// | value                         | fragment                 | renders as                 |
    /// |-------------------------------|--------------------------|----------------------------|
    /// | `BTreeSet::<u8>::new()`       | `\left\{\right\}`        | $\left\\{\right\\}$        |
    /// | `BTreeSet::from([3u8, 1, 2])` | `\left\{1, 2, 3\right\}` | $\left\\{1, 2, 3\right\\}$ |
    #[inline]
    fn fmt_latex(&self, f: &mut Formatter) -> Result {
        fmt_latex_sequence(self.iter(), "\\left\\{", "\\right\\}", f)
    }
}

impl<T: Eq + Hash + Ord + ToLatex> ToLatex for HashSet<T> {
    /// Writes a [`HashSet`] as a LaTeX math-mode fragment.
    ///
    /// The elements' fragments are separated by commas and wrapped in braces, as a set is written
    /// in mathematics.
    ///
    /// The elements are sorted first, which is why this asks for [`Ord`] where a [`HashSet`] does
    /// not. A [`HashSet`] iterates in an order that depends on its hasher, so without sorting two
    /// equal sets could have different fragments, and the same set could have a different fragment
    /// in the next run. Sorting also makes a [`HashSet`]'s fragment agree with the [`BTreeSet`] of
    /// the same elements.
    ///
    /// The braces are written with `\left` and `\right`, so that they grow to fit an element that
    /// is taller than one line. An empty set becomes `\left\{\right\}` rather than nothing at all.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n + \sum_{i=0}^{n-1}T^\prime(i))$
    ///
    /// $M(n) = O(n + \max_{i=0}^{n-1}M^\prime(i))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $i$ is an element's index,
    /// and $T^\prime$ and $M^\prime$ are the time and memory functions of `fmt_latex` for `T`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::latex::ToLatex;
    /// use std::collections::HashSet;
    ///
    /// let empty = HashSet::<u8>::new();
    /// assert_eq!(empty.to_latex_string(), r"\left\{\right\}");
    ///
    /// // The elements are sorted, so the fragment does not depend on the hasher.
    /// let xs = HashSet::from([3u8, 1, 2]);
    /// assert_eq!(xs.to_latex_string(), r"\left\{1, 2, 3\right\}");
    /// ```
    ///
    /// | value                        | fragment                 | renders as                 |
    /// |------------------------------|--------------------------|----------------------------|
    /// | `HashSet::<u8>::new()`       | `\left\{\right\}`        | $\left\\{\right\\}$        |
    /// | `HashSet::from([3u8, 1, 2])` | `\left\{1, 2, 3\right\}` | $\left\\{1, 2, 3\right\\}$ |
    fn fmt_latex(&self, f: &mut Formatter) -> Result {
        let mut xs = self.iter().collect::<Vec<_>>();
        xs.sort_unstable();
        fmt_latex_sequence(xs.into_iter(), "\\left\\{", "\\right\\}", f)
    }
}
