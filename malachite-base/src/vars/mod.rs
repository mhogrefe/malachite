// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::chars::latex::fmt_latex_chars;
use crate::chars::typst::fmt_typst_chars;
use crate::strings::latex::ToLatex;
use crate::strings::typst::ToTypst;
use alloc::string::ToString;
use core::fmt::{Debug, Display, Formatter, Result, Write};

/// Determines whether a [`char`] is reserved, and so may not appear in a variable's name.
///
/// The reserved characters are the ASCII digits; the operators `+`, `-`, `*`, `/`, and `^`; the
/// parentheses `(` and `)`; the comma; and whitespace. Between them these are everything that
/// punctuates a polynomial written out in full, from the coefficients and exponents to the
/// operators joining the terms, the parentheses grouping them, and the commas separating one
/// variable from the next.
///
/// Keeping them out of the names is what lets a polynomial be written without separators and still
/// be read back: in `3*x^2*y`, the variables are exactly the longest runs of unreserved characters,
/// so no lookahead or escaping is needed to find where a name ends.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::vars::char_is_reserved;
///
/// assert_eq!(char_is_reserved('x'), false);
/// assert_eq!(char_is_reserved('α'), false);
/// assert_eq!(char_is_reserved('₀'), false);
/// assert_eq!(char_is_reserved('0'), true);
/// assert_eq!(char_is_reserved('^'), true);
/// assert_eq!(char_is_reserved(' '), true);
/// ```
pub const fn char_is_reserved(c: char) -> bool {
    c.is_ascii_digit()
        || c.is_whitespace()
        || matches!(c, '+' | '-' | '*' | '/' | '^' | '(' | ')' | ',')
}

// Writes a variable's plain name as a LaTeX math-mode fragment, for a scheme that has nothing
// better to say.
//
// A one-letter name is a variable in the ordinary sense, and is written bare so that LaTeX sets it
// in math italics. A longer name is set upright, as a multi-letter identifier should be, and goes
// through the `char` machinery, which escapes whatever it holds.
fn fmt_name_latex(name: &str, f: &mut Formatter) -> Result {
    let mut cs = name.chars();
    match (cs.next(), cs.next()) {
        (Some(c), None) if c.is_ascii_alphabetic() => f.write_char(c),
        _ => fmt_latex_chars(name.chars(), f),
    }
}

// Writes a variable's plain name as a Typst math-mode fragment, for a scheme that has nothing
// better to say. The reasoning is the same as `fmt_name_latex`'s.
fn fmt_name_typst(name: &str, f: &mut Formatter) -> Result {
    let mut cs = name.chars();
    match (cs.next(), cs.next()) {
        (Some(c), None) if c.is_ascii_alphabetic() => f.write_char(c),
        _ => fmt_typst_chars(name.chars(), f),
    }
}

/// A scheme for naming variables.
///
/// A polynomial's variables are numbered rather than named: the first is variable 0, the second is
/// variable 1, and so on. A scheme is what turns those numbers into names, and names back into
/// numbers, so that one polynomial may be shown as $x_0 + x_1$, or as $x + y$, or with whatever
/// names the caller has in mind, while the polynomial itself knows nothing about any of them.
///
/// A scheme names a variable in three languages: as plain text, as LaTeX, and as Typst. They are
/// the same name differently set, so that [`IndexedVars`](indexed::IndexedVars) writes `x₀` as
/// plain text and `x_0` in both LaTeX and Typst. Only the plain name is read back;
/// [`parse_var`](VarScheme::parse_var) is its inverse.
///
/// # Contract
/// For every index below [`capacity`](VarScheme::capacity), an implementation guarantees that the
/// plain name
/// - is read back by [`parse_var`](VarScheme::parse_var) as the index it was written for;
/// - is not empty;
/// - holds no character that [`char_is_reserved`] rejects;
/// - belongs to that variable alone.
///
/// The last three are what let a polynomial be written without separators and still be read back.
/// [`var_scheme_properties`](crate::test_util::vars::var_scheme_properties) checks all four.
///
/// # Typst
/// Typst reads a run of two or more letters as a single identifier, so an `x` and a `y` written
/// side by side are the unknown `xy` rather than a product. Anything that writes several variables
/// in a row must separate them — a space is enough, and so is anything that is not a letter, such
/// as the `_` or `^` of a script.
pub trait VarScheme {
    /// The number of variables the scheme can name, or `None` if it can name any number of them.
    ///
    /// Naming a variable whose index is not below this is a panic, not an error: a scheme is asked
    /// for a name by something that already knows how many variables it has.
    fn capacity(&self) -> Option<usize>;

