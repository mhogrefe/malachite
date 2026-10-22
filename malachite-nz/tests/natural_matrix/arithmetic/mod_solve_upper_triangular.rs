// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    CoprimeWith, ModIsReduced, ModMul, ModSolveLowerTriangular, ModSolveUpperTriangular,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_natural_matrix_natural_triple_gen_var_4;
use malachite_nz::test_util::natural_matrix::arithmetic::mod_solve_upper_triangular::*;

// The upper triangular part of `a`, with ones (reduced modulo `m`) on the diagonal if
// `unit_diagonal`.
fn triangular_part(a: &NaturalMatrix, m: &Natural, unit_diagonal: bool) -> NaturalMatrix {
    let n = a.height();
    let mut elements = Vec::new();
    for i in 0..n {
        for j in 0..n {
            elements.push(if i == j && unit_diagonal {
                // Modulo 1, every element is 0.
                if *m == 1u32 {
                    Natural::ZERO
                } else {
                    Natural::ONE
                }
            } else if j >= i {
                a.get(i, j).unwrap().clone()
            } else {
                Natural::ZERO
            });
        }
    }
    NaturalMatrix::from_owned_elements(n, n, elements)
}

// `a` with the order of its rows and the order of its columns reversed.
fn reversed(a: &NaturalMatrix) -> NaturalMatrix {
    let (h, w) = (a.height(), a.width());
    let mut elements = Vec::new();
    for i in (0..h).rev() {
        for j in (0..w).rev() {
            elements.push(a.get(i, j).unwrap().clone());
        }
    }
    NaturalMatrix::from_owned_elements(h, w, elements)
}

fn test_helper(
    a: &NaturalMatrix,
    b: &NaturalMatrix,
    m: &Natural,
    unit_diagonal: bool,
) -> NaturalMatrix {
    let x = a.mod_solve_upper_triangular(b, m, unit_diagonal);
    assert!(x.is_valid());
    for (a_owned, b_owned) in [(true, true), (true, false), (false, true), (false, false)] {
        for m_owned in [true, false] {
            let y = match (a_owned, b_owned, m_owned) {
                (true, true, true) => {
                    a.clone()
                        .mod_solve_upper_triangular(b.clone(), m.clone(), unit_diagonal)
                }
                (true, true, false) => {
                    a.clone()
                        .mod_solve_upper_triangular(b.clone(), m, unit_diagonal)
                }
                (true, false, true) => {
                    a.clone()
                        .mod_solve_upper_triangular(b, m.clone(), unit_diagonal)
                }
                (true, false, false) => a.clone().mod_solve_upper_triangular(b, m, unit_diagonal),
                (false, true, true) => {
                    a.mod_solve_upper_triangular(b.clone(), m.clone(), unit_diagonal)
                }
                (false, true, false) => a.mod_solve_upper_triangular(b.clone(), m, unit_diagonal),
                (false, false, true) => a.mod_solve_upper_triangular(b, m.clone(), unit_diagonal),
                (false, false, false) => a.mod_solve_upper_triangular(b, m, unit_diagonal),
            };
            assert_eq!(y, x);
        }
    }
    assert_eq!(
        natural_matrix_mod_solve_upper_triangular_naive(a, b, m, unit_diagonal),
        x
    );
    x
}

fn test(a: &str, b: &str, m: &str, unit_diagonal: bool, out: &str) {
    let a = NaturalMatrix::from_str(a).unwrap();
    let b = NaturalMatrix::from_str(b).unwrap();
    let m = Natural::from_str(m).unwrap();
    assert_eq!(test_helper(&a, &b, &m, unit_diagonal).to_string(), out);
}

