// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModAdd, ModMul, ModMulAssign};
use malachite_base::test_util::generators::{
    unsigned_matrix_gen_var_1, unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_3,
};
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_mul_matrix::*;
use malachite_base::unsigned_matrix::UnsignedMatrix;

#[test]
fn test_mod_mul_matrix() {
    let test = |s, t, m: u8, out| {
        let a = UnsignedMatrix::<u8>::from_str(s).unwrap();
        let b = UnsignedMatrix::<u8>::from_str(t).unwrap();
        let c = a.clone().mod_mul(b.clone(), m);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(a.clone().mod_mul(&b, m), c);
        assert_eq!((&a).mod_mul(b.clone(), m), c);
        assert_eq!((&a).mod_mul(&b, m), c);
        let mut d = a.clone();
        d.mod_mul_assign(b.clone(), m);
        assert_eq!(d, c);
        let mut d = a.clone();
        d.mod_mul_assign(&b, m);
        assert_eq!(d, c);
        assert_eq!(unsigned_matrix_mod_mul_matrix_naive(&a, &b, m), c);
    };
    test("[]_{0,0}", "[]_{0,0}", 5, "[]_{0,0}");
    test("[]_{0,3}", "[1, 2; 3, 4; 0, 1]", 5, "[]_{0,2}");
    test("[]_{2,0}", "[]_{0,3}", 7, "[0, 0, 0; 0, 0, 0]");
    test("[1, 2, 3; 4, 5, 6]", "[]_{3,0}", 7, "[]_{2,0}");
    test("[3]", "[4]", 5, "[2]");
    test("[1; 2; 3]", "[4, 5]", 7, "[4, 5; 1, 3; 5, 1]");
    test("[1, 2; 3, 4]", "[5, 6; 7, 8]", 9, "[1, 4; 7, 5]");
    test(
        "[1, 2, 3; 4, 5, 6]",
        "[6, 5; 4, 3; 2, 1]",
        7,
        "[6, 0; 0, 6]",
    );
    test("[0, 0; 0, 0]", "[0, 0; 0, 0]", 1, "[0, 0; 0, 0]");
    test(
        "[200, 250; 100, 7]",
        "[254, 253; 3, 128]",
        255,
        "[40, 235; 176, 186]",
    );
}

#[test]
#[should_panic]
fn mod_mul_matrix_fail_1() {
    // The modulus is zero.
    let a = UnsignedMatrix::<u8>::from_str("[]_{0,0}").unwrap();
    let _ = (&a).mod_mul(&a, 0);
}

#[test]
#[should_panic]
fn mod_mul_matrix_fail_2() {
    // The shapes are incompatible.
    let a = UnsignedMatrix::<u8>::from_str("[1, 2]").unwrap();
    let _ = (&a).mod_mul(&a, 5);
}

#[test]
#[should_panic]
fn mod_mul_matrix_fail_3() {
    // The first matrix is not reduced.
    let a = UnsignedMatrix::<u8>::from_str("[5]").unwrap();
    let b = UnsignedMatrix::<u8>::from_str("[1]").unwrap();
    let _ = a.mod_mul(b, 5);
}

#[test]
#[should_panic]
fn mod_mul_matrix_fail_4() {
    // The second matrix is not reduced.
    let a = UnsignedMatrix::<u8>::from_str("[1]").unwrap();
    let b = UnsignedMatrix::<u8>::from_str("[5]").unwrap();
    let _ = a.mod_mul(b, 5);
}

#[test]
#[should_panic]
fn mod_mul_matrix_assign_fail() {
    let mut a = UnsignedMatrix::<u8>::from_str("[1, 2]").unwrap();
    a.mod_mul_assign(UnsignedMatrix::<u8>::from_str("[1, 2]").unwrap(), 5);
}

// A matrix of words reduced modulo `m`, from a fixed pseudorandom sequence.
fn pseudorandom_matrix(height: u64, width: u64, m: u64, seed: u64) -> UnsignedMatrix<u64> {
    let mut state = seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1;
    let mut elements = Vec::new();
    for _ in 0..height * width {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        elements.push(state % m);
    }
    UnsignedMatrix::from_owned_elements(height, width, elements)
}

