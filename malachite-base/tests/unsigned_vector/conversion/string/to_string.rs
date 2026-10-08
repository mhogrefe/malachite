// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::strings::ToDebugString;
use malachite_base::test_util::generators::{unsigned_vec_gen, unsigned_vector_gen};
use malachite_base::unsigned_vector::UnsignedVector;

#[test]
fn test_to_string() {
    let test = |xs: &[u32], out| {
        let v = UnsignedVector::from_elements(xs);
        assert_eq!(v.to_string(), out);
        assert_eq!(v.to_debug_string(), out);
    };
    test(&[], "()");
    test(&[0], "(0)");
    test(&[1, 2, 3], "(1, 2, 3)");
    test(&[0, u32::MAX], "(0, 4294967295)");
}

#[test]
fn to_string_properties() {
    unsigned_vector_gen().test_properties(|v| {
        let s = v.to_string();
        assert_eq!(v.to_debug_string(), s);
        assert_eq!(s, format!("({})", v.elements.iter().join(", ")));
        let vs = vec![v.clone(), v.clone()];
        assert_eq!(vs.to_debug_string(), format!("[{v}, {v}]"));
    });

    unsigned_vec_gen::<u8>().test_properties(|xs| {
        let v = UnsignedVector {
            elements: xs.clone(),
        };
        let s = v.to_string();
        assert_eq!(v.to_debug_string(), s);
        assert_eq!(s, format!("({})", xs.iter().join(", ")));
    });
}
