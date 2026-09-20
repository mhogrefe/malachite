// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::fmt::{Display, Formatter, Result};

/// Converts a value to a LaTeX math-mode fragment.
///
/// The output is a fragment rather than a complete expression: it carries no `$`, `\(`, `\[`, or
/// environment of its own, leaving those to the caller. That is what lets one fragment be embedded
/// in another, so that a value built out of smaller values can write its parts directly.
///
/// Every implementation guarantees that its output
/// - is valid wherever LaTeX is in math mode, and remains valid when wrapped in a group, so that
///   `{`output`}` is well-formed;
/// - leaves no LaTeX state behind: braces are balanced, and nothing is defined or redefined.
pub trait ToLatex {
    /// Writes a value as a LaTeX math-mode fragment.
    ///
    /// This is the method implementors define. It takes a [`Formatter`] rather than returning a
    /// `String` so that a value can write its parts into a caller's buffer, which is what makes a
    /// fragment embeddable without an allocation per level of nesting.
    ///
    /// # Examples
    /// See [here](super::latex#fmt_latex).
    fn fmt_latex(&self, f: &mut Formatter) -> Result;

    /// Converts a value to a LaTeX math-mode fragment.
    ///
    /// The returned [`LatexWrapper`] implements [`Display`], so it can be converted to a `String`
    /// with `to_string`, or written directly with `write!` and friends.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::latex#to_latex).
    #[inline]
    fn to_latex(&self) -> LatexWrapper<'_, Self>
    where
        Self: Sized,
    {
        LatexWrapper { x: self }
    }
}

/// A `struct` that can be used to format a value as a LaTeX math-mode fragment.
///
/// It is returned by [`ToLatex::to_latex`].
pub struct LatexWrapper<'a, T: ToLatex> {
    pub(crate) x: &'a T,
}

impl<T: ToLatex> Display for LatexWrapper<'_, T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.x.fmt_latex(f)
    }
}
