// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{Determinant, DeterminantSign, Parity, Sign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::arithmetic::determinant_sign::*;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    integer_matrix_gen_var_1, integer_matrix_pair_gen_var_7,
};
use malachite_nz::test_util::integer_matrix::arithmetic::solve::integer_matrix_low_rank;
use std::panic::catch_unwind;

fn verify_determinant_sign(a: &IntegerMatrix) -> Ordering {
    let sign = a.clone().determinant_sign();
    assert_eq!(a.determinant_sign(), sign);
    assert_eq!(a.determinant().sign(), sign);
    assert_eq!(a.transpose().determinant_sign(), sign);
    if a.height() != 0 {
        let preconditioned = determinant_sign_preconditioned(a);
        if let Some(s) = preconditioned {
            assert_eq!(s, sign);
        }
        if sign == Equal {
            assert_eq!(preconditioned, None);
        }
    }
    sign
}

#[test]
fn test_determinant_sign() {
    let test = |s, out| {
        let a = IntegerMatrix::from_str(s).unwrap();
        assert_eq!(verify_determinant_sign(&a), out);
    };
    // - dimension 0
    test("[]_{0,0}", Greater);
    // - dimension 1
    test("[0]", Equal);
    test("[-7]", Less);
    // - dimension 2
    test("[1, 2; 3, 4]", Less);
    test("[0, 1; 1, 0]", Less);
    test("[2, 4; 1, 2]", Equal);
    test(
        "[1267650600228229401496703205376, 1267650600228229401496703205377; \
        1267650600228229401496703205375, 1267650600228229401496703205376]",
        Greater,
    );
    // - a zero row
    test("[1, 2, 3; 0, 0, 0; 7, 8, 9]", Equal);
    // - a zero column
    test("[1, 0, 3; 4, 0, 6; 7, 0, 9]", Equal);
    // - triangular, with a zero on the diagonal
    test("[1, 2, 3; 0, 0, 6; 0, 0, 9]", Equal);
    // - triangular, with an odd number of negative diagonal elements
    test("[1, 0, 0; 4, -5, 0; 7, 8, 9]", Less);
    // - triangular, with an even number of negative diagonal elements
    test("[-1, 2, 3; 0, -5, 6; 0, 0, 9]", Greater);
    // - elements small enough for the dimension: the determinant is computed
    test("[1, 2, 3; 4, 5, 6; 7, 8, 10]", Less);
    test("[1, 2, 3; 4, 5, 6; 7, 8, 9]", Equal);
    test("[1, -2, 3; 4, 5, -6; -7, 8, 10]", Greater);
    test("[1, 2, 3, 4; 5, 6, 7, 8; 2, 6, 4, 8; 3, 1, 1, 2]", Greater);
    test(
        "[1267650600228229401496703205376, 3, 5; 7, 1267650600228229401496703205376, 11; \
        13, 17, -1267650600228229401496703205376]",
        Less,
    );
    test(
        "[0, 2, 0, 0, 0; 1, 0, 0, 0, 0; 0, 0, 3, 0, 0; 0, 0, 0, 0, 5; 0, 0, 0, 4, 0]",
        Greater,
    );
    test(
        "[2, 1, 1, 1, 1; 1, 2, 1, 1, 1; 1, 1, 2, 1, 1; 1, 1, 1, 2, 1; 1, 1, 1, 1, -2]",
        Less,
    );
    test(
        "[1, 2, 3, 4, 5; 6, 7, 8, 9, 10; 11, 12, 13, 14, 15; 16, 17, 18, 19, 20; 1, 1, 1, 1, 2]",
        Equal,
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

// A pseudorandom `Integer` of at most `bits` bits, of either sign.
fn pseudorandom_integer(bits: u64, seed: &mut u64) -> Integer {
    let mut x = Natural::ZERO;
    for _ in 0..bits.div_ceil(64) {
        x = (x << 64u32) + Natural::from(splitmix64(seed));
    }
    let x = Integer::from(x >> (bits.div_ceil(64) * 64 - bits));
    if splitmix64(seed).even() { x } else { -x }
}

// A square matrix with pseudorandom elements of at most `bits` bits, of either sign.
fn pseudorandom_matrix(n: u64, bits: u64, seed: &mut u64) -> IntegerMatrix {
    let elements = (0..n * n)
        .map(|_| pseudorandom_integer(bits, seed))
        .collect();
    IntegerMatrix::from_owned_elements(n, n, elements)
}

#[test]
fn test_determinant_sign_large() {
    let mut seed = 3;
    // - elements too large for the dimension to compute the determinant, so the sign is certified
    //   by preconditioning (except for the dimension-5 matrices with 4- and 64-bit elements, whose
    //   certificates are checked directly)
    for (n, bits) in [(3, 2000), (5, 4), (5, 64), (5, 1000), (20, 64), (30, 10), (40, 300)] {
        let a = pseudorandom_matrix(n, bits, &mut seed);
        // Pseudorandom matrices are well-conditioned, so the certificate succeeds.
        assert!(determinant_sign_preconditioned(&a).is_some());
        assert_ne!(verify_determinant_sign(&a), Equal);
        // A singular matrix of the same dimension, whose last row is the first.
        let n_u = usize::try_from(n).unwrap();
        let mut xs = a.to_elements();
        let first: Vec<Integer> = xs[..n_u].to_vec();
        xs[(n_u - 1) * n_u..].clone_from_slice(&first);
        let b = IntegerMatrix::from_owned_elements(n, n, xs);
        assert_eq!(verify_determinant_sign(&b), Equal);
    }
    // - a multiple of the Hilbert matrix of dimension 12, whose condition number is about 2^54, is
    //   still certified
    let n = 12;
    let lcm = Natural::from_str("5354228880").unwrap();
    let elements = (0..n * n)
        .map(|k| Integer::from(&lcm / Natural::from(k / n + k % n + 1)))
        .collect();
    let a = IntegerMatrix::from_owned_elements(n, n, elements);
    assert_eq!(determinant_sign_preconditioned(&a), Some(Greater));
    assert_eq!(verify_determinant_sign(&a), Greater);
    // - the certificate fails, and the determinant is computed: a nonsingular matrix of 200-bit
    //   elements whose last row is its first plus a unit vector, so that its condition number is
    //   about 2^200
    let n = 5;
    let mut xs = pseudorandom_matrix(n, 200, &mut seed).into_elements();
    let n_u = usize::try_from(n).unwrap();
    let mut last: Vec<Integer> = xs[..n_u].to_vec();
    last[0] += Integer::ONE;
    xs[(n_u - 1) * n_u..].clone_from_slice(&last);
    let a = IntegerMatrix::from_owned_elements(n, n, xs);
    assert_eq!(determinant_sign_preconditioned(&a), None);
    assert_ne!(verify_determinant_sign(&a), Equal);
}

#[test]
fn determinant_sign_fail() {
    let a = IntegerMatrix::from_str("[1, 2]").unwrap();
    assert_panic!(a.clone().determinant_sign());
    assert_panic!((&a).determinant_sign());
}

#[test]
fn determinant_sign_properties() {
    integer_matrix_gen_var_1().test_properties(|a| {
        verify_determinant_sign(&a);
    });

    integer_matrix_pair_gen_var_7().test_properties(|(a, b)| {
        verify_determinant_sign(&integer_matrix_low_rank(&a, &b));
    });
}
