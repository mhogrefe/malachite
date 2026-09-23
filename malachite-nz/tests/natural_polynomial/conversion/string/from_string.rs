// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::generators::string_gen;
use malachite_base::vars::VarScheme;
use malachite_base::vars::greek::GreekVars;
use malachite_base::vars::xyz::XyzVars;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::natural_polynomial_gen;

#[test]
fn test_from_str() {
    let test = |s, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
    };
    test("0", "0");
    test("5", "5");
    test("x", "x");
    test("x^2+3*x+2", "x^2+3*x+2");
    // The terms may come in any order.
    test("2+3*x+x^2", "x^2+3*x+2");
    test("3*x+x^2+2", "x^2+3*x+2");
    // An exponent may be written `^1`, and a coefficient may have leading zeros.
    test("x^1", "x");
    test("007", "7");
    test("0007*x^2", "7*x^2");
    test("1*x^2", "x^2");
}

#[test]
fn test_from_str_fail() {
    let test = |s| {
        assert!(
            NaturalPolynomial::from_str(s).is_err(),
            "{s:?} should not parse"
        );
    };
    test("");
    // A variable other than the one asked for.
    test("y");
    test("y^2+1");
    test("α");
    // Spaces are reserved, so they cannot be anywhere.
    test("x^2 + 1");
    test("x ^2");
    // A zero coefficient in a term, which `Display` never writes.
    test("0*x");
    test("x^2+0");
    test("0+x");
    // Two terms of the same degree.
    test("x+x");
    test("x^2+2*x^2");
    test("1+1");
    // An exponent of zero, which is a term with no variable.
    test("x^0");
    // Negatives, which no `NaturalPolynomial` has.
    test("-x");
    test("-1");
    test("x-1");
    // Malformed syntax.
    test("+x");
    test("x+");
    test("x^");
    test("^2");
    test("x**2");
    test("x^^2");
    test("*x");
    test("x^2.5");
    test("(x+1)");
    // An exponent too large for a `u64`.
    test("x^99999999999999999999999999");
}

#[test]
fn test_from_string_with() {
    let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    assert_eq!(
        NaturalPolynomial::from_string_with(GreekVars.var(0), "α^2+3*α+2").unwrap(),
        p
    );
    assert_eq!(
        NaturalPolynomial::from_string_with(XyzVars.var(1), "y^2+3*y+2").unwrap(),
        p
    );
    // The variable must be the one that was asked for: another variable of the same scheme is no
    // more acceptable than one from a different scheme.
    assert!(NaturalPolynomial::from_string_with(GreekVars.var(0), "β^2").is_none());
    assert!(NaturalPolynomial::from_string_with(GreekVars.var(0), "x^2").is_none());
    assert!(NaturalPolynomial::from_string_with(XyzVars.var(1), "x^2").is_none());
}

#[test]
fn from_str_properties() {
    natural_polynomial_gen().test_properties(|p| {
        // Whatever `Display` writes reads back, whichever variable it was written with.
        assert_eq!(NaturalPolynomial::from_str(&p.to_string()).unwrap(), p);
        let greek = p.to_string_with(GreekVars.var(0));
        assert_eq!(
            NaturalPolynomial::from_string_with(GreekVars.var(0), &greek).unwrap(),
            p
        );
        // `FromStr` is `from_string_with` with the first `XyzVars` variable.
        assert_eq!(
            NaturalPolynomial::from_string_with(XyzVars.var(0), &p.to_string()).unwrap(),
            p
        );
    });

    string_gen().test_properties(|s| {
        // Whatever parses is written back the same way, or in the one normal form.
        if let Ok(p) = NaturalPolynomial::from_str(&s) {
            assert!(p.is_valid());
            assert_eq!(NaturalPolynomial::from_str(&p.to_string()).unwrap(), p);
        }
    });
}
