// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::vars::VarScheme;
use crate::vars::abc::{letter, letter_index};
use core::fmt::{Formatter, Result, Write};

// Turns a letter's place in the alphabet into its place in the xyz order, and back.
//
// The xyz order runs x, y, z, w, v, ..., a: the last three letters first, in their usual order,
// then the rest backwards. So the first three places are the letters 23, 24, and 25, and place $i$
// after that is letter $25 - i$. Each map is its own inverse's inverse on 0..26, which is what
// makes the scheme a naming of 26 variables rather than of some of them twice.
const fn xyz_rank(abc_index: usize) -> usize {
    if abc_index >= 23 {
        abc_index - 23
    } else {
        25 - abc_index
    }
}

const fn xyz_unrank(rank: usize) -> usize {
    if rank < 3 { rank + 23 } else { 25 - rank }
}

/// A scheme that names variables `x`, `y`, `z`, `w`, `v`, and so on, back through `a`.
///
/// The names are the letters of the alphabet, but ordered as a mathematician reaches for them: the
/// unknowns `x`, `y`, and `z` first, then backwards from `w` to `a`. This is the scheme to reach
/// for when a polynomial in one variable should call it `x`, or one in three variables should call
/// them `x`, `y`, and `z`. [`AbcVars`](super::abc::AbcVars) is the scheme that starts at `a`
/// instead.
///
/// There are 26 letters, so this scheme's [`capacity`](VarScheme::capacity) is 26.
///
/// # Examples
/// ```
/// use malachite_base::strings::latex::ToLatex;
/// use malachite_base::strings::typst::ToTypst;
/// use malachite_base::vars::VarScheme;
/// use malachite_base::vars::xyz::XyzVars;
///
/// assert_eq!(XyzVars.capacity(), Some(26));
/// assert_eq!(XyzVars.var(0).to_string(), "x");
/// assert_eq!(XyzVars.var(1).to_string(), "y");
/// assert_eq!(XyzVars.var(2).to_string(), "z");
/// assert_eq!(XyzVars.var(3).to_string(), "w");
/// assert_eq!(XyzVars.var(25).to_string(), "a");
/// assert_eq!(XyzVars.var(1).to_latex_string(), "y");
/// assert_eq!(XyzVars.var(1).to_typst_string(), "y");
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct XyzVars;

/// A scheme that names variables `X`, `Y`, `Z`, `W`, `V`, and so on, back through `A`.
///
/// This is [`XyzVars`] in capitals, for when the lowercase letters are wanted for something else.
///
/// # Examples
/// ```
/// use malachite_base::strings::latex::ToLatex;
/// use malachite_base::strings::typst::ToTypst;
/// use malachite_base::vars::VarScheme;
/// use malachite_base::vars::xyz::XyzCapsVars;
///
/// assert_eq!(XyzCapsVars.capacity(), Some(26));
/// assert_eq!(XyzCapsVars.var(0).to_string(), "X");
/// assert_eq!(XyzCapsVars.var(3).to_string(), "W");
/// assert_eq!(XyzCapsVars.var(25).to_string(), "A");
/// assert_eq!(XyzCapsVars.var(1).to_latex_string(), "Y");
/// assert_eq!(XyzCapsVars.var(1).to_typst_string(), "Y");
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct XyzCapsVars;

macro_rules! impl_xyz {
    ($t: ident, $a: expr) => {
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
                f.write_char(letter($a, xyz_unrank(index)))
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
                letter_index($a, name).map(xyz_rank)
            }
        }
    };
}
impl_xyz!(XyzVars, 'a');
impl_xyz!(XyzCapsVars, 'A');
