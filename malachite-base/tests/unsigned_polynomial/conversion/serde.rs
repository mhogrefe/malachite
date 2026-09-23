// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::generators::{string_gen, unsigned_polynomial_gen};
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

#[test]
fn test_serde() {
    let test = |s, out| {
        let p = UnsignedPolynomial::<u64>::from_str(s).unwrap();
        assert_eq!(serde_json::to_string(&p).unwrap(), out);
        assert_eq!(
            serde_json::from_str::<UnsignedPolynomial<u64>>(out).unwrap(),
            p
        );
    };
    // The zero polynomial has no coefficients, so it is the empty list.
    test("0", "[]");
    test("1", "[1]");
    test("5", "[5]");
    test("x", "[0,1]");
    test("x^2+3*x+2", "[2,3,1]");
    // A gap in the degrees is a zero coefficient, which is held like any other.
    test("x^3+1", "[1,0,0,1]");
    test("18446744073709551615*x", "[0,18446744073709551615]");
}

#[test]
fn test_serde_fail() {
    // A trailing zero coefficient is not something a `UnsignedPolynomial` can hold: the list is
    // rejected rather than quietly trimmed, since accepting it would build a polynomial that an
    // equal one would not match.
    let test = |s: &str| {
        assert!(
            serde_json::from_str::<UnsignedPolynomial<u64>>(s).is_err(),
            "{s:?} should not deserialize"
        );
    };
    test("[0]");
    test("[1,0]");
    test("[1,2,0]");
    // A coefficient that is not a `u64`.
    test("[-1]");
    test("[18446744073709551616]");
    // Malformed encodings.
    test("[\"1\"]");
    test("{}");
    test("");
}

#[test]
fn serde_properties() {
    unsigned_polynomial_gen().test_properties(|p| {
        let s = serde_json::to_string(&p).unwrap();
        let q = serde_json::from_str::<UnsignedPolynomial<u64>>(&s).unwrap();
        assert!(q.is_valid());
        assert_eq!(q, p);
        // The encoding is the coefficients and nothing else, so there is one entry per coefficient
        // and the empty list is the zero polynomial.
        assert_eq!(serde_json::to_string(p.coefficients_asc()).unwrap(), s);
        assert_eq!(s == "[]", p.degree().is_none());

        // Padding the list with a zero gives an encoding of the same polynomial that is not the one
        // it writes, and it is rejected rather than trimmed.
        let mut cs = p.coefficients_asc().to_vec();
        cs.push(0);
        let padded = serde_json::to_string(&cs).unwrap();
        assert_ne!(padded, s);
        assert!(serde_json::from_str::<UnsignedPolynomial<u64>>(&padded).is_err());
    });

    string_gen().test_properties(|s| {
        // Whatever an arbitrary string deserializes to, if anything, is a valid polynomial.
        if let Ok(p) = serde_json::from_str::<UnsignedPolynomial<u64>>(&s) {
            assert!(p.is_valid());
        }
    });
}
