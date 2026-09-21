// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::chars::latex_table::LATEX_TABLE;
use crate::strings::latex::ToLatex;
use core::fmt::{Formatter, Result, Write};

// Looks a `char` up in the generated table, giving whether its LaTeX is math-mode and what it is.
fn lookup(c: char) -> Option<(bool, &'static str)> {
    LATEX_TABLE
        .binary_search_by_key(&c, |&(k, _, _)| k)
        .ok()
        .map(|i| (LATEX_TABLE[i].1, LATEX_TABLE[i].2))
}

// The superscript and subscript characters are spelled `^0`, `_1`, and so on. Gives which of the
// two a `char` is, if it is one at all.
fn script_kind(c: char) -> Option<char> {
    match lookup(c) {
        Some((true, latex)) => match latex.as_bytes().first() {
            Some(&k @ (b'^' | b'_')) => Some(char::from(k)),
            _ => None,
        },
        _ => None,
    }
}

// A spelling that is a control word — a backslash followed by letters — swallows a letter
// written after it, so a `\textbackslash` beside an `n` would become the undefined
// `\textbackslashn`. Wrapping such a spelling in braces terminates it in every context. The test is
// deliberately loose: bracing a spelling that did not need it is harmless, while missing one is
// not.
fn needs_braces(latex: &str) -> bool {
    latex.starts_with('\\') && latex.ends_with(|c: char| c.is_ascii_alphabetic())
}

// TeX collapses a run of spaces into one, and reads a blank line as a paragraph break, which is an
// error in math mode. Writing an explicit interword space instead renders exactly as a raw space
// would, while keeping the count and removing the hazard. The other space-like characters render as
// a space in TeX too, so they are written the same way.
const fn is_space_like(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '\r' | '\u{b}' | '\u{c}')
}

// In text mode LaTeX draws ligatures from these pairs, so `--` would be typeset as an en dash
// rather than as the two hyphens the string actually holds. An empty group between them keeps the
// depiction faithful.
const fn forms_ligature(prev: char, c: char) -> bool {
    matches!(
        (prev, c),
        ('-', '-') | ('\'', '\'') | (',', ',') | ('`' | '!' | '?', '`')
    )
}

