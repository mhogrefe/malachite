// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::strings::typst::ToTypst;
use malachite_base::test_util::generators::string_gen;
use typst_as_lib::TypstEngine;

// Compiles some fragments with Typst itself, each in its own math block, and gives the first
// message if any of them is rejected.
//
// Every fragment is supposed to be valid wherever Typst is in math mode, which is not something a
// scan of the output can establish: only Typst knows whether `infinity` is a symbol it has, or
// whether a script may stand without a base. Since building an engine costs far more than compiling
// one small block, a whole batch goes into one document.
fn compile(frags: &[String]) -> Result<(), String> {
    let src = frags.iter().map(|frag| format!("$ {frag} $")).join("\n\n");
    let engine = TypstEngine::builder()
        .main_file(src)
        .fonts(typst_assets::fonts().collect_vec())
        .build();
    match engine.compile::<typst_layout::PagedDocument>().output {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{e:?}")),
    }
}

/// Asserts that Typst accepts every one of these fragments in math mode.
///
/// A batch that fails is bisected, so that the message names the fragment at fault rather than the
/// whole batch.
pub fn assert_typst_compiles(frags: &[String]) {
    if compile(frags).is_ok() {
        return;
    }
    let mut frags = frags;
    while frags.len() > 1 {
        let (a, b) = frags.split_at(frags.len() / 2);
        frags = if compile(a).is_err() { a } else { b };
    }
    if let Err(e) = compile(frags) {
        panic!("Typst rejects {:?}: {e}", frags[0]);
    }
}

/// Asserts that a fragment leaves no string literal open, which would swallow whatever is written
/// after it.
pub fn assert_strings_closed(s: &str) {
    let mut in_string = false;
    let mut escaped = false;
    for c in s.chars() {
        match c {
            _ if escaped => escaped = false,
            '\\' if in_string => escaped = true,
            '"' => in_string = !in_string,
            _ => {}
        }
    }
    assert!(!in_string, "unclosed string literal in {s:?}");
}

/// Asserts that a fragment's square brackets and parentheses balance, so that it can be embedded in
/// a larger expression. Delimiters inside a string literal are text, and are skipped.
pub fn assert_delimiters_balanced(s: &str) {
    let mut depth = 0i32;
    let mut in_string = false;
    let mut escaped = false;
    for c in s.chars() {
        match c {
            _ if escaped => escaped = false,
            '\\' if in_string => escaped = true,
            '"' => in_string = !in_string,
            _ if in_string => {}
            '[' | '(' => depth += 1,
            ']' | ')' => {
                depth -= 1;
                assert!(depth >= 0, "unbalanced delimiters in {s:?}");
            }
            _ => {}
        }
    }
    assert_eq!(depth, 0, "unbalanced delimiters in {s:?}");
}

/// Asserts that no two scripts share a base: two in a row would stack, so that "x¹₂" would be
/// drawn with the one above the two rather than beside it. Every script is followed by something
/// other than another script marker.
pub fn assert_scripts_separated(s: &str) {
    let cs = s.chars().collect_vec();
    let mut i = 0;
    let mut in_string = false;
    while i < cs.len() {
        match cs[i] {
            '\\' if in_string => {
                i += 2;
                continue;
            }
            '"' => in_string = !in_string,
            k @ ('^' | '_') if !in_string => {
                i += 1;
                assert_eq!(cs.get(i), Some(&'('), "{k:?} without a group in {s:?}");
                let mut depth = 0;
                let mut in_group_string = false;
                loop {
                    match cs[i] {
                        '\\' if in_group_string => i += 1,
                        '"' => in_group_string = !in_group_string,
                        _ if in_group_string => {}
                        '(' => depth += 1,
                        ')' => depth -= 1,
                        _ => {}
                    }
                    i += 1;
                    if depth == 0 {
                        break;
                    }
                }
                if let Some(&next) = cs.get(i) {
                    assert!(
                        next != '^' && next != '_',
                        "two scripts share a base in {s:?}"
                    );
                }
                continue;
            }
            _ => {}
        }
        i += 1;
    }
}

