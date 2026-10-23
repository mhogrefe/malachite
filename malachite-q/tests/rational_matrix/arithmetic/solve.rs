// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::Solve;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::rational_matrix::arithmetic::solve::*;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::{
    rational_matrix_pair_gen_var_5, rational_matrix_pair_gen_var_6,
    rational_matrix_rational_vector_pair_gen_var_3,
};
use malachite_q::test_util::rational_matrix::arithmetic::solve::rational_matrix_low_rank;
use std::panic::catch_unwind;

fn format_result(x: &Option<RationalMatrix>) -> String {
    x.as_ref()
        .map_or_else(|| "None".to_string(), |x| format!("Some({x})"))
}

// Checks that `x`, if it is some X, solves AX = B.
fn check_solution(a: &RationalMatrix, b: &RationalMatrix, x: &Option<RationalMatrix>) {
    if let Some(x) = x {
        assert!(x.is_valid());
        assert_eq!(x.height(), a.width());
        assert_eq!(x.width(), b.width());
        assert_eq!(a * x, *b);
    }
}

// Checks the square solvers for AX = B: they all agree, since the solution is unique when A is
// nonsingular, and their result solves the system. Returns the result.
fn verify_square(a: &RationalMatrix, b: &RationalMatrix) -> Option<RationalMatrix> {
    let x = solve_nonsingular(a, b);
    check_solution(a, b, &x);
    assert_eq!(solve_fraction_free(a, b), x);
    assert_eq!(solve_dixon(a, b), x);
    assert_eq!(solve_multi_mod(a, b), x);
    // When there is a solution, it is unique, so the general solver finds it too.
    if x.is_some() {
        assert_eq!(can_solve_fraction_free(a, b), x);
    }
    x
}

// Checks the general solvers for AX = B: they agree on whether there is a solution, each result
// solves the system, and `can_solve` is the fraction-free solver up to 15 rows and the multimodular
// one above that. Returns the result of `can_solve`. The Dixon solver is only run on consistent
// systems: on an inconsistent one, it solves a subsystem for every prime until their product
// exceeds a determinant bound, which takes too long for property tests of large matrices; the unit
// tests and the FLINT oracle cover it there.
fn verify_general(a: &RationalMatrix, b: &RationalMatrix) -> Option<RationalMatrix> {
    let fraction_free = can_solve_fraction_free(a, b);
    let multi_mod = can_solve_multi_mod(a, b);
    check_solution(a, b, &fraction_free);
    check_solution(a, b, &multi_mod);
    assert_eq!(fraction_free.is_some(), multi_mod.is_some());
    if fraction_free.is_some() {
        let dixon = can_solve_dixon(a, b);
        assert!(dixon.is_some());
        check_solution(a, b, &dixon);
    }
    let x = can_solve(a, b);
    assert_eq!(
        x,
        if a.height() <= 15 {
            fraction_free
        } else {
            multi_mod
        }
    );
    x
}

// Checks the public `Solve` for AX = B: the four ways of passing the arguments agree, and the
// result is the unique solution when A is square and nonsingular and that of `can_solve` otherwise.
// Returns the result.
fn verify_public(a: &RationalMatrix, b: &RationalMatrix) -> Option<RationalMatrix> {
    let x = a.clone().solve(b.clone());
    assert_eq!(a.clone().solve(b), x);
    assert_eq!(a.solve(b.clone()), x);
    assert_eq!(a.solve(b), x);
    let general = verify_general(a, b);
    if a.height() == a.width()
        && let Some(y) = verify_square(a, b)
    {
        assert_eq!(x, Some(y));
    } else {
        assert_eq!(x, general);
    }
    x
}

