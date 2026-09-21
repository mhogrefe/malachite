// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::strings::latex::ToLatex;
use alloc::string::{String, ToString};
use core::fmt::{Display, Formatter, Result};

// Writes some values as one parenthesized, comma-separated LaTeX math-mode fragment.
//
// The elements arrive as trait objects rather than through a generic parameter apiece, so that
// every arity shares one writer. `fmt_latex` is object-safe, which is what allows it.
fn fmt_latex_parts(parts: &[&dyn ToLatex], f: &mut Formatter) -> Result {
    f.write_str("\\left(")?;
    for (i, x) in parts.iter().enumerate() {
        if i != 0 {
            f.write_str(", ")?;
        }
        x.fmt_latex(f)?;
    }
    f.write_str("\\right)")
}

struct TupleParts<'a>(&'a [&'a dyn ToLatex]);

impl Display for TupleParts<'_> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        fmt_latex_parts(self.0, f)
    }
}

/// Converts some values to one parenthesized, comma-separated LaTeX math-mode fragment.
///
/// This is what the [`latex_tuple`](crate::latex_tuple) macro calls. It is also the way to build a
/// fragment for a tuple of more than eight elements: the orphan rule keeps a crate other than this
/// one from implementing [`ToLatex`] for such a tuple, but nothing keeps it from writing the
/// fragment.
///
/// # Worst-case complexity
/// $T(n) = O(n + \sum_{i=0}^{n-1}T^\prime(i))$
///
/// $M(n) = O(n + \max_{i=0}^{n-1}M^\prime(i))$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `parts.len()`, $i$ is an element's index,
/// and $T^\prime$ and $M^\prime$ are the time and memory functions of `fmt_latex` for that element.
///
/// # Examples
/// ```
/// use malachite_base::tuples::latex::latex_tuple;
///
/// assert_eq!(latex_tuple(&[&1u8, &2u8]), r"\left(1, 2\right)");
/// assert_eq!(latex_tuple(&[]), "\\left(\\right)");
/// ```
#[inline]
pub fn latex_tuple(parts: &[&dyn ToLatex]) -> String {
    TupleParts(parts).to_string()
}

macro_rules! impl_to_latex_for_tuple {
    ($($t:ident: $i:tt),+) => {
        impl<$($t: ToLatex),+> ToLatex for ($($t,)+) {
            /// Writes a tuple as a LaTeX math-mode fragment.
            ///
            /// See [here](super::latex#fmt_latex).
            #[inline]
            fn fmt_latex(&self, f: &mut Formatter) -> Result {
                fmt_latex_parts(&[$(&self.$i as &dyn ToLatex),+], f)
            }
        }
    };
}
impl_to_latex_for_tuple!(A: 0);
impl_to_latex_for_tuple!(A: 0, B: 1);
impl_to_latex_for_tuple!(A: 0, B: 1, C: 2);
impl_to_latex_for_tuple!(A: 0, B: 1, C: 2, D: 3);
impl_to_latex_for_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4);
impl_to_latex_for_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5);
impl_to_latex_for_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6);
impl_to_latex_for_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7);

impl ToLatex for () {
    /// Writes the unit type as a LaTeX math-mode fragment.
    ///
    /// The fragment is `()`, an empty pair of parentheses, since the unit type is the tuple of no
    /// elements.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::latex::ToLatex;
    ///
    /// assert_eq!(().to_latex_string(), "()");
    /// ```
    ///
    /// | value | fragment | renders as |
    /// |-------|----------|------------|
    /// | `()`  | `()`     | $()$       |
    #[inline]
    fn fmt_latex(&self, f: &mut Formatter) -> Result {
        f.write_str("()")
    }
}

/// Converts some values to one parenthesized, comma-separated LaTeX math-mode fragment.
///
/// [`ToLatex`](crate::strings::latex::ToLatex) is implemented for tuples of up to eight elements.
/// The orphan rule keeps a crate other than this one from implementing it for a longer tuple, so
/// this macro is the way to write such a tuple's fragment: give it the elements, and it produces
/// what the implementation would have.
///
/// # Worst-case complexity
/// $T(n) = O(n + \sum_{i=0}^{n-1}T^\prime(i))$
///
/// $M(n) = O(n + \max_{i=0}^{n-1}M^\prime(i))$
///
/// where $T$ is time, $M$ is additional memory, $n$ is the number of elements, $i$ is an element's
/// index, and $T^\prime$ and $M^\prime$ are the time and memory functions of `fmt_latex` for that
/// element.
///
/// # Examples
/// ```
/// use malachite_base::latex_tuple;
///
/// let t = (1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8);
/// assert_eq!(
///     latex_tuple!(t.0, t.1, t.2, t.3, t.4, t.5, t.6, t.7, t.8),
///     r"\left(1, 2, 3, 4, 5, 6, 7, 8, 9\right)"
/// );
/// ```
#[macro_export]
macro_rules! latex_tuple {
    ($($x:expr),* $(,)?) => {
        $crate::tuples::latex::latex_tuple(&[$(&$x as &dyn $crate::strings::latex::ToLatex),*])
    };
}
