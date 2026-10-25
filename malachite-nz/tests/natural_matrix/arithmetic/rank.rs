// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Rank;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::{natural_matrix_gen, natural_matrix_pair_gen_var_4};

// Checks `rank` of `a`: both ways of computing it agree, and it is the result for the
// `IntegerMatrix` with the same elements.
fn verify(a: &NaturalMatrix) {
    let x = a.rank();
    assert_eq!(a.clone().rank(), x);
    assert_eq!(IntegerMatrix::from(a).rank(), x);
}

#[test]
fn test_rank() {
    let test = |s, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        verify(&a);
        assert_eq!(a.rank().to_string(), out);
    };
    test("[]_{0,0}", "0");
    test("[]_{0,3}", "0");
    test("[]_{2,0}", "0");
    test("[0, 0; 0, 0]", "0");
    test("[3]", "1");
    test("[1, 2; 3, 4]", "2");
    test("[2, 4; 1, 2]", "1");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 9]", "2");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 10]", "3");
    test("[2, 0; 0, 3]", "2");
    test("[1, 0, 1; 1, 1, 0]", "2");
    test("[6, 4; 4, 6; 9, 3]", "2");
}

#[test]
fn rank_properties() {
    natural_matrix_gen().test_properties(|a| {
        verify(&a);
    });

    natural_matrix_pair_gen_var_4().test_properties(|(a, b)| {
        verify(&(a * b));
    });
}
