// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModPowerOf2Determinant, ModPowerOf2Permanent};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_4;
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_power_of_2_permanent::*;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::unsigned_matrix::arithmetic::permanent_over_ring::*;
use std::panic::catch_unwind;

// Checks the permanent modulo 2^pow of `a` against every algorithm that applies to it.
fn verify<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, pow: u64) -> T {
    let p = a.mod_power_of_2_permanent(pow);
    assert_eq!(a.clone().mod_power_of_2_permanent(pow), p);
    assert_eq!(a.transpose().mod_power_of_2_permanent(pow), p);
    let n = usize::exact_from(a.height());
    let xs = a.elements_ref();
    let ring = ModPowerOf2Ring::new(pow);
    assert_eq!(permanent_structured(&ring, xs.to_vec(), n), p);
    if n <= 10 {
        assert_eq!(permanent_dense(&ring, xs, n), p);
    }
    if (1..=10).contains(&n) {
        assert_eq!(permanent_ryser(&ring, xs, n), p);
        if pow + u64::exact_from(n) - 1 <= T::WIDTH {
            assert_eq!(permanent_glynn_mod_power_of_2(xs, n, pow), p);
        }
    }
    if n <= 6 {
        assert_eq!(permanent_cofactor(&ring, xs, n), p);
    }
    if n <= 8 {
        assert_eq!(unsigned_matrix_mod_power_of_2_permanent_naive(a, pow), p);
    }
    // Modulo 2, the permanent is the determinant, since -1 is 1.
    if pow == 1 {
        assert_eq!(a.mod_power_of_2_determinant(1), p);
    }
    p
}

#[test]
fn test_mod_power_of_2_permanent() {
    let test = |s, pow, out| {
        let a = UnsignedMatrix::<u8>::from_str(s).unwrap();
        assert_eq!(verify(&a, pow).to_string(), out);
    };
    // - dimension 0
    test("[]_{0,0}", 3, "1");
    // - modulo 1
    test("[0]", 0, "0");
    test("[5]", 3, "5");
    // - modulo 2: the determinant
    test("[1, 0; 1, 1]", 1, "1");
    test("[1, 1, 0; 0, 1, 1; 1, 0, 1]", 1, "0");
    test(
        "[1, 1, 1, 1, 1; 1, 1, 1, 1, 1; 1, 1, 1, 1, 1; 1, 1, 1, 1, 1; 1, 1, 1, 1, 0]",
        1,
        "0",
    );
    // - dimension 2
    test("[1, 2; 3, 4]", 3, "2");
    // - dimension 3: cofactor expansion
    test("[1, 2, 3; 4, 5, 6; 7, 0, 1]", 4, "10");
    test("[2, 0, 0; 0, 2, 0; 0, 0, 2]", 2, "0");
    // - dimension 5: Ryser's formula
    test(
        "[3, 1, 4, 1, 5; 9, 2, 6, 5, 3; 5, 8, 9, 7, 9; 3, 2, 3, 8, 4; 6, 2, 6, 4, 3]",
        4,
        "13",
    );
    // - modulo 2^8, the full width of a u8
    test(
        "[3, 1, 4, 1, 5; 9, 2, 6, 5, 3; 5, 8, 9, 7, 9; 3, 2, 3, 8, 4; 6, 2, 6, 4, 3]",
        8,
        "189",
    );
    test(
        "[255, 255, 255, 255, 255, 255; 255, 255, 255, 255, 255, 255; \
        255, 255, 255, 255, 255, 255; 255, 255, 255, 255, 255, 255; \
        255, 255, 255, 255, 255, 255; 255, 255, 255, 255, 255, 255]",
        8,
        "208",
    );
    // - rows with two nonzero elements: contraction
    test("[1, 1, 0, 0; 0, 1, 1, 0; 0, 0, 1, 1; 1, 0, 0, 1]", 8, "2");
}

// The square u8 matrix of dimension `n` whose element in row `i` and column `j` is `f(i, j)`.
fn matrix_from_fn(n: u64, f: impl Fn(u64, u64) -> u8) -> UnsignedMatrix<u8> {
    let elements = (0..n * n).map(|k| f(k / n, k % n)).collect();
    UnsignedMatrix::from_owned_elements(n, n, elements)
}

#[test]
fn test_mod_power_of_2_permanent_structured() {
    let test = |a: UnsignedMatrix<u8>, pow, out| {
        assert_eq!(verify(&a, pow).to_string(), out);
    };
    // - a row with a single nonzero element, repeatedly: the identity
    test(UnsignedMatrix::identity(100), 8, "1");
    // - more than 63 rows, connected, with rows of two nonzero elements: contraction
    test(
        matrix_from_fn(70, |i, j| u8::from(i.abs_diff(j) <= 1)),
        8,
        "209",
    );
    // - more than 63 rows, connected, with three nonzero elements in every row and column:
    //   expansion along the sparsest row. The permanent of I + P + P^2, for a cyclic permutation
    //   matrix P, is 2 plus a Lucas number.
    test(
        matrix_from_fn(66, |i, j| u8::from((j + 66 - i) % 66 <= 2)),
        8,
        "148",
    );
    // - independent blocks that are not square
    test(
        UnsignedMatrix::from_str(
            "[1, 1, 0, 0, 0; 1, 1, 0, 0, 0; 1, 1, 0, 0, 0; 0, 0, 1, 1, 1; 0, 0, 1, 1, 1]",
        )
        .unwrap(),
        8,
        "0",
    );
    // - a zero row
    test(
        UnsignedMatrix::from_str("[1, 2, 3; 0, 0, 0; 7, 8, 9]").unwrap(),
        8,
        "0",
    );
}

#[test]
fn mod_power_of_2_permanent_fail() {
    // - not square
    let a = UnsignedMatrix::<u8>::from_str("[1, 2]").unwrap();
    assert_panic!(a.mod_power_of_2_permanent(3));
    // - not reduced
    let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 9]").unwrap();
    assert_panic!(a.mod_power_of_2_permanent(3));
    // - pow too large
    let a = UnsignedMatrix::<u8>::from_str("[1]").unwrap();
    assert_panic!(a.mod_power_of_2_permanent(9));
}

#[test]
fn mod_power_of_2_permanent_properties() {
    unsigned_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        if a.is_square() && a.height() <= 10 {
            verify(&a, pow);
        }
    });
}