#[test]
fn test_str_to_typst() {
    let mut frags = Vec::new();
    let mut test = |s: &str, out: &str| {
        assert_eq!(s.to_typst().to_string(), out);
        // The `String` implementation agrees with the `&str` one.
        assert_eq!(s.to_string().to_typst().to_string(), out);
        frags.push(out.to_string());
    };
    // ordinary text is one quoted string; no quotation marks beyond its own are added
    test("hello", r#""hello""#);
    test("", r#""""#);
    test("hello world", r#""hello world""#);
    // Typst reads Unicode natively, so nothing needs a spelling of its own
    test("100% α", r#""100% α""#);
    test("A ≤ B", r#""A ≤ B""#);
    test("café", r#""café""#);
    test("漢字", r#""漢字""#);
    // LaTeX's special characters are not Typst's, and need no escape
    test("#$&{}~^_", r##""#$&{}~^_""##);
    test("100%", r#""100%""#);
    // only a backslash and a quotation mark do
    test(r"back\slash", r#""back\\slash""#);
    test("he said \"hi\"", r#""he said \"hi\"""#);
    // a control character is spelled rather than written
    test("a\nb", r#""a\nb""#);
    test("a\tb", r#""a\tb""#);
    test("a\rb", r#""a\rb""#);
    test("a\u{b}b", r#""a\u{b}b""#);
    // Typst applies no ligature to `--`, and preserves a run of spaces, so both stand as they are
    test("--flag", r#""--flag""#);
    test("a  b", r#""a  b""#);
    // a run of superscript characters is one superscript, not one per character
    test("2¹⁰", r#""2"^("10")"#);
    test("2⁴⁵", r#""2"^("45")"#);
    test("¹²³⁴⁵⁶⁷⁸⁹⁰", r#"""^("1234567890")"#);
    // and so is a run of subscript characters
    test("H₂O", r#""H"_("2")"O""#);
    test("a₁₂", r#""a"_("12")"#);
    test("₀₁₂₃₄₅₆₇₈₉", r#"""_("0123456789")"#);
    // the run is of every superscript or subscript character, not just the digits
    test("10⁻³", r#""10"^("-3")"#);
    test("aₙ₊₁", r#""a"_("n+1")"#);
    test("x⁽ⁿ⁾", r#""x"^("(n)")"#);
    // one kind does not run into the other: they would stack rather than sit side by side
    test("x¹₂", r#""x"^("1")""_("2")"#);
    test("x₂¹", r#""x"_("2")""^("1")"#);
    test("x¹₂³", r#""x"^("1")""_("2")""^("3")"#);
    // a script attaches to whatever precedes it, and gets an empty base when nothing does
    test("α¹", r#""α"^("1")"#);
    test(" ¹", r#"" "^("1")"#);
    test("¹", r#"""^("1")"#);
    test("⁰⁰", r#"""^("00")"#);
    // the run ends where the superscript characters do
    test("2¹⁰ + 1", r#""2"^("10")" + 1""#);
    assert_typst_compiles(&frags);
}

#[test]
fn str_to_typst_properties() {
    let mut frags = Vec::new();
    string_gen().test_properties(|s| {
        let typst = s.to_typst().to_string();
        assert!(!typst.is_empty());
        assert_strings_closed(&typst);
        assert_delimiters_balanced(&typst);
        assert_scripts_separated(&typst);
        // A raw newline would end the math block it is written in.
        assert!(!typst.contains('\n'));
        // A fragment never opens with a bare script, which Typst rejects for want of a base.
        assert!(!typst.starts_with('^') && !typst.starts_with('_'));
        // The `String` implementation agrees with the `&str` one.
        assert_eq!(s.clone().to_typst().to_string(), typst);
        // A one-character string agrees with that character on its own.
        let mut cs = s.chars();
        if let (Some(c), None) = (cs.next(), cs.next()) {
            assert_eq!(c.to_typst().to_string(), typst);
        }
        frags.push(typst);
    });
    // Typst itself has the last word on every fragment the generator produced.
    assert_typst_compiles(&frags);
}
