// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::Inverse;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::arithmetic::determinant::determinant_bareiss;
use malachite_nz::integer_matrix::arithmetic::solve::solve_nonsingular;
use malachite_nz::test_util::generators::{
    integer_matrix_gen_var_1, integer_matrix_pair_gen_var_7,
};
use std::panic::catch_unwind;

// Checks the inverse (B, d) of A: both ways of passing A agree, AB = BA = dI with d nonzero, and A
// is singular if there is none. Returns it.
fn verify_inverse(a: &IntegerMatrix) -> String {
    let result = a.clone().inverse();
    assert_eq!(a.inverse(), result);
    let n = a.height();
    if let Some((b, den)) = &result {
        assert!(b.is_valid());
        assert_eq!(b.height(), n);
        assert_eq!(b.width(), n);
        assert_ne!(*den, 0u32);
        let identity = IntegerMatrix::identity(n) * den;
        assert_eq!(a * b, identity);
        assert_eq!(b * a, identity);
        if n <= 2 {
            assert_eq!(*den, determinant_bareiss(a));
        } else {
            assert_eq!(solve_nonsingular(a, &IntegerMatrix::identity(n)), result);
        }
    } else {
        assert_eq!(determinant_bareiss(a), 0u32);
    }
    result.map_or_else(
        || "None".to_string(),
        |(b, den)| format!("Some(({b}, {den}))"),
    )
}

#[test]
fn test_inverse() {
    let test = |a, out| {
        let a = IntegerMatrix::from_str(a).unwrap();
        assert_eq!(verify_inverse(&a), out);
    };
    test("[]_{0,0}", "Some(([]_{0,0}, 1))");
    test("[0]", "None");
    test("[5]", "Some(([1], 5))");
    test("[-5]", "Some(([1], -5))");
    test("[1, 2; 3, 4]", "Some(([4, -2; -3, 1], -2))");
    test("[2, 4; 1, 2]", "None");
    test("[0, 1; 1, 0]", "Some(([0, -1; -1, 0], -1))");
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 10]",
        "Some(([2, 4, -3; 2, -11, 6; -3, 6, -3], -3))",
    );
    test("[1, 2, 3; 4, 5, 6; 7, 8, 9]", "None");
    test(
        "[2, 0, 0; 0, 3, 0; 0, 0, 5]",
        "Some(([15, 0, 0; 0, 10, 0; 0, 0, 6], 30))",
    );
    test(
        "[1, 2, 3, 4; 5, 6, 7, 8; 2, 6, 4, 8; 3, 1, 1, 2]",
        "Some(([-12, 4, -4, 24; -60, 20, 16, -24; 12, 20, -20, -24; 42, -26, 8, 24], 72))",
    );
    test(
        "[1267650600228229401496703205376, 1; 1, 1267650600228229401496703205376]",
        "Some(([1267650600228229401496703205376, -1; -1, 1267650600228229401496703205376], 16069380\
        44258990275541962092341162602522202993782792835301375))",
    );
}

#[test]
fn inverse_fail() {
    let a = IntegerMatrix::from_str("[1, 2]").unwrap();
    assert_panic!(a.clone().inverse());
    assert_panic!((&a).inverse());
}

#[test]
fn inverse_properties() {
    integer_matrix_gen_var_1().test_properties(|a| {
        verify_inverse(&a);
    });

    integer_matrix_pair_gen_var_7().test_properties(|(a, b)| {
        let n = a.height();
        let k = b.width();
        if k < n {
            verify_inverse(&(&b * a.window(0..k, 0..n)));
        }
    });
}
