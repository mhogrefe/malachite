// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::chars::latex_table::LATEX_TABLE;
use crate::strings::typst::ToTypst;
use core::fmt::{Formatter, Result, Write};

// Gives whether a `char` is a superscript or a subscript, and what it stands for.
//
// Which characters these are, and what each one means, is a fact about Unicode rather than about
// any one typesetting language, and the generated LaTeX table already records it: those characters
// are exactly the ones it spells `^0`, `_1`, and so on. Reading the answer off that table keeps
// there from being a second table to maintain alongside it.
fn script(c: char) -> Option<(char, &'static str)> {
    let i = LATEX_TABLE.binary_search_by_key(&c, |&(k, _, _)| k).ok()?;
    let (_, is_math, latex) = LATEX_TABLE[i];
    if !is_math {
        return None;
    }
    match latex.as_bytes().first() {
        Some(&k @ (b'^' | b'_')) => Some((char::from(k), &latex[1..])),
        _ => None,
    }
}

// Writes a sequence of `char`s as one Typst math-mode fragment.
//
// Typst reads Unicode natively, and a quoted string renders its contents as text, so almost every
// string is one string literal and nothing else. Only `\` and `"` have to be escaped, and only
// control characters have to be spelled rather than written, since a raw one in a string literal is
// at best invisible.
//
// The superscript and subscript characters are the exception. A text font often lacks the
// Superscripts and Subscripts block, so a literal `⁰` may not render at all, and a run of them is
// what the reader means as one script in any case.
pub(crate) fn fmt_typst_chars<I: Iterator<Item = char>>(cs: I, f: &mut Formatter) -> Result {
    let mut cs = cs.peekable();
    let mut in_string = false;
    let mut any = false;
    // Whether a script written next would have nothing to attach to: either nothing has been
    // written yet, or what was written last is itself a script, and a second one on the same base
    // would stack the two rather than set them side by side.
    let mut needs_base = true;
    while let Some(c) = cs.next() {
        any = true;
        if let Some((kind, head)) = script(c) {
            if in_string {
                f.write_char('"')?;
                in_string = false;
            }
            if needs_base {
                f.write_str("\"\"")?;
            }
            f.write_char(kind)?;
            // A run of them is one script, so that "2¹⁰" is two raised to the tenth rather than
            // two raised to the first and then to the zeroth. The parentheses are what make it one,
            // and they group without being drawn.
            //
            // What they hold is a quoted string, like the rest of the fragment. That is not only
            // for consistency: `'⁽'` stands for a parenthesis, and written bare it would close
            // the script's own grouping, leaving `^(()`.
            f.write_str("(\"")?;
            f.write_str(head)?;
            while let Some((k, tail)) = cs.peek().copied().and_then(script) {
                if k != kind {
                    break;
                }
                cs.next();
                f.write_str(tail)?;
            }
            f.write_str("\")")?;
            needs_base = true;
            continue;
        }
        needs_base = false;
        if !in_string {
            f.write_char('"')?;
            in_string = true;
        }
        match c {
            '\\' => f.write_str("\\\\")?,
            '"' => f.write_str("\\\"")?,
            '\n' => f.write_str("\\n")?,
            '\r' => f.write_str("\\r")?,
            '\t' => f.write_str("\\t")?,
            _ if c.is_control() => write!(f, "\\u{{{:x}}}", u32::from(c))?,
            _ => f.write_char(c)?,
        }
    }
    if in_string {
        f.write_char('"')
    } else if any {
        Ok(())
    } else {
        // An empty sequence still says "this is a string", rather than vanishing.
        f.write_str("\"\"")
    }
}

impl ToTypst for char {
    /// Writes a [`char`] as a Typst math-mode fragment.
    ///
    /// The character is written inside a quoted string, where Typst typesets it as itself. Typst
    /// reads Unicode natively, so a character needs no spelling of its own; only `\` and `"`, and
    /// the control characters, are written as escapes.
    ///
    /// A superscript or subscript character is the exception: it becomes a real script, and is
    /// given an empty base to attach to, since on its own it has none. `'²'` becomes `""^(2)`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::strings::typst::ToTypst;
    ///
    /// assert_eq!('a'.to_typst_string(), r#""a""#);
    /// assert_eq!('%'.to_typst_string(), r#""%""#);
    /// assert_eq!('α'.to_typst_string(), r#""α""#);
    /// assert_eq!('"'.to_typst_string(), r#""\"""#);
    /// assert_eq!('²'.to_typst_string(), r#"""^("2")"#);
    /// ```
    ///
    /// | value | fragment |
    /// |-------|----------|
    /// | `'a'` | `"a"`    |
    /// | `'%'` | `"%"`    |
    /// | `'α'` | `"α"`    |
    /// | `'"'` | `"\""`   |
    /// | `'²'` | `""^("2")` |
    #[inline]
    fn fmt_typst(&self, f: &mut Formatter) -> Result {
        fmt_typst_chars(core::iter::once(*self), f)
    }
}
