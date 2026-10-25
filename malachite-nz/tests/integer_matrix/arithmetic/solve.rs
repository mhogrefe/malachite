// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{FractionFreeLuDecomposition, Gcd, Parity, Solve};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::arithmetic::can_solve_multi_mod::can_solve_multi_mod_den;
use malachite_nz::integer_matrix::arithmetic::determinant::determinant_bareiss;
use malachite_nz::integer_matrix::arithmetic::solve::{
    can_solve, can_solve_fflu, solve_cramer, solve_fflu, solve_nonsingular,
};
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::{
    integer_matrix_integer_vector_pair_gen_var_2, integer_matrix_pair_gen_var_3,
    integer_matrix_pair_gen_var_7, integer_matrix_pair_gen_var_8,
};
use malachite_nz::test_util::integer_matrix::arithmetic::solve::*;
use std::panic::catch_unwind;

fn format_solution(x: &Option<(IntegerMatrix, Integer)>) -> String {
    x.as_ref().map_or_else(
        || "None".to_string(),
        |(x, den)| format!("Some(({x}, {den}))"),
    )
}

// Checks a solution (X, d) of AX = dB: unless A or B has no elements, in which case d is 1, d is
// the determinant of A, which is nonzero; and, for small A, the result agrees with the naive
// adjugate solution. Returns the solution.
fn verify_solution(a: &IntegerMatrix, b: &IntegerMatrix) -> Option<(IntegerMatrix, Integer)> {
    let result = solve_fflu(a, b);
    // The naive solution computes a determinant for every element of the adjugate, so it is only
    // compared for small matrices; for any matrix, X and d are determined by AX = dB and d = det A.
    if a.height() <= 6 {
        assert_eq!(integer_matrix_solve_naive(a, b), result);
    }
    if let Some((x, den)) = &result {
        assert!(x.is_valid());
        assert_eq!(x.height(), a.width());
        assert_eq!(x.width(), b.width());
        if b.width() == 0 || a.height() == 0 {
            assert_eq!(*den, 1u32);
        } else {
            assert_eq!(*den, determinant_bareiss(a));
            assert_ne!(*den, 0u32);
            assert_eq!(a * x, b * den);
        }
    } else {
        assert_eq!(determinant_bareiss(a), 0u32);
    }
    if a.height() <= 3 {
        assert_eq!(solve_cramer(a, b), result);
    }
    result
}

#[test]
fn test_solve_fflu() {
    let test = |a, b, out| {
        let a = IntegerMatrix::from_str(a).unwrap();
        let b = IntegerMatrix::from_str(b).unwrap();
        assert_eq!(format_solution(&verify_solution(&a, &b)), out);
    };
    test("[]_{0,0}", "[]_{0,2}", "Some(([]_{0,2}, 1))");
    test("[2, 1; 1, 3]", "[]_{2,0}", "Some(([]_{2,0}, 1))");
    test("[3]", "[6]", "Some(([6], 3))");
    test("[0]", "[1]", "None");
    test("[1, 2; 3, 4]", "[5; 6]", "Some(([8; -9], -2))");
    test(
        "[0, 1; 1, 0]",
        "[1, 2; 3, 4]",
        "Some(([-3, -4; -1, -2], -1))",
    );
    test("[2, 4; 1, 2]", "[1; 1]", "None");
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 10]",
        "[1; 1; 1]",
        "Some(([3; -3; 0], -3))",
    );
    test(
        "[2, 0, 0; 0, 3, 0; 0, 0, 5]",
        "[1, 2; 3, 4; 5, 6]",
        "Some(([15, 30; 30, 40; 30, 36], 30))",
    );
    test(
        "[0, 0, 1; 0, 1, 0; 1, 0, 0]",
        "[1; 2; 3]",
        "Some(([-3; -2; -1], -1))",
    );
    test(
        "[1, 2, 3, 4; 5, 6, 7, 8; 2, 6, 4, 8; 3, 1, 1, 2]",
        "[1; 0; 0; 0]",
        "Some(([-12; -60; 12; 42], 72))",
    );
    test(
        "[1267650600228229401496703205376, 1; 1, 1267650600228229401496703205376]",
        "[1267650600228229401496703205376; 0]",
        "Some(([1606938044258990275541962092341162602522202993782792835301376; -126765060022822940\
        1496703205376], 1606938044258990275541962092341162602522202993782792835301375))",
    );
}

