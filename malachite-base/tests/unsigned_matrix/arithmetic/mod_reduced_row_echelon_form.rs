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
use malachite_base::num::arithmetic::traits::{
    ModMul, ModRank, ModReducedRowEchelonForm, ModReducedRowEchelonFormAssign,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_10;
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_reduced_row_echelon_form::*;
use malachite_base::unsigned_matrix::UnsignedMatrix;

// Checks that `r` is in reduced row echelon form: its nonzero rows come first, each begins with 1,
// these leading ones move strictly to the right, and every other element of their columns is 0.
// Returns the number of nonzero rows.
fn assert_in_rref<T: PrimitiveUnsigned>(r: &UnsignedMatrix<T>) -> u64 {
    let mut pivots = Vec::new();
    let mut zero_rows_started = false;
    for i in 0..r.height() {
        match r.row_iter(i).position(|&x| x != T::ZERO) {
            None => zero_rows_started = true,
            Some(p) => {
                assert!(!zero_rows_started);
                let p = p as u64;
                if let Some(&q) = pivots.last() {
                    assert!(p > q);
                }
                assert_eq!(*r.get(i, p).unwrap(), T::ONE);
                for k in 0..r.height() {
                    if k != i {
                        assert_eq!(*r.get(k, p).unwrap(), T::ZERO);
                    }
                }
                pivots.push(p);
            }
        }
    }
    pivots.len() as u64
}

fn verify_rref<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, m: T) -> UnsignedMatrix<T> {
    let r = a.mod_reduced_row_echelon_form(m);
    assert!(r.is_valid());
    assert_eq!(a.clone().mod_reduced_row_echelon_form(m), r);
    let mut b = a.clone();
    b.mod_reduced_row_echelon_form_assign(m);
    assert_eq!(b, r);
    assert_eq!(unsigned_matrix_mod_reduced_row_echelon_form_naive(a, m), r);
    assert_eq!(r.height(), a.height());
    assert_eq!(r.width(), a.width());
    let rank = assert_in_rref(&r);
    if m != T::ONE {
        assert_eq!(a.mod_rank(m), rank);
    }
    assert_eq!((&r).mod_reduced_row_echelon_form(m), r);
    r
}

fn test<T: PrimitiveUnsigned>(a: &str, m: T, out: &str)
where
    <UnsignedMatrix<T> as FromStr>::Err: Debug,
{
    let a = UnsignedMatrix::<T>::from_str(a).unwrap();
    assert_eq!(verify_rref(&a, m).to_string(), out);
}

#[test]
fn test_mod_reduced_row_echelon_form() {
    test::<u64>("[]_{0,0}", 5, "[]_{0,0}");
    test::<u64>("[]_{2,0}", 5, "[]_{2,0}");
    test::<u64>("[]_{0,3}", 5, "[]_{0,3}");
    test::<u64>("[0]", 7, "[0]");
    test::<u64>("[3]", 7, "[1]");
    test::<u64>("[0, 3, 6]", 7, "[0, 1, 2]");
    test::<u64>("[0, 1, 5]", 7, "[0, 1, 5]");
    test::<u64>("[1, 2; 2, 4]", 7, "[1, 2; 0, 0]");
    test::<u64>("[2, 1; 4, 3]", 5, "[1, 0; 0, 1]");
    test::<u64>(
        "[1, 2, 3; 2, 4, 6; 1, 0, 1]",
        7,
        "[1, 0, 1; 0, 1, 1; 0, 0, 0]",
    );
    test::<u64>("[0, 0, 1; 0, 2, 3]", 11, "[0, 1, 0; 0, 0, 1]");
    test::<u64>("[1; 2; 3]", 5, "[1; 0; 0]");
    test::<u8>("[200, 100; 50, 25]", 251, "[1, 126; 0, 0]");
    test::<u64>("[0, 0]", 1, "[0, 0]");
    test::<u64>(
        "[18446744073709551556, 5; 7, 18446744073709551000]",
        18446744073709551557,
        "[1, 0; 0, 1]",
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
fn test_mod_reduced_row_echelon_form_large() {
    let mut seed = 0;
    for (h, w, r, m) in LARGE_CASES {
        let a = pseudorandom_matrix_with_rank(h, w, r, m, &mut seed);
        verify_rref(&a, m);
    }
}

#[test]
#[should_panic]
fn mod_reduced_row_echelon_form_fail_1() {
    UnsignedMatrix::<u64>::from_str("[1]")
        .unwrap()
        .mod_reduced_row_echelon_form(0);
}

#[test]
#[should_panic]
fn mod_reduced_row_echelon_form_fail_2() {
    UnsignedMatrix::<u64>::from_str("[5]")
        .unwrap()
        .mod_reduced_row_echelon_form(5);
}

#[test]
#[should_panic]
fn mod_reduced_row_echelon_form_fail_3() {
    UnsignedMatrix::<u64>::from_str("[2, 1; 1, 1]")
        .unwrap()
        .mod_reduced_row_echelon_form(4);
}

#[test]
#[should_panic]
fn mod_reduced_row_echelon_form_fail_4() {
    UnsignedMatrix::<u64>::from_str("[2, 1]")
        .unwrap()
        .mod_reduced_row_echelon_form(4);
}

#[test]
fn mod_reduced_row_echelon_form_properties() {
    unsigned_matrix_unsigned_pair_gen_var_10().test_properties(|(a, m)| {
        let r = verify_rref(&a, m);
        // The reduced row echelon form depends only on the row space, so it is the same for the
        // matrix with its rows reversed.
        let h = a.height();
        let reversed = UnsignedMatrix::from_owned_elements(
            h,
            a.width(),
            (0..h)
                .rev()
                .flat_map(|i| a.row_iter(i).copied().collect::<Vec<_>>())
                .collect(),
        );
        assert_eq!(reversed.mod_reduced_row_echelon_form(m), r);
    });
}