// Writes a sequence of `char`s as one LaTeX math-mode fragment.
//
// Characters that LaTeX spells with a math-mode macro are written on their own; everything else is
// gathered into `\text{...}` groups, where a string's ordinary characters are typeset as
// themselves. A math-mode macro therefore ends the group that precedes it, which is why a string
// mixing the two comes out as, say, `\text{100\% }\alpha`.
//
// A math-mode macro never needs bracing: whatever follows it is either another macro or a `\text{`,
// and both begin with a backslash, which ends a control word on its own.
pub(crate) fn fmt_latex_chars<I: Iterator<Item = char>>(cs: I, f: &mut Formatter) -> Result {
    let mut cs = cs.peekable();
    let mut in_text = false;
    // The last character written into the open `\text{...}` group, for ligature detection.
    let mut last = None;
    let mut any = false;
    // Whether a superscript or subscript written next would have nothing to attach to: either
    // nothing has been written yet, or what was written last is itself a script group, and a second
    // script on the same base would both stack the two and, for two of a kind, be a double
    // superscript, which is an error.
    let mut needs_base = true;
    while let Some(c) = cs.next() {
        any = true;
        match lookup(c) {
            Some((true, latex)) if script_kind(c).is_some() => {
                let kind = script_kind(c).unwrap();
                if in_text {
                    f.write_char('}')?;
                    in_text = false;
                    last = None;
                }
                if needs_base {
                    f.write_str("{}")?;
                }
                f.write_char(kind)?;
                // A run of them is one script, so that "2¹⁰" is two raised to the tenth rather
                // than two raised to the first and then to the zeroth. Braces are what make it one;
                // a single character does not need them.
                let braced = cs.peek().copied().and_then(script_kind) == Some(kind);
                if braced {
                    f.write_str("{")?;
                }
                f.write_str(&latex[1..])?;
                while cs.peek().copied().and_then(script_kind) == Some(kind) {
                    let c = cs.next().unwrap();
                    f.write_str(&lookup(c).unwrap().1[1..])?;
                }
                if braced {
                    f.write_str("}")?;
                }
                needs_base = true;
            }
            Some((true, latex)) => {
                if in_text {
                    f.write_char('}')?;
                    in_text = false;
                    last = None;
                }
                f.write_str(latex)?;
                needs_base = false;
            }
            Some((false, latex)) => {
                needs_base = false;
                if !in_text {
                    f.write_str("\\text{")?;
                    in_text = true;
                }
                if let (Some(p), Some(n)) = (last, latex.chars().next())
                    && forms_ligature(p, n)
                {
                    f.write_str("{}")?;
                }
                last = if needs_braces(latex) {
                    f.write_char('{')?;
                    f.write_str(latex)?;
                    f.write_char('}')?;
                    Some('}')
                } else {
                    f.write_str(latex)?;
                    latex.chars().next_back()
                };
            }
            None => {
                needs_base = false;
                if !in_text {
                    f.write_str("\\text{")?;
                    in_text = true;
                }
                if is_space_like(c) {
                    if c == ' ' && last != Some(' ') {
                        f.write_char(' ')?;
                    } else {
                        f.write_str("\\ ")?;
                    }
                    last = Some(' ');
                    continue;
                }
                if let Some(p) = last
                    && forms_ligature(p, c)
                {
                    f.write_str("{}")?;
                }
                f.write_char(c)?;
                last = Some(c);
            }
        }
    }
    if in_text {
        f.write_char('}')
    } else if any {
        Ok(())
    } else {
        // An empty sequence still says "this is a string", rather than vanishing.
        f.write_str("\\text{}")
    }
}

impl ToLatex for char {
    /// Writes a [`char`] as a LaTeX math-mode fragment.
    ///
    /// A character that LaTeX spells with a math-mode macro becomes that macro, so `'α'` becomes
    /// `\alpha`. Every other character is wrapped in a `\text{...}` group, where it is typeset as
    /// itself, with LaTeX's special characters escaped: `'%'` becomes `\text{\%}`. A character with
    /// no LaTeX spelling at all is written literally inside the group, which needs an engine and a
    /// font that can render it.
    ///
    /// A superscript or subscript character is math-mode, and is given an empty base to attach to,
    /// since on its own it has none: `'²'` becomes `{}^2`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::latex::ToLatex;
    ///
    /// assert_eq!('a'.to_latex_string(), r"\text{a}");
    /// assert_eq!('%'.to_latex_string(), r"\text{\%}");
    /// assert_eq!('α'.to_latex_string(), r"\alpha");
    /// assert_eq!('∞'.to_latex_string(), r"\infty");
    /// assert_eq!('²'.to_latex_string(), r"{}^2");
    /// ```
    ///
    /// | value | fragment    | renders as   |
    /// |-------|-------------|--------------|
    /// | `'a'` | `\text{a}`  | $\text{a}$   |
    /// | `'%'` | `\text{\%}` | $\text{\\%}$ |
    /// | `'α'` | `\alpha`    | $\alpha$     |
    /// | `'∞'` | `\infty`    | $\infty$     |
    /// | `'²'` | `{}^2`      | ${}^2$       |
    #[inline]
    fn fmt_latex(&self, f: &mut Formatter) -> Result {
        fmt_latex_chars(core::iter::once(*self), f)
    }
}

// The LaTeX spelling of a `char`, if the table has one.
//
// This is the spelling alone, without the table's word on whether it is a math-mode or a text-mode
// one. A caller that already knows the context a character will stand in — a variable's name,
// say, which is always math mode — needs only the spelling.
pub(crate) fn latex_spelling(c: char) -> Option<&'static str> {
    lookup(c).map(|(_, latex)| latex)
}
