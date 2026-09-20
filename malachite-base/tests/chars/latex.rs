// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::strings::latex::ToLatex;
use malachite_base::test_util::generators::char_gen;

// A LaTeX fragment must never contain an unescaped `%`, which comments out the rest of the line,
// nor an unescaped `$`, `#`, or `&`. Each must be immediately preceded by a backslash.
pub fn assert_specials_escaped(s: &str) {
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if matches!(b, b'%' | b'$' | b'#' | b'&') {
            assert!(
                i > 0 && bytes[i - 1] == b'\\',
                "unescaped {:?} in {s:?}",
                char::from(b)
            );
        }
    }
}

// Braces must balance, or the fragment cannot be embedded in a group.
pub fn assert_braces_balanced(s: &str) {
    let mut depth = 0i32;
    let mut escaped = false;
    for c in s.chars() {
        match c {
            _ if escaped => escaped = false,
            '\\' => escaped = true,
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                assert!(depth >= 0, "unbalanced braces in {s:?}");
            }
            _ => {}
        }
    }
    assert_eq!(depth, 0, "unbalanced braces in {s:?}");
}

#[test]
fn test_char_to_latex() {
    let test = |c: char, out: &str| assert_eq!(c.to_latex().to_string(), out);
    // ordinary characters are typeset as themselves, inside a text group
    test('a', r"\text{a}");
    test('Z', r"\text{Z}");
    test('7', r"\text{7}");
    test(' ', r"\text{ }");
    // every LaTeX special character is escaped
    test('#', r"\text{\#}");
    test('$', r"\text{\$}");
    test('%', r"\text{\%}");
    test('&', r"\text{\&}");
    test('_', r"\text{\_}");
    test('{', r"\text{\{}");
    test('}', r"\text{\}}");
    // these three are control words, so they are braced to keep them from swallowing what follows
    test('\\', r"\text{{\textbackslash}}");
    test('^', r"\text{{\textasciicircum}}");
    test('~', r"\text{{\textasciitilde}}");
    // characters LaTeX spells with a math macro stand outside the text group
    test('α', r"\alpha");
    test('∞', r"\infty");
    test('≤', r"\leq");
    test('±', r"\pm");
    test('<', "<");
    // an accented letter keeps its text-mode spelling
    test('é', r"\text{{\'e}}");
    // a character with no LaTeX spelling is written literally
    test('漢', r"\text{漢}");
}

#[test]
fn char_to_latex_properties() {
    char_gen().test_properties(|c| {
        let s = c.to_latex().to_string();
        assert!(!s.is_empty());
        assert_specials_escaped(&s);
        assert_braces_balanced(&s);
        if s.starts_with(r"\text{") {
            assert!(s.ends_with('}'));
        }
        // A fragment never opens with a bare superscript or subscript, which would have no base.
        assert!(!s.starts_with('^') && !s.starts_with('_'));
        // A raw newline could become a paragraph break, which is an error in math mode.
        assert!(!s.contains('\n'));
    });
}
