// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModLuDecomposition, ModMul, ModRank};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_natural_pair_gen_var_5;
use malachite_nz::test_util::natural_matrix::arithmetic::mod_rank::*;

fn verify_rank(a: &NaturalMatrix, m: &Natural) -> u64 {
    let r = a.mod_rank(m);
    assert_eq!(a.clone().mod_rank(m), r);
    assert_eq!(a.clone().mod_rank(m.clone()), r);
    assert_eq!(a.mod_rank(m.clone()), r);
    assert_eq!(natural_matrix_mod_rank_naive(a, m), r);
    assert!(r <= a.height().min(a.width()));
    assert_eq!(a.transpose().mod_rank(m), r);
    assert_eq!(a.mod_lu_decomposition(m).1.width(), r);
    r
}

fn test(a: &str, m: &str, out: u64) {
    let a = NaturalMatrix::from_str(a).unwrap();
    let m = Natural::from_str(m).unwrap();
    assert_eq!(verify_rank(&a, &m), out);
}

#[test]
fn test_mod_rank() {
    test("[]_{0,0}", "170141183460469231731687303715884105727", 0);
    test("[]_{2,0}", "170141183460469231731687303715884105727", 0);
    test("[]_{0,3}", "170141183460469231731687303715884105727", 0);
    test("[3]", "170141183460469231731687303715884105727", 1);
    test("[0]", "170141183460469231731687303715884105727", 0);
    test("[0, 1; 1, 0]", "170141183460469231731687303715884105727", 2);
    test("[2, 1; 4, 3]", "36893488147419103363", 2);
    test(
        "[1, 2, 3; 2, 4, 6; 1, 0, 1]",
        "170141183460469231731687303715884105727",
        2,
    );
    test("[0, 0, 1; 0, 2, 3]", "36893488147419103363", 2);
    test("[1; 2; 3]", "170141183460469231731687303715884105727", 1);
    test(
        "[170141183460469231731687303715884105726, 5; 7, 170141183460469231731687303715884105725]",
        "170141183460469231731687303715884105727",
        2,
    );
    test("[3, 4]", "7", 1);
    test("[1, 2; 2, 4]", "11", 1);
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

#[test]
fn test_mod_rank_large() {
    let mut seed = 0;
    for (h, w, r, m) in [
        (20, 20, 6, "170141183460469231731687303715884105727"),
        (25, 18, 10, "36893488147419103363"),
    ] {
        let m = Natural::from_str(m).unwrap();
        let a = pseudorandom_matrix_with_rank(h, w, r, &m, &mut seed);
        assert!(verify_rank(&a, &m) <= r);
    }
}

#[test]
#[should_panic]
fn mod_rank_fail_1() {
    NaturalMatrix::from_str("[1]")
        .unwrap()
        .mod_rank(Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_rank_fail_2() {
    NaturalMatrix::from_str("[5]")
        .unwrap()
        .mod_rank(Natural::from(5u32));
}

#[test]
#[should_panic]
fn mod_rank_fail_3() {
    NaturalMatrix::from_str("[2, 1; 1, 1]")
        .unwrap()
        .mod_rank(Natural::from_str("1267650600228229401496703205376").unwrap());
}

#[test]
fn mod_rank_properties() {
    natural_matrix_natural_pair_gen_var_5().test_properties(|(a, m)| {
        let r = verify_rank(&a, &m);
        let at = a.transpose();
        assert!((&a).mod_mul(&at, &m).mod_rank(&m) <= r);
    });
}
