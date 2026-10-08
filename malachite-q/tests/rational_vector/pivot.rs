// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::vector::Vector;
use malachite_nz::test_util::generators::integer_vector_gen;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::rational_vector_gen;

#[test]
fn test_pivot() {
    let test = |s, out: Option<&str>| {
        let v = RationalVector::from_str(s).unwrap();
        assert_eq!(v.pivot().map(ToString::to_string).as_deref(), out);
        assert_eq!(v.pivot_index().is_some(), out.is_some());
    };
    test("()", None);
    test("(0)", None);
    test("(0, 0, 0)", None);
    test("(-5)", Some("-5"));
    test("(0, 0, -3/4, 0, 5)", Some("-3/4"));
    test("(7/2, 0, 0)", Some("7/2"));
    test("(0, 1/1000000000000)", Some("1/1000000000000"));
}

#[test]
fn pivot_properties() {
    rational_vector_gen().test_properties(|v| {
        let pivot = v.pivot();
        // There is a pivot exactly when some element is nonzero.
        assert_eq!(pivot.is_none(), v.elements.iter().all(|x| *x == 0u32));
        // It is the first nonzero element: every element before it is zero, and its index is
        // `pivot_index`.
        let index = v.pivot_index();
        assert_eq!(index.is_some(), pivot.is_some());
        if let Some(i) = index {
            let i = usize::try_from(i).unwrap();
            assert_eq!(pivot, Some(&v[i]));
            assert!(v.elements[..i].iter().all(|x| *x == 0u32));
        }
        if let Some(i) = v.elements.iter().position(|x| *x != 0u32) {
            assert_eq!(pivot, Some(&v.elements[i]));
        }
        // Negating the vector negates its pivot.
        assert_eq!((-&v).pivot().cloned(), v.pivot().map(|x| -x));
    });

    integer_vector_gen().test_properties(|v| {
        // The `Integer` and `Rational` vectors have the same pivot.
        assert_eq!(
            RationalVector::from(v.clone()).pivot().cloned(),
            v.pivot().map(Rational::from)
        );
    });
}
