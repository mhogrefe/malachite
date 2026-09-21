// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::vars::{VarScheme, char_is_reserved};
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{Formatter, Result};

/// A scheme that names variables with names the caller supplies.
///
/// The names are given in order, so that the first is variable 0, the second is variable 1, and so
/// on, and the scheme's [`capacity`](VarScheme::capacity) is how many there are. This is the scheme
/// to reach for when the variables mean something — `n` and `t`, say, or `price` and `quantity`
/// — and none of the alphabetical schemes says it.
///
/// The names are checked once, when the scheme is made, so that the scheme itself cannot fail:
/// [`new`](ListVars::new) panics on a list that would break the [`VarScheme`] contract.
///
/// A name of a single ASCII letter is written bare in LaTeX and Typst, so that both set it in math
/// italics, as a variable should be. A longer name is set upright, since that is what a name of
/// more than one letter should be, and is escaped for the language it is written in.
///
/// # Examples
/// ```
/// use malachite_base::strings::latex::ToLatex;
/// use malachite_base::strings::typst::ToTypst;
/// use malachite_base::vars::VarScheme;
/// use malachite_base::vars::list::ListVars;
///
/// let vars = ListVars::new(["t", "price"]);
/// assert_eq!(vars.capacity(), Some(2));
/// assert_eq!(vars.var(0).to_string(), "t");
/// assert_eq!(vars.var(1).to_string(), "price");
/// assert_eq!(vars.var(0).to_latex_string(), "t");
/// assert_eq!(vars.var(1).to_latex_string(), r"\text{price}");
/// assert_eq!(vars.var(1).to_typst_string(), "\"price\"");
/// assert_eq!(vars.parse_var("price"), Some(1));
/// assert_eq!(vars.parse_var("cost"), None);
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ListVars {
    names: Vec<String>,
}

impl ListVars {
    /// Makes a scheme that names variables with the given names, in order.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of names, and $m$ is the
    /// length of the longest one.
    ///
    /// # Panics
    /// Panics if any name is empty, if any name holds a character that
    /// [`char_is_reserved`](crate::vars::char_is_reserved) rejects, or if two names are the same.
    /// These are the three things the [`VarScheme`] contract asks of a scheme's names, and checking
    /// them here is what lets everything after this take them for granted.
    ///
    /// # Examples
    /// See [here](self).
    pub fn new<I: IntoIterator<Item = T>, T: Into<String>>(names: I) -> Self {
        let names: Vec<String> = names.into_iter().map(Into::into).collect();
        for (i, name) in names.iter().enumerate() {
            assert!(!name.is_empty(), "the name of variable {i} is empty");
            if let Some(c) = name.chars().find(|&c| char_is_reserved(c)) {
                panic!("the name {name:?} of variable {i} holds the reserved character {c:?}");
            }
            if let Some(j) = names[..i].iter().position(|other| other == name) {
                panic!("variables {j} and {i} have the same name, {name:?}");
            }
        }
        Self { names }
    }

    /// The names, in order.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::vars::list::ListVars;
    ///
    /// assert_eq!(ListVars::new(["t", "price"]).names(), &["t", "price"]);
    /// ```
    #[inline]
    pub fn names(&self) -> &[String] {
        &self.names
    }
}

impl VarScheme for ListVars {
    /// The number of variables the scheme can name, which is how many names it was given.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](self).
    #[inline]
    fn capacity(&self) -> Option<usize> {
        Some(self.names.len())
    }

    /// Writes a variable's plain name, which is the name the scheme was given for it.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the length of the name.
    ///
    /// # Panics
    /// Panics if `index` is not less than the number of names.
    ///
    /// # Examples
    /// See [here](self).
    #[inline]
    fn fmt_var(&self, index: usize, f: &mut Formatter) -> Result {
        f.write_str(&self.names[index])
    }

    /// Reads a variable's index from its plain name.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm)$
    ///
    /// $M(n, m) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of names, and $m$ is the
    /// length of the longest one.
    ///
    /// # Examples
    /// See [here](self).
    #[inline]
    fn parse_var(&self, name: &str) -> Option<usize> {
        self.names.iter().position(|other| other == name)
    }
}
