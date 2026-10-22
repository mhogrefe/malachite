// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModMul, ModSolve};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::{
    natural_matrix_natural_matrix_natural_triple_gen_var_5,
    natural_matrix_natural_matrix_natural_triple_gen_var_6,
};
use malachite_nz::test_util::natural_matrix::arithmetic::mod_solve::*;

fn verify_solve(a: &NaturalMatrix, b: &NaturalMatrix, m: &Natural) -> Option<NaturalMatrix> {
    let x = a.mod_solve(b, m);
    assert_eq!(a.clone().mod_solve(b.clone(), m.clone()), x);
    assert_eq!(a.clone().mod_solve(b.clone(), m), x);
    assert_eq!(a.clone().mod_solve(b, m.clone()), x);
    assert_eq!(a.clone().mod_solve(b, m), x);
    assert_eq!(a.mod_solve(b.clone(), m.clone()), x);
    assert_eq!(a.mod_solve(b.clone(), m), x);
    assert_eq!(a.mod_solve(b, m.clone()), x);
    assert_eq!(natural_matrix_mod_solve_naive(a, b, m), x);
    if let Some(x) = &x {
        assert!(x.is_valid());
        assert!(x.mod_is_reduced(m));
        assert_eq!(x.height(), a.width());
        assert_eq!(x.width(), b.width());
        assert_eq!(a.mod_mul(x, m), *b);
    }
    // Each column of B, solved as a vector, gives the corresponding column of X.
    for j in 0..b.width() {
        let column = b.column(j);
        let y = a.mod_solve(&column, m);
        assert_eq!(a.clone().mod_solve(column.clone(), m.clone()), y);
        assert_eq!(a.mod_solve(column.clone(), m), y);
        match (&x, y) {
            (Some(x), Some(y)) => assert_eq!(x.column(j), y),
            (Some(_), None) => panic!("a column has no solution, but the whole system does"),
            _ => {}
        }
    }
    x
}

fn test(a: &str, b: &str, m: &str, out: Option<&str>) {
    let a = NaturalMatrix::from_str(a).unwrap();
    let b = NaturalMatrix::from_str(b).unwrap();
    let m = Natural::from_str(m).unwrap();
    let x = verify_solve(&a, &b, &m);
    assert_eq!(x.map(|x| x.to_string()).as_deref(), out);
}

