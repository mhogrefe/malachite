// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::strings::latex::ToLatex;
use malachite_q::Rational;
use malachite_q::test_util::generators::rational_gen;
use std::str::FromStr;

// Turns each fraction back into the `a/b` that `Display` writes, so that a fragment can be compared
// with it directly. A numerator and a denominator hold only digits and possibly an `i`, so there is
// nothing nested to worry about.
fn undo_fractions(s: &str) -> String {
    let mut out = String::new();
    let mut rest = s;
    while let Some(i) = rest.find(r"\frac{") {
        out.push_str(&rest[..i]);
        rest = &rest[i + 6..];
        let mid = rest.find("}{").unwrap();
        let numerator = &rest[..mid];
        rest = &rest[mid + 2..];
        let end = rest.find('}').unwrap();
        out.push_str(numerator);
        out.push('/');
        out.push_str(&rest[..end]);
        rest = &rest[end + 1..];
    }
    out.push_str(rest);
    out
}

#[test]
fn test_rational_to_latex() {
    let test = |s: &str, out: &str| {
        assert_eq!(Rational::from_str(s).unwrap().to_latex_string(), out);
    };
    // a denominator of 1 is no fraction at all
    test("0", "0");
    test("123", "123");
    test("-123", "-123");
    // and otherwise it is
    test("22/7", r"\frac{22}{7}");
    test("1/2", r"\frac{1}{2}");
    // the sign goes outside the fraction, never into the numerator
    test("-2/3", r"-\frac{2}{3}");
    test("-1/2", r"-\frac{1}{2}");
}

#[test]
fn rational_to_latex_properties() {
    rational_gen().test_properties(|x| {
        let s = x.to_latex_string();
        // Undoing the fraction gives back exactly what `Display` writes.
        assert_eq!(undo_fractions(&s), x.to_string());
        // A minus sign never appears inside a fraction.
        assert!(!s.contains(r"\frac{-"));
        // A fragment is empty for no value.
        assert!(!s.is_empty());
    });
}
