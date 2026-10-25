// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::DivisibleBy;
use malachite_base::num::basic::traits::{Two, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_natural_pair_gen_var_8;
use malachite_q::Rational;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::generators::rational_matrix_gen;
use malachite_q::test_util::rational_matrix::arithmetic::reconstruct::*;
use std::panic::catch_unwind;

fn verify_reconstruct(a: &NaturalMatrix, m: &Natural) -> Option<RationalMatrix> {
    let result = RationalMatrix::reconstruct_ref(a, m);
    assert_eq!(RationalMatrix::reconstruct(a.clone(), m), result);
    if let Some(x) = &result {
        assert!(x.is_valid());
        assert_eq!(x.height(), a.height());
        assert_eq!(x.width(), a.width());
        // Every element n/d has the given residue r: n = rd (mod m).
        for (y, r) in x.elements_ref().iter().zip(a.elements_ref()) {
            let n = Integer::from_sign_and_abs_ref(*y >= 0u32, y.numerator_ref());
            let rd = Integer::from(r * y.denominator_ref());
            assert!((rd - n).divisible_by(Integer::from(m)));
        }
    }
    result
}

fn test(a: &str, m: &str, out: &str) {
    let a = NaturalMatrix::from_str(a).unwrap();
    let m = Natural::from_str(m).unwrap();
    assert_eq!(
        verify_reconstruct(&a, &m).map_or_else(|| "None".to_string(), |x| format!("Some({x})")),
        out
    );
}

#[test]
fn test_reconstruct() {
    test("[]_{0,0}", "101", "Some([]_{0,0})");
    test("[]_{2,0}", "101", "Some([]_{2,0})");
    test("[]_{0,3}", "101", "Some([]_{0,3})");
    test("[51, 68; 0, 100]", "101", "Some([1/2, 2/3; 0, -1])");
    test(
        "[50002, 33335; 0, 100002]",
        "100003",
        "Some([1/2, 2/3; 0, -1])",
    );
    test("[1430; 4288; 4289]", "10007", "Some([3/7; -5/7; 2/7])");
    test("[1, 2, 3]", "11", "None");
    test("[50]", "101", "Some([-1/2])");
    test("[5, 6]", "13", "None");
    test("[3, 4; 5, 6]", "1000003", "Some([3, 4; 5, 6])");
    test(
        "[1308289358842929716364031448206762441342]",
        "1361129467683753853853498429727072845819",
        "Some([-13717421/109739369])",
    );
}

#[test]
fn reconstruct_fail() {
    let a = NaturalMatrix::from_str("[1, 5]").unwrap();
    assert_panic!(RationalMatrix::reconstruct_ref(&a, &Natural::ZERO));
    // The modulus must be greater than 2, as for `Rational::reconstruct`.
    assert_panic!(RationalMatrix::reconstruct_ref(
        &NaturalMatrix::from_str("[0, 1]").unwrap(),
        &Natural::TWO
    ));
    assert_panic!(RationalMatrix::reconstruct_ref(
        &NaturalMatrix::from_str("[]_{0,0}").unwrap(),
        &Natural::TWO
    ));
    // The elements must be reduced.
    assert_panic!(RationalMatrix::reconstruct_ref(&a, &Natural::TWO));
    assert_panic!(RationalMatrix::reconstruct(a, &Natural::from(3u32)));
}

#[test]
fn reconstruct_properties() {
    natural_matrix_natural_pair_gen_var_8().test_properties(|(a, m)| {
        let result = verify_reconstruct(&a, &m);
        // The first element is reconstructed on its own, by `Rational::reconstruct`.
        if let Some(r) = a.elements_ref().first() {
            let first = Rational::reconstruct_ref(r, &m);
            match &result {
                Some(x) => assert_eq!(first.as_ref(), x.elements_ref().first()),
                None => assert!(
                    first.is_none() || a.elements_ref().len() != 1,
                    "a single element that reconstructs on its own must reconstruct"
                ),
            }
        }
    });

    rational_matrix_gen().test_properties(|a| {
        // With a large enough modulus coprime to the denominators, a matrix is recovered from its
        // residues.
        let m = rational_matrix_reconstruction_modulus(&a);
        let residues = rational_matrix_residues(&a, &m);
        assert_eq!(verify_reconstruct(&residues, &m), Some(a));
    });
}
