// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModMul, ModPowerOf2Mul, ModPowerOf2MulAssign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::test_util::generators::{
    unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_4,
    unsigned_matrix_unsigned_pair_gen_var_4,
};
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_power_of_2_mul_matrix::*;
use malachite_base::unsigned_matrix::UnsignedMatrix;

#[test]
fn test_mod_power_of_2_mul_matrix() {
    let test = |s, t, pow, out| {
        let a = UnsignedMatrix::<u8>::from_str(s).unwrap();
        let b = UnsignedMatrix::<u8>::from_str(t).unwrap();
        let c = a.clone().mod_power_of_2_mul(b.clone(), pow);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(a.clone().mod_power_of_2_mul(&b, pow), c);
        assert_eq!((&a).mod_power_of_2_mul(b.clone(), pow), c);
        assert_eq!((&a).mod_power_of_2_mul(&b, pow), c);
        let mut d = a.clone();
        d.mod_power_of_2_mul_assign(b.clone(), pow);
        assert_eq!(d, c);
        let mut d = a.clone();
        d.mod_power_of_2_mul_assign(&b, pow);
        assert_eq!(d, c);
        assert_eq!(
            unsigned_matrix_mod_power_of_2_mul_matrix_naive(&a, &b, pow),
            c
        );
    };
    test("[]_{0,0}", "[]_{0,0}", 3, "[]_{0,0}");
    test("[]_{0,3}", "[1, 2; 3, 4; 0, 1]", 3, "[]_{0,2}");
    test("[]_{2,0}", "[]_{0,3}", 3, "[0, 0, 0; 0, 0, 0]");
    test("[1, 2, 3; 4, 5, 6]", "[]_{3,0}", 3, "[]_{2,0}");
    test("[3]", "[4]", 3, "[4]");
    test("[1; 2; 3]", "[4, 5]", 3, "[4, 5; 0, 2; 4, 7]");
    test("[1, 2; 3, 4]", "[5, 6; 7, 8]", 4, "[3, 6; 11, 2]");
    test(
        "[1, 2, 3; 4, 5, 6]",
        "[6, 5; 4, 3; 2, 1]",
        5,
        "[20, 14; 24, 9]",
    );
    test("[0, 0; 0, 0]", "[0, 0; 0, 0]", 0, "[0, 0; 0, 0]");
    test(
        "[200, 250; 100, 7]",
        "[254, 253; 3, 128]",
        8,
        "[94, 168; 77, 84]",
    );
    test(
        "[127, 99; 64, 1]",
        "[3, 100; 127, 126]",
        7,
        "[26, 86; 63, 126]",
    );
    test("[1, 1; 1, 0]", "[1, 1; 1, 0]", 1, "[0, 1; 1, 1]");
}

