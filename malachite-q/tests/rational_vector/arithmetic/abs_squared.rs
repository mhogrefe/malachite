// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{AbsSquared, Dot};
use malachite_nz::test_util::generators::integer_vector_gen;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::rational_vector_gen;
use malachite_q::test_util::rational_vector::arithmetic::abs_squared::*;

#[test]
fn test_abs_squared() {
    let test = |s, out| {
        let v = RationalVector::from_str(s).unwrap();
        let n = (&v).abs_squared();
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);
        assert_eq!(rational_vector_abs_squared_naive(&v), n);
        assert_eq!(v.abs_squared(), n);
    };
    test("()", "0");
    test("(-5/2)", "25/4");
    test("(1/2, -2/3)", "25/36");
    test("(1/3, 1/6, 1/2)", "7/18");
    test("(3, -4)", "25");
}

#[test]
fn abs_squared_properties() {
    rational_vector_gen().test_properties(|v| {
        let n = (&v).abs_squared();
        assert!(n.is_valid());
        assert_eq!(v.clone().abs_squared(), n);
        assert_eq!(rational_vector_abs_squared_naive(&v), n);
        // It is the dot product of the vector with itself, it is zero exactly when the vector is,
        // and negating the vector does not change it.
        assert_eq!((&v).dot(&v), n);
        assert_eq!(n == 0u32, v.elements.iter().all(|x| *x == 0u32));
        assert_eq!((-&v).abs_squared(), n);
    });

    integer_vector_gen().test_properties(|v| {
        // It agrees with the IntegerVector implementation.
        assert_eq!(
            RationalVector::from(v.clone()).abs_squared(),
            Rational::from(v.abs_squared())
        );
    });
}
