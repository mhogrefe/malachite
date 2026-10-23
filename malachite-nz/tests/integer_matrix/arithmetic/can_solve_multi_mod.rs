// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::arithmetic::can_solve_multi_mod::{
    can_solve_multi_mod, can_solve_multi_mod_den,
};
use malachite_nz::integer_matrix::arithmetic::fraction_matrix::*;
use malachite_nz::integer_matrix::arithmetic::solve::can_solve_fflu;
use malachite_nz::test_util::generators::integer_matrix_pair_gen_var_3;
use std::panic::catch_unwind;

fn format_solution(x: &Option<(IntegerMatrix, Integer)>) -> String {
    x.as_ref().map_or_else(
        || "None".to_string(),
        |(x, den)| format!("Some(({x}, {den}))"),
    )
}

// Checks the solution (X, d) of AX = dB, if any: d is positive, AX = dB, the fractions agree with
// (X, d), and fraction-free LU decomposition agrees on whether there is a solution. Returns it.
fn verify_solution(a: &IntegerMatrix, b: &IntegerMatrix) -> Option<(IntegerMatrix, Integer)> {
    let result = can_solve_multi_mod_den(a, b);
    assert_eq!(
        can_solve_multi_mod(a, b).map(|x| {
            let (x, den) = fractions_to_numerators_and_denominator(
                &x,
                usize::try_from(a.width()).unwrap(),
                usize::try_from(b.width()).unwrap(),
            );
            (x, Integer::from(den))
        }),
        result
    );
    assert_eq!(result.is_some(), can_solve_fflu(a, b).is_some());
    if let Some((x, den)) = &result {
        assert!(x.is_valid());
        assert_eq!(x.height(), a.width());
        assert_eq!(x.width(), b.width());
        assert!(*den > 0u32);
        assert_eq!(a * x, b * den);
    }
    result
}

#[test]
fn test_can_solve_multi_mod_den() {
    let test = |a, b, out| {
        let a = IntegerMatrix::from_str(a).unwrap();
        let b = IntegerMatrix::from_str(b).unwrap();
        assert_eq!(format_solution(&verify_solution(&a, &b)), out);
    };
    test("[]_{0,3}", "[]_{0,2}", "Some(([0, 0; 0, 0; 0, 0], 1))");
    test("[]_{2,0}", "[0; 0]", "Some(([]_{0,1}, 1))");
    test("[]_{2,0}", "[0; 1]", "None");
    test("[1, 2; 3, 4]", "[]_{2,0}", "Some(([]_{2,0}, 1))");
    test("[0]", "[0]", "Some(([0], 1))");
    test("[0]", "[1]", "None");
    test("[3]", "[7]", "Some(([7], 3))");
    test("[1, 2; 3, 4]", "[5; 6]", "Some(([-8; 9], 2))");
    test("[2, 4; 1, 2]", "[2; 1]", "Some(([1; 0], 1))");
    test("[2, 4; 1, 2]", "[1; 1]", "None");
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 9]",
        "[1; 1; 1]",
        "Some(([-1; 1; 0], 1))",
    );
    test("[1, 2, 3; 4, 5, 6; 7, 8, 9]", "[1; 0; 0]", "None");
    test("[0, 2, 3]", "[6]", "Some(([0; 3; 0], 1))");
    test("[1, 2, 3; 2, 4, 6]", "[1; 2]", "Some(([1; 0; 0], 1))");
    test("[1, 2, 3; 2, 4, 6]", "[1; 3]", "None");
    test("[2, 1; 4, 2; 6, 3]", "[1; 2; 3]", "Some(([1; 0], 2))");
    test("[2, 1; 1, 1; 1, 2]", "[3; 2; 3]", "Some(([1; 1], 1))");
    test("[2, 1; 1, 1; 1, 2]", "[3; 2; 4]", "None");
    test(
        "[0, 1; 1, 0; 0, 0]",
        "[1, 2; 3, 4; 0, 0]",
        "Some(([3, 4; 1, 2], 1))",
    );
    test(
        "[1267650600228229401496703205376, 1; 2535301200456458802993406410752, 2]",
        "[1; 2]",
        "Some(([1; 0], 1267650600228229401496703205376))",
    );
    // - the rank modulo a later prime is lower than the best so far
    test(
        "[576460752303423649, 0; 0, 1]",
        "[0; 1]",
        "Some(([0; 1], 1))",
    );
    // - the rank modulo a later prime is higher than the best so far
    test(
        "[576460752303423619, 0; 0, 1]",
        "[0; 1]",
        "Some(([0; 1], 1))",
    );
    // - a later prime gives the same rank with earlier row swaps
    test(
        "[576460752303423619, 576460752303423619; 1, 1]",
        "[576460752303423619; 1]",
        "Some(([1; 0], 1))",
    );
    // - a later prime gives the same rank with later row swaps
    test(
        "[576460752303423649, 576460752303423649; 1, 1]",
        "[576460752303423649; 1]",
        "Some(([1; 0], 1))",
    );
}

#[test]
fn can_solve_multi_mod_fail() {
    let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    let b = IntegerMatrix::from_str("[1; 2; 3]").unwrap();
    assert_panic!(can_solve_multi_mod_den(&a, &b));
}

#[test]
fn can_solve_multi_mod_properties() {
    integer_matrix_pair_gen_var_3().test_properties(|(a, b)| {
        verify_solution(&a, &b);
        let (h, w) = (a.height(), a.width());
        let k = b.width();
        if k < h.min(w) {
            verify_solution(&(&b * a.window(0..k, 0..w)), &b);
        }
        // A consistent system.
        let b = &a * (a.transpose() * &b);
        assert!(verify_solution(&a, &b).is_some());
    });
}
