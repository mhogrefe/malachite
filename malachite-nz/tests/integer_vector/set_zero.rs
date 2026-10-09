// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::vector::Vector;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::integer_vector_gen;

#[test]
fn test_set_zero() {
    let test = |s, out| {
        let mut v = IntegerVector::from_str(s).unwrap();
        v.set_zero();
        assert_eq!(v.to_string(), out);
    };
    test("()", "()");
    test("(0)", "(0)");
    test("(1, -2, 3)", "(0, 0, 0)");
}

#[test]
fn set_zero_properties() {
    integer_vector_gen().test_properties(|v| {
        let mut x = v.clone();
        x.set_zero();
        // The dimension is kept, and the result is the zero vector of that dimension.
        assert_eq!(x.dimension(), v.dimension());
        assert_eq!(x, IntegerVector::zero(v.dimension()));
        assert_eq!(x.pivot_index(), None);
        // Doing it again changes nothing.
        let mut y = x.clone();
        y.set_zero();
        assert_eq!(y, x);
        // It is the same as emptying the vector and growing it back with zeros.
        let mut z = v.clone();
        z.set_dimension(0);
        z.set_dimension(v.dimension());
        assert_eq!(z, x);
    });
}
