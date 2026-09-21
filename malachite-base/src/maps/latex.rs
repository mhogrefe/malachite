// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::strings::latex::ToLatex;
#[cfg(not(feature = "std"))]
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::fmt::{Formatter, Result};
use core::hash::Hash;
#[cfg(not(feature = "std"))]
use hashbrown::HashMap;
#[cfg(feature = "std")]
use std::collections::{BTreeMap, HashMap};

// Writes a sequence of entries as one braced, comma-separated LaTeX math-mode fragment, each
// entry's key and value joined by a "maps to" arrow.
fn fmt_latex_entries<'a, K: ToLatex + 'a, V: ToLatex + 'a>(
    entries: impl Iterator<Item = (&'a K, &'a V)>,
    f: &mut Formatter,
) -> Result {
    f.write_str("\\left\\{")?;
    for (i, (k, v)) in entries.enumerate() {
        if i != 0 {
            f.write_str(", ")?;
        }
        k.fmt_latex(f)?;
        f.write_str(" \\mapsto ")?;
        v.fmt_latex(f)?;
    }
    f.write_str("\\right\\}")
}

impl<K: ToLatex, V: ToLatex> ToLatex for BTreeMap<K, V> {
    /// Writes a [`BTreeMap`] as a LaTeX math-mode fragment.
    ///
    /// Each entry is written as its key, a "maps to" arrow, and its value; the entries are
    /// separated by commas and wrapped in braces, as a map is a set of associations. The entries
    /// come in the map's own order, which is ascending by key.
    ///
    /// The braces are written with `\left` and `\right`, so that they grow to fit an entry that is
    /// taller than one line. An empty map becomes `\left\{\right\}` rather than nothing at all.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n + \sum_{i=0}^{n-1}(T^\prime(i) + T^{\prime\prime}(i)))$
    ///
    /// $M(n) = O(\max_{i=0}^{n-1}(M^\prime(i) + M^{\prime\prime}(i)))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $i$ is an entry's index,
    /// $T^\prime$ and $M^\prime$ are the time and memory functions of `fmt_latex` for `K`, and
    /// $T^{\prime\prime}$ and $M^{\prime\prime}$ are those for `V`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::latex::ToLatex;
    /// use std::collections::BTreeMap;
    ///
    /// let empty = BTreeMap::<u8, u8>::new();
    /// assert_eq!(empty.to_latex().to_string(), r"\left\{\right\}");
    ///
    /// let m = BTreeMap::from([(2u8, 20u8), (1, 10)]);
    /// assert_eq!(
    ///     m.to_latex().to_string(),
    ///     r"\left\{1 \mapsto 10, 2 \mapsto 20\right\}"
    /// );
    /// ```
    ///
    /// | value                                    | fragment                                    | renders as                                    |
    /// |------------------------------------------|---------------------------------------------|-----------------------------------------------|
    /// | `BTreeMap::<u8, u8>::new()`              | `\left\{\right\}`                           | $\left\\{\right\\}$                           |
    /// | `BTreeMap::from([(2u8, 20u8), (1, 10)])` | `\left\{1 \mapsto 10, 2 \mapsto 20\right\}` | $\left\\{1 \mapsto 10, 2 \mapsto 20\right\\}$ |
    #[cfg_attr(dylint_lib = "malachite_lints", expect(long_lines))]
    #[inline]
    fn fmt_latex(&self, f: &mut Formatter) -> Result {
        fmt_latex_entries(self.iter(), f)
    }
}

impl<K: Eq + Hash + Ord + ToLatex, V: ToLatex> ToLatex for HashMap<K, V> {
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
    /// The braces are written with `\left` and `\right`, so that they grow to fit an entry that is
    /// taller than one line. An empty map becomes `\left\{\right\}` rather than nothing at all.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n + \sum_{i=0}^{n-1}(T^\prime(i) + T^{\prime\prime}(i)))$
    ///
    /// $M(n) = O(n + \max_{i=0}^{n-1}(M^\prime(i) + M^{\prime\prime}(i)))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $i$ is an entry's index,
    /// $T^\prime$ and $M^\prime$ are the time and memory functions of `fmt_latex` for `K`, and
    /// $T^{\prime\prime}$ and $M^{\prime\prime}$ are those for `V`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::latex::ToLatex;
    /// use std::collections::HashMap;
    ///
    /// let empty = HashMap::<u8, u8>::new();
    /// assert_eq!(empty.to_latex().to_string(), r"\left\{\right\}");
    ///
    /// // The entries are sorted by key, so the fragment does not depend on the hasher.
    /// let m = HashMap::from([(2u8, 20u8), (1, 10)]);
    /// assert_eq!(
    ///     m.to_latex().to_string(),
    ///     r"\left\{1 \mapsto 10, 2 \mapsto 20\right\}"
    /// );
    /// ```
    ///
    /// | value                                   | fragment                                    | renders as                                    |
    /// |-----------------------------------------|---------------------------------------------|-----------------------------------------------|
    /// | `HashMap::<u8, u8>::new()`              | `\left\{\right\}`                           | $\left\\{\right\\}$                           |
    /// | `HashMap::from([(2u8, 20u8), (1, 10)])` | `\left\{1 \mapsto 10, 2 \mapsto 20\right\}` | $\left\\{1 \mapsto 10, 2 \mapsto 20\right\\}$ |
    #[cfg_attr(dylint_lib = "malachite_lints", expect(long_lines))]
    fn fmt_latex(&self, f: &mut Formatter) -> Result {
        let mut entries = self.iter().collect::<Vec<_>>();
        entries.sort_unstable_by_key(|&(k, _)| k);
        fmt_latex_entries(entries.into_iter(), f)
    }
}
