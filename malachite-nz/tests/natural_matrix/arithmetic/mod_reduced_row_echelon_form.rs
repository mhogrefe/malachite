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
    ModMul, ModRank, ModReducedRowEchelonForm, ModReducedRowEchelonFormAssign,
};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_natural_pair_gen_var_5;
use malachite_nz::test_util::natural_matrix::arithmetic::mod_reduced_row_echelon_form::*;

// Checks that `r` is in reduced row echelon form: its nonzero rows come first, each begins with 1,
// these leading ones move strictly to the right, and every other element of their columns is 0.
// Returns the number of nonzero rows.
fn assert_in_rref(r: &NaturalMatrix) -> u64 {
    let mut pivots: Vec<u64> = Vec::new();
    let mut zero_rows_started = false;
    for i in 0..r.height() {
        match r.row_iter(i).position(|x| *x != 0u32) {
            None => zero_rows_started = true,
            Some(p) => {
                assert!(!zero_rows_started);
                let p = u64::try_from(p).unwrap();
                if let Some(&q) = pivots.last() {
                    assert!(p > q);
                }
                assert_eq!(*r.get(i, p).unwrap(), 1u32);
                for k in 0..r.height() {
                    if k != i {
                        assert_eq!(*r.get(k, p).unwrap(), 0u32);
                    }
                }
                pivots.push(p);
            }
        }
    }
    u64::try_from(pivots.len()).unwrap()
}

fn verify_rref(a: &NaturalMatrix, m: &Natural) -> NaturalMatrix {
    let r = a.mod_reduced_row_echelon_form(m);
    assert!(r.is_valid());
    assert_eq!(a.mod_reduced_row_echelon_form(m.clone()), r);
    assert_eq!(a.clone().mod_reduced_row_echelon_form(m), r);
    assert_eq!(a.clone().mod_reduced_row_echelon_form(m.clone()), r);
    let mut b = a.clone();
    b.mod_reduced_row_echelon_form_assign(m);
    assert_eq!(b, r);
    let mut b = a.clone();
    b.mod_reduced_row_echelon_form_assign(m.clone());
    assert_eq!(b, r);
    assert_eq!(natural_matrix_mod_reduced_row_echelon_form_naive(a, m), r);
    assert_eq!(r.height(), a.height());
    assert_eq!(r.width(), a.width());
    let rank = assert_in_rref(&r);
    if *m != 1u32 {
        assert_eq!(a.mod_rank(m), rank);
    }
    assert_eq!((&r).mod_reduced_row_echelon_form(m), r);
    r
}

fn test(a: &str, m: &str, out: &str) {
    let a = NaturalMatrix::from_str(a).unwrap();
    let m = Natural::from_str(m).unwrap();
    assert_eq!(verify_rref(&a, &m).to_string(), out);
}

#[test]
fn test_mod_reduced_row_echelon_form() {
    test(
        "[]_{0,0}",
        "170141183460469231731687303715884105727",
        "[]_{0,0}",
    );
    test(
        "[]_{2,0}",
        "170141183460469231731687303715884105727",
        "[]_{2,0}",
    );
    test(
        "[]_{0,3}",
        "170141183460469231731687303715884105727",
        "[]_{0,3}",
    );
    test("[0]", "170141183460469231731687303715884105727", "[0]");
    test("[3]", "170141183460469231731687303715884105727", "[1]");
    test(
        "[0, 3, 6]",
        "170141183460469231731687303715884105727",
        "[0, 1, 2]",
    );
    test(
        "[0, 1, 5]",
        "170141183460469231731687303715884105727",
        "[0, 1, 5]",
    );
    test(
        "[1, 2; 2, 4]",
        "170141183460469231731687303715884105727",
        "[1, 2; 0, 0]",
    );
    test("[2, 1; 4, 3]", "36893488147419103363", "[1, 0; 0, 1]");
    test(
        "[1, 2, 3; 2, 4, 6; 1, 0, 1]",
        "170141183460469231731687303715884105727",
        "[1, 0, 1; 0, 1, 1; 0, 0, 0]",
    );
    test(
        "[0, 0, 1; 0, 2, 3]",
        "36893488147419103363",
        "[0, 1, 0; 0, 0, 1]",
    );
    test(
        "[1; 2; 3]",
        "170141183460469231731687303715884105727",
        "[1; 0; 0]",
    );
    test(
        "[170141183460469231731687303715884105726, 5; 7, 170141183460469231731687303715884105725]",
        "170141183460469231731687303715884105727",
        "[1, 0; 0, 1]",
    );
    test(
        "[1, 2, 3, 4; 2, 4, 6, 8; 3, 5, 7, 9]",
        "36893488147419103363",
        "[1, 0, 36893488147419103362, 36893488147419103361; 0, 1, 2, 3; 0, 0, 0, 0]",
    );
    test("[1, 2; 2, 4]", "7", "[1, 2; 0, 0]");
    test("[0, 3, 6]", "7", "[0, 1, 2]");
    test("[0, 0]", "1", "[0, 0]");
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
    (0..3).fold(Natural::ZERO, |x, _| {
        (x << 64u32) + Natural::from(splitmix64(seed))
    }) % m
}

