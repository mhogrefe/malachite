// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{Determinant, DivisibleBy, Parity};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::arithmetic::determinant::*;
use malachite_nz::integer_matrix::arithmetic::determinant_divisor::determinant_divisor;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    integer_matrix_gen_var_1, integer_matrix_gen_var_2, integer_matrix_pair_gen_var_7,
};
use malachite_nz::test_util::integer_matrix::arithmetic::determinant::*;
use malachite_nz::test_util::integer_matrix::arithmetic::solve::integer_matrix_low_rank;
use std::panic::catch_unwind;

#[test]
fn test_determinant_bareiss() {
    let test = |s, out| {
        let a = IntegerMatrix::from_str(s).unwrap();
        let det = determinant_bareiss(&a);
        assert_eq!(det.to_string(), out);
        assert_eq!(integer_matrix_determinant_naive(&a), det);
    };
    test("[]_{0,0}", "1");
    test("[0]", "0");
    test("[-7]", "-7");
    test("[1, 2; 3, 4]", "-2");
    test("[0, 1; 1, 0]", "-1");
    test("[2, 4; 1, 2]", "0");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 10]", "-3");
    test("[0, 0, 1; 0, 1, 0; 1, 0, 0]", "-1");
    test("[2, 0, 0, 0; 0, 3, 0, 0; 0, 0, 5, 0; 0, 0, 0, 7]", "210");
    test("[1, 2, 3, 4; 5, 6, 7, 8; 2, 6, 4, 8; 3, 1, 1, 2]", "72");
    test(
        "[1267650600228229401496703205376, 1; 1, 1267650600228229401496703205376]",
        "1606938044258990275541962092341162602522202993782792835301375",
    );
    test(
        "[0, 2, 0, 0, 0; 1, 0, 0, 0, 0; 0, 0, 3, 0, 0; 0, 0, 0, 0, 5; 0, 0, 0, 4, 0]",
        "120",
    );
}

#[test]
fn test_determinant_cofactor() {
    let test = |s, out| {
        let a = IntegerMatrix::from_str(s).unwrap();
        let det = determinant_cofactor(&a);
        assert_eq!(det.to_string(), out);
        assert_eq!(integer_matrix_determinant_naive(&a), det);
    };
    test("[]_{0,0}", "1");
    test("[0]", "0");
    test("[-7]", "-7");
    test("[1, 2; 3, 4]", "-2");
    test("[0, 1; 1, 0]", "-1");
    test("[2, 4; 1, 2]", "0");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 10]", "-3");
    test("[0, 0, 1; 0, 1, 0; 1, 0, 0]", "-1");
    test("[2, 0, 0, 0; 0, 3, 0, 0; 0, 0, 5, 0; 0, 0, 0, 7]", "210");
    test("[1, 2, 3, 4; 5, 6, 7, 8; 2, 6, 4, 8; 3, 1, 1, 2]", "72");
    test(
        "[1267650600228229401496703205376, 1; 1, 1267650600228229401496703205376]",
        "1606938044258990275541962092341162602522202993782792835301375",
    );
}

// Checks the determinant of `a` against Bareiss's elimination and the multimodular algorithms, with
// and without a divisor, and checks the divisor. Returns the determinant.
fn verify_determinant(a: &IntegerMatrix) -> Integer {
    let det = a.clone().determinant();
    assert_eq!(a.determinant(), det);
    assert_eq!(determinant_bareiss(a), det);
    assert_eq!(determinant_modular(a, true), det);
    assert_eq!(determinant_modular_accelerated(a, true), det);
    let d = determinant_divisor(a);
    if det == 0u32 {
        assert_eq!(d, 0u32);
    } else {
        assert_ne!(d, 0u32);
        assert!(det.unsigned_abs_ref().divisible_by(&d));
    }
    assert_eq!(determinant_modular_given_divisor(a, &d, true), det);
    det
}

