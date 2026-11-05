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
use malachite_base::num::linear_algebra::traits::ModGramSchmidt;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::modular::traits::ModMul;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::{
    natural_matrix_natural_pair_gen_var_2, natural_matrix_natural_pair_gen_var_4,
};
use malachite_nz::test_util::natural_matrix::linear_algebra::gram_schmidt::*;

// Checks the Gram-Schmidt process applied to `a` modulo `m` in every way of calling it, against the
// modified algorithm and, when `m` fits in a `u64`, against `UnsignedMatrix`; checks that the
// columns of the result are orthogonal, and that orthogonalizing them again changes nothing; and
// returns the result.
fn verify(a: &NaturalMatrix, m: &Natural) -> Option<NaturalMatrix> {
    let b = a.mod_gram_schmidt(m);
    assert_eq!(a.clone().mod_gram_schmidt(m.clone()), b);
    assert_eq!(a.clone().mod_gram_schmidt(m), b);
    assert_eq!(a.mod_gram_schmidt(m.clone()), b);
    assert_eq!(mod_gram_schmidt_modified(a, m), b);
    if m.significant_bits() <= 64 {
        let a_u64 = UnsignedMatrix::<u64>::from_str(&a.to_string()).unwrap();
        assert_eq!(
            a_u64
                .mod_gram_schmidt(u64::exact_from(m))
                .map(|b| b.to_string()),
            b.as_ref().map(ToString::to_string)
        );
    }
    if let Some(b) = &b {
        assert!(b.is_valid());
        let gram = (&b.transpose()).mod_mul(b, m);
        let width = usize::exact_from(b.width());
        for i in 0..width {
            for j in 0..width {
                if i != j {
                    assert_eq!(gram[(i, j)], 0u32);
                }
            }
        }
        assert_eq!(b.mod_gram_schmidt(m).as_ref(), Some(b));
    }
    b
}

#[test]
fn test_mod_gram_schmidt() {
    let test = |a, m, out: Option<&str>| {
        let a = NaturalMatrix::from_str(a).unwrap();
        let m = Natural::from_str(m).unwrap();
        let b = verify(&a, &m).map(|b| b.to_string());
        assert_eq!(b.as_deref(), out);
    };
    // - height == 0
    test("[]_{0,0}", "5", Some("[]_{0,0}"));
    test("[]_{0,3}", "5", Some("[]_{0,3}"));
    test("[]_{2,0}", "5", Some("[]_{2,0}"));
    // - ring.is_zero(&den) && column j is zero
    test("[0, 0; 0, 0]", "1", Some("[0, 0; 0, 0]"));
    // - !ring.is_zero(&den) && ring.inverse(&den).is_some()
    test("[1, 1; 1, 0]", "5", Some("[1, 3; 1, 2]"));
    // - ring.is_zero(&den) && column j is nonzero
    test("[1, 1; 1, 0]", "2", None);
    test("[1; 1]", "2", Some("[1; 1]"));
    test("[0, 1; 0, 1]", "5", Some("[0, 1; 0, 1]"));
    // - !ring.is_zero(&den) && ring.inverse(&den).is_none()
    test("[1, 0; 1, 1]", "6", None);
    test("[1, 1; 0, 1]", "6", Some("[1, 0; 0, 1]"));
    test("[1, 0, 1; 0, 1, 1]", "7", Some("[1, 0, 0; 0, 1, 0]"));
    test(
        "[1, 2, 3; 2, 4, 1; 0, 0, 5]",
        "7",
        Some("[1, 0, 2; 2, 0, 6; 0, 0, 5]"),
    );
    test("[3, 1; 4, 1]", "5", None);
    test("[1, 2, 3; 4, 5, 6; 7, 8, 10]", "11", None);
    test("[2, 3; 5, 7; 11, 13]", "17", Some("[2, 1; 5, 2; 11, 2]"));
    test(
        "[1, 1; 1, 0]",
        "6277101735386680763835789423207666416102355444464034513029",
        Some(
            "[1, 3138550867693340381917894711603833208051177722232017256515; 1, \
            3138550867693340381917894711603833208051177722232017256514]",
        ),
    );
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 10]",
        "6277101735386680763835789423207666416102355444464034513029",
        Some(
            "[1, 3994519286155160486077320542041242264792408010113476508292, \
            1046183622564446793972631570534611069350392574077339085505; 4, \
            3423873673847280416637703321749636226964921151525837007107, \
            4184734490257787175890526282138444277401570296309356342019; 7, \
            2853228061539400347198086101458030189137434292938197505922, \
            1046183622564446793972631570534611069350392574077339085505]",
        ),
    );
    test(
        "[1, 0; 1, 1]",
        "18831305206160042291507368269622999248307066333392103539087",
        Some(
            "[1, 9415652603080021145753684134811499624153533166696051769543; 1, \
            9415652603080021145753684134811499624153533166696051769544]",
        ),
    );
    test(
        "[3, 0; 0, 1]",
        "18831305206160042291507368269622999248307066333392103539087",
        None,
    );
    test(
        "[1, 1; 1, 0]",
        "340282366920938463463374607431768211297",
        Some(
            "[1, 170141183460469231731687303715884105649; 1, \
            170141183460469231731687303715884105648]",
        ),
    );
}

#[test]
#[should_panic]
fn mod_gram_schmidt_fail_1() {
    NaturalMatrix::from_str("[1, 7]")
        .unwrap()
        .mod_gram_schmidt(Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_gram_schmidt_fail_2() {
    NaturalMatrix::from_str("[1, 2]")
        .unwrap()
        .mod_gram_schmidt(Natural::ZERO);
}

#[test]
fn mod_gram_schmidt_properties() {
    natural_matrix_natural_pair_gen_var_2().test_properties(|(a, m)| {
        verify(&a, &m);
    });

    natural_matrix_natural_pair_gen_var_4().test_properties(|(a, m)| {
        verify(&a, &m);
    });
}