#[test]
fn test_mod_solve() {
    test(
        "[]_{0,0}",
        "[]_{0,2}",
        "170141183460469231731687303715884105727",
        Some("[]_{0,2}"),
    );
    test(
        "[]_{0,3}",
        "[]_{0,2}",
        "170141183460469231731687303715884105727",
        Some("[0, 0; 0, 0; 0, 0]"),
    );
    test(
        "[]_{2,0}",
        "[0; 0]",
        "170141183460469231731687303715884105727",
        Some("[]_{0,1}"),
    );
    test(
        "[]_{2,0}",
        "[0; 1]",
        "170141183460469231731687303715884105727",
        None,
    );
    test(
        "[3]",
        "[1, 2]",
        "170141183460469231731687303715884105727",
        Some("[113427455640312821154458202477256070485, 56713727820156410577229101238628035243]"),
    );
    test(
        "[0]",
        "[1]",
        "170141183460469231731687303715884105727",
        None,
    );
    test(
        "[1, 2; 3, 4]",
        "[1; 1]",
        "170141183460469231731687303715884105727",
        Some("[170141183460469231731687303715884105726; 1]"),
    );
    test(
        "[1, 2; 2, 4]",
        "[1; 2]",
        "36893488147419103363",
        Some("[1; 0]"),
    );
    test("[1, 2; 2, 4]", "[1; 1]", "36893488147419103363", None);
    test(
        "[1, 2, 3; 4, 5, 6]",
        "[1, 0; 0, 1]",
        "170141183460469231731687303715884105727",
        Some(
            "[113427455640312821154458202477256070483, 56713727820156410577229101238628035243; \
            113427455640312821154458202477256070486, 56713727820156410577229101238628035242; 0, 0]",
        ),
    );
    test(
        "[1, 2; 3, 4; 5, 6]",
        "[1; 2; 3]",
        "170141183460469231731687303715884105727",
        Some("[0; 85070591730234615865843651857942052864]"),
    );
    test(
        "[1, 2; 3, 4; 5, 6]",
        "[1; 2; 4]",
        "36893488147419103363",
        None,
    );
    test(
        "[170141183460469231731687303715884105726, 5; 7, 170141183460469231731687303715884105725]",
        "[1; 2]",
        "170141183460469231731687303715884105727",
        Some("[46402140943764335926823810104332028835; 77336901572940559878039683507220048058]"),
    );
    test("[1, 2; 3, 4]", "[1; 1]", "7", Some("[6; 1]"));
    test(
        "[3]",
        "[]_{1,0}",
        "170141183460469231731687303715884105727",
        Some("[]_{1,0}"),
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

// A pseudorandom element less than `m`, made of several outputs of the SplitMix64 generator.
fn pseudorandom_natural(m: &Natural, seed: &mut u64) -> Natural {
    let limbs: Vec<u64> = (0..3).map(|_| splitmix64(seed)).collect();
    Natural::from_owned_limbs_asc(limbs) % m
}

// A matrix with pseudorandom elements less than `m`.
fn pseudorandom_matrix(height: u64, width: u64, m: &Natural, seed: &mut u64) -> NaturalMatrix {
    let elements = (0..height * width)
        .map(|_| pseudorandom_natural(m, seed))
        .collect();
    NaturalMatrix::from_owned_elements(height, width, elements)
}

#[test]
fn test_mod_solve_large() {
    // - square, nonsingular, recursive LU
    // - square, singular
    // - more rows than columns
    // - more columns than rows
    let mut seed = 0;
    for (h, w, r, k, m) in [
        (20, 20, 20, 3, "170141183460469231731687303715884105727"),
        (20, 20, 12, 12, "36893488147419103363"),
        (
            24,
            14,
            9,
            2,
            "1606938044258990275541962092341162602522202993782792835301611",
        ),
        (12, 20, 12, 11, "170141183460469231731687303715884105727"),
    ] {
        let m = Natural::from_str(m).unwrap();
        let a = pseudorandom_matrix(h, r, &m, &mut seed)
            .mod_mul(pseudorandom_matrix(r, w, &m, &mut seed), &m);
        let b = (&a).mod_mul(pseudorandom_matrix(w, k, &m, &mut seed), &m);
        assert!(verify_solve(&a, &b, &m).is_some());
        let b = pseudorandom_matrix(h, k, &m, &mut seed);
        verify_solve(&a, &b, &m);
    }
}

#[test]
#[should_panic]
fn mod_solve_fail_1() {
    let a = NaturalMatrix::from_str("[1]").unwrap();
    let b = NaturalMatrix::from_str("[1]").unwrap();
    a.mod_solve(b, Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_solve_fail_2() {
    let a = NaturalMatrix::from_str("[1; 2]").unwrap();
    let b = NaturalMatrix::from_str("[1]").unwrap();
    a.mod_solve(b, Natural::from(5u32));
}

#[test]
#[should_panic]
fn mod_solve_fail_3() {
    let a = NaturalMatrix::from_str("[5]").unwrap();
    let b = NaturalMatrix::from_str("[1]").unwrap();
    a.mod_solve(b, Natural::from(5u32));
}

#[test]
#[should_panic]
fn mod_solve_fail_4() {
    let a = NaturalMatrix::from_str("[1]").unwrap();
    let b = NaturalMatrix::from_str("[5]").unwrap();
    a.mod_solve(b, Natural::from(5u32));
}

#[test]
#[should_panic]
fn mod_solve_fail_5() {
    let a = NaturalMatrix::from_str("[2, 1; 1, 1]").unwrap();
    let b = NaturalMatrix::from_str("[1; 1]").unwrap();
    a.mod_solve(
        b,
        Natural::from_str("1267650600228229401496703205376").unwrap(),
    );
}

#[test]
fn mod_solve_properties() {
    natural_matrix_natural_matrix_natural_triple_gen_var_5().test_properties(|(a, b, m)| {
        verify_solve(&a, &b, &m);
    });

    natural_matrix_natural_matrix_natural_triple_gen_var_6().test_properties(|(a, b, m)| {
        assert!(verify_solve(&a, &b, &m).is_some());
    });
}
