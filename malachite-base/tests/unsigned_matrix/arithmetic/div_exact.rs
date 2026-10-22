// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_13;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use std::panic::catch_unwind;

#[test]
fn test_div_exact() {
    let test = |s, c: u8, out| {
        let a = UnsignedMatrix::<u8>::from_str(s).unwrap();
        let b = (&a).div_exact(c);
        assert!(b.is_valid());
        assert_eq!(b.to_string(), out);
        assert_eq!(a.clone().div_exact(c), b);
        let mut x = a;
        x.div_exact_assign(c);
        assert_eq!(x, b);
    };
    test("[]_{0,0}", 5, "[]_{0,0}");
    test("[]_{2,0}", 5, "[]_{2,0}");
    test("[0, 6; 255, 3]", 1, "[0, 6; 255, 3]");
    test("[0, 6; 255, 3]", 3, "[0, 2; 85, 1]");
    test("[4, 8; 12, 16]", 4, "[1, 2; 3, 4]");
    test("[250, 0]", 125, "[2, 0]");
    test("[255; 255]", 255, "[1; 1]");
}

#[test]
fn div_exact_fail() {
    let a = UnsignedMatrix::<u64>::from_str("[1, 2]").unwrap();
    assert_panic!(a.clone().div_exact(0));
    assert_panic!((&a).div_exact(0));
    assert_panic!(
        UnsignedMatrix::<u64>::from_str("[]_{0,0}")
            .unwrap()
            .div_exact(0)
    );
    assert_panic!({
        let mut b = a.clone();
        b.div_exact_assign(0);
    });
}

#[test]
fn div_exact_properties() {
    unsigned_matrix_unsigned_pair_gen_var_13().test_properties(|(a, c)| {
        let b = (&a).div_exact(c);
        assert!(b.is_valid());
        // The forms agree.
        assert_eq!(a.clone().div_exact(c), b);
        let mut x = a.clone();
        x.div_exact_assign(c);
        assert_eq!(x, b);

        // Element by element, this is the scalar exact quotient, which is also the ordinary
        // quotient, and multiplying by the scalar recovers the element. The shape is unchanged.
        assert_eq!(b.height(), a.height());
        assert_eq!(b.width(), a.width());
        for (&x, &y) in a.elements.iter().zip(&b.elements) {
            assert_eq!(y, x.div_exact(c));
            assert_eq!(y, x / c);
            assert_eq!(y * c, x);
        }
        // Dividing by 1 changes nothing.
        assert_eq!((&a).div_exact(1), a);
    });
}
