// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::slices::typst::fmt_typst_sequence;
use crate::strings::typst::ToTypst;
#[cfg(not(feature = "std"))]
use alloc::collections::BTreeSet;
use alloc::vec::Vec;
use core::fmt::{Formatter, Result};
use core::hash::Hash;
#[cfg(not(feature = "std"))]
use hashbrown::HashSet;
#[cfg(feature = "std")]
use std::collections::{BTreeSet, HashSet};

impl<T: ToTypst> ToTypst for BTreeSet<T> {
    /// Writes a [`BTreeSet`] as a LaTeX math-mode fragment.
    ///
    /// The elements' fragments are separated by commas and wrapped in braces, as a set is written
    /// in mathematics. The elements come in the set's own order, which is ascending.
    ///
    /// Typst grows a matched pair of delimiters to fit what is between them, so the braces fit an
    /// element that is taller than one line without being asked to. An empty set becomes `{}`
    /// rather than nothing at all.
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
    /// use std::collections::BTreeSet;
    ///
    /// let empty = BTreeSet::<u8>::new();
    /// assert_eq!(empty.to_typst_string(), "{}");
    ///
    /// let xs = BTreeSet::from([3u8, 1, 2]);
    /// assert_eq!(xs.to_typst_string(), "{1, 2, 3}");
    /// ```
    ///
    /// | value                         | fragment    |
    /// |-------------------------------|-------------|
    /// | `BTreeSet::<u8>::new()`       | `{}`        |
    /// | `BTreeSet::from([3u8, 1, 2])` | `{1, 2, 3}` |
    #[inline]
    fn fmt_typst(&self, f: &mut Formatter) -> Result {
        fmt_typst_sequence(self.iter(), "{", "}", f)
    }
}

impl<T: Eq + Hash + Ord + ToTypst> ToTypst for HashSet<T> {
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
    /// Typst grows a matched pair of delimiters to fit what is between them, so the braces fit an
    /// element that is taller than one line without being asked to. An empty set becomes `{}`
    /// rather than nothing at all.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n + \sum_{i=0}^{n-1}T^\prime(i))$
    ///
    /// $M(n) = O(n + \max_{i=0}^{n-1}M^\prime(i))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $i$ is an element's index,
    /// and $T^\prime$ and $M^\prime$ are the time and memory functions of `fmt_typst` for `T`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::typst::ToTypst;
    /// use std::collections::HashSet;
    ///
    /// let empty = HashSet::<u8>::new();
    /// assert_eq!(empty.to_typst_string(), "{}");
    ///
    /// // The elements are sorted, so the fragment does not depend on the hasher.
    /// let xs = HashSet::from([3u8, 1, 2]);
    /// assert_eq!(xs.to_typst_string(), "{1, 2, 3}");
    /// ```
    ///
    /// | value                        | fragment    |
    /// |------------------------------|-------------|
    /// | `HashSet::<u8>::new()`       | `{}`        |
    /// | `HashSet::from([3u8, 1, 2])` | `{1, 2, 3}` |
    fn fmt_typst(&self, f: &mut Formatter) -> Result {
        let mut xs = self.iter().collect::<Vec<_>>();
        xs.sort_unstable();
        fmt_typst_sequence(xs.into_iter(), "{", "}", f)
    }
}
