// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::chars::latex::{
    assert_braces_balanced, assert_scripts_separated, assert_specials_escaped,
};
use malachite_base::strings::latex::ToLatex;
use malachite_base::test_util::generators::string_gen;

#[test]
fn test_str_to_latex() {
    let test = |s: &str, out: &str| {
        assert_eq!(s.to_latex().to_string(), out);
        // The `String` implementation agrees with the `&str` one.
        assert_eq!(s.to_string().to_latex().to_string(), out);
    };
    // ordinary text is gathered into one group; no quotation marks are added
    test("hello", r"\text{hello}");
    test("", r"\text{}");
    test("hello world", r"\text{hello world}");
    // LaTeX's special characters are escaped
    test("100%", r"\text{100\%}");
    test("a_b", r"\text{a\_b}");
    test("#$&{}", r"\text{\#\$\&\{\}}");
    // control words are braced so they cannot swallow the letter after them
    test(r"\n", r"\text{{\textbackslash}n}");
    test("x^2", r"\text{x{\textasciicircum}2}");
    // math-mode characters leave the text group, and it reopens after them
    test("α", r"\alpha");
    test("100% α", r"\text{100\% }\alpha");
    test("A ≤ B", r"\text{A }\leq\text{ B}");
    test("a<b", r"\text{a}<\text{b}");
    // ligatures are broken, so the depiction keeps the characters the string actually holds
    test("--flag", r"\text{-{}-flag}");
    test("a--b", r"\text{a-{}-b}");
    // runs of spaces are preserved rather than collapsed, and so are other space-like characters
    test("a  b", r"\text{a \ b}");
    test("a\tb", r"\text{a\ b}");
    test("a\nb", r"\text{a\ b}");
    // characters with no LaTeX spelling are written literally
    test("漢字", r"\text{漢字}");
    test("café", r"\text{caf{\'e}}");
    // a run of superscript characters is one superscript, not one per character
    test("2¹⁰", r"\text{2}^{10}");
    test("2⁴⁵", r"\text{2}^{45}");
    test("¹²³⁴⁵⁶⁷⁸⁹⁰", r"{}^{1234567890}");
    // and so is a run of subscript characters
    test("H₂O", r"\text{H}_2\text{O}");
    test("a₁₂", r"\text{a}_{12}");
    test("₀₁₂₃₄₅₆₇₈₉", r"{}_{0123456789}");
    // a single one keeps its own spelling, unbraced
    test("2⁰", r"\text{2}^0");
    test("x²", r"\text{x}^2");
    test("x₀", r"\text{x}_0");
    // the run is of every superscript or subscript character, not just the digits
    test("10⁻³", r"\text{10}^{-3}");
    test("aₙ₊₁", r"\text{a}_{n+1}");
    test("x⁽ⁿ⁾", r"\text{x}^{(n)}");
    // one kind does not run into the other: they would stack rather than sit side by side
    test("x¹₂", r"\text{x}^1{}_2");
    test("x₂¹", r"\text{x}_2{}^1");
    test("x¹₂³", r"\text{x}^1{}_2{}^3");
    // a script attaches to whatever precedes it, and gets an empty base when nothing does
    test("α¹", r"\alpha^1");
    test(" ¹", r"\text{ }^1");
    test("¹", r"{}^1");
    test("⁰⁰", r"{}^{00}");
    // the run ends where the superscript characters do
    test("2¹⁰ + 1", r"\text{2}^{10}\text{ + 1}");
}

#[test]
fn str_to_latex_properties() {
    string_gen().test_properties(|s| {
        let latex = s.to_latex().to_string();
        assert!(!latex.is_empty());
        assert_specials_escaped(&latex);
        assert_braces_balanced(&latex);
        assert_scripts_separated(&latex);
        // A raw newline could become a paragraph break, which is an error in math mode.
        assert!(!latex.contains('\n'));
        // A fragment never opens with a bare superscript or subscript, which would have no base.
        assert!(!latex.starts_with('^') && !latex.starts_with('_'));
        // The `String` implementation agrees with the `&str` one.
        assert_eq!(s.clone().to_latex().to_string(), latex);
        // A one-character string agrees with that character on its own.
        let mut cs = s.chars();
        if let (Some(c), None) = (cs.next(), cs.next()) {
            assert_eq!(c.to_latex().to_string(), latex);
        }
    });
}
