// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::chars::latex::latex_spelling;
use crate::vars::VarScheme;
use core::fmt::{Formatter, Result, Write};

// Each Greek alphabet is 24 letters that Unicode lays out over 25 code points, with one place in
// the middle taken by something that is not one of the 24: among the lowercase letters it is the
// final sigma, a second form of a letter already counted, and among the uppercase letters it is
// unassigned. Writing the letters out rather than counting from the first one keeps that gap from
// having to be described twice, and shows at a glance which characters are meant.
const GREEK_LETTERS: [char; 24] = [
    'α', 'β', 'γ', 'δ', 'ε', 'ζ', 'η', 'θ', 'ι', 'κ', 'λ', 'μ', 'ν', 'ξ', 'ο', 'π', 'ρ', 'σ', 'τ',
    'υ', 'φ', 'χ', 'ψ', 'ω',
];

const GREEK_CAPS_LETTERS: [char; 24] = [
    'Α', 'Β', 'Γ', 'Δ', 'Ε', 'Ζ', 'Η', 'Θ', 'Ι', 'Κ', 'Λ', 'Μ', 'Ν', 'Ξ', 'Ο', 'Π', 'Ρ', 'Σ', 'Τ',
    'Υ', 'Φ', 'Χ', 'Ψ', 'Ω',
];

fn greek_index(letters: &[char; 24], name: &str) -> Option<usize> {
    let mut cs = name.chars();
    let c = cs.next()?;
    if cs.next().is_some() {
        return None;
    }
    letters.iter().position(|&x| x == c)
}

// Writes a Greek letter as a LaTeX math-mode fragment.
//
// LaTeX has a macro for a Greek letter only where that letter is not already a Latin one to look
// at: there is a `\Gamma` and a `\Sigma`, but no `\Alpha` or `\Rho`, since an `A` and a `P` are
// what those are set with. The table that spells `char`s for LaTeX draws exactly that line already,
// so the spelling is read off it rather than written out a second time. A spelling from that table
// is either a macro or a single Latin letter, and each stands in math mode as it is.
//
// Every Greek letter is in the table, which is why the spelling is taken rather than asked for.
fn fmt_greek_latex(c: char, f: &mut Formatter) -> Result {
    f.write_str(latex_spelling(c).unwrap())
}

/// A scheme that names variables `α`, `β`, `γ`, and so on, through `ω`.
///
/// The names are the 24 lowercase Greek letters. The final sigma `ς` is not among them: it is a
/// second form of a letter that is already there, and two names for one variable is exactly what a
/// scheme may not have.
///
/// There are 24 letters, so this scheme's [`capacity`](VarScheme::capacity) is 24.
///
/// In LaTeX a letter is written with its macro where it has one, and as the Latin letter it looks
/// like where it does not, since LaTeX has no `\omicron`. In Typst the letter is written as itself.
///
/// # Examples
/// ```
/// use malachite_base::strings::latex::ToLatex;
/// use malachite_base::strings::typst::ToTypst;
/// use malachite_base::vars::VarScheme;
/// use malachite_base::vars::greek::GreekVars;
///
/// assert_eq!(GreekVars.capacity(), Some(24));
/// assert_eq!(GreekVars.var(0).to_string(), "α");
/// assert_eq!(GreekVars.var(17).to_string(), "σ");
/// assert_eq!(GreekVars.var(23).to_string(), "ω");
/// assert_eq!(GreekVars.var(0).to_latex_string(), r"\alpha");
/// assert_eq!(GreekVars.var(14).to_latex_string(), "o");
/// assert_eq!(GreekVars.var(0).to_typst_string(), "α");
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GreekVars;

/// A scheme that names variables `Α`, `Β`, `Γ`, and so on, through `Ω`.
///
/// This is [`GreekVars`] in capitals, for when the lowercase letters are wanted for something else.
/// The 24 letters skip the one unassigned code point that Unicode leaves among them.
///
/// # Examples
/// ```
/// use malachite_base::strings::latex::ToLatex;
/// use malachite_base::strings::typst::ToTypst;
/// use malachite_base::vars::VarScheme;
/// use malachite_base::vars::greek::GreekCapsVars;
///
/// assert_eq!(GreekCapsVars.capacity(), Some(24));
/// assert_eq!(GreekCapsVars.var(0).to_string(), "Α");
/// assert_eq!(GreekCapsVars.var(17).to_string(), "Σ");
/// assert_eq!(GreekCapsVars.var(23).to_string(), "Ω");
/// assert_eq!(GreekCapsVars.var(2).to_latex_string(), r"\Gamma");
/// assert_eq!(GreekCapsVars.var(0).to_latex_string(), "A");
/// assert_eq!(GreekCapsVars.var(0).to_typst_string(), "Α");
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GreekCapsVars;

macro_rules! impl_greek {
    ($t: ident, $letters: ident) => {
        impl VarScheme for $t {
            /// The number of variables the scheme can name, which is the number of letters in the
            /// Greek alphabet.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](self).
            #[inline]
            fn capacity(&self) -> Option<usize> {
                Some(24)
            }

            /// Writes a variable's plain name, which is a Greek letter.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `index` is greater than or equal to 24.
            ///
            /// # Examples
            /// See [here](self).
            #[inline]
            fn fmt_var(&self, index: usize, f: &mut Formatter) -> Result {
                f.write_char($letters[index])
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
                greek_index(&$letters, name)
            }

            /// Writes a variable's name as a LaTeX math-mode fragment, which is a Greek letter.
            ///
            /// LaTeX has a macro for a Greek letter only where that letter is not already a Latin
            /// one to look at, so `κ` is written `\kappa` while `ο` is written `o` and `Α` is
            /// written `A`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `index` is greater than or equal to 24.
            ///
            /// # Examples
            /// See [here](self).
            #[inline]
            fn fmt_var_latex(&self, index: usize, f: &mut Formatter) -> Result {
                fmt_greek_latex($letters[index], f)
            }

            /// Writes a variable's name as a Typst math-mode fragment, which is a Greek letter.
            ///
            /// Typst reads the letter itself, so the fragment is the same character the plain name
            /// is, and Typst sets it in math italics as it would any other single letter.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `index` is greater than or equal to 24.
            ///
            /// # Examples
            /// See [here](self).
            #[inline]
            fn fmt_var_typst(&self, index: usize, f: &mut Formatter) -> Result {
                f.write_char($letters[index])
            }
        }
    };
}

impl_greek!(GreekVars, GREEK_LETTERS);
impl_greek!(GreekCapsVars, GREEK_CAPS_LETTERS);
