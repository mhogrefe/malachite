// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::string_gen;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::integer_vector_gen;

#[test]
fn test_serde() {
    let test = |xs: &[i32], out| {
        let v = IntegerVector {
            elements: xs.iter().copied().map(Integer::from).collect(),
        };
        assert_eq!(serde_json::to_string(&v).unwrap(), out);
        assert_eq!(serde_json::from_str::<IntegerVector>(out).unwrap(), v);
    };
    test(&[], "[]");
    test(&[0], "[\"0x0\"]");
    test(&[1, 2, 3], "[\"0x1\",\"0x2\",\"0x3\"]");
    // Trailing zeros are kept: unlike a polynomial's coefficients, they are part of the vector.
    test(&[5, 0, 0], "[\"0x5\",\"0x0\",\"0x0\"]");
    test(&[-1, 2], "[\"-0x1\",\"0x2\"]");
}

#[test]
fn test_serde_fail() {
    let test = |s: &str| {
        assert!(
            serde_json::from_str::<IntegerVector>(s).is_err(),
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
    integer_vector_gen().test_properties(|v| {
        let s = serde_json::to_string(&v).unwrap();
        // The encoding is exactly the encoding of the elements' `Vec`.
        assert_eq!(serde_json::to_string(&v.elements).unwrap(), s);
        assert_eq!(serde_json::from_str::<IntegerVector>(&s).unwrap(), v);
        assert_eq!(s == "[]", v.dimension() == 0);
    });

    string_gen().test_properties(|s| {
        if let Ok(v) = serde_json::from_str::<IntegerVector>(&s) {
            let t = serde_json::to_string(&v).unwrap();
            assert_eq!(serde_json::from_str::<IntegerVector>(&t).unwrap(), v);
            // Whatever deserializes as a vector deserializes as its elements.
            assert_eq!(
                serde_json::from_str::<Vec<Integer>>(&s).unwrap(),
                v.elements
            );
        }
    });
}
