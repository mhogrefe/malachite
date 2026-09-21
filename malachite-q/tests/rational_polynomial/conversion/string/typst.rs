// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::strings::latex::ToLatex;
use malachite_base::strings::typst::ToTypst;
use malachite_base::vars::VarScheme;
use malachite_base::vars::greek::GreekVars;
use malachite_base::vars::indexed::IndexedVars;
use malachite_base::vars::list::ListVars;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::rational_polynomial_gen;

#[test]
fn test_to_typst() {
    let test = |s, out| {
        assert_eq!(
            RationalPolynomial::from_str(s).unwrap().to_typst_string(),
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
    // A superscript of more than one digit is parenthesized.
    test("x^9", "x^9");
    test("x^10", "x^(10)");
    test("x^12+x^2", "x^(12)+x^2");
    test("x^100+7", "x^(100)+7");
}

#[test]
fn test_to_typst_string_with() {
    let p = RationalPolynomial::from_str("x^2+3*x+2").unwrap();
    assert_eq!(p.to_typst_string_with(GreekVars.var(0)), "α^2+3α+2");
    assert_eq!(p.to_typst_string_with(IndexedVars.var(7)), "x_7^2+3x_7+2");
    assert_eq!(
        p.to_typst_string_with(IndexedVars.var(42)),
        "x_(42)^2+3x_(42)+2"
    );
    let vars = ListVars::new(["price"]);
    assert_eq!(
        p.to_typst_string_with(vars.var(0)),
        "\"price\"^2+3\"price\"+2"
    );
}

#[test]
fn to_typst_properties() {
    rational_polynomial_gen().test_properties(|p| {
        let s = p.to_typst_string();
        assert!(!s.is_empty());
        // Delimiters balance, so the fragment can be embedded in a larger expression.
        let mut depth = 0i32;
        for c in s.chars() {
            match c {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    assert!(depth >= 0, "unbalanced parentheses in {s:?}");
                }
                _ => {}
            }
        }
        assert_eq!(depth, 0, "unbalanced parentheses in {s:?}");

        assert_eq!(s.matches('+').count(), p.to_string().matches('+').count());
        assert!(!s.contains('*'));
        // Unlike an integer coefficient, a rational one is spelled differently in the two languages
        // — `frac(1, 2)` against `\\frac{1}{2}` — so the two forms are not each other with the
        // brackets swapped. What they do share is their terms and their signs.
        let latex = p.to_latex_string();
        assert_eq!(s.matches('+').count(), latex.matches('+').count());
        assert_eq!(s.starts_with('-'), latex.starts_with('-'));
        assert_eq!(s == "0", latex == "0");
    });
}