    /// Writes a variable's plain name.
    ///
    /// This is the name that [`parse_var`](VarScheme::parse_var) reads back.
    ///
    /// # Panics
    /// Panics if `index` is not less than [`capacity`](VarScheme::capacity).
    fn fmt_var(&self, index: usize, f: &mut Formatter) -> Result;

    /// Reads a variable's index from its plain name.
    ///
    /// The whole string must be the name of one variable; a string that is more than a name, or
    /// less than one, or the name of a variable the scheme cannot reach, gives `None`.
    fn parse_var(&self, name: &str) -> Option<usize>;

    /// Writes a variable's name as a LaTeX math-mode fragment.
    ///
    /// The default writes the plain name, as a bare letter when it is a single ASCII letter — so
    /// that LaTeX sets it in math italics, as a variable should be — and as upright text
    /// otherwise, with LaTeX's special characters escaped. A scheme whose names are not letters, or
    /// that has a better spelling than the one its plain name suggests, overrides this.
    ///
    /// # Panics
    /// Panics if `index` is not less than [`capacity`](VarScheme::capacity).
    #[inline]
    fn fmt_var_latex(&self, index: usize, f: &mut Formatter) -> Result {
        fmt_name_latex(&Var::new(self, index).to_string(), f)
    }

    /// Writes a variable's name as a Typst math-mode fragment.
    ///
    /// The default writes the plain name, as a bare letter when it is a single ASCII letter — so
    /// that Typst sets it in math italics, as a variable should be — and as a quoted string
    /// otherwise. A scheme whose names are not letters, or that has a better spelling than the one
    /// its plain name suggests, overrides this.
    ///
    /// # Panics
    /// Panics if `index` is not less than [`capacity`](VarScheme::capacity).
    #[inline]
    fn fmt_var_typst(&self, index: usize, f: &mut Formatter) -> Result {
        fmt_name_typst(&Var::new(self, index).to_string(), f)
    }

    /// Gives a handle to one of the scheme's variables.
    ///
    /// The handle is what carries the name around: it implements [`Display`],
    /// [`ToLatex`](crate::strings::latex::ToLatex), and
    /// [`ToTypst`](crate::strings::typst::ToTypst), so that a variable can be written wherever any
    /// of those is expected. Nothing is checked here; an index past the scheme's
    /// [`capacity`](VarScheme::capacity) panics when the handle is written rather than when it is
    /// made.
    ///
    /// A scheme behind a `dyn` has no `var` of its own, since the handle's type mentions the
    /// scheme's; [`Var::new`] does the same thing for one.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::vars::VarScheme;
    /// use malachite_base::vars::xyz::XyzVars;
    ///
    /// assert_eq!(XyzVars.var(0).to_string(), "x");
    /// assert_eq!(XyzVars.var(1).to_string(), "y");
    /// ```
    #[inline]
    fn var(&self, index: usize) -> Var<'_, Self>
    where
        Self: Sized,
    {
        Var::new(self, index)
    }
}

/// One variable of a [`VarScheme`], which is to say a scheme together with an index.
///
/// It is returned by [`VarScheme::var`], and it is what a variable's name is written from: it
/// implements [`Display`], [`ToLatex`](crate::strings::latex::ToLatex), and
/// [`ToTypst`](crate::strings::typst::ToTypst).
pub struct Var<'a, S: VarScheme + ?Sized> {
    scheme: &'a S,
    index: usize,
}

impl<'a, S: VarScheme + ?Sized> Var<'a, S> {
    /// Makes a handle to one of a scheme's variables.
    ///
    /// [`VarScheme::var`] is the shorter way to say this, and works whenever the scheme's type is
    /// known. This one also works for a scheme behind a `dyn`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::vars::abc::AbcVars;
    /// use malachite_base::vars::{Var, VarScheme};
    ///
    /// let scheme: &dyn VarScheme = &AbcVars;
    /// assert_eq!(Var::new(scheme, 2).to_string(), "c");
    /// ```
    pub const fn new(scheme: &'a S, index: usize) -> Self {
        Var { scheme, index }
    }