#[test]
fn test_solve_cramer() {
    let test = |a, b, out| {
        let a = IntegerMatrix::from_str(a).unwrap();
        let b = IntegerMatrix::from_str(b).unwrap();
        assert_eq!(format_solution(&solve_cramer(&a, &b)), out);
    };
    test("[]_{0,0}", "[]_{0,2}", "Some(([]_{0,2}, 1))");
    test("[2, 1; 1, 3]", "[]_{2,0}", "Some(([]_{2,0}, 1))");
    test("[3]", "[6]", "Some(([6], 3))");
    test("[0]", "[1]", "None");
    test("[1, 2; 3, 4]", "[5; 6]", "Some(([8; -9], -2))");
    test(
        "[0, 1; 1, 0]",
        "[1, 2; 3, 4]",
        "Some(([-3, -4; -1, -2], -1))",
    );
    test("[2, 4; 1, 2]", "[1; 1]", "None");
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 10]",
        "[1; 1; 1]",
        "Some(([3; -3; 0], -3))",
    );
    test(
        "[2, 0, 0; 0, 3, 0; 0, 0, 5]",
        "[1, 2; 3, 4; 5, 6]",
        "Some(([15, 30; 30, 40; 30, 36], 30))",
    );
    test(
        "[0, 0, 1; 0, 1, 0; 1, 0, 0]",
        "[1; 2; 3]",
        "Some(([-3; -2; -1], -1))",
    );
    test(
        "[1267650600228229401496703205376, 1; 1, 1267650600228229401496703205376]",
        "[1267650600228229401496703205376; 0]",
        "Some(([1606938044258990275541962092341162602522202993782792835301376; -126765060022822940\
        1496703205376], 1606938044258990275541962092341162602522202993782792835301375))",
    );
}

// Checks the result of `solve_fflu_precomp` for the fraction-free LU decomposition of `a` and the
// right-hand side `b`: if it is some X, then the first r rows of PAX = dPB hold, where r is the
// rank, P the permutation, and d the denominator.
fn verify_precomp(a: &IntegerMatrix, b: &IntegerMatrix) -> Option<IntegerMatrix> {
    let result = integer_matrix_solve_fflu_precomp_from_matrix(a, b);
    assert_eq!(
        integer_matrix_solve_fflu_precomp_from_matrix_general(a, b),
        result
    );
    if let Some(x) = &result {
        assert!(x.is_valid());
        assert_eq!(x.height(), a.width());
        assert_eq!(x.width(), b.width());
        let (p, l, _, den) = a.fraction_free_lu_decomposition();
        let r = l.width();
        if a.width() != 0 && b.width() != 0 {
            let ax = a * x;
            for i in 0..r {
                let row = p[usize::try_from(i).unwrap()];
                for j in 0..b.width() {
                    assert_eq!(*ax.get(row, j).unwrap(), b.get(row, j).unwrap() * &den);
                }
            }
        }
    }
    result
}

#[test]
fn test_solve_fflu_precomp() {
    let test = |a, b, out| {
        let a = IntegerMatrix::from_str(a).unwrap();
        let b = IntegerMatrix::from_str(b).unwrap();
        let x = verify_precomp(&a, &b);
        assert_eq!(
            x.map_or_else(|| "None".to_string(), |x| format!("Some({x})")),
            out
        );
    };
    test("[1, 2; 3, 4]", "[5; 6]", "Some([8; -9])");
    test(
        "[1, 2, 3; 2, 4, 6; 1, 0, 1]",
        "[1; 2; 1]",
        "Some([-2; 0; 0])",
    );
    test(
        "[1, 2, 3; 2, 4, 6; 1, 0, 1]",
        "[1; 3; 1]",
        "Some([-2; 0; 0])",
    );
    test("[1, 2, 3, 4]", "[5]", "Some([5; 0; 0; 0])");
    test("[1; 2; 3]", "[1; 2; 3]", "Some([1])");
    test("[1; 2; 3]", "[1; 1; 1]", "Some([1])");
    test("[0, 0; 0, 0]", "[0; 0]", "Some([0; 0])");
    test("[2, 1; 4, 2; 6, 3]", "[1; 2; 4]", "Some([1; 0])");
    test("[0, 2, 1; 0, 4, 3]", "[1; 1]", "Some([0; 2; -2])");
    test("[]_{0,3}", "[]_{0,2}", "Some([0, 0; 0, 0; 0, 0])");
    test("[]_{2,0}", "[1; 2]", "Some([]_{0,1})");
    test(
        "[-2, -1, -1; 1, -1, 2; -1, -1, 0; 0, -2, 2]",
        "[-2; 0; -1; 1]",
        "None",
    );
    test(
        "[2, 2, -2; 0, 0, 0; -1, 2, 1; -1, -1, 1]",
        "[1; -1; 2; 2]",
        "None",
    );
}

