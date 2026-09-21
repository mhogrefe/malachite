// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::test_util::generators::string_gen;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::rational_polynomial_gen;

#[test]
fn test_serde() {
    let test = |s, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        assert_eq!(serde_json::to_string(&p).unwrap(), out);
        assert_eq!(serde_json::from_str::<RationalPolynomial>(out).unwrap(), p);
    };
    // Both parts are encoded: the numerator as a list of coefficients, the denominator as one
    // number. The zero polynomial's denominator is 1.
    test("0", "{\"n\":[],\"d\":\"0x1\"}");
    test("1", "{\"n\":[\"0x1\"],\"d\":\"0x1\"}");
    test(
        "x^2-3*x+2",
        "{\"n\":[\"0x2\",\"-0x3\",\"0x1\"],\"d\":\"0x1\"}",
    );
    // One denominator serves the whole polynomial, so 1/2 and 1/3 share a 6.
    test("1/2*x+1/3", "{\"n\":[\"0x2\",\"0x3\"],\"d\":\"0x6\"}");
    test("-1/2", "{\"n\":[\"-0x1\"],\"d\":\"0x2\"}");
}

#[test]
fn test_serde_fail() {
    let test = |s: &str| {
        assert!(
            serde_json::from_str::<RationalPolynomial>(s).is_err(),
            "{s:?} should not deserialize"
        );
    };
    // A zero denominator.
    test("{\"n\":[\"0x1\"],\"d\":\"0x0\"}");
    // The zero polynomial with a denominator other than 1.
    test("{\"n\":[],\"d\":\"0x2\"}");
    // A numerator and denominator that share a factor, which a canonical pair never does.
    test("{\"n\":[\"0x2\",\"0x4\"],\"d\":\"0x2\"}");
    // A numerator with a trailing zero coefficient.
    test("{\"n\":[\"0x1\",\"0x0\"],\"d\":\"0x1\"}");
    // Malformed encodings.
    test("{\"n\":[\"0x1\"]}");
    test("[]");
    test("");
}

#[test]
fn serde_properties() {
    rational_polynomial_gen().test_properties(|p| {
        let s = serde_json::to_string(&p).unwrap();
        let q = serde_json::from_str::<RationalPolynomial>(&s).unwrap();
        assert!(q.is_valid());
        assert_eq!(q, p);
        // The encoding holds the two parts the polynomial is made of, and nothing else.
        assert_eq!(
            s,
            format!(
                "{{\"n\":{},\"d\":{}}}",
                serde_json::to_string(p.numerator_ref()).unwrap(),
                serde_json::to_string(p.denominator_ref()).unwrap()
            )
        );

        // Doubling both parts leaves the polynomial alone but makes them share a factor, which a
        // canonical pair never does, so the encoding is rejected rather than reduced.
        let scaled = format!(
            "{{\"n\":{},\"d\":{}}}",
            serde_json::to_string(&IntegerPolynomial::from_coefficients_asc(
                p.numerator_ref()
                    .coefficients_asc()
                    .iter()
                    .map(|c| c << 1u32)
                    .collect()
            ))
            .unwrap(),
            serde_json::to_string(&(p.denominator_ref() << 1u32)).unwrap()
        );
        assert_ne!(scaled, s);
        assert!(serde_json::from_str::<RationalPolynomial>(&scaled).is_err());
    });

    string_gen().test_properties(|s| {
        if let Ok(p) = serde_json::from_str::<RationalPolynomial>(&s) {
            assert!(p.is_valid());
        }
    });
}