// A matrix with pseudorandom elements less than `m`.
fn pseudorandom_matrix(height: u64, width: u64, m: &Natural, seed: &mut u64) -> NaturalMatrix {
    let elements = (0..height * width)
        .map(|_| pseudorandom_natural(m, seed))
        .collect();
    NaturalMatrix::from_owned_elements(height, width, elements)
}

// A pseudorandom `height` by `width` matrix modulo `m` with rank at most `rank`: a product of
// `height` by `rank` and `rank` by `width` matrices.
fn pseudorandom_matrix_with_rank(
    height: u64,
    width: u64,
    rank: u64,
    m: &Natural,
    seed: &mut u64,
) -> NaturalMatrix {
    let b = pseudorandom_matrix(height, rank, m, seed);
    let c = pseudorandom_matrix(rank, width, m, seed);
    b.mod_mul(c, m)
}

// Large enough for the recursive LU decomposition and triangular solving.
const LARGE_CASES: [(u64, u64, u64, &str); 5] = [
    (30, 40, 20, "170141183460469231731687303715884105727"),
    (25, 25, 25, "36893488147419103363"),
    (40, 30, 12, "170141183460469231731687303715884105727"),
    (12, 50, 12, "36893488147419103363"),
    (35, 35, 34, "18446744073709551629"),
];

#[test]
fn test_mod_reduced_row_echelon_form_large() {
    let mut seed = 0;
    for (h, w, r, m) in LARGE_CASES {
        let m = Natural::from_str(m).unwrap();
        let a = pseudorandom_matrix_with_rank(h, w, r, &m, &mut seed);
        verify_rref(&a, &m);
    }
}

#[test]
#[should_panic]
fn mod_reduced_row_echelon_form_fail_1() {
    NaturalMatrix::from_str("[1]")
        .unwrap()
        .mod_reduced_row_echelon_form(Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_reduced_row_echelon_form_fail_2() {
    NaturalMatrix::from_str("[5]")
        .unwrap()
        .mod_reduced_row_echelon_form(Natural::from(5u32));
}

#[test]
#[should_panic]
fn mod_reduced_row_echelon_form_fail_3() {
    NaturalMatrix::from_str("[2, 1; 1, 1]")
        .unwrap()
        .mod_reduced_row_echelon_form(
            Natural::from_str("1267650600228229401496703205376").unwrap(),
        );
}

#[test]
#[should_panic]
fn mod_reduced_row_echelon_form_fail_4() {
    let mut a = NaturalMatrix::from_str("[2, 1; 1, 1]").unwrap();
    a.mod_reduced_row_echelon_form_assign(
        Natural::from_str("1267650600228229401496703205376").unwrap(),
    );
}

#[test]
fn mod_reduced_row_echelon_form_properties() {
    natural_matrix_natural_pair_gen_var_5().test_properties(|(a, m)| {
        let r = verify_rref(&a, &m);
        // The reduced row echelon form depends only on the row space, so it is the same for the
        // matrix with its rows reversed.
        let h = a.height();
        let reversed = NaturalMatrix::from_owned_elements(
            h,
            a.width(),
            (0..h)
                .rev()
                .flat_map(|i| a.row_iter(i).cloned().collect::<Vec<_>>())
                .collect(),
        );
        assert_eq!(reversed.mod_reduced_row_echelon_form(&m), r);
    });
}
