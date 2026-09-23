// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;
use malachite_base::vars::greek::GreekVars;
use malachite_base::vars::indexed::IndexedVars;
use malachite_base::vars::list::ListVars;
use malachite_base::vars::xyz::XyzVars;
use malachite_base::vars::{VarScheme, char_is_reserved};
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::integer_polynomial_gen;

#[test]
fn test_to_string() {
    let test = |s| {
        assert_eq!(IntegerPolynomial::from_str(s).unwrap().to_string(), s);
    };
    test("0");
    test("1");
    test("5");
    test("x");
    test("x^2");
    test("2*x");
    test("2*x^3");
    test("x+1");
    test("x^2+3*x+2");
    test("x^10+x");
    test("123456789012345678901234567890*x^2+1");
}

#[test]
fn test_to_string_normalizes() {
    // Everything `FromStr` accepts comes back in the one form `Display` writes.
    let test = |s, out| {
        assert_eq!(IntegerPolynomial::from_str(s).unwrap().to_string(), out);
    };
    test("2+3*x+x^2", "x^2+3*x+2");
    test("x^1", "x");
    test("007", "7");
    test("1*x", "x");
}

#[test]
fn test_to_string_with() {
    let p = IntegerPolynomial::from_str("x^2+3*x+2").unwrap();
    assert_eq!(p.to_string_with(XyzVars.var(0)), "x^2+3*x+2");
    assert_eq!(p.to_string_with(XyzVars.var(1)), "y^2+3*y+2");
    assert_eq!(p.to_string_with(GreekVars.var(0)), "α^2+3*α+2");
    assert_eq!(p.to_string_with(IndexedVars.var(0)), "x₀^2+3*x₀+2");
    assert_eq!(p.to_string_with(IndexedVars.var(10)), "x₁₀^2+3*x₁₀+2");
    let vars = ListVars::new(["price"]);
    assert_eq!(p.to_string_with(vars.var(0)), "price^2+3*price+2");
}

#[test]
fn to_string_properties() {
    integer_polynomial_gen().test_properties(|p| {
        let s = p.to_string();
        // The string reads back as the polynomial it was written from.
        assert_eq!(IntegerPolynomial::from_str(&s).unwrap(), p);
        assert!(!s.is_empty());
        // Nothing in the output can be mistaken for a variable's name.
        assert!(!s.contains(' '));
        // A `-` appears exactly when some coefficient is negative, and never at the end.
        assert_eq!(
            s.contains('-'),
            p.coefficients_asc().iter().any(|c| *c < 0u32)
        );
        assert!(!s.ends_with('-') && !s.ends_with('+'));
        assert_eq!(s == "0", p == IntegerPolynomial::ZERO);

        // Naming the variable something else changes only the variable.
        let greek = p.to_string_with(GreekVars.var(0));
        assert_eq!(
            IntegerPolynomial::from_string_with(GreekVars.var(0), &greek).unwrap(),
            p
        );
        assert_eq!(greek.replace('α', "x"), s);
        // The default is the first `XyzVars` variable.
        assert_eq!(p.to_string_with(XyzVars.var(0)), s);

        // A name of more than one character is written out in full, and is still readable back.
        let vars = ListVars::new(["price"]);
        let named = p.to_string_with(vars.var(0));
        assert_eq!(
            IntegerPolynomial::from_string_with(vars.var(0), &named).unwrap(),
            p
        );
        assert_eq!(named.replace("price", "x"), s);

        // Every character is either part of a name or reserved.
        for c in s.chars() {
            assert!(c == 'x' || char_is_reserved(c), "{c:?} in {s:?}");
        }
    });
}