// The next output of the SplitMix64 generator with state `seed`.
const fn splitmix64(seed: &mut u64) -> u64 {
    *seed = seed.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = *seed;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}

// A matrix with pseudorandom elements of at most `bits` bits, of either sign.
fn pseudorandom_matrix_with_bits(
    height: u64,
    width: u64,
    bits: u64,
    seed: &mut u64,
) -> IntegerMatrix {
    let elements = (0..height * width)
        .map(|_| {
            let x = Integer::from(splitmix64(seed) >> (64 - bits));
            if splitmix64(seed).even() { x } else { -x }
        })
        .collect();
    IntegerMatrix::from_owned_elements(height, width, elements)
}

#[test]
fn test_solve_small_and_large_elements() {
    let mut seed = 0;
    // - elements of 3 and 30 bits, whose products eventually outgrow a word
    // - elements of 61 and 62 bits, whose first products already outgrow a word
    // - elements of 63 bits, too large for the small-element path from the start
    for bits in [3, 30, 61, 62, 63] {
        for (n, k) in [(3, 1), (4, 4), (6, 2), (9, 3)] {
            let a = pseudorandom_matrix_with_bits(n, n, bits, &mut seed);
            let b = pseudorandom_matrix_with_bits(n, k, bits, &mut seed);
            verify_solution(&a, &b);
            verify_precomp(&a, &b);
        }
        for (h, w, k) in [(5, 3, 2), (3, 6, 2), (6, 6, 1)] {
            let a = pseudorandom_matrix_with_bits(h, w, bits, &mut seed);
            let b = pseudorandom_matrix_with_bits(h, k, bits, &mut seed);
            verify_precomp(&a, &b);
        }
    }
}

// Checks that `result`, if it is some (X, d), solves AX = dB: d is nonzero, X has the right shape,
// and AX = dB.
fn check_solution(a: &IntegerMatrix, b: &IntegerMatrix, result: &Option<(IntegerMatrix, Integer)>) {
    if let Some((x, den)) = result {
        assert!(x.is_valid());
        assert_eq!(x.height(), a.width());
        assert_eq!(x.width(), b.width());
        assert_ne!(*den, 0u32);
        assert_eq!(a * x, b * den);
    }
}

// Whether the solutions (X, d) and (Y, e) are equal as rational matrices: Xe = Yd.
fn same_rational_solution(x: &(IntegerMatrix, Integer), y: &(IntegerMatrix, Integer)) -> bool {
    &x.0 * &y.1 == &y.0 * &x.1
}

// Checks `can_solve_fflu`, `can_solve_multi_mod_den`, and `can_solve` for AX = dB: their results
// are solutions, they agree on whether there is one, and `can_solve` is one of the others. Returns
// the result of `can_solve`.
fn verify_can_solve(a: &IntegerMatrix, b: &IntegerMatrix) -> Option<(IntegerMatrix, Integer)> {
    let fflu = can_solve_fflu(a, b);
    let multi_mod = can_solve_multi_mod_den(a, b);
    check_solution(a, b, &fflu);
    check_solution(a, b, &multi_mod);
    assert_eq!(fflu.is_some(), multi_mod.is_some());
    if let Some((x, den)) = &multi_mod {
        assert!(*den > 0u32);
        // The denominator is the least common denominator of the solution.
        let g = x
            .elements_ref()
            .iter()
            .fold(den.unsigned_abs_ref().clone(), |g, y| {
                g.gcd(y.unsigned_abs_ref())
            });
        assert_eq!(g, 1u32);
    }
    let result = can_solve(a, b);
    assert_eq!(result, if a.height() <= 15 { fflu } else { multi_mod });
    result
}

