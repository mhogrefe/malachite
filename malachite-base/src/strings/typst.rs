// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::chars::typst::fmt_typst_chars;
use alloc::string::String;
use core::fmt::{Display, Formatter, Result};

/// Converts a value to a Typst math-mode fragment.
///
/// The output is a fragment rather than a complete expression: it carries no `$` of its own,
/// leaving that to the caller. That is what lets one fragment be embedded in another, so that a
/// value built out of smaller values can write its parts directly.
///
/// Every implementation guarantees that its output
/// - is valid wherever Typst is in math mode, and remains valid when wrapped in parentheses, so
///   that `(`output`)` is well-formed;
/// - leaves nothing open behind it: delimiters and string literals are closed, and nothing is
///   defined or redefined.
pub trait ToTypst {
    /// Writes a value as a Typst math-mode fragment.
    ///
    /// This is the method implementors define. It takes a [`Formatter`] rather than returning a
    /// `String` so that a value can write its parts into a caller's buffer, which is what makes a
    /// fragment embeddable without an allocation per level of nesting.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::typst::ToTypst;
    /// use std::fmt::{Display, Formatter, Result};
    ///
    /// // A type that embeds another value's fragment inside its own.
    /// struct Negated(i32);
    ///
    /// impl Display for Negated {
    ///     fn fmt(&self, f: &mut Formatter) -> Result {
    ///         f.write_str("-(")?;
    ///         self.0.fmt_typst(f)?;
    ///         f.write_str(")")
    ///     }
    /// }
    ///
    /// assert_eq!(Negated(5).to_string(), "-(5)");
    /// ```
    /// That fragment draws the negation of five.
    fn fmt_typst(&self, f: &mut Formatter) -> Result;

    /// Converts a value to a Typst math-mode fragment.
    ///
    /// The returned [`TypstWrapper`] implements [`Display`], so it can be converted to a `String`
    /// with `to_string`, or written directly with `write!` and friends.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::typst::ToTypst;
    ///
    /// assert_eq!(123u32.to_typst().to_string(), "123");
    /// assert_eq!((-45i16).to_typst().to_string(), "-45");
    ///
    /// // The output is a fragment, so it can be embedded in a larger expression.
    /// assert_eq!(format!("x^({})", 10u8.to_typst()), "x^(10)");
    /// ```
    #[inline]
    fn to_typst(&self) -> TypstWrapper<'_, Self>
    where
        Self: Sized,
    {
        TypstWrapper { x: self }
    }
}

/// A `struct` that can be used to format a value as a Typst math-mode fragment.
///
/// It is returned by [`ToTypst::to_typst`].
pub struct TypstWrapper<'a, T: ToTypst> {
    pub(crate) x: &'a T,
}

impl<T: ToTypst> Display for TypstWrapper<'_, T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.x.fmt_typst(f)
    }
}

impl ToTypst for &str {
    /// Writes a string slice as a Typst math-mode fragment.
    ///
    /// The fragment depicts the string: it is one quoted string, which Typst typesets as text, with
    /// `\` and `"` escaped and control characters spelled rather than written. Typst reads Unicode
    /// natively, so no character needs a spelling of its own, and `"100% α"` comes out as itself.
    /// No quotation marks beyond the string literal's own are added; the fragment is the string's
    /// content and nothing else.
    ///
    /// A run of superscript or subscript characters is the exception, and becomes a single script:
    /// `"2¹⁰"` is two raised to the tenth rather than the two characters, which a text font may
    /// not have at all. A run of one kind does not run into the next: `"x¹₂"` keeps its one
    /// beside its two rather than stacking them.
    ///
    /// The empty string becomes `""` rather than nothing at all.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.chars().count()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::typst::ToTypst;
    ///
    /// assert_eq!("hello".to_typst().to_string(), r#""hello""#);
    /// assert_eq!("100%".to_typst().to_string(), r#""100%""#);
    /// assert_eq!("100% α".to_typst().to_string(), r#""100% α""#);
    /// assert_eq!("A ≤ B".to_typst().to_string(), r#""A ≤ B""#);
    /// assert_eq!("--flag".to_typst().to_string(), r#""--flag""#);
    /// assert_eq!("2¹⁰".to_typst().to_string(), r#""2"^("10")"#);
    /// assert_eq!("H₂O".to_typst().to_string(), r#""H"_("2")"O""#);
    /// ```
    ///
    /// | value      | fragment     |
    /// |------------|--------------|
    /// | `"hello"`  | `"hello"`    |
    /// | `"100%"`   | `"100%"`     |
    /// | `"100% α"` | `"100% α"`   |
    /// | `"A ≤ B"`  | `"A ≤ B"`    |
    /// | `"--flag"` | `"--flag"`   |
    /// | `"2¹⁰"`    | `"2"^("10")`   |
    /// | `"H₂O"`    | `"H"_("2")"O"` |
    #[inline]
    fn fmt_typst(&self, f: &mut Formatter) -> Result {
        fmt_typst_chars(self.chars(), f)
    }
}