#[test]
fn test_mod_power_of_2_mul_matrix_algorithms() {
    // Shapes past the cutoff for Strassen's algorithm, with odd and even dimensions in each
    // position, compared with the classical algorithm and checked for agreement among the
    // algorithms.
    fn test<T: PrimitiveUnsigned + WrappingFrom<u64>>(
        height: u64,
        inner: u64,
        width: u64,
        pow: u64,
    ) {
        let mut state = 0x9e37_79b9_7f4a_7c15u64 ^ (height * 1000 + inner);
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            T::wrapping_from(state).mod_power_of_2(pow)
        };
        let a = UnsignedMatrix::from_owned_elements(
            height,
            inner,
            (0..height * inner).map(|_| next()).collect(),
        );
        let b = UnsignedMatrix::from_owned_elements(
            inner,
            width,
            (0..inner * width).map(|_| next()).collect(),
        );
        let c = unsigned_matrix_mod_power_of_2_mul_matrix_classical(&a, &b, pow);
        assert_eq!((&a).mod_power_of_2_mul(&b, pow), c);
        assert_eq!(
            unsigned_matrix_mod_power_of_2_mul_matrix_strassen(&a, &b, pow),
            c
        );
        let mut d = a.clone();
        d.mod_power_of_2_mul_assign(&b, pow);
        assert_eq!(d, c);
    }
    for pow in [13, 64] {
        // Strassen's algorithm called directly on a dimension of 1 falls back to the classical
        // algorithm.
        test::<u64>(1, 4, 4, pow);
        test::<u64>(4, 1, 4, pow);
        test::<u64>(4, 4, 1, pow);
        test::<u64>(48, 48, 48, pow);
        test::<u64>(49, 51, 53, pow);
        test::<u64>(100, 97, 64, pow);
        test::<u64>(64, 100, 97, pow);
        test::<u64>(200, 201, 199, pow);
    }
    test::<u32>(193, 192, 195, 32);
    test::<u128>(50, 49, 48, 128);
    test::<u8>(513, 512, 514, 7);
    test::<u16>(385, 384, 386, 16);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_matrix_fail_1() {
    // `pow` is greater than the width of the type.
    let a = UnsignedMatrix::<u8>::from_str("[]_{0,0}").unwrap();
    let _ = (&a).mod_power_of_2_mul(&a, 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_matrix_fail_2() {
    // The shapes are incompatible.
    let a = UnsignedMatrix::<u8>::from_str("[1, 2]").unwrap();
    let _ = (&a).mod_power_of_2_mul(&a, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_matrix_fail_3() {
    // The first matrix is not reduced.
    let a = UnsignedMatrix::<u8>::from_str("[8]").unwrap();
    let b = UnsignedMatrix::<u8>::from_str("[1]").unwrap();
    let _ = a.mod_power_of_2_mul(b, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_matrix_fail_4() {
    // The second matrix is not reduced.
    let a = UnsignedMatrix::<u8>::from_str("[1]").unwrap();
    let b = UnsignedMatrix::<u8>::from_str("[8]").unwrap();
    let _ = a.mod_power_of_2_mul(b, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_matrix_assign_fail() {
    // The shapes are incompatible.
    let mut a = UnsignedMatrix::<u8>::from_str("[1; 2]").unwrap();
    a.mod_power_of_2_mul_assign(UnsignedMatrix::from_str("[1; 2]").unwrap(), 3);
}

#[test]
fn mod_power_of_2_mul_matrix_properties() {
    unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_4().test_properties(|(a, b, pow)| {
        let c = (&a).mod_power_of_2_mul(&b, pow);
        assert!(c.is_valid());
        assert_eq!((c.height(), c.width()), (a.height(), b.width()));
        assert_eq!(a.clone().mod_power_of_2_mul(b.clone(), pow), c);
        assert_eq!(a.clone().mod_power_of_2_mul(&b, pow), c);
        assert_eq!((&a).mod_power_of_2_mul(b.clone(), pow), c);
        let mut d = a.clone();
        d.mod_power_of_2_mul_assign(b.clone(), pow);
        assert_eq!(d, c);
        let mut d = a.clone();
        d.mod_power_of_2_mul_assign(&b, pow);
        assert_eq!(d, c);
        assert_eq!(
            unsigned_matrix_mod_power_of_2_mul_matrix_naive(&a, &b, pow),
            c
        );
        // The product modulo 2^k agrees with the product modulo the number 2^k, when it is a u64.
        if pow < u64::WIDTH {
            assert_eq!((&a).mod_mul(&b, 1 << pow), c);
        }
        // The transpose of a product is the product of the transposes in the other order.
        assert_eq!(
            b.transpose().mod_power_of_2_mul(a.transpose(), pow),
            c.transpose()
        );
    });

    unsigned_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        let (h, w) = (a.height(), a.width());
        // Multiplying by the identity matrix leaves a matrix unchanged, unless 2^k is 1.
        if pow != 0 {
            assert_eq!(UnsignedMatrix::identity(h).mod_power_of_2_mul(&a, pow), a);
            assert_eq!((&a).mod_power_of_2_mul(UnsignedMatrix::identity(w), pow), a);
        }
        // Zero matrices give zero products.
        assert_eq!(
            (&a).mod_power_of_2_mul(UnsignedMatrix::zero(w, 3), pow),
            UnsignedMatrix::zero(h, 3)
        );
    });
}