// Checks the public `Solve` for AX = dB: the four ways of passing the arguments agree, and the
// result is that of `solve_nonsingular` when A is square and nonsingular, and that of `can_solve`
// otherwise. Returns the result.
fn verify_public_solve(a: &IntegerMatrix, b: &IntegerMatrix) -> Option<(IntegerMatrix, Integer)> {
    let result = a.clone().solve(b.clone());
    assert_eq!(a.clone().solve(b), result);
    assert_eq!(a.solve(b.clone()), result);
    assert_eq!(a.solve(b), result);
    let can = verify_can_solve(a, b);
    if a.height() == a.width()
        && let Some(x) = solve_nonsingular(a, b)
    {
        check_solution(a, b, &Some(x.clone()));
        assert_eq!(result, Some(x.clone()));
        if let Some(y) = &can {
            assert!(same_rational_solution(&x, y));
        }
    } else {
        assert_eq!(result, can);
    }
    result
}

#[test]
fn test_solve() {
    let test = |a, b, out| {
        let a = IntegerMatrix::from_str(a).unwrap();
        let b = IntegerMatrix::from_str(b).unwrap();
        assert_eq!(format_solution(&verify_public_solve(&a, &b)), out);
    };
    test("[]_{0,0}", "[]_{0,2}", "Some(([]_{0,2}, 1))");
    test("[]_{0,3}", "[]_{0,2}", "Some(([0, 0; 0, 0; 0, 0], 1))");
    test("[]_{2,0}", "[0; 0]", "Some(([]_{0,1}, 1))");
    test("[]_{2,0}", "[0; 1]", "None");
    test("[2, 1; 1, 3]", "[]_{2,0}", "Some(([]_{2,0}, 1))");
    test("[0, 0; 0, 0]", "[]_{2,0}", "Some(([]_{2,0}, 1))");
    test("[3]", "[6]", "Some(([6], 3))");
    test("[0]", "[0]", "Some(([0], 1))");
    test("[0]", "[1]", "None");
    test("[1, 2; 3, 4]", "[5; 6]", "Some(([8; -9], -2))");
    test("[2, 4; 1, 2]", "[2; 1]", "Some(([2; 0], 2))");
    test("[2, 4; 1, 2]", "[1; 1]", "None");
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 10]",
        "[1; 1; 1]",
        "Some(([3; -3; 0], -3))",
    );
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 9]",
        "[1; 1; 1]",
        "Some(([3; -3; 0], -3))",
    );
    test("[1, 2, 3; 4, 5, 6; 7, 8, 9]", "[1; 0; 0]", "None");
    test(
        "[1, 2, 3, 4; 5, 6, 7, 8; 2, 6, 4, 8; 3, 1, 1, 2]",
        "[1; 0; 0; 0]",
        "Some(([-12; -60; 12; 42], 72))",
    );
    test(
        "[1, 2, 3, 4; 2, 4, 6, 8; 2, 6, 4, 8; 3, 1, 1, 2]",
        "[1; 2; 0; 0]",
        "Some(([2; 10; -16; 0], -26))",
    );
    test("[1, 2, 3]", "[6]", "Some(([6; 0; 0], 1))");
    test("[0, 2, 3]", "[6]", "Some(([0; 6; 0], 2))");
    test("[1, 2, 3; 2, 4, 6]", "[1; 2]", "Some(([1; 0; 0], 1))");
    test("[1, 2, 3; 2, 4, 6]", "[1; 3]", "None");
    test("[1; 2; 3]", "[2; 4; 6]", "Some(([2], 1))");
    test("[1; 2; 3]", "[2; 4; 7]", "None");
    test("[2, 1; 4, 2; 6, 3]", "[1; 2; 3]", "Some(([1; 0], 2))");
    test("[2, 1; 1, 1; 1, 2]", "[3; 2; 3]", "Some(([1; 1], 1))");
    test("[2, 1; 1, 1; 1, 2]", "[3; 2; 4]", "None");
    test(
        "[1267650600228229401496703205376, 1; 1, 1267650600228229401496703205376]",
        "[1267650600228229401496703205376; 0]",
        "Some(([1606938044258990275541962092341162602522202993782792835301376; -1267650600228229401\
        496703205376], 1606938044258990275541962092341162602522202993782792835301375))",
    );
}