#[test]
fn test_mod_mul_matrix_algorithms() {
    let test = |height, inner, width, m, seed| {
        let a = pseudorandom_matrix(height, inner, m, seed);
        let b = pseudorandom_matrix(inner, width, m, seed + 100);
        let c = unsigned_matrix_mod_mul_matrix_naive(&a, &b, m);
        assert_eq!((&a).mod_mul(&b, m), c);
        let mut d = a.clone();
        d.mod_mul_assign(&b, m);
        assert_eq!(d, c);
        assert_eq!(unsigned_matrix_mod_mul_matrix_classical(&a, &b, m), c);
        assert_eq!(unsigned_matrix_mod_mul_matrix_strassen(&a, &b, m), c);
        let e = pseudorandom_matrix(height, width, m, seed + 200);
        assert_eq!(
            unsigned_matrix_mod_add_mul_matrix_classical(&e, &a, &b, m),
            (&e).mod_add(&c, m)
        );
    };
    for m in [2, 7, 1000, 1 << 32, (1 << 63) + 25, u64::MAX] {
        // Small dimensions: each column of the second matrix is read in place.
        test(3, 4, 5, m, 1);
        // Dimensions above 10, which the packed algorithm needs for small moduli.
        test(12, 13, 11, m, 2);
        // Dimensions of at least 20: the second matrix is transposed when the packed algorithm does
        // not apply.
        test(21, 22, 23, m, 3);
        // Strassen's algorithm with odd dimensions in each position.
        test(9, 11, 7, m, 4);
        test(10, 10, 10, m, 5);
    }
    // The inner dimension is 0, and the modulus is 1.
    test(3, 0, 2, 5, 6);
    test(4, 5, 6, 1, 7);
}

#[test]
fn test_mod_mul_matrix_strassen_dispatch() {
    // At these dimensions the product uses Strassen's algorithm: above 200 for large moduli, and
    // above 400 for moduli below 2048.
    for (n, m) in [(201, (1 << 63) + 25), (401, 1000)] {
        let a = pseudorandom_matrix(n, n, m, 8);
        let b = pseudorandom_matrix(n, n, m, 9);
        assert_eq!(
            (&a).mod_mul(&b, m),
            unsigned_matrix_mod_mul_matrix_classical(&a, &b, m)
        );
    }
}

#[test]
fn mod_mul_matrix_properties() {
    unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_3().test_properties(|(a, b, m)| {
        let c = (&a).mod_mul(&b, m);
        assert!(c.is_valid());
        assert_eq!((c.height(), c.width()), (a.height(), b.width()));
        assert_eq!(a.clone().mod_mul(b.clone(), m), c);
        assert_eq!(a.clone().mod_mul(&b, m), c);
        assert_eq!((&a).mod_mul(b.clone(), m), c);
        let mut d = a.clone();
        d.mod_mul_assign(&b, m);
        assert_eq!(d, c);
        assert_eq!(unsigned_matrix_mod_mul_matrix_naive(&a, &b, m), c);
        assert_eq!(unsigned_matrix_mod_mul_matrix_classical(&a, &b, m), c);
        assert_eq!(unsigned_matrix_mod_mul_matrix_strassen(&a, &b, m), c);
        // The transpose of a product is the product of the transposes in the other order.
        assert_eq!(b.transpose().mod_mul(a.transpose(), m), c.transpose());
        // Each column of the product is the first matrix times that column of the second.
        for j in 0..b.width() {
            assert_eq!((&a).mod_mul(b.column(j), m), c.column(j));
        }
    });

    unsigned_matrix_gen_var_1().test_properties(|a| {
        let n = a.height();
        // The modulus must exceed every element and 1, the identity's diagonal element.
        if let Some(m) = a
            .elements_ref()
            .iter()
            .max()
            .and_then(|&x| x.checked_add(1))
            .map(|m| m.max(2))
        {
            // The identity matrix leaves a reduced matrix unchanged.
            assert_eq!(UnsignedMatrix::identity(n).mod_mul(&a, m), a);
            assert_eq!((&a).mod_mul(UnsignedMatrix::identity(n), m), a);
        }
    });
}
