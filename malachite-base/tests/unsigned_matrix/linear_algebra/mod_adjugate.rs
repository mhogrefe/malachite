// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::linear_algebra::traits::{ModAdjugate, ModCofactor, ModDeterminant};
use malachite_base::num::modular::traits::ModMul;
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_15;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use std::panic::catch_unwind;

#[test]
fn test_mod_adjugate() {
    let test = |s, m: u64, out: &str| {
        let a = UnsignedMatrix::<u64>::from_str(s).unwrap();
        let x = (&a).mod_adjugate(m);
        assert_eq!(x.to_string(), out);
        assert_eq!(a.mod_adjugate(m), x);
    };
    test("[]_{0,0}", 7, "[]_{0,0}");
    test("[5]", 7, "[1]");
    test("[1, 2; 3, 4]", 5, "[4, 3; 2, 1]");
    test(
        "[1, 2, 3; 4, 5, 6; 0, 1, 6]",
        7,
        "[3, 5, 4; 4, 6, 6; 4, 6, 4]",
    );
    test(
        "[2, 0, 1, 3; 3, 1, 0, 2; 0, 4, 2, 1; 1, 1, 1, 1]",
        6,
        "[5, 3, 4, 5; 1, 3, 2, 1; 2, 0, 4, 2; 4, 0, 2, 4]",
    );
    test(
        "[1, 2, 3; 2, 4, 0; 1, 0, 1]",
        6,
        "[4, 4, 0; 4, 4, 0; 2, 2, 0]",
    );
    test("[0, 0; 0, 0]", 1, "[0, 0; 0, 0]");
    test(
        "[1, 2, 3; 2, 4, 6; 3, 6, 2]",
        7,
        "[0, 0, 0; 0, 0, 0; 0, 0, 0]",
    );
}

#[test]
fn mod_adjugate_fail() {
    assert_panic!(
        UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6]")
            .unwrap()
            .mod_adjugate(7)
    );
    assert_panic!(
        UnsignedMatrix::<u64>::from_str("[9, 2; 3, 4]")
            .unwrap()
            .mod_adjugate(7)
    );
    assert_panic!(
        UnsignedMatrix::<u64>::from_str("[1, 2; 3, 4]")
            .unwrap()
            .mod_adjugate(0)
    );
}

#[test]
fn mod_adjugate_properties() {
    unsigned_matrix_unsigned_pair_gen_var_15().test_properties(|(a, m)| {
        let adj = (&a).mod_adjugate(m);
        assert!(adj.is_valid());
        let n = a.height();
        for i in 0..n {
            for j in 0..n {
                assert_eq!(*adj.get(i, j).unwrap(), (&a).mod_cofactor(j, i, m));
            }
        }
        if m != 1 {
            let det = (&a).mod_determinant(m);
            let scaled_identity = UnsignedMatrix::<u64>::identity(n).mod_mul(det, m);
            assert_eq!((&a).mod_mul(&adj, m), scaled_identity);
            assert_eq!((&adj).mod_mul(&a, m), scaled_identity);
        }
        assert_eq!(a.transpose().mod_adjugate(m), adj.transpose());
    });
}
