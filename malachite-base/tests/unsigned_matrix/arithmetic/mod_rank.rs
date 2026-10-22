// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::fmt::Debug;
use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModLuDecomposition, ModMul, ModRank};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_10;
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_rank::*;
use malachite_base::unsigned_matrix::UnsignedMatrix;

fn test<T: PrimitiveUnsigned>(a: &str, m: T, out: u64)
where
    <UnsignedMatrix<T> as FromStr>::Err: Debug,
{
    let a = UnsignedMatrix::<T>::from_str(a).unwrap();
    let r = (&a).mod_rank(m);
    assert_eq!(r, out);
    assert_eq!(a.clone().mod_rank(m), r);
    assert_eq!(unsigned_matrix_mod_rank_naive(&a, m), r);
}

#[test]
fn test_mod_rank() {
    test::<u64>("[]_{0,0}", 5, 0);
    test::<u64>("[]_{2,0}", 5, 0);
    test::<u64>("[]_{0,3}", 5, 0);
    test::<u64>("[3]", 7, 1);
    test::<u64>("[0]", 7, 0);
    test::<u64>("[0, 1; 1, 0]", 5, 2);
    test::<u64>("[2, 1; 4, 3]", 5, 2);
    test::<u64>("[1, 2, 3; 2, 4, 6; 1, 0, 1]", 7, 2);
    test::<u64>("[0, 0, 1; 0, 2, 3]", 11, 2);
    test::<u64>("[1; 2; 3]", 5, 1);
    test::<u8>("[200, 100; 50, 25]", 251, 1);
    test::<u64>("[0, 0]", 1, 0);
    test::<u64>(
        "[18446744073709551556, 5; 7, 18446744073709551000]",
        18446744073709551557,
        2,
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

// A matrix with pseudorandom elements less than `m`, from the SplitMix64 generator.
fn pseudorandom_matrix(height: u64, width: u64, m: u64, seed: &mut u64) -> UnsignedMatrix<u64> {
    let mut elements = Vec::new();
    for _ in 0..height * width {
        let x = splitmix64(seed);
        elements.push(x % m);
    }
    UnsignedMatrix::from_owned_elements(height, width, elements)
}

// A pseudorandom `height` by `width` matrix modulo `m` with rank at most `rank`: a product of
// `height` by `rank` and `rank` by `width` matrices.
fn pseudorandom_matrix_with_rank(
    height: u64,
    width: u64,
    rank: u64,
    m: u64,
    seed: &mut u64,
) -> UnsignedMatrix<u64> {
    let b = pseudorandom_matrix(height, rank, m, seed);
    let c = pseudorandom_matrix(rank, width, m, seed);
    b.mod_mul(c, m)
}

#[test]
fn test_mod_rank_large() {
    let mut seed = 0;
    for (h, w, r, m) in
        [(70, 70, 30, 2), (100, 80, 50, 1099511627791), (130, 130, 40, 18446744073709551557)]
    {
        let a = pseudorandom_matrix_with_rank(h, w, r, m, &mut seed);
        let rank = (&a).mod_rank(m);
        assert_eq!(rank, unsigned_matrix_mod_rank_naive(&a, m));
        assert!(rank <= r);
    }
}

#[test]
#[should_panic]
fn mod_rank_fail_1() {
    UnsignedMatrix::<u64>::from_str("[1]").unwrap().mod_rank(0);
}

#[test]
#[should_panic]
fn mod_rank_fail_2() {
    UnsignedMatrix::<u64>::from_str("[5]").unwrap().mod_rank(5);
}

#[test]
#[should_panic]
fn mod_rank_fail_3() {
    UnsignedMatrix::<u64>::from_str("[2, 1; 1, 1]")
        .unwrap()
        .mod_rank(4);
}

#[test]
fn mod_rank_properties() {
    unsigned_matrix_unsigned_pair_gen_var_10().test_properties(|(a, m)| {
        let r = (&a).mod_rank(m);
        assert_eq!(a.clone().mod_rank(m), r);
        assert_eq!(unsigned_matrix_mod_rank_naive(&a, m), r);
        assert!(r <= a.height().min(a.width()));
        assert_eq!(a.transpose().mod_rank(m), r);
        assert_eq!((&a).mod_lu_decomposition(m).1.width(), r);
        let at = a.transpose();
        assert_eq!((&a).mod_mul(&at, m).mod_rank(m) <= r, true);
    });
}
