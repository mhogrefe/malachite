// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
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

// A superscript or subscript needs a base of its own. Two in a row would stack, so that "x¹₂"
// would be drawn with the one above the two rather than beside it, and two of the same kind would
// be a double superscript, which is an error. Every script group is therefore followed by something
// other than another script marker.
pub fn assert_scripts_separated(s: &str) {
    let cs = s.chars().collect_vec();
    let mut i = 0;
    while i < cs.len() {
        let k = cs[i];
        if k == '\\' {
            // An escaped character, or the first letter of a control word; the rest of a control
            // word is letters, which this scan passes over harmlessly.
            i += 2;
            continue;
        }
        if k != '^' && k != '_' {
            i += 1;
            continue;
        }
        i += 1;
        assert!(i < cs.len(), "trailing {k:?} in {s:?}");
        // Skip the script's argument: a braced group, a control word, or a single character.
        match cs[i] {
            '{' => {
                let mut depth = 0;
                loop {
                    match cs[i] {
                        '\\' => i += 1,
                        '{' => depth += 1,
                        '}' => depth -= 1,
                        _ => {}
                    }
                    i += 1;
                    if depth == 0 {
                        break;
                    }
                }
            }
            '\\' => {
                i += 1;
                while i < cs.len() && cs[i].is_ascii_alphabetic() {
                    i += 1;
                }
            }
            _ => i += 1,
        }
        if let Some(&next) = cs.get(i) {
            assert!(
                next != '^' && next != '_',
                "two scripts share a base in {s:?}"
            );
        }
    }
}

// Canaries for `assert_scripts_separated`: a checker that accepted everything would pass every
// property test in silence.
#[test]
#[should_panic(expected = "two scripts share a base")]
fn assert_scripts_separated_catches_double_superscript() {
    assert_scripts_separated(r"\text{x}^1^2");
}

#[test]
#[should_panic(expected = "two scripts share a base")]
fn assert_scripts_separated_catches_stacking() {
    assert_scripts_separated(r"\text{x}^{10}_{2}");
}

#[test]
fn assert_scripts_separated_accepts_separated_scripts() {
    // an escaped underscore is not a subscript, and neither is one inside a control word
    assert_scripts_separated(r"\text{a\_b}");
    assert_scripts_separated(r"\text{x}^1{}_2{}^3");
    assert_scripts_separated(r"{}^\circ\mathrm{F}");
    assert_scripts_separated(r"\text{2}^{10}\text{ + 1}");
}

#[test]
fn test_char_to_latex() {
    let test = |c: char, out: &str| assert_eq!(c.to_latex_string(), out);
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
    // a superscript or subscript is math-mode, and gets an empty base to attach to
    test('¹', r"{}^1");
    test('²', r"{}^2");
    test('³', r"{}^3");
    test('⁰', r"{}^0");
    test('⁹', r"{}^9");
    test('⁻', r"{}^-");
    test('ⁿ', r"{}^n");
    test('₀', r"{}_0");
    test('₂', r"{}_2");
    test('₉', r"{}_9");
    test('₊', r"{}_+");
    test('ₙ', r"{}_n");
}

#[test]
fn char_to_latex_properties() {
    char_gen().test_properties(|c| {
        let s = c.to_latex_string();
        assert!(!s.is_empty());
        assert_specials_escaped(&s);
        assert_braces_balanced(&s);
        assert_scripts_separated(&s);
        if s.starts_with(r"\text{") {
            assert!(s.ends_with('}'));
        }
        // A fragment never opens with a bare superscript or subscript, which would have no base.
        assert!(!s.starts_with('^') && !s.starts_with('_'));
        // A raw newline could become a paragraph break, which is an error in math mode.
        assert!(!s.contains('\n'));
    });
}