#[test]
fn test_solve() {
    let test = |a, b, out| {
        let a = RationalMatrix::from_str(a).unwrap();
        let b = RationalMatrix::from_str(b).unwrap();
        assert_eq!(format_result(&verify_public(&a, &b)), out);
    };
    test("[]_{0,0}", "[]_{0,2}", "Some([]_{0,2})");
    test("[]_{0,3}", "[]_{0,2}", "Some([0, 0; 0, 0; 0, 0])");
    test("[]_{2,0}", "[0; 0]", "Some([]_{0,1})");
    test("[]_{2,0}", "[0; 1/2]", "None");
    test("[2, 1; 1, 3]", "[]_{2,0}", "Some([]_{2,0})");
    test("[1/2]", "[3/4]", "Some([3/2])");
    test("[0]", "[0]", "Some([0])");
    test("[0]", "[1]", "None");
    test("[1, 2; 3, 4]", "[5; 6]", "Some([-4; 9/2])");
    test("[1/2, 1/3; 1/4, 1/5]", "[1; 1]", "Some([-8; 15])");
    test("[2, 4; 1, 2]", "[2; 1]", "Some([1; 0])");
    test("[2, 4; 1, 2]", "[1; 1]", "None");
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 10]",
        "[1; 1; 1]",
        "Some([-1; 1; 0])",
    );
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 9]",
        "[1; 1; 1]",
        "Some([-1; 1; 0])",
    );
    test("[1, 2, 3; 4, 5, 6; 7, 8, 9]", "[1; 0; 0]", "None");
    test(
        "[1, 1/2, 1/3, 1/4; 1/2, 1/3, 1/4, 1/5; 1/3, 1/4, 1/5, 1/6; 1/4, 1/5, 1/6, 1/7]",
        "[1; 0; 0; 0]",
        "Some([16; -120; 240; -140])",
    );
    test("[1/2, 2/3, 3/4]", "[5/6]", "Some([5/3; 0; 0])");
    test("[1, 2, 3; 2, 4, 6]", "[1; 2]", "Some([1; 0; 0])");
    test("[1, 2, 3; 2, 4, 6]", "[1; 3]", "None");
    test("[1; 2; 3]", "[1/2; 1; 3/2]", "Some([1/2])");
    test("[1; 2; 3]", "[1/2; 1; 2]", "None");
    test("[2, 1; 1, 1; 1, 2]", "[3; 2; 3]", "Some([1; 1])");
    test("[2, 1; 1, 1; 1, 2]", "[3; 2; 4]", "None");
}

#[test]
fn test_solve_vector() {
    let test = |a, b, out| {
        let a = RationalMatrix::from_str(a).unwrap();
        let b = RationalVector::from_str(b).unwrap();
        let x = a.clone().solve(b.clone());
        assert_eq!(a.clone().solve(&b), x);
        assert_eq!((&a).solve(b.clone()), x);
        assert_eq!((&a).solve(&b), x);
        assert_eq!(
            x.as_ref()
                .map_or_else(|| "None".to_string(), |x| format!("Some({x})")),
            out
        );
    };
    test("[]_{0,2}", "()", "Some((0, 0))");
    test("[1/2]", "(3/4)", "Some((3/2))");
    test("[1, 2; 3, 4]", "(5, 6)", "Some((-4, 9/2))");
    test("[2, 4; 1, 2]", "(1, 1)", "None");
    test("[1, 2, 3; 2, 4, 6]", "(1, 2)", "Some((1, 0, 0))");
}

#[test]
fn test_solve_square() {
    let test = |a, b, out| {
        let a = RationalMatrix::from_str(a).unwrap();
        let b = RationalMatrix::from_str(b).unwrap();
        assert_eq!(format_result(&verify_square(&a, &b)), out);
    };
    test("[]_{0,0}", "[]_{0,2}", "Some([]_{0,2})");
    test("[2, 1; 1, 3]", "[]_{2,0}", "Some([]_{2,0})");
    test("[1/2]", "[3/4]", "Some([3/2])");
    test("[0]", "[1]", "None");
    test("[1, 2; 3, 4]", "[5; 6]", "Some([-4; 9/2])");
    test("[2, 4; 1, 2]", "[2; 1]", "None");
    test(
        "[1/2, 1/3; 1/4, 1/5]",
        "[1, 0; 0, 1]",
        "Some([12, -20; -15, 30])",
    );
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 10]",
        "[1/2; 1/3; 1/4]",
        "Some([-19/36; 7/18; 1/12])",
    );
}

