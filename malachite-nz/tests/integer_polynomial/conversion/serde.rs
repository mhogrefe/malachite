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
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::{integer_polynomial_gen, natural_polynomial_gen};

#[test]
fn test_serde() {
    let test = |s, out| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        assert_eq!(serde_json::to_string(&p).unwrap(), out);
        assert_eq!(serde_json::from_str::<IntegerPolynomial>(out).unwrap(), p);
    };
    test("0", "[]");
    test("1", "[\"0x1\"]");
    test("-1", "[\"-0x1\"]");
    test("x", "[\"0x0\",\"0x1\"]");
    test("x^2-3*x+2", "[\"0x2\",\"-0x3\",\"0x1\"]");
    test("-x^3+1", "[\"0x1\",\"0x0\",\"0x0\",\"-0x1\"]");
}

#[test]
fn test_serde_fail() {
    let test = |s: &str| {
        assert!(
            serde_json::from_str::<IntegerPolynomial>(s).is_err(),
            "{s:?} should not deserialize"
        );
    };
    // A trailing zero coefficient is rejected rather than quietly trimmed.
    test("[\"0x0\"]");
    test("[\"-0x1\",\"0x0\"]");
    // Malformed encodings.
    test("[\"1\"]");
    test("[1]");
    test("{}");
    test("");
}

#[test]
fn serde_properties() {
    integer_polynomial_gen().test_properties(|p| {
        let s = serde_json::to_string(&p).unwrap();
        let q = serde_json::from_str::<IntegerPolynomial>(&s).unwrap();
        assert!(q.is_valid());
        assert_eq!(q, p);
        assert_eq!(serde_json::to_string(p.coefficients_asc()).unwrap(), s);
        assert_eq!(s == "[]", p.degree().is_none());

        // Padding the list with a zero gives an encoding of the same polynomial that is not the one
        // it writes, and it is rejected rather than trimmed.
        let mut cs = p.coefficients_asc().to_vec();
        cs.push(Integer::ZERO);
        let padded = serde_json::to_string(&cs).unwrap();
        assert_ne!(padded, s);
        assert!(serde_json::from_str::<IntegerPolynomial>(&padded).is_err());
    });

    natural_polynomial_gen().test_properties(|p| {
        // A `NaturalPolynomial`'s encoding reads back as the `IntegerPolynomial` it converts to,
        // since a `Natural` and a nonnegative `Integer` are encoded the same way.
        let s = serde_json::to_string(&p).unwrap();
        assert_eq!(
            serde_json::from_str::<IntegerPolynomial>(&s).unwrap(),
            IntegerPolynomial::from(p)
        );
    });

    string_gen().test_properties(|s| {
        if let Ok(p) = serde_json::from_str::<IntegerPolynomial>(&s) {
            assert!(p.is_valid());
        }
    });
}
