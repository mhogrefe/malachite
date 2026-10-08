// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::test_util::generators::string_gen;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::rational_vector_gen;

#[test]
fn test_serde() {
    let test = |s, out| {
        let v = RationalVector::from_str(s).unwrap();
        assert_eq!(serde_json::to_string(&v).unwrap(), out);
        assert_eq!(serde_json::from_str::<RationalVector>(out).unwrap(), v);
    };
    test("()", "[]");
    test("(0)", "[{\"s\":true,\"n\":\"0x0\",\"d\":\"0x1\"}]");
    test(
        "(22/7, -1)",
        "[{\"s\":true,\"n\":\"0x16\",\"d\":\"0x7\"},{\"s\":false,\"n\":\"0x1\",\"d\":\"0x1\"}]",
    );
    test(
        "(1/2, -3/4)",
        "[{\"s\":true,\"n\":\"0x1\",\"d\":\"0x2\"},{\"s\":false,\"n\":\"0x3\",\"d\":\"0x4\"}]",
    );
}

#[test]
fn test_serde_fail() {
    let test = |s: &str| {
        assert!(
            serde_json::from_str::<RationalVector>(s).is_err(),
            "{s:?} should not deserialize"
        );
    };
    test("[\"1\"]");
    test("[1]");
    test("[\"--0x1\"]");
    test("{}");
    test("");
}

#[test]
fn serde_properties() {
    rational_vector_gen().test_properties(|v| {
        let s = serde_json::to_string(&v).unwrap();
        // The encoding is exactly the encoding of the elements' `Vec`.
        assert_eq!(serde_json::to_string(&v.elements).unwrap(), s);
        assert_eq!(serde_json::from_str::<RationalVector>(&s).unwrap(), v);
        assert_eq!(s == "[]", v.dimension() == 0);
    });

    string_gen().test_properties(|s| {
        if let Ok(v) = serde_json::from_str::<RationalVector>(&s) {
            let t = serde_json::to_string(&v).unwrap();
            assert_eq!(serde_json::from_str::<RationalVector>(&t).unwrap(), v);
            // Whatever deserializes as a vector deserializes as its elements.
            assert_eq!(
                serde_json::from_str::<Vec<Rational>>(&s).unwrap(),
                v.elements
            );
        }
    });
}
