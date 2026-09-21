// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::conversion::traits::ExactFrom;
use crate::vars::VarScheme;
use core::fmt::{Formatter, Result, Write};

// The letters of the alphabet are contiguous in ASCII, so a letter is its index away from the first
// one. Both cases work the same way; only where they begin differs.
pub(crate) fn letter(first: char, index: usize) -> char {
    char::from(u8::exact_from(u32::from(first) + u32::exact_from(index)))
}

pub(crate) fn letter_index(first: char, name: &str) -> Option<usize> {
    let mut cs = name.chars();
    let c = cs.next()?;
    if cs.next().is_some() {
        return None;
    }
    let offset = u32::from(c).checked_sub(u32::from(first))?;
    if offset < 26 {
        Some(usize::exact_from(offset))
    } else {
        None
    }
}

/// A scheme that names variables `a`, `b`, `c`, and so on, through `z`.
///
/// The names are the letters in their usual order, which is what a polynomial in a few variables is
/// usually written with when nothing suggests otherwise. [`XyzVars`](super::xyz::XyzVars) is the
/// scheme to reach for when the first variable should be `x` rather than `a`.
///
/// There are 26 letters, so this scheme's [`capacity`](VarScheme::capacity) is 26.
///
/// # Examples
/// ```
/// use malachite_base::strings::latex::ToLatex;
/// use malachite_base::strings::typst::ToTypst;
/// use malachite_base::vars::VarScheme;
/// use malachite_base::vars::abc::AbcVars;
///
/// assert_eq!(AbcVars.capacity(), Some(26));
/// assert_eq!(AbcVars.var(0).to_string(), "a");
/// assert_eq!(AbcVars.var(25).to_string(), "z");
/// assert_eq!(AbcVars.var(1).to_latex_string(), "b");
/// assert_eq!(AbcVars.var(1).to_typst_string(), "b");
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AbcVars;

/// A scheme that names variables `A`, `B`, `C`, and so on, through `Z`.
///
/// This is [`AbcVars`] in capitals, for when the lowercase letters are wanted for something else.
///
/// # Examples
/// ```
/// use malachite_base::strings::latex::ToLatex;
/// use malachite_base::strings::typst::ToTypst;
/// use malachite_base::vars::VarScheme;
/// use malachite_base::vars::abc::AbcCapsVars;
///
/// assert_eq!(AbcCapsVars.capacity(), Some(26));
/// assert_eq!(AbcCapsVars.var(0).to_string(), "A");
/// assert_eq!(AbcCapsVars.var(25).to_string(), "Z");
/// assert_eq!(AbcCapsVars.var(1).to_latex_string(), "B");
/// assert_eq!(AbcCapsVars.var(1).to_typst_string(), "B");
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AbcCapsVars;

macro_rules! impl_abc {
    ($t: ident, $first: expr) => {
        impl VarScheme for $t {
            /// The number of variables the scheme can name, which is the number of letters in the
            /// alphabet.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](self).
            #[inline]
            fn capacity(&self) -> Option<usize> {
                Some(26)
            }

            /// Writes a variable's plain name, which is a letter.
            ///
            /// The LaTeX and Typst names are the same letter, written bare so that both set it in
            /// math italics; the default implementations do that already.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `index` is greater than or equal to 26.
            ///
            /// # Examples
            /// See [here](self).
            #[inline]
            fn fmt_var(&self, index: usize, f: &mut Formatter) -> Result {
                assert!(
                    index < 26,
                    "index {index} is past the 26 letters of the alphabet"
                );
                f.write_char(letter($first, index))
            }

            /// Reads a variable's index from its plain name.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](self).
            #[inline]
            fn parse_var(&self, name: &str) -> Option<usize> {
                letter_index($first, name)
            }
        }
    };
}
impl_abc!(AbcVars, 'a');
impl_abc!(AbcCapsVars, 'A');
