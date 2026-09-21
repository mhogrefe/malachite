// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::test_util::generators::string_gen;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::natural_polynomial_gen;

#[test]
fn test_serde() {
    let test = |s, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        assert_eq!(serde_json::to_string(&p).unwrap(), out);
        assert_eq!(serde_json::from_str::<NaturalPolynomial>(out).unwrap(), p);
    };
    // The zero polynomial has no coefficients, so it is the empty list.
    test("0", "[]");
    test("1", "[\"0x1\"]");
    test("5", "[\"0x5\"]");
    test("x", "[\"0x0\",\"0x1\"]");
    test("x^2+3*x+2", "[\"0x2\",\"0x3\",\"0x1\"]");
    // A gap in the degrees is a zero coefficient, which is held like any other.
    test("x^3+1", "[\"0x1\",\"0x0\",\"0x0\",\"0x1\"]");
    test(
        "18446744073709551616*x",
        "[\"0x0\",\"0x10000000000000000\"]",
    );
}

#[test]
fn test_serde_fail() {
    // A trailing zero coefficient is not something a `NaturalPolynomial` can hold: the list is
    // rejected rather than quietly trimmed, since accepting it would build a polynomial that an
    // equal one would not match.
    let test = |s: &str| {
        assert!(
            serde_json::from_str::<NaturalPolynomial>(s).is_err(),
            "{s:?} should not deserialize"
        );
    };
    test("[\"0x0\"]");
    test("[\"0x1\",\"0x0\"]");
    test("[\"0x1\",\"0x2\",\"0x0\"]");
    // A negative coefficient is not a `Natural`.
    test("[\"-0x1\"]");
    // Malformed encodings.
    test("[\"1\"]");
    test("[1]");
    test("{}");
    test("");
}

#[test]
fn serde_properties() {
    natural_polynomial_gen().test_properties(|p| {
        let s = serde_json::to_string(&p).unwrap();
        let q = serde_json::from_str::<NaturalPolynomial>(&s).unwrap();
        assert!(q.is_valid());
        assert_eq!(q, p);
        // The encoding is the coefficients and nothing else, so there is one entry per coefficient
        // and the empty list is the zero polynomial.
        assert_eq!(serde_json::to_string(p.coefficients_asc()).unwrap(), s);
        assert_eq!(s == "[]", p.degree().is_none());

        // Padding the list with a zero gives an encoding of the same polynomial that is not the one
        // it writes, and it is rejected rather than trimmed.
        let mut cs = p.coefficients_asc().to_vec();
        cs.push(Natural::ZERO);
        let padded = serde_json::to_string(&cs).unwrap();
        assert_ne!(padded, s);
        assert!(serde_json::from_str::<NaturalPolynomial>(&padded).is_err());
    });

    string_gen().test_properties(|s| {
        // Whatever an arbitrary string deserializes to, if anything, is a valid polynomial.
        if let Ok(p) = serde_json::from_str::<NaturalPolynomial>(&s) {
            assert!(p.is_valid());
        }
    });
}
