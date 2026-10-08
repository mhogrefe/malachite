// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::strings::ToDebugString;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::rational_vector_gen;

fn vector(xs: &[i32]) -> RationalVector {
    RationalVector {
        elements: xs.iter().copied().map(Rational::from).collect(),
    }
}

#[test]
fn test_to_string() {
    let test = |xs: &[i32], out| {
        let v = vector(xs);
        assert_eq!(v.to_string(), out);
        assert_eq!(v.to_debug_string(), out);
    };
    test(&[], "()");
    test(&[0], "(0)");
    test(&[5], "(5)");
    test(&[1, 2, 3], "(1, 2, 3)");
    test(&[0, 0], "(0, 0)");
    test(&[-5], "(-5)");
    test(&[1, -2, 3], "(1, -2, 3)");
    test(&[i32::MIN, 10], "(-2147483648, 10)");
}

#[test]
fn to_string_properties() {
    rational_vector_gen().test_properties(|v| {
        let s = v.to_string();
        assert_eq!(v.to_debug_string(), s);
        assert_eq!(s, format!("({})", v.elements.iter().join(", ")));
        assert!(s.starts_with('('));
        assert!(s.ends_with(')'));
    });

    rational_vector_gen().test_properties(|v| {
        // A `Vec` of vectors is written with each vector as `Display` writes it.
        let vs = vec![v.clone(), v.clone()];
        assert_eq!(vs.to_debug_string(), format!("[{v}, {v}]"));
    });
}
