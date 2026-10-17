// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    Mod, ModPowerOf2, ModPowerOf2IsReduced, PowerOf2, RemPowerOf2, RemPowerOf2Assign,
};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::{integer_matrix_gen, integer_matrix_unsigned_pair_gen};

#[test]
fn test_mod_power_of_2() {
    let test = |s, pow, out| {
        let a = IntegerMatrix::from_str(s).unwrap();
        let b = (&a).mod_power_of_2(pow);
        assert_eq!(b.to_string(), out);
        assert_eq!(a.mod_power_of_2(pow), b);
    };
    test("[]_{0,0}", 0, "[]_{0,0}");
    test("[]_{3,0}", 0, "[]_{3,0}");
    test("[]_{0,4}", 0, "[]_{0,4}");
    test("[5, -4; -1, 9]", 2, "[1, 0; 3, 1]");
    test("[-4; 3]", 1, "[0; 1]");
    test("[]_{0,0}", 8, "[]_{0,0}");
    test("[0]", 3, "[0]");
    // Negative elements become non-negative.
    test("[1, -3, -2]", 2, "[1, 1, 2]");
    // Modulo 2^0 every element is zero, but the shape is unchanged.
    test("[1, -3, -2]", 0, "[0, 0, 0]");
    // A negative multiple of the modulus becomes zero, and stays.
    test("[-4, 3]", 2, "[0, 3]");
    // Powers at and beyond a word.
    test("[-1]", 64, "[18446744073709551615]");
    test("[-1]", 100, "[1267650600228229401496703205375]");
    test("[-18446744073709551616]", 64, "[0]");
}

#[test]
fn test_rem_power_of_2() {
    let test = |s, pow, out| {
        let a = IntegerMatrix::from_str(s).unwrap();
        let b = (&a).rem_power_of_2(pow);
        assert_eq!(b.to_string(), out);
        assert_eq!(a.clone().rem_power_of_2(pow), b);
        let mut x = a;
        x.rem_power_of_2_assign(pow);
        assert_eq!(x, b);
    };
    test("[]_{0,0}", 0, "[]_{0,0}");
    test("[]_{3,0}", 0, "[]_{3,0}");
    test("[]_{0,4}", 0, "[]_{0,4}");
    test("[5, -4; -1, 9]", 2, "[1, 0; -1, 1]");
    test("[-4; 3]", 1, "[0; 1]");
    test("[]_{0,0}", 8, "[]_{0,0}");
    test("[0]", 3, "[0]");
    // Every element keeps its sign.
    test("[1, -7, -2]", 2, "[1, -3, -2]");
    // Modulo 2^0 every element is zero, but the shape is unchanged.
    test("[1, -3]", 0, "[0, 0]");
    test("[-4, -3]", 2, "[0, -3]");
    // Powers at and beyond a word.
    test("[-1]", 64, "[-1]");
    test("[-18446744073709551617]", 64, "[-1]");
    test("[-18446744073709551617]", 100, "[-18446744073709551617]");
}

#[test]
fn mod_power_of_2_properties() {
    integer_matrix_unsigned_pair_gen().test_properties(|(a, pow)| {
        let b = (&a).mod_power_of_2(pow);
        assert_eq!(a.clone().mod_power_of_2(pow), b);
        // The result is reduced, and the shape is unchanged.
        assert!(b.mod_power_of_2_is_reduced(pow));
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        // Element by element, this is the `Integer` operation.
        for (x, y) in a.elements.iter().zip(&b.elements) {
            assert_eq!(*y, x.mod_power_of_2(pow));
        }
        // It is `mod_op` with a power-of-2 modulus.
        assert_eq!((&a).mod_op(Natural::power_of_2(pow)), b);
        // On a matrix with no negative elements, this is the `NaturalMatrix` operation.
        if let Ok(n) = NaturalMatrix::try_from(&a) {
            assert_eq!(n.mod_power_of_2(pow), b);
        }
        // Taking the signed remainder first changes nothing.
        assert_eq!((&a).rem_power_of_2(pow).mod_power_of_2(pow), b);
    });

    integer_matrix_gen().test_properties(|a| {
        // Modulo 2^0 every element vanishes, but the shape is unchanged.
        assert_eq!(
            (&a).mod_power_of_2(0),
            NaturalMatrix::from_owned_elements(
                a.height(),
                a.width(),
                vec![Natural::ZERO; a.elements.len()]
            )
        );
        for pow in [64, 100] {
            assert_eq!(
                (&a).mod_power_of_2(pow),
                (&a).mod_op(Natural::power_of_2(pow))
            );
        }
    });
}

#[test]
fn rem_power_of_2_properties() {
    integer_matrix_unsigned_pair_gen().test_properties(|(a, pow)| {
        let b = (&a).rem_power_of_2(pow);
        assert_eq!(a.clone().rem_power_of_2(pow), b);
        let mut x = a.clone();
        x.rem_power_of_2_assign(pow);
        assert_eq!(x, b);
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        // Element by element, this is the `Integer` operation.
        for (x, y) in a.elements.iter().zip(&b.elements) {
            assert_eq!(*y, x.rem_power_of_2(pow));
        }
        // It is `%` with a power-of-2 modulus, and it commutes with negation.
        assert_eq!(&a % Integer::power_of_2(pow), b);
        assert_eq!((-&a).rem_power_of_2(pow), -&b);
        // Reducing again changes nothing.
        assert_eq!((&b).rem_power_of_2(pow), b);
    });
}
