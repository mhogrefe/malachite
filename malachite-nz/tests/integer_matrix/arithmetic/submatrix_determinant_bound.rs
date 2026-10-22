// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::basic::traits::Two;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::integer_matrix_gen;
use malachite_nz::test_util::integer_matrix::arithmetic::determinant::*;
use malachite_nz::test_util::integer_matrix::arithmetic::determinant_bound::*;

// The bound is computed with floating-point magnitudes that round up, so it can exceed the exact
// Hadamard bound, but only by a small relative error.
fn assert_close_above(bound: &Natural, exact: &Natural) {
    assert!(bound >= exact);
    assert!(*bound <= exact + (exact >> 10u32) + Natural::TWO);
}

#[test]
fn test_submatrix_determinant_bound() {
    let test = |s, out| {
        let a = IntegerMatrix::from_str(s).unwrap();
        let bound = a.submatrix_determinant_bound();
        assert_eq!(bound.to_string(), out);
        assert_close_above(&bound, &integer_matrix_hadamard_bound_naive(&a, false));
    };
    test("[]_{0,0}", "1");
    test("[]_{2,0}", "1");
    test("[]_{0,3}", "1");
    test("[0, 0]", "2");
    test("[0]", "2");
    test("[5]", "6");
    test("[1, 2, 3]", "4");
    test("[1; 2; 3]", "4");
    test("[1, 2; 0, 0]", "3");
    test("[0, 0; 0, 7]", "8");
    test(
        "[1267650600228229401496703205376, 1, 0; 1, 1267650600228229401496703205376, 0]",
        "1606938068204233101571475504190334901745783987825591619420160",
    );
    test("[1, 2; 3, 4]", "12");
}

#[test]
fn submatrix_determinant_bound_properties() {
    integer_matrix_gen().test_properties(|a| {
        let bound = a.submatrix_determinant_bound();
        assert_close_above(&bound, &integer_matrix_hadamard_bound_naive(&a, false));
        assert_eq!(a.transpose().submatrix_determinant_bound(), bound);
        assert_eq!((-&a).submatrix_determinant_bound(), bound);
        assert_ne!(bound, 0u32);
        if a.is_square() {
            assert!(bound >= *integer_matrix_determinant_naive(&a).unsigned_abs_ref());
        }
        // The bound holds for the square submatrix in the top-left corner.
        let n = a.height().min(a.width());
        let corner = a.window(0..n, 0..n);
        assert!(bound >= *integer_matrix_determinant_naive(&corner).unsigned_abs_ref());
    });
}