    /// The variable's index.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::vars::VarScheme;
    /// use malachite_base::vars::xyz::XyzVars;
    ///
    /// assert_eq!(XyzVars.var(3).index(), 3);
    /// ```
    pub const fn index(&self) -> usize {
        self.index
    }

    /// The scheme the variable belongs to.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::vars::VarScheme;
    /// use malachite_base::vars::abc::AbcVars;
    ///
    /// assert_eq!(AbcVars.var(3).scheme().capacity(), Some(26));
    /// ```
    pub const fn scheme(&self) -> &'a S {
        self.scheme
    }
}

impl<S: VarScheme + ?Sized> Clone for Var<'_, S> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<S: VarScheme + ?Sized> Copy for Var<'_, S> {}

impl<S: VarScheme + ?Sized> Display for Var<'_, S> {
    /// Converts a variable to a [`String`](alloc::string::String).
    ///
    /// This is the plain name, the one [`VarScheme::parse_var`] reads back.
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of `fmt_var` for the scheme.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::vars::VarScheme;
    /// use malachite_base::vars::indexed::IndexedVars;
    ///
    /// assert_eq!(IndexedVars.var(10).to_string(), "x₁₀");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.scheme.fmt_var(self.index, f)
    }
}

impl<S: VarScheme + ?Sized> Debug for Var<'_, S> {
    /// Converts a variable to a [`String`](alloc::string::String).
    ///
    /// This is the same as the [`Display`] implementation: a variable is its name, and a scheme is
    /// not required to have a depiction of its own.
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of `fmt_var` for the scheme.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_base::vars::VarScheme;
    /// use malachite_base::vars::indexed::IndexedVars;
    ///
    /// assert_eq!(IndexedVars.var(10).to_debug_string(), "x₁₀");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(self, f)
    }
}

impl<S: VarScheme + ?Sized> ToLatex for Var<'_, S> {
    /// Writes a variable as a LaTeX math-mode fragment.
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of `fmt_var_latex` for the scheme.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::latex::ToLatex;
    /// use malachite_base::vars::VarScheme;
    /// use malachite_base::vars::greek::GreekVars;
    /// use malachite_base::vars::indexed::IndexedVars;
    ///
    /// assert_eq!(IndexedVars.var(10).to_latex_string(), "x_{10}");
    /// assert_eq!(GreekVars.var(0).to_latex_string(), r"\alpha");
    /// ```
    /// Those fragments render as $x_{10}$ and $\alpha$.
    #[inline]
    fn fmt_latex(&self, f: &mut Formatter) -> Result {
        self.scheme.fmt_var_latex(self.index, f)
    }
}

impl<S: VarScheme + ?Sized> ToTypst for Var<'_, S> {
    /// Writes a variable as a Typst math-mode fragment.
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of `fmt_var_typst` for the scheme.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::typst::ToTypst;
    /// use malachite_base::vars::VarScheme;
    /// use malachite_base::vars::greek::GreekVars;
    /// use malachite_base::vars::indexed::IndexedVars;
    ///
    /// assert_eq!(IndexedVars.var(10).to_typst_string(), "x_(10)");
    /// assert_eq!(GreekVars.var(0).to_typst_string(), "α");
    /// ```
    #[inline]
    fn fmt_typst(&self, f: &mut Formatter) -> Result {
        self.scheme.fmt_var_typst(self.index, f)
    }
}

/// [`AbcVars`](abc::AbcVars) and [`AbcCapsVars`](abc::AbcCapsVars), which name variables with the
/// letters of the alphabet in their usual order.
pub mod abc;
/// [`GreekVars`](greek::GreekVars) and [`GreekCapsVars`](greek::GreekCapsVars), which name
/// variables with the letters of the Greek alphabet.
pub mod greek;
/// [`IndexedVars`](indexed::IndexedVars) and [`IndexedCapsVars`](indexed::IndexedCapsVars), which
/// name variables with a letter and a subscript.
pub mod indexed;
/// [`ListVars`](list::ListVars), which names variables with names the caller supplies.
pub mod list;
/// [`XyzVars`](xyz::XyzVars) and [`XyzCapsVars`](xyz::XyzCapsVars), which name variables with the
/// letters of the alphabet, beginning at the end.
pub mod xyz;
