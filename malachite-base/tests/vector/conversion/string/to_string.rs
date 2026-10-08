// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::strings::ToDebugString;
use malachite_base::test_util::generators::unsigned_vec_gen;
use malachite_base::vector::Vector;

#[test]
fn test_to_string() {
    let test = |xs: &[i32], out| {
        let v = Vector::from_elements(xs);
        assert_eq!(v.to_string(), out);
        assert_eq!(v.to_debug_string(), out);
    };
    test(&[], "()");
    test(&[0], "(0)");
    test(&[1, -2, 3], "(1, -2, 3)");
    test(&[i32::MIN, i32::MAX], "(-2147483648, 2147483647)");
    // A vector of vectors.
    let vv = Vector::from_elements(&[Vector::from_elements(&[1u8, 2]), Vector::from_elements(&[])]);
    assert_eq!(vv.to_string(), "((1, 2), ())");
}

#[test]
fn to_string_properties() {
    unsigned_vec_gen::<u8>().test_properties(|xs| {
        let v = Vector {
            elements: xs.clone(),
        };
        let s = v.to_string();
        assert_eq!(v.to_debug_string(), s);
        assert_eq!(s, format!("({})", xs.iter().join(", ")));
    });
}