#[test]
fn test_solve_vector() {
    let test = |a, b, out| {
        let a = IntegerMatrix::from_str(a).unwrap();
        let b = IntegerVector::from_str(b).unwrap();
        let result = a.clone().solve(b.clone());
        assert_eq!(a.clone().solve(&b), result);
        assert_eq!((&a).solve(b.clone()), result);
        assert_eq!((&a).solve(&b), result);
        assert_eq!(
            result.as_ref().map_or_else(
                || "None".to_string(),
                |(x, den)| format!("Some(({x}, {den}))")
            ),
            out
        );
    };
    test("[]_{0,2}", "()", "Some(((0, 0), 1))");
    test("[3]", "(6)", "Some(((6), 3))");
    test("[1, 2; 3, 4]", "(5, 6)", "Some(((8, -9), -2))");
    test("[2, 4; 1, 2]", "(1, 1)", "None");
    test("[1, 2, 3; 2, 4, 6]", "(1, 2)", "Some(((1, 0, 0), 1))");
}

#[test]
fn test_can_solve_fflu() {
    let test = |a, b, out| {
        let a = IntegerMatrix::from_str(a).unwrap();
        let b = IntegerMatrix::from_str(b).unwrap();
        let result = can_solve_fflu(&a, &b);
        check_solution(&a, &b, &result);
        assert_eq!(format_solution(&result), out);
    };
    test("[]_{0,3}", "[]_{0,2}", "Some(([0, 0; 0, 0; 0, 0], 1))");
    test("[]_{2,0}", "[0; 0]", "Some(([]_{0,1}, 1))");
    test("[]_{2,0}", "[0; 1]", "None");
    test("[0, 0; 0, 0]", "[]_{2,0}", "Some(([]_{2,0}, 1))");
    test("[0]", "[0]", "Some(([0], 1))");
    test("[0]", "[1]", "None");
    test("[1, 2; 3, 4]", "[5; 6]", "Some(([8; -9], -2))");
    test("[2, 4; 1, 2]", "[2; 1]", "Some(([2; 0], 2))");
    test("[2, 4; 1, 2]", "[1; 1]", "None");
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 9]",
        "[1; 1; 1]",
        "Some(([3; -3; 0], -3))",
    );
    test("[1, 2, 3; 4, 5, 6; 7, 8, 9]", "[1; 0; 0]", "None");
    test("[0, 2, 3]", "[6]", "Some(([0; 6; 0], 2))");
    test("[1, 2, 3; 2, 4, 6]", "[1; 2]", "Some(([1; 0; 0], 1))");
    test("[1, 2, 3; 2, 4, 6]", "[1; 3]", "None");
    test("[2, 1; 4, 2; 6, 3]", "[1; 2; 3]", "Some(([1; 0], 2))");
    test("[2, 1; 1, 1; 1, 2]", "[3; 2; 3]", "Some(([1; 1], 1))");
    test("[2, 1; 1, 1; 1, 2]", "[3; 2; 4]", "None");
    test(
        "[0, 1; 1, 0; 0, 0]",
        "[1, 2; 3, 4; 0, 0]",
        "Some(([-3, -4; -1, -2], -1))",
    );
    // - the forward substitution finds a division that is not exact
    test(
        "[-2, -1, -1; 1, -1, 2; -1, -1, 0; 0, -2, 2]",
        "[-2; 0; -1; 1]",
        "None",
    );
    test(
        "[2, 2, -2; 0, 0, 0; -1, 2, 1; -1, -1, 1]",
        "[1; -1; 2; 2]",
        "None",
    );
}

#[test]
fn test_can_solve() {
    let test = |a, b, out| {
        let a = IntegerMatrix::from_str(a).unwrap();
        let b = IntegerMatrix::from_str(b).unwrap();
        assert_eq!(format_solution(&verify_can_solve(&a, &b)), out);
    };
    test("[]_{0,3}", "[]_{0,2}", "Some(([0, 0; 0, 0; 0, 0], 1))");
    test("[]_{2,0}", "[0; 1]", "None");
    test("[1, 2; 3, 4]", "[5; 6]", "Some(([8; -9], -2))");
    test("[2, 4; 1, 2]", "[2; 1]", "Some(([2; 0], 2))");
    test("[2, 4; 1, 2]", "[1; 1]", "None");
    test("[1, 2, 3; 2, 4, 6]", "[1; 2]", "Some(([1; 0; 0], 1))");
    test("[2, 1; 1, 1; 1, 2]", "[3; 2; 4]", "None");
}

