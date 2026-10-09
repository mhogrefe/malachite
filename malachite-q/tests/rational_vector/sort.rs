// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::vector::Vector;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::rational_vector_gen;

#[test]
fn test_sort() {
    let test = |s, out| {
        let mut v = RationalVector::from_str(s).unwrap();
        v.sort();
        assert_eq!(v.to_string(), out);
    };
    test("()", "()");
    test("(5)", "(5)");
    test("(1, 2, 3)", "(1, 2, 3)");
    test("(3, 2, 1)", "(1, 2, 3)");
    test("(1/2, -1, 1/3, 0)", "(-1, 0, 1/3, 1/2)");
    test("(2/3, 3/5, 1/2)", "(1/2, 3/5, 2/3)");
}

#[test]
fn sort_properties() {
    rational_vector_gen().test_properties(|v| {
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
        // Sorting the negation gives the negation of the sorted vector, reversed.
        let mut n = -&v;
        n.sort();
        let mut rev = -&w;
        rev.elements.reverse();
        assert_eq!(n, rev);
    });
}
