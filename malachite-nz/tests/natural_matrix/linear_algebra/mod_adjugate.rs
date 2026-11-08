// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::linear_algebra::traits::{
    Adjugate, ModAdjugate, ModCofactor, ModDeterminant,
};
use malachite_base::num::modular::traits::{Mod, ModMul};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_natural_pair_gen_var_9;
use std::panic::catch_unwind;

#[test]
fn test_mod_adjugate() {
    let test = |s, m: u32, out: &str| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let m = Natural::from(m);
        let x = (&a).mod_adjugate(&m);
        assert_eq!(x.to_string(), out);
        assert_eq!((&a).mod_adjugate(m.clone()), x);
        assert_eq!(a.clone().mod_adjugate(&m), x);
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
        NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]")
            .unwrap()
            .mod_adjugate(Natural::from(7u32))
    );
    assert_panic!(
        NaturalMatrix::from_str("[9, 2; 3, 4]")
            .unwrap()
            .mod_adjugate(Natural::from(7u32))
    );
    assert_panic!(
        NaturalMatrix::from_str("[1, 2; 3, 4]")
            .unwrap()
            .mod_adjugate(Natural::ZERO)
    );
}

#[test]
fn mod_adjugate_properties() {
    natural_matrix_natural_pair_gen_var_9().test_properties(|(a, m)| {
        let adj = (&a).mod_adjugate(&m);
        assert!(adj.is_valid());
        let n = a.height();
        for i in 0..n {
            for j in 0..n {
                assert_eq!(*adj.get(i, j).unwrap(), (&a).mod_cofactor(j, i, &m));
            }
        }
        if m != 1u32 {
            let det = (&a).mod_determinant(&m);
            let scaled_identity = NaturalMatrix::identity(n).mod_mul(&det, &m);
            assert_eq!((&a).mod_mul(&adj, &m), scaled_identity);
            assert_eq!((&adj).mod_mul(&a, &m), scaled_identity);
        }
        assert_eq!(
            NaturalMatrix::from_owned_elements(
                n,
                n,
                (&a).adjugate()
                    .into_elements()
                    .into_iter()
                    .map(|x| Natural::exact_from(x.mod_op(Integer::from(&m))))
                    .collect()
            ),
            adj
        );
        assert_eq!(a.transpose().mod_adjugate(&m), adj.transpose());
    });
}
