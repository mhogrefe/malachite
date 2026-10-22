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
use malachite_base::num::arithmetic::traits::{ModLeftNullspace, ModMul, ModNullspace, ModRank};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_10;
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_nullspace::*;
use malachite_base::unsigned_matrix::UnsignedMatrix;

fn verify_nullspace<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, m: T) -> UnsignedMatrix<T> {
    let n = a.mod_nullspace(m);
    assert!(n.is_valid());
    assert_eq!(a.clone().mod_nullspace(m), n);
    assert_eq!(unsigned_matrix_mod_nullspace_naive(a, m), n);
    assert_eq!(n.height(), a.width());
    assert_eq!(
        a.mod_mul(&n, m),
        UnsignedMatrix::zero(a.height(), n.width())
    );
    if m != T::ONE {
        assert_eq!(n.width(), a.width() - a.mod_rank(m));
        assert_eq!((&n).mod_rank(m), n.width());
    }
    n
}

fn verify_left_nullspace<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, m: T) -> UnsignedMatrix<T> {
    let l = a.mod_left_nullspace(m);
    assert!(l.is_valid());
    assert_eq!(a.clone().mod_left_nullspace(m), l);
    assert_eq!(unsigned_matrix_mod_left_nullspace_naive(a, m), l);
    assert_eq!(l.width(), a.height());
    assert_eq!(
        (&l).mod_mul(a, m),
        UnsignedMatrix::zero(l.height(), a.width())
    );
    assert_eq!(a.transpose().mod_nullspace(m).transpose(), l);
    if m != T::ONE {
        assert_eq!(l.height(), a.height() - a.mod_rank(m));
        assert_eq!((&l).mod_rank(m), l.height());
    }
    l
}

fn test<T: PrimitiveUnsigned>(a: &str, m: T, out: &str, out_left: &str)
where
    <UnsignedMatrix<T> as FromStr>::Err: Debug,
{
    let a = UnsignedMatrix::<T>::from_str(a).unwrap();
    assert_eq!(verify_nullspace(&a, m).to_string(), out);
    assert_eq!(verify_left_nullspace(&a, m).to_string(), out_left);
}

#[test]
fn test_mod_nullspace() {
    test::<u64>("[]_{0,0}", 5, "[]_{0,0}", "[]_{0,0}");
    test::<u64>("[]_{2,0}", 5, "[]_{0,0}", "[1, 0; 0, 1]");
    test::<u64>("[]_{0,3}", 5, "[1, 0, 0; 0, 1, 0; 0, 0, 1]", "[]_{0,0}");
    test::<u64>("[0]", 7, "[1]", "[1]");
    test::<u64>("[3]", 7, "[]_{1,0}", "[]_{0,1}");
    test::<u64>("[0, 3, 6]", 7, "[1, 0; 0, 5; 0, 1]", "[]_{0,1}");
    test::<u64>("[0, 1, 5]", 7, "[1, 0; 0, 2; 0, 1]", "[]_{0,1}");
    test::<u64>("[1, 2; 2, 4]", 7, "[5; 1]", "[5, 1]");
    test::<u64>("[2, 1; 4, 3]", 5, "[]_{2,0}", "[]_{0,2}");
    test::<u64>("[1, 2, 3; 2, 4, 6; 1, 0, 1]", 7, "[6; 6; 1]", "[5, 1, 0]");
    test::<u64>("[0, 0, 1; 0, 2, 3]", 11, "[1; 0; 0]", "[]_{0,2}");
    test::<u64>("[1; 2; 3]", 5, "[]_{1,0}", "[3, 1, 0; 2, 0, 1]");
    test::<u8>("[200, 100; 50, 25]", 251, "[125; 1]", "[188, 1]");
    test::<u64>("[0, 0]", 1, "[0, 0; 0, 0]", "[0]");
    test::<u64>(
        "[18446744073709551556, 5; 7, 18446744073709551000]",
        18446744073709551557,
        "[]_{2,0}",
        "[]_{0,2}",
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

const LARGE_CASES: [(u64, u64, u64, u64); 5] = [
    (70, 70, 70, 2),
    (70, 70, 30, 2),
    (100, 80, 50, 1099511627791),
    (40, 90, 25, 18446744073709551557),
    (130, 130, 120, 2305843009213693951),
];

#[test]
fn test_mod_nullspace_large() {
    let mut seed = 0;
    for (h, w, r, m) in LARGE_CASES {
        let a = pseudorandom_matrix_with_rank(h, w, r, m, &mut seed);
        verify_nullspace(&a, m);
        verify_left_nullspace(&a, m);
    }
}

#[test]
#[should_panic]
fn mod_nullspace_fail_1() {
    UnsignedMatrix::<u64>::from_str("[1]")
        .unwrap()
        .mod_nullspace(0);
}

#[test]
#[should_panic]
fn mod_nullspace_fail_2() {
    UnsignedMatrix::<u64>::from_str("[5]")
        .unwrap()
        .mod_nullspace(5);
}

#[test]
#[should_panic]
fn mod_left_nullspace_fail_1() {
    UnsignedMatrix::<u64>::from_str("[1]")
        .unwrap()
        .mod_left_nullspace(0);
}

#[test]
#[should_panic]
fn mod_left_nullspace_fail_2() {
    UnsignedMatrix::<u64>::from_str("[5]")
        .unwrap()
        .mod_left_nullspace(5);
}

#[test]
fn mod_nullspace_properties() {
    unsigned_matrix_unsigned_pair_gen_var_10().test_properties(|(a, m)| {
        verify_nullspace(&a, m);
        verify_left_nullspace(&a, m);
    });
}
