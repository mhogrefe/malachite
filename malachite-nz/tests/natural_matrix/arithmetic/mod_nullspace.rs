// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModLeftNullspace, ModMul, ModNullspace, ModRank};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_natural_pair_gen_var_5;
use malachite_nz::test_util::natural_matrix::arithmetic::mod_nullspace::*;

fn verify_nullspace(a: &NaturalMatrix, m: &Natural) -> NaturalMatrix {
    let n = a.mod_nullspace(m);
    assert!(n.is_valid());
    assert_eq!(a.mod_nullspace(m.clone()), n);
    assert_eq!(a.clone().mod_nullspace(m), n);
    assert_eq!(a.clone().mod_nullspace(m.clone()), n);
    assert_eq!(natural_matrix_mod_nullspace_naive(a, m), n);
    assert_eq!(n.height(), a.width());
    assert_eq!(a.mod_mul(&n, m), NaturalMatrix::zero(a.height(), n.width()));
    if *m != 1u32 {
        assert_eq!(n.width(), a.width() - a.mod_rank(m));
        assert_eq!((&n).mod_rank(m), n.width());
    }
    n
}

fn verify_left_nullspace(a: &NaturalMatrix, m: &Natural) -> NaturalMatrix {
    let l = a.mod_left_nullspace(m);
    assert!(l.is_valid());
    assert_eq!(a.mod_left_nullspace(m.clone()), l);
    assert_eq!(a.clone().mod_left_nullspace(m), l);
    assert_eq!(a.clone().mod_left_nullspace(m.clone()), l);
    assert_eq!(natural_matrix_mod_left_nullspace_naive(a, m), l);
    assert_eq!(l.width(), a.height());
    assert_eq!(
        (&l).mod_mul(a, m),
        NaturalMatrix::zero(l.height(), a.width())
    );
    assert_eq!(a.transpose().mod_nullspace(m).transpose(), l);
    if *m != 1u32 {
        assert_eq!(l.height(), a.height() - a.mod_rank(m));
        assert_eq!((&l).mod_rank(m), l.height());
    }
    l
}

fn test(a: &str, m: &str, out: &str, out_left: &str) {
    let a = NaturalMatrix::from_str(a).unwrap();
    let m = Natural::from_str(m).unwrap();
    assert_eq!(verify_nullspace(&a, &m).to_string(), out);
    assert_eq!(verify_left_nullspace(&a, &m).to_string(), out_left);
}

#[test]
fn test_mod_nullspace() {
    test(
        "[]_{0,0}",
        "170141183460469231731687303715884105727",
        "[]_{0,0}",
        "[]_{0,0}",
    );
    test(
        "[]_{2,0}",
        "170141183460469231731687303715884105727",
        "[]_{0,0}",
        "[1, 0; 0, 1]",
    );
    test(
        "[]_{0,3}",
        "170141183460469231731687303715884105727",
        "[1, 0, 0; 0, 1, 0; 0, 0, 1]",
        "[]_{0,0}",
    );
    test(
        "[0]",
        "170141183460469231731687303715884105727",
        "[1]",
        "[1]",
    );
    test(
        "[3]",
        "170141183460469231731687303715884105727",
        "[]_{1,0}",
        "[]_{0,1}",
    );
    test(
        "[0, 3, 6]",
        "170141183460469231731687303715884105727",
        "[1, 0; 0, 170141183460469231731687303715884105725; 0, 1]",
        "[]_{0,1}",
    );
    test(
        "[0, 1, 5]",
        "170141183460469231731687303715884105727",
        "[1, 0; 0, 170141183460469231731687303715884105722; 0, 1]",
        "[]_{0,1}",
    );
    test(
        "[1, 2; 2, 4]",
        "170141183460469231731687303715884105727",
        "[170141183460469231731687303715884105725; 1]",
        "[170141183460469231731687303715884105725, 1]",
    );
    test(
        "[2, 1; 4, 3]",
        "36893488147419103363",
        "[]_{2,0}",
        "[]_{0,2}",
    );
    test(
        "[1, 2, 3; 2, 4, 6; 1, 0, 1]",
        "170141183460469231731687303715884105727",
        "[170141183460469231731687303715884105726; 170141183460469231731687303715884105726; 1]",
        "[170141183460469231731687303715884105725, 1, 0]",
    );
    test(
        "[0, 0, 1; 0, 2, 3]",
        "36893488147419103363",
        "[1; 0; 0]",
        "[]_{0,2}",
    );
    test(
        "[1; 2; 3]",
        "170141183460469231731687303715884105727",
        "[]_{1,0}",
        "[170141183460469231731687303715884105725, 1, 0; 170141183460469231731687303715884105724, \
        0, 1]",
    );
    test(
        "[170141183460469231731687303715884105726, 5; 7, 170141183460469231731687303715884105725]",
        "170141183460469231731687303715884105727",
        "[]_{2,0}",
        "[]_{0,2}",
    );
    test(
        "[1, 2, 3, 4; 2, 4, 6, 8; 3, 5, 7, 9]",
        "36893488147419103363",
        "[1, 2; 36893488147419103361, 36893488147419103360; 1, 0; 0, 1]",
        "[36893488147419103361, 1, 0]",
    );
    test("[1, 2; 2, 4]", "7", "[5; 1]", "[5, 1]");
    test("[0, 3, 6]", "7", "[1, 0; 0, 5; 0, 1]", "[]_{0,1}");
    test("[0, 0]", "1", "[0, 0; 0, 0]", "[0]");
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
fn test_mod_nullspace_large() {
    let mut seed = 0;
    for (h, w, r, m) in LARGE_CASES {
        let m = Natural::from_str(m).unwrap();
        let a = pseudorandom_matrix_with_rank(h, w, r, &m, &mut seed);
        verify_nullspace(&a, &m);
        verify_left_nullspace(&a, &m);
    }
}

#[test]
#[should_panic]
fn mod_nullspace_fail_1() {
    NaturalMatrix::from_str("[1]")
        .unwrap()
        .mod_nullspace(Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_nullspace_fail_2() {
    NaturalMatrix::from_str("[5]")
        .unwrap()
        .mod_nullspace(Natural::from(5u32));
}

#[test]
#[should_panic]
fn mod_nullspace_fail_3() {
    NaturalMatrix::from_str("[2, 1; 1, 1]")
        .unwrap()
        .mod_nullspace(Natural::from_str("1267650600228229401496703205376").unwrap());
}

#[test]
#[should_panic]
fn mod_left_nullspace_fail_1() {
    NaturalMatrix::from_str("[1]")
        .unwrap()
        .mod_left_nullspace(Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_left_nullspace_fail_2() {
    NaturalMatrix::from_str("[5]")
        .unwrap()
        .mod_left_nullspace(Natural::from(5u32));
}

#[test]
#[should_panic]
fn mod_left_nullspace_fail_3() {
    NaturalMatrix::from_str("[2, 1; 1, 1]")
        .unwrap()
        .mod_left_nullspace(Natural::from_str("1267650600228229401496703205376").unwrap());
}

#[test]
fn mod_nullspace_properties() {
    natural_matrix_natural_pair_gen_var_5().test_properties(|(a, m)| {
        verify_nullspace(&a, &m);
        verify_left_nullspace(&a, &m);
    });
}