impl ToTypst for String {
    /// Writes a [`String`] as a Typst math-mode fragment.
    ///
    /// This is the same as the [`&str`] implementation.
    ///
    /// The fragment depicts the string: it is one quoted string, which Typst typesets as text, with
    /// `\` and `"` escaped and control characters spelled rather than written. Typst reads Unicode
    /// natively, so no character needs a spelling of its own, and `"100% α"` comes out as itself.
    /// No quotation marks beyond the string literal's own are added; the fragment is the string's
    /// content and nothing else.
    ///
    /// A run of superscript or subscript characters is the exception, and becomes a single script:
    /// `"2¹⁰"` is two raised to the tenth rather than the two characters, which a text font may
    /// not have at all. A run of one kind does not run into the next: `"x¹₂"` keeps its one
    /// beside its two rather than stacking them.
    ///
    /// The empty string becomes `""` rather than nothing at all.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.chars().count()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::typst::ToTypst;
    ///
    /// assert_eq!("hello".to_string().to_typst().to_string(), r#""hello""#);
    /// assert_eq!("100%".to_string().to_typst().to_string(), r#""100%""#);
    /// assert_eq!("100% α".to_string().to_typst().to_string(), r#""100% α""#);
    /// assert_eq!("A ≤ B".to_string().to_typst().to_string(), r#""A ≤ B""#);
    /// assert_eq!("--flag".to_string().to_typst().to_string(), r#""--flag""#);
    /// assert_eq!("2¹⁰".to_string().to_typst().to_string(), r#""2"^("10")"#);
    /// assert_eq!("H₂O".to_string().to_typst().to_string(), r#""H"_("2")"O""#);
    /// ```
    ///
    /// | value      | fragment     |
    /// |------------|--------------|
    /// | `"hello"`  | `"hello"`    |
    /// | `"100%"`   | `"100%"`     |
    /// | `"100% α"` | `"100% α"`   |
    /// | `"A ≤ B"`  | `"A ≤ B"`    |
    /// | `"--flag"` | `"--flag"`   |
    /// | `"2¹⁰"`    | `"2"^("10")`   |
    /// | `"H₂O"`    | `"H"_("2")"O"` |
    #[inline]
    fn fmt_typst(&self, f: &mut Formatter) -> Result {
        fmt_typst_chars(self.chars(), f)
    }
}

impl<T: ToTypst> ToTypst for &T {
    /// Writes a reference as a Typst math-mode fragment.
    ///
    /// The fragment is the referent's own, so a reference is invisible: `&5u8` and `5u8` have the
    /// same fragment. That is what lets a value be written without being dereferenced first, and a
    /// collection of references be written at all.
    ///
    /// [`&str`] and slices have implementations of their own rather than reaching this one, since
    /// their referents are unsized. They write the same fragments either way.
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of `fmt_typst` for `T`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::typst::ToTypst;
    ///
    /// // A reference is invisible, which is what lets a collection of references be written.
    /// assert_eq!(
    ///     vec![&1u8, &2u8].to_typst().to_string(),
    ///     vec![1u8, 2u8].to_typst().to_string()
    /// );
    ///
    /// // A method call on a reference resolves to the referent's own implementation, so this is
    /// // reached through a generic context instead.
    /// fn fragment<T: ToTypst>(x: T) -> String {
    ///     x.to_typst().to_string()
    /// }
    /// let n = 5u8;
    /// let n_ref: &u8 = &n;
    /// assert_eq!(fragment(n_ref), "5");
    /// ```
    #[inline]
    fn fmt_typst(&self, f: &mut Formatter) -> Result {
        (**self).fmt_typst(f)
    }
}
