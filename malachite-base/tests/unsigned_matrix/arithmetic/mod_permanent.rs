// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModPermanent, ModPowerOf2Permanent, Parity};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_11;
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_permanent::*;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::unsigned_matrix::arithmetic::permanent_over_ring::*;
use std::panic::catch_unwind;

// Checks the permanent modulo m of `a` against every algorithm that applies to it.
fn verify<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, m: T) -> T {
    let p = a.mod_permanent(m);
    assert_eq!(a.clone().mod_permanent(m), p);
    assert_eq!(a.transpose().mod_permanent(m), p);
    let n = usize::exact_from(a.height());
    let xs = a.elements_ref();
    let ring = ModRing::new(m);
    assert_eq!(permanent_structured(&ring, xs.to_vec(), n), p);
    if n <= 10 {
        assert_eq!(permanent_dense(&ring, xs, n), p);
    }
    if (1..=10).contains(&n) {
        assert_eq!(permanent_ryser(&ring, xs, n), p);
        if let Some(inverse) = ring.inverse_power_of_2(u64::wrapping_from(n - 1)) {
            assert_eq!(permanent_glynn(&ring, xs, n, &inverse), p);
        }
    }
    if n <= 6 {
        assert_eq!(permanent_cofactor(&ring, xs, n), p);
    }
    if n <= 8 {
        assert_eq!(unsigned_matrix_mod_permanent_naive(a, m), p);
    }
    if m.is_power_of_2() {
        assert_eq!(a.mod_power_of_2_permanent(m.trailing_zeros()), p);
    }
    p
}

#[test]
fn test_mod_permanent() {
    let test = |s, m, out| {
        let a = UnsignedMatrix::<u8>::from_str(s).unwrap();
        assert_eq!(verify(&a, m).to_string(), out);
    };
    // - dimension 0
    test("[]_{0,0}", 7, "1");
    // - modulo 1
    test("[]_{0,0}", 1, "0");
    test("[0]", 1, "0");
    test("[5]", 7, "5");
    // - dimension 2
    test("[1, 2; 3, 4]", 7, "3");
    // - dimension 3: cofactor expansion
    test("[1, 2, 3; 4, 5, 6; 7, 0, 1]", 11, "4");
    // - odd modulus, dimension 5: Glynn's formula
    test(
        "[3, 1, 4, 1, 5; 9, 2, 6, 5, 3; 5, 8, 9, 7, 9; 3, 2, 3, 8, 4; 6, 2, 6, 4, 3]",
        251,
        "88",
    );
    // - power of 2 modulus
    test(
        "[3, 1, 4, 1, 5; 9, 2, 6, 5, 3; 5, 8, 9, 7, 9; 3, 2, 3, 8, 4; 6, 2, 6, 4, 3]",
        128,
        "61",
    );
    // - modulus 2 times an odd number: the determinant modulo 2 and Glynn's formula
    test(
        "[3, 1, 4, 1, 5; 9, 2, 6, 5, 3; 5, 8, 9, 7, 9; 3, 2, 3, 8, 4; 6, 2, 6, 4, 3]",
        10,
        "7",
    );
    // - modulus a larger power of 2 times an odd number: Ryser's and Glynn's formulas
    test(
        "[3, 1, 4, 1, 5; 9, 2, 6, 5, 3; 5, 8, 9, 7, 9; 3, 2, 3, 8, 4; 6, 2, 6, 4, 3]",
        240,
        "237",
    );
    // - the largest odd modulus
    test(
        "[254, 254, 254, 254, 254, 254; 254, 254, 254, 254, 254, 254; \
        254, 254, 254, 254, 254, 254; 254, 254, 254, 254, 254, 254; \
        254, 254, 254, 254, 254, 254; 254, 254, 254, 254, 254, 254]",
        255,
        "210",
    );
    // - the largest even modulus
    test(
        "[253, 253, 253, 253, 253, 253; 253, 253, 253, 253, 253, 253; \
        253, 253, 253, 253, 253, 253; 253, 253, 253, 253, 253, 253; \
        253, 253, 253, 253, 253, 253; 253, 253, 253, 253, 253, 253]",
        254,
        "212",
    );
    // - rows with two nonzero elements: contraction
    test("[1, 1, 0, 0; 0, 1, 1, 0; 0, 0, 1, 1; 1, 0, 0, 1]", 3, "2");
}

// The square u8 matrix of dimension `n` whose element in row `i` and column `j` is `f(i, j)`.
fn matrix_from_fn(n: u64, mut f: impl FnMut(u64, u64) -> u8) -> UnsignedMatrix<u8> {
    let elements = (0..n * n).map(|k| f(k / n, k % n)).collect();
    UnsignedMatrix::from_owned_elements(n, n, elements)
}

#[test]
fn test_mod_permanent_structured() {
    let test = |a: UnsignedMatrix<u8>, m, out| {
        assert_eq!(verify(&a, m).to_string(), out);
    };
    // - a row with a single nonzero element, repeatedly: the identity
    test(UnsignedMatrix::identity(100), 7, "1");
    // - more than 63 rows, connected, with rows of two nonzero elements: contraction
    test(
        matrix_from_fn(70, |i, j| u8::from(i.abs_diff(j) <= 1)),
        251,
        "150",
    );
    test(
        matrix_from_fn(70, |i, j| u8::from(i.abs_diff(j) <= 1)),
        250,
        "129",
    );
    // - more than 63 rows, connected, with three nonzero elements in every row and column:
    //   expansion along the sparsest row. The permanent of I + P + P^2, for a cyclic permutation
    //   matrix P, is 2 plus a Lucas number.
    test(
        matrix_from_fn(66, |i, j| u8::from((j + 66 - i) % 66 <= 2)),
        255,
        "20",
    );
    // - independent blocks that are not square
    test(
        UnsignedMatrix::from_str(
            "[1, 1, 0, 0, 0; 1, 1, 0, 0, 0; 1, 1, 0, 0, 0; 0, 0, 1, 1, 1; 0, 0, 1, 1, 1]",
        )
        .unwrap(),
        5,
        "0",
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

#[test]
fn test_mod_permanent_structured_sparse() {
    // Pseudorandom sparse matrices modulo small numbers, half of whose elements are 0, whose rows
    // and columns often have one or two nonzero elements, and whose contractions often cancel to 0.
    let mut seed = 0;
    for k in 0..2000u64 {
        let n = 3 + k % 6;
        let m = 2 + u8::try_from(k % 29).unwrap();
        let a = matrix_from_fn(n, |_, _| {
            let x = splitmix64(&mut seed);
            if x.even() {
                0
            } else {
                u8::try_from((x >> 1) % u64::from(m)).unwrap()
            }
        });
        verify(&a, m);
    }
}

#[test]
fn mod_permanent_fail() {
    // - m = 0
    let a = UnsignedMatrix::<u8>::from_str("[1]").unwrap();
    assert_panic!(a.mod_permanent(0));
    // - not square
    let a = UnsignedMatrix::<u8>::from_str("[1, 2]").unwrap();
    assert_panic!(a.mod_permanent(5));
    // - not reduced
    let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 9]").unwrap();
    assert_panic!(a.mod_permanent(7));
}

#[test]
fn mod_permanent_properties() {
    unsigned_matrix_unsigned_pair_gen_var_11().test_properties(|(a, m)| {
        if a.height() <= 10 {
            verify(&a, m);
        }
    });
}
