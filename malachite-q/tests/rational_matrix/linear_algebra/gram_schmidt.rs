// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::linear_algebra::traits::{GramSchmidt, GramSchmidtAssign, Rank};
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::generators::rational_matrix_gen_var_3;
use malachite_q::test_util::rational_matrix::linear_algebra::gram_schmidt::*;

// Checks the Gram-Schmidt process applied to `a` in every way of calling it, against the modified
// algorithm; checks that the columns of the result are orthogonal, that orthogonalizing them again
// changes nothing, and that the number of nonzero columns is the rank; and returns the result.
fn verify(a: &RationalMatrix) -> RationalMatrix {
    let b = a.gram_schmidt();
    assert!(b.is_valid());
    assert_eq!(a.clone().gram_schmidt(), b);
    let mut c = a.clone();
    c.gram_schmidt_assign();
    assert_eq!(c, b);
    assert_eq!(gram_schmidt_modified_rational(a), b);
    let gram = &b.transpose() * &b;
    let width = usize::exact_from(b.width());
    for i in 0..width {
        for j in 0..width {
            if i != j {
                assert_eq!(gram[(i, j)], 0u32);
            }
        }
    }
    let nonzero_columns = (0..width).filter(|&j| gram[(j, j)] != 0u32).count();
    assert_eq!(u64::exact_from(nonzero_columns), a.rank());
    assert_eq!((&b).gram_schmidt(), b);
    b
}

#[test]
fn test_gram_schmidt() {
    let test = |a, out: &str| {
        let a = RationalMatrix::from_str(a).unwrap();
        let b = verify(&a).to_string();
        assert_eq!(b, out);
    };
    // - height == 0
    test("[]_{0,0}", "[]_{0,0}");
    test("[]_{0,3}", "[]_{0,3}");
    test("[]_{2,0}", "[]_{2,0}");
    // - !ring.is_zero(&den) && ring.inverse(&den).is_some()
    test("[1, 1; 1, 0]", "[1, 1/2; 1, -1/2]");
    // - ring.is_zero(&den) && column j is zero
    test(
        "[1, 2, 3; 2, 4, 1; 0, 0, 5]",
        "[1, 0, 2; 2, 0, -1; 0, 0, 5]",
    );
    test("[0, 1; 0, 2]", "[0, 1; 0, 2]");
    test("[3; 4]", "[3; 4]");
    test("[1, 0, 1; 0, 1, 1]", "[1, 0, 0; 0, 1, 0]");
    test("[1/2, -1/3; 2, 5/7]", "[1/2, -172/357; 2, 43/357]");
    test(
        "[1, 1, 1; 1, 2, 3; 1, 3, 6]",
        "[1, -1, 1/6; 1, 0, -1/3; 1, 1, 1/6]",
    );
    test("[-1, 2; 3, -4; 5, 6]", "[-1, 86/35; 3, -188/35; 5, 26/7]");
}

#[test]
fn gram_schmidt_properties() {
    rational_matrix_gen_var_3().test_properties(|a| {
        verify(&a);
    });
}