#[test]
fn test_can_solve() {
    let test = |a, b, out| {
        let a = RationalMatrix::from_str(a).unwrap();
        let b = RationalMatrix::from_str(b).unwrap();
        assert_eq!(format_result(&verify_general(&a, &b)), out);
    };
    test("[]_{0,3}", "[]_{0,2}", "Some([0, 0; 0, 0; 0, 0])");
    test("[]_{2,0}", "[0; 0]", "Some([]_{0,1})");
    test("[]_{2,0}", "[0; 1]", "None");
    test("[1, 2; 3, 4]", "[]_{2,0}", "Some([]_{2,0})");
    test("[0]", "[0]", "Some([0])");
    test("[0]", "[1]", "None");
    test("[1, 2; 3, 4]", "[5; 6]", "Some([-4; 9/2])");
    test("[2, 4; 1, 2]", "[2; 1]", "Some([1; 0])");
    test("[2, 4; 1, 2]", "[1; 1]", "None");
    test("[1/2, 2/3, 3/4]", "[5/6]", "Some([5/3; 0; 0])");
    test("[1, 2, 3; 2, 4, 6]", "[1; 2]", "Some([1; 0; 0])");
    test("[1, 2, 3; 2, 4, 6]", "[1; 3]", "None");
    test("[2, 1; 4, 2; 6, 3]", "[1; 2; 3]", "Some([1/2; 0])");
    test("[2, 1; 1, 1; 1, 2]", "[3; 2; 3]", "Some([1; 1])");
    test("[2, 1; 1, 1; 1, 2]", "[3; 2; 4]", "None");
    test(
        "[0, 1; 1, 0; 0, 0]",
        "[1, 2; 3, 4; 0, 0]",
        "Some([3, 4; 1, 2])",
    );
}

#[test]
fn solve_fail() {
    let a = RationalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    let b = RationalMatrix::from_str("[1; 2; 3]").unwrap();
    assert_panic!(a.clone().solve(b.clone()));
    assert_panic!(solve_nonsingular(&a, &b));
    assert_panic!(can_solve(&a, &b));
    assert_panic!(can_solve_dixon(&a, &b));
    let b = RationalVector::from_str("(1, 2, 3)").unwrap();
    assert_panic!(a.solve(b));
    let a = RationalMatrix::from_str("[1, 2]").unwrap();
    let b = RationalMatrix::from_str("[1]").unwrap();
    assert_panic!(solve_nonsingular(&a, &b));
    assert_panic!(solve_dixon(&a, &b));
    assert_panic!(solve_multi_mod(&a, &b));
}

#[test]
fn solve_properties() {
    rational_matrix_pair_gen_var_5().test_properties(|(a, b)| {
        verify_square(&a, &b);
        verify_square(&rational_matrix_low_rank(&a, &b), &b);
    });

    rational_matrix_pair_gen_var_6().test_properties(|(a, b)| {
        verify_public(&a, &b);
        let a = rational_matrix_low_rank(&a, &b);
        verify_public(&a, &b);
        // A consistent system: B = A A^T B is in the column space of A.
        let b = &a * (a.transpose() * &b);
        assert!(verify_public(&a, &b).is_some());
    });

    rational_matrix_rational_vector_pair_gen_var_3().test_properties(|(a, b)| {
        let column = RationalMatrix::from_owned_elements(a.height(), 1, b.elements.clone());
        assert_eq!(
            (&a).solve(&b),
            (&a).solve(&column).map(|x| RationalVector {
                elements: x.into_elements()
            })
        );
    });
}