#[test]
fn test_solve_large() {
    let mut seed = 1;
    // Square systems of dimension above 15, solved by Dixon's p-adic lifting when the right-hand
    // side has at most 32 columns and with the multimodular algorithm otherwise, and general
    // systems of height above 15, solved with the multimodular algorithm.
    for (n, k, bits) in [(16, 1, 10), (17, 3, 64), (20, 33, 5), (16, 40, 63)] {
        let a = pseudorandom_matrix_with_bits(n, n, bits, &mut seed);
        let b = pseudorandom_matrix_with_bits(n, k, bits, &mut seed);
        let x = verify_public_solve(&a, &b).unwrap();
        assert!(same_rational_solution(&x, &solve_fflu(&a, &b).unwrap()));
        // A singular matrix of the same dimension, whose last row is the sum of the others.
        let sum = (0..n - 1).fold(
            vec![Integer::ZERO; usize::try_from(n).unwrap()],
            |mut s, i| {
                for (t, y) in s.iter_mut().zip(a.window(i..i + 1, 0..n).into_elements()) {
                    *t += y;
                }
                s
            },
        );
        let last = usize::try_from((n - 1) * n).unwrap();
        let mut xs = a.into_elements();
        xs[last..].clone_from_slice(&sum);
        let a = IntegerMatrix::from_owned_elements(n, n, xs);
        verify_public_solve(&a, &b);
        // A consistent system with that matrix.
        let b = &a * pseudorandom_matrix_with_bits(n, k, bits, &mut seed);
        assert!(verify_public_solve(&a, &b).is_some());
    }
    for (h, w, k, bits) in [(16, 10, 2, 10), (18, 30, 1, 40), (20, 20, 3, 3)] {
        let a = pseudorandom_matrix_with_bits(h, w, bits, &mut seed);
        let b = pseudorandom_matrix_with_bits(h, k, bits, &mut seed);
        verify_public_solve(&a, &b);
        let b = &a * pseudorandom_matrix_with_bits(w, k, bits, &mut seed);
        assert!(verify_public_solve(&a, &b).is_some());
    }
}

#[test]
fn solve_fail() {
    let a = IntegerMatrix::from_str("[1, 2]").unwrap();
    let b = IntegerMatrix::from_str("[1]").unwrap();
    assert_panic!(solve_fflu(&a, &b));
    assert_panic!(solve_cramer(&a, &b));
    let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    let b = IntegerMatrix::from_str("[1; 2; 3]").unwrap();
    assert_panic!(solve_fflu(&a, &b));
    assert_panic!(solve_cramer(&a, &b));
    assert_panic!(a.clone().solve(b.clone()));
    assert_panic!(can_solve_fflu(&a, &b));
    assert_panic!(can_solve(&a, &b));
    let b = IntegerVector::from_str("(1, 2, 3)").unwrap();
    assert_panic!(a.solve(b));
    let a = IntegerMatrix::from_str("[1, 0, 0, 0; 0, 1, 0, 0; 0, 0, 1, 0; 0, 0, 0, 1]").unwrap();
    let b = IntegerMatrix::from_str("[1; 2; 3; 4]").unwrap();
    assert_panic!(solve_cramer(&a, &b));
}

#[test]
fn solve_properties() {
    integer_matrix_pair_gen_var_7().test_properties(|(a, b)| {
        verify_solution(&a, &b);
    });

    integer_matrix_pair_gen_var_8().test_properties(|(a, b)| {
        assert_eq!(solve_cramer(&a, &b), solve_fflu(&a, &b));
    });

    integer_matrix_pair_gen_var_3().test_properties(|(a, b)| {
        verify_precomp(&a, &b);
    });
}

#[test]
fn solve_public_properties() {
    integer_matrix_pair_gen_var_3().test_properties(|(a, b)| {
        verify_public_solve(&a, &b);
        let a = integer_matrix_low_rank(&a, &b);
        verify_public_solve(&a, &b);
        // A consistent system: B = A A^T B is in the column space of A.
        let b = &a * (a.transpose() * &b);
        assert!(verify_public_solve(&a, &b).is_some());
    });

    integer_matrix_integer_vector_pair_gen_var_2().test_properties(|(a, b)| {
        let result = (&a).solve(&b);
        let column = IntegerMatrix::from_owned_elements(a.height(), 1, b.elements.clone());
        assert_eq!(
            result,
            (&a).solve(&column).map(|(x, den)| (
                IntegerVector {
                    elements: x.into_elements()
                },
                den
            ))
        );
    });
}
