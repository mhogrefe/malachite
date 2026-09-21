// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::chars::scripts::{fmt_subscript_digits, parse_subscript_digits};
use crate::num::conversion::traits::ExactFrom;
use crate::vars::VarScheme;
use core::fmt::{Formatter, Result, Write};

// Writes a variable named by a letter and a subscript, in each of the three languages.
//
// The plain name uses the Unicode subscript digits, which need no markup to be read as a subscript.
// The other two use a real script, and bracket its contents only when there is more than one digit,
// since a script of one character needs nothing to hold it together.
fn fmt_indexed(base: char, index: usize, f: &mut Formatter) -> Result {
    f.write_char(base)?;
    fmt_subscript_digits(u64::exact_from(index), f)
}

fn fmt_indexed_latex(base: char, index: usize, f: &mut Formatter) -> Result {
    if index < 10 {
        write!(f, "{base}_{index}")
    } else {
        write!(f, "{base}_{{{index}}}")
    }
}

fn fmt_indexed_typst(base: char, index: usize, f: &mut Formatter) -> Result {
    if index < 10 {
        write!(f, "{base}_{index}")
    } else {
        write!(f, "{base}_({index})")
    }
}

fn parse_indexed(base: char, name: &str) -> Option<usize> {
    usize::try_from(parse_subscript_digits(name.strip_prefix(base)?)?).ok()
}

/// A scheme that names variables `x₀`, `x₁`, `x₂`, and so on.
///
/// This is the scheme to reach for when there may be more variables than there are letters, which
/// is why it is the usual choice for a polynomial in many variables. It is the only scheme with no
/// [`capacity`](VarScheme::capacity): a variable's index is written out, so any index has a name.
///
/// The plain name uses the Unicode subscript digits, so that variable 10 is `x₁₀` and not
/// `x10`, which would be variable 1 followed by a zero. In LaTeX and Typst the subscript is a real
/// one: `x_{10}` and `x_(10)`.
///
/// # Examples
/// ```
/// use malachite_base::strings::latex::ToLatex;
/// use malachite_base::strings::typst::ToTypst;
/// use malachite_base::vars::VarScheme;
/// use malachite_base::vars::indexed::IndexedVars;
///
/// assert_eq!(IndexedVars.var(0).to_string(), "x₀");
/// assert_eq!(IndexedVars.var(10).to_string(), "x₁₀");
/// assert_eq!(IndexedVars.var(10).to_latex_string(), "x_{10}");
/// assert_eq!(IndexedVars.var(10).to_typst_string(), "x_(10)");
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IndexedVars;

/// A scheme that names variables `X₀`, `X₁`, `X₂`, and so on.
///
/// This is [`IndexedVars`] in capitals, for when the lowercase letters are wanted for something
/// else.
///
/// # Examples
/// ```
/// use malachite_base::strings::latex::ToLatex;
/// use malachite_base::strings::typst::ToTypst;
/// use malachite_base::vars::VarScheme;
/// use malachite_base::vars::indexed::IndexedCapsVars;
///
/// assert_eq!(IndexedCapsVars.var(0).to_string(), "X₀");
/// assert_eq!(IndexedCapsVars.var(10).to_string(), "X₁₀");
/// assert_eq!(IndexedCapsVars.var(10).to_latex_string(), "X_{10}");
/// assert_eq!(IndexedCapsVars.var(10).to_typst_string(), "X_(10)");
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IndexedCapsVars;

macro_rules! impl_indexed {
    ($t: ident, $base: expr) => {
        impl VarScheme for $t {
            /// The number of variables the scheme can name, which is any number of them.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](self).
            #[inline]
            fn capacity(&self) -> Option<usize> {
                None
            }

            /// Writes a variable's plain name, which is a letter and a run of Unicode subscript
            /// digits.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `index.significant_bits()`.
            ///
            /// # Examples
            /// See [here](self).
            #[inline]
            fn fmt_var(&self, index: usize, f: &mut Formatter) -> Result {
                fmt_indexed($base, index, f)
            }

            /// Reads a variable's index from its plain name.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `name.len()`.
            ///
            /// # Examples
            /// See [here](self).
            #[inline]
            fn parse_var(&self, name: &str) -> Option<usize> {
                parse_indexed($base, name)
            }

            /// Writes a variable's name as a LaTeX math-mode fragment, which is a letter with a
            /// subscript.
            ///
            /// The subscript is braced only when it has more than one digit, since a subscript of
            /// one character needs nothing to hold it together.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `index.significant_bits()`.
            ///
            /// # Examples
            /// See [here](self).
            #[inline]
            fn fmt_var_latex(&self, index: usize, f: &mut Formatter) -> Result {
                fmt_indexed_latex($base, index, f)
            }

            /// Writes a variable's name as a Typst math-mode fragment, which is a letter with a
            /// subscript.
            ///
            /// The subscript is parenthesized only when it has more than one digit, since a
            /// subscript of one character needs nothing to hold it together.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `index.significant_bits()`.
            ///
            /// # Examples
            /// See [here](self).
            #[inline]
            fn fmt_var_typst(&self, index: usize, f: &mut Formatter) -> Result {
                fmt_indexed_typst($base, index, f)
            }
        }
    };
}
impl_indexed!(IndexedVars, 'x');
impl_indexed!(IndexedCapsVars, 'X');
