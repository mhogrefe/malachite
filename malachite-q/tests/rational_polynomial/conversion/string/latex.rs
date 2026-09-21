// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::strings::latex::ToLatex;
use malachite_base::vars::VarScheme;
use malachite_base::vars::greek::GreekVars;
use malachite_base::vars::indexed::IndexedVars;
use malachite_base::vars::list::ListVars;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::rational_polynomial_gen;

// A LaTeX fragment must never hold an unescaped character that LaTeX reads as markup: a `%`
// comments out the rest of the line, and `$`, `#`, and `&` are special too.
fn assert_specials_escaped(s: &str) {
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
fn assert_braces_balanced(s: &str) {
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
fn test_to_latex() {
    let test = |s, out| {
        assert_eq!(
            RationalPolynomial::from_str(s).unwrap().to_latex_string(),
            out
        );
    };
    test("0", "0");
    test("1", "1");
    test("5", "5");
    test("x", "x");
    test("x^2", "x^2");
    test("2*x", "2x");
    test("2*x^3", "2x^3");
    test("x^2+3*x+2", "x^2+3x+2");
    // A superscript of more than one digit is braced.
    test("x^9", "x^9");
    test("x^10", "x^{10}");
    test("x^12+x^2", "x^{12}+x^2");
    test("x^100+7", "x^{100}+7");
}

#[test]
fn test_to_latex_string_with() {
    let p = RationalPolynomial::from_str("x^2+3*x+2").unwrap();
    assert_eq!(
        p.to_latex_string_with(GreekVars.var(0)),
        r"\alpha^2+3\alpha+2"
    );
    assert_eq!(p.to_latex_string_with(IndexedVars.var(7)), "x_7^2+3x_7+2");
    assert_eq!(
        p.to_latex_string_with(IndexedVars.var(42)),
        "x_{42}^2+3x_{42}+2"
    );
    let vars = ListVars::new(["price"]);
    assert_eq!(
        p.to_latex_string_with(vars.var(0)),
        r"\text{price}^2+3\text{price}+2"
    );
}

#[test]
fn to_latex_properties() {
    rational_polynomial_gen().test_properties(|p| {
        let s = p.to_latex_string();
        assert!(!s.is_empty());
        assert_specials_escaped(&s);
        assert_braces_balanced(&s);
        // A LaTeX fragment is the plain string with the `*`s taken out and the long exponents
        // braced, so the two have the same terms.
        assert_eq!(s.matches('+').count(), p.to_string().matches('+').count());
        // The variable is named the same way the plain form names it.
        assert_eq!(s.contains('x'), p.degree().is_some_and(|d| d > 0));
        assert!(!s.contains('*'));

        for var in [GreekVars.var(0), GreekVars.var(3)] {
            let t = p.to_latex_string_with(var);
            assert!(!t.is_empty());
            assert_specials_escaped(&t);
            assert_braces_balanced(&t);
        }
    });
}
