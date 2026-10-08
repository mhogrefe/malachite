// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::{string_gen, unsigned_vec_gen, unsigned_vector_gen};
use malachite_base::unsigned_vector::UnsignedVector;

#[test]
fn test_serde() {
    let test = |xs: &[u32], out| {
        let v = UnsignedVector::from_elements(xs);
        assert_eq!(serde_json::to_string(&v).unwrap(), out);
        assert_eq!(serde_json::from_str::<UnsignedVector<u32>>(out).unwrap(), v);
    };
    test(&[], "[]");
    test(&[0], "[0]");
    test(&[1, 2, 3], "[1,2,3]");
}

#[test]
fn test_serde_fail() {
    let test = |s: &str| {
        assert!(
            serde_json::from_str::<UnsignedVector<u32>>(s).is_err(),
            "{s:?} should not deserialize"
        );
    };
    test("[-1]");
    test("[\"1\"]");
    test("{}");
    test("");
}

#[test]
fn serde_properties() {
    unsigned_vector_gen().test_properties(|v| {
        let s = serde_json::to_string(&v).unwrap();
        assert_eq!(serde_json::to_string(&v.elements).unwrap(), s);
        assert_eq!(serde_json::from_str::<UnsignedVector<u64>>(&s).unwrap(), v);
    });

    unsigned_vec_gen::<u32>().test_properties(|xs| {
        let v = UnsignedVector {
            elements: xs.clone(),
        };
        let s = serde_json::to_string(&v).unwrap();
        // The encoding is exactly the encoding of the elements' `Vec`.
        assert_eq!(serde_json::to_string(&xs).unwrap(), s);
        assert_eq!(serde_json::from_str::<UnsignedVector<u32>>(&s).unwrap(), v);
    });

    string_gen().test_properties(|s| {
        if let Ok(v) = serde_json::from_str::<UnsignedVector<u32>>(&s) {
            assert_eq!(serde_json::from_str::<Vec<u32>>(&s).unwrap(), v.elements);
        }
    });
}
