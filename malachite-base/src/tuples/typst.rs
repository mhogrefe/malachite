// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::strings::typst::ToTypst;
use alloc::string::{String, ToString};
use core::fmt::{Display, Formatter, Result};

// Writes some values as one parenthesized, comma-separated Typst math-mode fragment.
//
// The elements arrive as trait objects rather than through a generic parameter apiece, so that
// every arity shares one writer. `fmt_typst` is object-safe, which is what allows it.
fn fmt_typst_parts(parts: &[&dyn ToTypst], f: &mut Formatter) -> Result {
    f.write_str("(")?;
    for (i, x) in parts.iter().enumerate() {
        if i != 0 {
            f.write_str(", ")?;
        }
        x.fmt_typst(f)?;
    }
    f.write_str(")")
}

struct TupleParts<'a>(&'a [&'a dyn ToTypst]);

impl Display for TupleParts<'_> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        fmt_typst_parts(self.0, f)
    }
}

/// Converts some values to one parenthesized, comma-separated Typst math-mode fragment.
///
/// This is what the [`typst_tuple`](crate::typst_tuple) macro calls. It is also the way to build a
/// fragment for a tuple of more than eight elements: the orphan rule keeps a crate other than this
/// one from implementing [`ToTypst`] for such a tuple, but nothing keeps it from writing the
/// fragment.
///
/// # Worst-case complexity
/// $T(n) = O(n + \sum_{i=0}^{n-1}T^\prime(i))$
///
/// $M(n) = O(n + \max_{i=0}^{n-1}M^\prime(i))$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `parts.len()`, $i$ is an element's index,
/// and $T^\prime$ and $M^\prime$ are the time and memory functions of `fmt_typst` for that element.
///
/// # Examples
/// ```
/// use malachite_base::tuples::typst::typst_tuple;
///
/// assert_eq!(typst_tuple(&[&1u8, &2u8]), "(1, 2)");
/// assert_eq!(typst_tuple(&[]), "()");
/// ```
#[inline]
pub fn typst_tuple(parts: &[&dyn ToTypst]) -> String {
    TupleParts(parts).to_string()
}

macro_rules! impl_to_typst_for_tuple {
    ($($t:ident: $i:tt),+) => {
        impl<$($t: ToTypst),+> ToTypst for ($($t,)+) {
            /// Writes a tuple as a Typst math-mode fragment.
            ///
            /// See [here](super::typst#fmt_typst).
            #[inline]
            fn fmt_typst(&self, f: &mut Formatter) -> Result {
                fmt_typst_parts(&[$(&self.$i as &dyn ToTypst),+], f)
            }
        }
    };
}
impl_to_typst_for_tuple!(A: 0);
impl_to_typst_for_tuple!(A: 0, B: 1);
impl_to_typst_for_tuple!(A: 0, B: 1, C: 2);
impl_to_typst_for_tuple!(A: 0, B: 1, C: 2, D: 3);
impl_to_typst_for_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4);
impl_to_typst_for_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5);
impl_to_typst_for_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6);
impl_to_typst_for_tuple!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7);

impl ToTypst for () {
    /// Writes the unit type as a Typst math-mode fragment.
    ///
    /// The fragment is `()`, an empty pair of parentheses, since the unit type is the tuple of no
    /// elements.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::typst::ToTypst;
    ///
    /// assert_eq!(().to_typst_string(), "()");
    /// ```
    ///
    /// | value | fragment |
    /// |-------|----------|
    /// | `()`  | `()`     |
    #[inline]
    fn fmt_typst(&self, f: &mut Formatter) -> Result {
        f.write_str("()")
    }
}

/// Converts some values to one parenthesized, comma-separated Typst math-mode fragment.
///
/// [`ToTypst`](crate::strings::typst::ToTypst) is implemented for tuples of up to eight elements.
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
/// index, and $T^\prime$ and $M^\prime$ are the time and memory functions of `fmt_typst` for that
/// element.
///
/// # Examples
/// ```
/// use malachite_base::typst_tuple;
///
/// let t = (1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8);
/// assert_eq!(
///     typst_tuple!(t.0, t.1, t.2, t.3, t.4, t.5, t.6, t.7, t.8),
///     "(1, 2, 3, 4, 5, 6, 7, 8, 9)"
/// );
/// ```
#[macro_export]
macro_rules! typst_tuple {
    ($($x:expr),* $(,)?) => {
        $crate::tuples::typst::typst_tuple(&[$(&$x as &dyn $crate::strings::typst::ToTypst),*])
    };
}
