// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::strings::typst::{
    assert_delimiters_balanced, assert_scripts_separated, assert_strings_closed,
    assert_typst_compiles,
};
use malachite_base::strings::typst::ToTypst;
use malachite_base::test_util::generators::char_gen;

#[test]
fn test_char_to_typst() {
    let mut frags = Vec::new();
    let mut test = |c: char, out: &str| {
        assert_eq!(c.to_typst().to_string(), out);
        frags.push(out.to_string());
    };
    // an ordinary character is typeset as itself, inside a quoted string
    test('a', r#""a""#);
    test('Z', r#""Z""#);
    test('7', r#""7""#);
    test(' ', r#"" ""#);
    // LaTeX's special characters are not Typst's, and need no escape
    test('#', r##""#""##);
    test('$', r#""$""#);
    test('%', r#""%""#);
    test('&', r#""&""#);
    test('_', r#""_""#);
    test('{', r#""{""#);
    test('}', r#""}""#);
    test('^', r#""^""#);
    test('~', r#""~""#);
    // only a backslash and a quotation mark do
    test('\\', r#""\\""#);
    test('"', r#""\"""#);
    // a control character is spelled rather than written
    test('\n', r#""\n""#);
    test('\r', r#""\r""#);
    test('\t', r#""\t""#);
    test('\u{b}', r#""\u{b}""#);
    test('\0', r#""\u{0}""#);
    // Typst reads Unicode natively, so nothing else needs a spelling of its own
    test('α', r#""α""#);
    test('∞', r#""∞""#);
    test('≤', r#""≤""#);
    test('±', r#""±""#);
    test('é', r#""é""#);
    test('漢', r#""漢""#);
    // a superscript or subscript is a real script, and gets an empty base to attach to
    test('¹', r#"""^("1")"#);
    test('²', r#"""^("2")"#);
    test('³', r#"""^("3")"#);
    test('⁰', r#"""^("0")"#);
    test('⁹', r#"""^("9")"#);
    test('⁻', r#"""^("-")"#);
    test('ⁿ', r#"""^("n")"#);
    test('₀', r#"""_("0")"#);
    test('₂', r#"""_("2")"#);
    test('₉', r#"""_("9")"#);
    test('₊', r#"""_("+")"#);
    test('ₙ', r#"""_("n")"#);
    assert_typst_compiles(&frags);
}

#[test]
fn char_to_typst_properties() {
    let mut frags = Vec::new();
    char_gen().test_properties(|c| {
        let s = c.to_typst().to_string();
        assert!(!s.is_empty());
        assert_strings_closed(&s);
        assert_delimiters_balanced(&s);
        assert_scripts_separated(&s);
        // A raw newline would end the math block it is written in.
        assert!(!s.contains('\n'));
        // A fragment never opens with a bare script, which Typst rejects for want of a base.
        assert!(!s.starts_with('^') && !s.starts_with('_'));
        frags.push(s);
    });
    // Typst itself has the last word on every fragment the generator produced.
    assert_typst_compiles(&frags);
}
