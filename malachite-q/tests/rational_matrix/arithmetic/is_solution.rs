// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::basic::traits::One;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::arithmetic::fraction_matrix::{
    Fraction, is_solution as is_fraction_solution,
};
use malachite_nz::integer_matrix::arithmetic::solve::solve_fflu;
use malachite_nz::test_util::generators::integer_matrix_pair_gen_var_7;
use malachite_q::Rational;
use malachite_q::rational_matrix::RationalMatrix;
use std::panic::catch_unwind;

// Whether the rational matrix `x` solves AX = B, by malachite-nz's check on its fractions.
fn is_solution(x: &RationalMatrix, a: &IntegerMatrix, b: &IntegerMatrix) -> bool {
    let fractions: Vec<Fraction> = x
        .elements_ref()
        .iter()
        .map(|y| Fraction {
            sign: *y >= 0u32,
            numerator: y.numerator_ref().clone(),
            denominator: y.denominator_ref().clone(),
        })
        .collect();
    is_fraction_solution(
        &fractions,
        usize::try_from(x.height()).unwrap(),
        usize::try_from(x.width()).unwrap(),
        a,
        b,
    )
}

#[test]
fn test_is_solution() {
    let test = |x, a, b, out| {
        let x = RationalMatrix::from_str(x).unwrap();
        let a = IntegerMatrix::from_str(a).unwrap();
        let b = IntegerMatrix::from_str(b).unwrap();
        assert_eq!(is_solution(&x, &a, &b), out);
    };
    test("[]_{0,0}", "[]_{0,0}", "[]_{0,0}", true);
    test("[]_{2,0}", "[1, 2]", "[]_{1,0}", true);
    test("[]_{0,1}", "[]_{2,0}", "[0; 0]", true);
    test("[]_{0,1}", "[]_{2,0}", "[0; 1]", false);
    test("[-4; 9/2]", "[1, 2; 3, 4]", "[5; 6]", true);
    test("[-4; 9/2]", "[1, 2; 3, 4]", "[5; 7]", false);
    test("[4; 9/2]", "[1, 2; 3, 4]", "[5; 6]", false);
    test("[1/2, 1/3]", "[2]", "[1, 1]", false);
    test("[1/2, 1/3]", "[6]", "[3, 2]", true);
    test("[1, 0; 0, 1]", "[2, 4; 1, 2]", "[2, 4; 1, 2]", true);
    test("[1; 0]", "[2, 4; 1, 2]", "[2; 2]", false);
}

#[test]
fn is_solution_fail() {
    let x = RationalMatrix::from_str("[1; 2]").unwrap();
    let a = IntegerMatrix::from_str("[1, 2, 3]").unwrap();
    let b = IntegerMatrix::from_str("[1]").unwrap();
    assert_panic!(is_solution(&x, &a, &b));
    let a = IntegerMatrix::from_str("[1, 2]").unwrap();
    let b = IntegerMatrix::from_str("[1; 2]").unwrap();
    assert_panic!(is_solution(&x, &a, &b));
    let b = IntegerMatrix::from_str("[1, 2]").unwrap();
    assert_panic!(is_solution(&x, &a, &b));
}

#[test]
fn is_solution_properties() {
    integer_matrix_pair_gen_var_7().test_properties(|(a, b)| {
        // The solution of a nonsingular system, X / d, solves it; changing an element of B by 1
        // then makes it not solve the system.
        if let Some((x, d)) = solve_fflu(&a, &b) {
            let x = RationalMatrix::from_owned_elements(
                x.height(),
                x.width(),
                x.into_elements()
                    .into_iter()
                    .map(|n| Rational::from_integers(n, d.clone()))
                    .collect(),
            );
            assert!(is_solution(&x, &a, &b));
            if !b.elements_ref().is_empty() && !a.elements_ref().is_empty() {
                let mut c = b.clone();
                c[(0, 0)] += Integer::ONE;
                assert!(!is_solution(&x, &a, &c));
            }
        }
    });
}