#[test]
fn test_determinant() {
    let test = |s, out| {
        let a = IntegerMatrix::from_str(s).unwrap();
        let det = verify_determinant(&a);
        assert_eq!(det.to_string(), out);
        // On these small matrices, the unproved form stops early but is still correct.
        assert_eq!(determinant_modular(&a, false), det);
    };
    test("[]_{0,0}", "1");
    test("[0]", "0");
    test("[-7]", "-7");
    test("[1, 2; 3, 4]", "-2");
    test("[0, 1; 1, 0]", "-1");
    test("[2, 4; 1, 2]", "0");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 10]", "-3");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 9]", "0");
    test("[2, 0, 0, 0; 0, 3, 0, 0; 0, 0, 5, 0; 0, 0, 0, 7]", "210");
    test("[1, 2, 3, 4; 5, 6, 7, 8; 2, 6, 4, 8; 3, 1, 1, 2]", "72");
    test(
        "[1267650600228229401496703205376, 1; 1, 1267650600228229401496703205376]",
        "1606938044258990275541962092341162602522202993782792835301375",
    );
    test(
        "[0, 2, 0, 0, 0; 1, 0, 0, 0, 0; 0, 0, 3, 0, 0; 0, 0, 0, 0, 5; 0, 0, 0, 4, 0]",
        "120",
    );
    // - the determinant is much smaller than its bound, so the unproved form stops early
    test(
        "[1267650600228229401496703205376, 1267650600228229401496703205377; \
        1267650600228229401496703205375, 1267650600228229401496703205376]",
        "1",
    );
    // - the matrix is singular modulo the first prime, but not over the integers
    // - the divisor is a multiple of the first prime, which is skipped
    test("[576460752303423619, 0; 0, 1]", "576460752303423619");
}

#[test]
fn test_determinant_divisor() {
    let test = |s, out| {
        let a = IntegerMatrix::from_str(s).unwrap();
        assert_eq!(determinant_divisor(&a).to_string(), out);
    };
    test("[]_{0,0}", "1");
    test("[0]", "0");
    test("[-7]", "7");
    test("[1, 2; 3, 4]", "1");
    test("[2, 4; 1, 2]", "0");
    test("[2, 0, 0, 0; 0, 3, 0, 0; 0, 0, 5, 0; 0, 0, 0, 7]", "210");
    test("[576460752303423619, 0; 0, 1]", "576460752303423619");
}

// The next output of the SplitMix64 generator with state `seed`.
const fn splitmix64(seed: &mut u64) -> u64 {
    *seed = seed.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = *seed;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}

// A square matrix with pseudorandom elements of at most `bits` bits, of either sign.
fn pseudorandom_matrix(n: u64, bits: u64, seed: &mut u64) -> IntegerMatrix {
    let elements = (0..n * n)
        .map(|_| {
            let x = Integer::from(splitmix64(seed) >> (64 - bits));
            if splitmix64(seed).even() { x } else { -x }
        })
        .collect();
    IntegerMatrix::from_owned_elements(n, n, elements)
}

#[test]
fn test_determinant_large() {
    let mut seed = 2;
    // - dimension 25 to 59: the multimodular algorithm
    // - dimension at least 60, less than the number of bits: the multimodular algorithm
    // - dimension at least 60, at least the number of bits: the multimodular algorithm with a
    //   divisor
    for (n, bits) in [(25, 10), (30, 64), (61, 64), (62, 4)] {
        let a = pseudorandom_matrix(n, bits, &mut seed);
        let det = verify_determinant(&a);
        assert_ne!(det, 0u32);
        // A singular matrix of the same dimension, whose last row is the first.
        let mut b = a.clone();
        let n = usize::try_from(n).unwrap();
        let first: Vec<Integer> = b.elements[..n].to_vec();
        b.elements[(n - 1) * n..].clone_from_slice(&first);
        assert_eq!(verify_determinant(&b), 0u32);
        assert_eq!(determinant_divisor(&b), Natural::ZERO);
    }
}

#[test]
fn determinant_fail() {
    let a = IntegerMatrix::from_str("[1, 2]").unwrap();
    assert_panic!(determinant_bareiss(&a));
    assert_panic!(determinant_cofactor(&a));
    assert_panic!(a.clone().determinant());
    assert_panic!((&a).determinant());
    assert_panic!(determinant_modular(&a, true));
    assert_panic!(determinant_divisor(&a));
    let a = IntegerMatrix::from_str(
        "[1, 0, 0, 0, 0; 0, 1, 0, 0, 0; 0, 0, 1, 0, 0; 0, 0, 0, 1, 0; 0, 0, 0, 0, 1]",
    )
    .unwrap();
    assert_panic!(determinant_cofactor(&a));
}

#[test]
fn determinant_properties() {
    integer_matrix_gen_var_1().test_properties(|a| {
        let det = determinant_bareiss(&a);
        assert_eq!(integer_matrix_determinant_naive(&a), det);
        assert_eq!(determinant_bareiss(&a.transpose()), det);
        assert!(*det.unsigned_abs_ref() <= a.determinant_bound());
    });

    integer_matrix_gen_var_2().test_properties(|a| {
        assert_eq!(determinant_cofactor(&a), determinant_bareiss(&a));
    });

    integer_matrix_gen_var_1().test_properties(|a| {
        verify_determinant(&a);
    });

    integer_matrix_pair_gen_var_7().test_properties(|(a, b)| {
        verify_determinant(&integer_matrix_low_rank(&a, &b));
    });
}
