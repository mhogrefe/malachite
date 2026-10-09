// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::vector::Vector;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_vector_gen;

#[test]
fn test_sort() {
    let test = |s, out| {
        let mut v = NaturalVector::from_str(s).unwrap();
        v.sort();
        assert_eq!(v.to_string(), out);
    };
    test("()", "()");
    test("(5)", "(5)");
    test("(1, 2, 3)", "(1, 2, 3)");
    test("(3, 2, 1)", "(1, 2, 3)");
    test("(3, 1, 2, 1)", "(1, 1, 2, 3)");
    test(
        "(18446744073709551616, 18446744073709551615, 1)",
        "(1, 18446744073709551615, 18446744073709551616)",
    );
}

#[test]
fn sort_properties() {
    natural_vector_gen().test_properties(|v| {
        let mut w = v.clone();
        w.sort();
        // The dimension is unchanged, and the elements are in ascending order.
        assert_eq!(w.dimension(), v.dimension());
        assert!(w.elements.windows(2).all(|p| p[0] <= p[1]));
        // The elements are the same, as a stable sort finds them.
        let mut elements = v.elements.clone();
        elements.sort();
        assert_eq!(w.elements, elements);
        // Sorting again, or sorting a reordering, gives the same vector.
        let mut x = w.clone();
        x.sort();
        assert_eq!(x, w);
        let mut y = v.clone();
        y.elements.reverse();
        y.sort();
        assert_eq!(y, w);
    });
}
