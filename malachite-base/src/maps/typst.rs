// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::strings::typst::ToTypst;
#[cfg(not(feature = "std"))]
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::fmt::{Formatter, Result};
use core::hash::Hash;
#[cfg(not(feature = "std"))]
use hashbrown::HashMap;
#[cfg(feature = "std")]
use std::collections::{BTreeMap, HashMap};

// Writes a sequence of entries as one braced, comma-separated Typst math-mode fragment, each
// entry's key and value joined by a "maps to" arrow.
fn fmt_typst_entries<'a, K: ToTypst + 'a, V: ToTypst + 'a>(
    entries: impl Iterator<Item = (&'a K, &'a V)>,
    f: &mut Formatter,
) -> Result {
    f.write_str("{")?;
    for (i, (k, v)) in entries.enumerate() {
        if i != 0 {
            f.write_str(", ")?;
        }
        k.fmt_typst(f)?;
        f.write_str(" |-> ")?;
        v.fmt_typst(f)?;
    }
    f.write_str("}")
}

impl<K: ToTypst, V: ToTypst> ToTypst for BTreeMap<K, V> {
    /// Writes a [`BTreeMap`] as a LaTeX math-mode fragment.
    ///
    /// Each entry is written as its key, a "maps to" arrow, and its value; the entries are
    /// separated by commas and wrapped in braces, as a map is a set of associations. The entries
    /// come in the map's own order, which is ascending by key.
    ///
    /// Typst grows a matched pair of delimiters to fit what is between them, so the braces fit an
    /// entry that is taller than one line without being asked to. An empty map becomes `{}` rather
    /// than nothing at all.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n + \sum_{i=0}^{n-1}(T^\prime(i) + T^{\prime\prime}(i)))$
    ///
    /// $M(n) = O(\max_{i=0}^{n-1}(M^\prime(i) + M^{\prime\prime}(i)))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $i$ is an entry's index,
    /// $T^\prime$ and $M^\prime$ are the time and memory functions of `fmt_typst` for `K`, and
    /// $T^{\prime\prime}$ and $M^{\prime\prime}$ are those for `V`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::typst::ToTypst;
    /// use std::collections::BTreeMap;
    ///
    /// let empty = BTreeMap::<u8, u8>::new();
    /// assert_eq!(empty.to_typst().to_string(), "{}");
    ///
    /// let m = BTreeMap::from([(2u8, 20u8), (1, 10)]);
    /// assert_eq!(m.to_typst().to_string(), "{1 |-> 10, 2 |-> 20}");
    /// ```
    ///
    /// | value                                    | fragment                 |
    /// |------------------------------------------|--------------------------|
    /// | `BTreeMap::<u8, u8>::new()`              | `{}`                     |
    /// | `BTreeMap::from([(2u8, 20u8), (1, 10)])` | `{1 \|-> 10, 2 \|-> 20}` |
    #[inline]
    fn fmt_typst(&self, f: &mut Formatter) -> Result {
        fmt_typst_entries(self.iter(), f)
    }
}

impl<K: Eq + Hash + Ord + ToTypst, V: ToTypst> ToTypst for HashMap<K, V> {
    /// Writes a [`HashMap`] as a LaTeX math-mode fragment.
    ///
    /// Each entry is written as its key, a "maps to" arrow, and its value; the entries are
    /// separated by commas and wrapped in braces, as a map is a set of associations.
    ///
    /// The entries are sorted by key first, which is why this asks for [`Ord`] where a [`HashMap`]
    /// does not. A [`HashMap`] iterates in an order that depends on its hasher, so without sorting
    /// two equal maps could have different fragments, and the same map could have a different
    /// fragment in the next run. Sorting also makes a [`HashMap`]'s fragment agree with the
    /// [`BTreeMap`] of the same entries.
    ///
    /// Typst grows a matched pair of delimiters to fit what is between them, so the braces fit an
    /// entry that is taller than one line without being asked to. An empty map becomes `{}` rather
    /// than nothing at all.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n + \sum_{i=0}^{n-1}(T^\prime(i) + T^{\prime\prime}(i)))$
    ///
    /// $M(n) = O(n + \max_{i=0}^{n-1}(M^\prime(i) + M^{\prime\prime}(i)))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $i$ is an entry's index,
    /// $T^\prime$ and $M^\prime$ are the time and memory functions of `fmt_typst` for `K`, and
    /// $T^{\prime\prime}$ and $M^{\prime\prime}$ are those for `V`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::typst::ToTypst;
    /// use std::collections::HashMap;
    ///
    /// let empty = HashMap::<u8, u8>::new();
    /// assert_eq!(empty.to_typst().to_string(), "{}");
    ///
    /// // The entries are sorted by key, so the fragment does not depend on the hasher.
    /// let m = HashMap::from([(2u8, 20u8), (1, 10)]);
    /// assert_eq!(m.to_typst().to_string(), "{1 |-> 10, 2 |-> 20}");
    /// ```
    ///
    /// | value                                   | fragment                 |
    /// |-----------------------------------------|--------------------------|
    /// | `HashMap::<u8, u8>::new()`              | `{}`                     |
    /// | `HashMap::from([(2u8, 20u8), (1, 10)])` | `{1 \|-> 10, 2 \|-> 20}` |
    fn fmt_typst(&self, f: &mut Formatter) -> Result {
        let mut entries = self.iter().collect::<Vec<_>>();
        entries.sort_unstable_by_key(|&(k, _)| k);
        fmt_typst_entries(entries.into_iter(), f)
    }
}
