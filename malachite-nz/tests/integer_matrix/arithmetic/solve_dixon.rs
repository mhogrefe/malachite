// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::Gcd;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::arithmetic::fraction_matrix::*;
use malachite_nz::integer_matrix::arithmetic::solve::solve_fflu;
use malachite_nz::integer_matrix::arithmetic::solve_dixon::{solve_dixon, solve_dixon_den};
use malachite_nz::test_util::generators::integer_matrix_pair_gen_var_7;
use std::panic::catch_unwind;

fn format_solution(x: &Option<(IntegerMatrix, Integer)>) -> String {
    x.as_ref().map_or_else(
        || "None".to_string(),
        |(x, den)| format!("Some(({x}, {den}))"),
    )
}

// Checks the solution (X, d) of AX = dB: d is the positive least common denominator of the
// solution, the fractions agree with (X, d), and, as rational matrices, the solution is the one
// from fraction-free LU decomposition. Returns the solution.
fn verify_solution(a: &IntegerMatrix, b: &IntegerMatrix) -> Option<(IntegerMatrix, Integer)> {
    let result = solve_dixon_den(a, b);
    let fractions = solve_dixon(a, b);
    assert_eq!(
        fractions.map(|x| {
            let (x, den) = fractions_to_numerators_and_denominator(
                &x,
                usize::try_from(a.width()).unwrap(),
                usize::try_from(b.width()).unwrap(),
            );
            (x, Integer::from(den))
        }),
        result
    );
    let fflu = solve_fflu(a, b);
    assert_eq!(result.is_some(), fflu.is_some());
    if let Some((x, den)) = &result {
        assert!(x.is_valid());
        assert_eq!(x.height(), a.width());
        assert_eq!(x.width(), b.width());
        assert!(*den > 0u32);
        assert_eq!(a * x, b * den);
        let g = x
            .elements_ref()
            .iter()
            .fold(den.unsigned_abs_ref().clone(), |g, y| {
                g.gcd(y.unsigned_abs_ref())
            });
        assert_eq!(g, 1u32);
        let (y, e) = fflu.unwrap();
        assert_eq!(x * e, y * den);
    }
    result
}

#[test]
fn test_solve_dixon_den() {
    let test = |a, b, out| {
        let a = IntegerMatrix::from_str(a).unwrap();
        let b = IntegerMatrix::from_str(b).unwrap();
        assert_eq!(format_solution(&verify_solution(&a, &b)), out);
    };
    test("[]_{0,0}", "[]_{0,2}", "Some(([]_{0,2}, 1))");
    test("[2, 1; 1, 3]", "[]_{2,0}", "Some(([]_{2,0}, 1))");
    test("[0, 0; 0, 0]", "[]_{2,0}", "Some(([]_{2,0}, 1))");
    test("[3]", "[6]", "Some(([2], 1))");
    test("[3]", "[7]", "Some(([7], 3))");
    test("[-3]", "[7]", "Some(([-7], 3))");
    test("[0]", "[1]", "None");
    test("[1, 2; 3, 4]", "[5; 6]", "Some(([-8; 9], 2))");
    test("[0, 1; 1, 0]", "[1, 2; 3, 4]", "Some(([3, 4; 1, 2], 1))");
    test("[2, 4; 1, 2]", "[1; 1]", "None");
    test("[2, 4; 1, 2]", "[2; 1]", "None");
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 10]",
        "[1; 1; 1]",
        "Some(([-1; 1; 0], 1))",
    );
    test("[1, 2, 3; 4, 5, 6; 7, 8, 9]", "[1; 1; 1]", "None");
    test(
        "[1, 2, 3, 4; 5, 6, 7, 8; 2, 6, 4, 8; 3, 1, 1, 2]",
        "[1, 0; 0, 1; 0, 0; 0, 0]",
        "Some(([-6, 2; -30, 10; 6, 10; 21, -13], 36))",
    );
    test(
        "[1267650600228229401496703205376, 1; 1, 1267650600228229401496703205376]",
        "[1267650600228229401496703205376; 0]",
        "Some(([1606938044258990275541962092341162602522202993782792835301376; -1267650600228229401\
        496703205376], 1606938044258990275541962092341162602522202993782792835301375))",
    );
    test(
        "[1000000000000000000000, 1; 1, 1000000000000000000000]",
        "[1; 1]",
        "Some(([1; 1], 1000000000000000000001))",
    );
    // - the matrix is singular modulo the first prime, but not over the integers
    test(
        "[576460752303423619, 0; 0, 1]",
        "[1; 1]",
        "Some(([1; 576460752303423619], 576460752303423619))",
    );
}

#[test]
fn solve_dixon_fail() {
    let a = IntegerMatrix::from_str("[1, 2]").unwrap();
    let b = IntegerMatrix::from_str("[1]").unwrap();
    assert_panic!(solve_dixon_den(&a, &b));
    let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    let b = IntegerMatrix::from_str("[1; 2; 3]").unwrap();
    assert_panic!(solve_dixon_den(&a, &b));
}

#[test]
fn solve_dixon_properties() {
    integer_matrix_pair_gen_var_7().test_properties(|(a, b)| {
        verify_solution(&a, &b);
        // A singular matrix of the same dimension, when the right-hand side is narrower.
        let n = a.height();
        let k = b.width();
        if k < n {
            verify_solution(&(&b * a.window(0..k, 0..n)), &b);
        }
    });
}
