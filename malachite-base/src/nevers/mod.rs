// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::strings::latex::ToLatex;
use crate::strings::typst::ToTypst;
use core::fmt::Formatter;
use core::iter::{Empty, empty};

impl ToLatex for ! {
    /// Would write a [`!`](prim@never) as a LaTeX math-mode fragment.
    ///
    /// A [`!`](prim@never) cannot be instantiated, so this can never be called. The implementation
    /// exists so that a type parameter bounded by [`ToLatex`] may be [`!`](prim@never), which is
    /// what lets `Option<!>` have a fragment of its own.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::latex::ToLatex;
    ///
    /// // The only `Option<!>` there is.
    /// assert_eq!(None::<!>.to_latex_string(), r"\bot");
    /// ```
    fn fmt_latex(&self, _f: &mut Formatter) -> core::fmt::Result {
        match *self {}
    }
}

impl ToTypst for ! {
    /// Would write a [`!`](prim@never) as a Typst math-mode fragment.
    ///
    /// A [`!`](prim@never) cannot be instantiated, so this can never be called. The implementation
    /// exists so that a type parameter bounded by [`ToTypst`] may be [`!`](prim@never), which is
    /// what lets `Option<!>` have a fragment of its own.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::typst::ToTypst;
    ///
    /// // The only `Option<!>` there is.
    /// assert_eq!(None::<!>.to_typst_string(), "bot");
    /// ```
    fn fmt_typst(&self, _f: &mut Formatter) -> core::fmt::Result {
        match *self {}
    }
}

/// Generates all (none) of the values of [`!`](prim@never), the never type.
///
/// The output length is 0.
///
/// # Worst-case complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::nevers::nevers;
///
/// assert_eq!(nevers().collect_vec(), &[]);
/// ```
#[inline]
pub const fn nevers() -> Empty<!> {
    empty()
}