#[test]
fn test_mod_solve_upper_triangular() {
    test("[]_{0,0}", "[]_{0,3}", "7", false, "[]_{0,3}");
    test("[3]", "[1, 2]", "7", false, "[5, 3]");
    test("[3]", "[1, 2]", "7", true, "[1, 2]");
    test("[2, 9; 3, 4]", "[1; 2]", "11", false, "[1; 6]");
    test("[0]", "[0]", "1", false, "[0]");
    test(
        "[2, 5; 3, 7]",
        "[1; 2]",
        "1000000000000000000000000000001",
        false,
        "[357142857142857142857142857143; 857142857142857142857142857144]",
    );
    test(
        "[2, 5; 3, 7]",
        "[1; 2]",
        "1000000000000000000000000000001",
        true,
        "[999999999999999999999999999992; 2]",
    );
    test(
        "[170141183460469231731687303715884105726, 1, 2; 5, \
        170141183460469231731687303715884105725, 3; 7, 11, 13]",
        "[1, 170141183460469231731687303715884105722; 2, 3; 4, 5]",
        "170141183460469231731687303715884105727",
        false,
        "[117790050088017160429629671803304380887, 104702266744904142604115263825159449683; \
        26175566686226035651028815956289862419, 117790050088017160429629671803304380887; \
        130877833431130178255144079781449312098, 78526700058678106953086447868869587259]",
    );
    test(
        "[170141183460469231731687303715884105726, 1, 2; 5, \
        170141183460469231731687303715884105725, 3; 7, 11, 13]",
        "[1, 170141183460469231731687303715884105722; 2, 3; 4, 5]",
        "170141183460469231731687303715884105727",
        true,
        "[3, 170141183460469231731687303715884105724; 170141183460469231731687303715884105717, \
        170141183460469231731687303715884105715; 4, 5]",
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

// A matrix with pseudorandom elements less than `m`, with diagonal elements that are units if it is
// square.
fn pseudorandom_matrix(height: u64, width: u64, m: &Natural, seed: &mut u64) -> NaturalMatrix {
    let mut elements = Vec::new();
    for i in 0..height {
        for j in 0..width {
            let mut x = pseudorandom_natural(m, seed);
            if i == j && height == width {
                while !(&x).coprime_with(m) {
                    x = pseudorandom_natural(m, seed);
                }
            }
            elements.push(x);
        }
    }
    NaturalMatrix::from_owned_elements(height, width, elements)
}

#[test]
fn test_mod_solve_upper_triangular_large() {
    // - recursive, with a modulus of more than one limb
    let mut seed = 0;
    for (n, k, m) in [
        (30, 20, "170141183460469231731687303715884105727"),
        (25, 12, "1000000000000000000000000000001"),
    ] {
        let m = Natural::from_str(m).unwrap();
        let a = pseudorandom_matrix(n, n, &m, &mut seed);
        let b = pseudorandom_matrix(n, k, &m, &mut seed);
        for unit_diagonal in [false, true] {
            let x = test_helper(&a, &b, &m, unit_diagonal);
            assert_eq!(triangular_part(&a, &m, unit_diagonal).mod_mul(&x, &m), b);
        }
    }
}

#[test]
#[should_panic]
fn mod_solve_upper_triangular_fail_1() {
    let a = NaturalMatrix::from_str("[1, 2]").unwrap();
    let b = NaturalMatrix::from_str("[1]").unwrap();
    a.mod_solve_upper_triangular(b, Natural::from(5u32), false);
}

#[test]
#[should_panic]
fn mod_solve_upper_triangular_fail_2() {
    let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    let b = NaturalMatrix::from_str("[1]").unwrap();
    a.mod_solve_upper_triangular(b, Natural::from(5u32), false);
}

#[test]
#[should_panic]
fn mod_solve_upper_triangular_fail_3() {
    let a = NaturalMatrix::from_str("[1]").unwrap();
    let b = NaturalMatrix::from_str("[1]").unwrap();
    a.mod_solve_upper_triangular(b, Natural::ZERO, false);
}

#[test]
#[should_panic]
fn mod_solve_upper_triangular_fail_4() {
    let a = NaturalMatrix::from_str("[5]").unwrap();
    let b = NaturalMatrix::from_str("[1]").unwrap();
    a.mod_solve_upper_triangular(b, Natural::from(5u32), false);
}

#[test]
#[should_panic]
fn mod_solve_upper_triangular_fail_5() {
    let a = NaturalMatrix::from_str("[1]").unwrap();
    let b = NaturalMatrix::from_str("[5]").unwrap();
    a.mod_solve_upper_triangular(b, Natural::from(5u32), false);
}

#[test]
#[should_panic]
fn mod_solve_upper_triangular_fail_6() {
    let a = NaturalMatrix::from_str("[2]").unwrap();
    let b = NaturalMatrix::from_str("[1]").unwrap();
    a.mod_solve_upper_triangular(
        b,
        Natural::from_str("1267650600228229401496703205376").unwrap(),
        false,
    );
}

#[test]
fn mod_solve_upper_triangular_properties() {
    natural_matrix_natural_matrix_natural_triple_gen_var_4().test_properties(|(a, b, m)| {
        for unit_diagonal in [false, true] {
            let x = test_helper(&a, &b, &m, unit_diagonal);
            assert!(x.mod_is_reduced(&m));
            assert_eq!(x.height(), b.height());
            assert_eq!(x.width(), b.width());
            assert_eq!(triangular_part(&a, &m, unit_diagonal).mod_mul(&x, &m), b);
            assert_eq!(
                triangular_part(&a, &m, unit_diagonal).mod_solve_upper_triangular(&b, &m, false),
                x
            );
            assert_eq!(
                reversed(&a).mod_solve_lower_triangular(reversed(&b), &m, unit_diagonal),
                reversed(&x)
            );
        }
    });
}
