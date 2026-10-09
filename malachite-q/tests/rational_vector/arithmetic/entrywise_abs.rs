// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    Abs, EntrywiseAbs, EntrywiseAbsAssign, Height, L1Norm,
};
use malachite_base::vector::Vector;
use malachite_nz::test_util::generators::integer_vector_gen;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::rational_vector_gen;

#[test]
fn test_entrywise_abs() {
    let test = |s, out| {
        let v = RationalVector::from_str(s).unwrap();
        let w = (&v).entrywise_abs();
        assert_eq!(w.to_string(), out);
        assert_eq!(v.clone().entrywise_abs(), w);
        let mut x = v;
        x.entrywise_abs_assign();
        assert_eq!(x, w);
    };
    test("()", "()");
    test("(0)", "(0)");
    test("(1/2, -2/3, 0, -3)", "(1/2, 2/3, 0, 3)");
    test("(-1/18446744073709551616)", "(1/18446744073709551616)");
}

#[test]
fn entrywise_abs_properties() {
    rational_vector_gen().test_properties(|v| {
        let w = (&v).entrywise_abs();
        assert_eq!(v.clone().entrywise_abs(), w);
        let mut x = v.clone();
        x.entrywise_abs_assign();
        assert_eq!(x, w);

        // Element by element, this is the scalar operation, and the dimension is unchanged.
        assert_eq!(w.dimension(), v.dimension());
        for (x, y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(*y, x.abs());
        }
        // It ignores signs, and doing it twice changes nothing.
        assert_eq!((-&v).entrywise_abs(), w);
        assert_eq!((&w).entrywise_abs(), w);
        // The l^1 norm and the height depend only on the absolute values.
        assert_eq!(w.to_l1_norm(), v.to_l1_norm());
        assert_eq!(w.to_height(), v.to_height());
    });

    integer_vector_gen().test_properties(|v| {
        // The integer vector's absolute values are the same as rationals.
        assert_eq!(
            RationalVector::from(v.clone()).entrywise_abs(),
            RationalVector::from(v.entrywise_abs())
        );
    });
}
