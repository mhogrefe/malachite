// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::chars::latex::fmt_latex_chars;
use alloc::string::String;
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

impl ToLatex for &str {
    /// Writes a string slice as a LaTeX math-mode fragment.
    ///
    /// The fragment depicts the string: ordinary characters are gathered into `\text{...}` groups
    /// and typeset as themselves, with LaTeX's special characters escaped, while characters that
    /// LaTeX spells with a math-mode macro are written as that macro, outside any group. A string
    /// mixing the two therefore comes out as, for example, `\text{100\% }\alpha`. No quotation
    /// marks are added; the fragment is the string's content and nothing else.
    ///
    /// The empty string becomes `\text{}` rather than nothing at all.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.chars().count()`.
    ///
    /// # Examples
    /// See [here](super::latex#fmt_latex).
    #[inline]
    fn fmt_latex(&self, f: &mut Formatter) -> Result {
        fmt_latex_chars(self.chars(), f)
    }
}

impl ToLatex for String {
    /// Writes a [`String`] as a LaTeX math-mode fragment.
    ///
    /// This is identical to the [`&str`] implementation.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.chars().count()`.
    ///
    /// # Examples
    /// See [here](super::latex#fmt_latex).
    #[inline]
    fn fmt_latex(&self, f: &mut Formatter) -> Result {
        fmt_latex_chars(self.chars(), f)
    }
}
