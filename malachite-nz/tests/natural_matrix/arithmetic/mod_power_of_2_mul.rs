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
    ModPowerOf2, ModPowerOf2Add, ModPowerOf2IsReduced, ModPowerOf2Mul, ModPowerOf2MulAssign,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_unsigned_triple_gen_var_2;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_natural_unsigned_triple_gen_var_1;

#[test]
fn test_mod_power_of_2_mul() {
    let test = |s, c, pow, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let c = Natural::from_str(c).unwrap();
        // All four combinations of value and reference, and in place with both.
        let b = (&a).mod_power_of_2_mul(&c, pow);
        assert_eq!(b.to_string(), out);
        assert_eq!((&a).mod_power_of_2_mul(c.clone(), pow), b);
        assert_eq!(a.clone().mod_power_of_2_mul(&c, pow), b);
        assert_eq!(a.clone().mod_power_of_2_mul(c.clone(), pow), b);
        let mut x = a.clone();
        x.mod_power_of_2_mul_assign(&c, pow);
        assert_eq!(x, b);
        let mut x = a;
        x.mod_power_of_2_mul_assign(c, pow);
        assert_eq!(x, b);
    };
    test("[]_{0,0}", "5", 3, "[]_{0,0}");
    test("[0, 0]", "0", 0, "[0, 0]");
    test("[]_{3,0}", "5", 3, "[]_{3,0}");
    test("[5, 1; 3, 2]", "3", 3, "[7, 3; 1, 6]");
    test("[5, 1, 3]", "0", 3, "[0, 0, 0]");
    test("[5, 1, 3]", "1", 3, "[5, 1, 3]");
    test("[5, 1, 3]", "3", 3, "[7, 3, 1]");
    test(
        "[18446744073709551615, 1]",
        "18446744073709551615",
        64,
        "[1, 18446744073709551615]",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_fail_1() {
    // An element is not reduced.
    let _ = NaturalMatrix::from_str("[8]")
        .unwrap()
        .mod_power_of_2_mul(Natural::ONE, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_fail_2() {
    // The scalar is not reduced.
    let _ = (&NaturalMatrix::from_str("[1]").unwrap()).mod_power_of_2_mul(&Natural::from(8u32), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_assign_fail() {
    let mut a = NaturalMatrix::from_str("[1]").unwrap();
    a.mod_power_of_2_mul_assign(Natural::from(8u32), 3);
}

// The empty matrix has no elements to check, so only the scalar check makes this panic.
#[test]
#[should_panic]
fn mod_power_of_2_mul_empty_matrix_fail() {
    let _ = NaturalMatrix::from_str("[]_{0,0}")
        .unwrap()
        .mod_power_of_2_mul(Natural::from(8u32), 3);
}

#[test]
fn mod_power_of_2_mul_properties() {
    natural_matrix_natural_unsigned_triple_gen_var_1().test_properties(|(a, c, pow)| {
        let b = (&a).mod_power_of_2_mul(&c, pow);
        // The forms agree.
        assert_eq!((&a).mod_power_of_2_mul(c.clone(), pow), b);
        assert_eq!(a.clone().mod_power_of_2_mul(&c, pow), b);
        assert_eq!(a.clone().mod_power_of_2_mul(c.clone(), pow), b);
        let mut x = a.clone();
        x.mod_power_of_2_mul_assign(&c, pow);
        assert_eq!(x, b);
        let mut x = a.clone();
        x.mod_power_of_2_mul_assign(c.clone(), pow);
        assert_eq!(x, b);

        // It commutes with transposition.
        assert_eq!(a.transpose().mod_power_of_2_mul(&c, pow), b.transpose());
        // The result is reduced, and is the ordinary product reduced.
        assert!(b.mod_power_of_2_is_reduced(pow));
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        assert_eq!((&a * &c).mod_power_of_2(pow), b);
        for (x, y) in a.elements.iter().zip(&b.elements) {
            assert_eq!(*y, x.mod_power_of_2_mul(&c, pow));
        }

        // Multiplying by 0 gives the zero matrix, by 1 (when 1 is reduced) changes nothing,
        // multiplying twice is multiplying by the square, and it distributes over modular addition.
        assert_eq!(
            (&a).mod_power_of_2_mul(Natural::ZERO, pow),
            NaturalMatrix::zero(a.height(), a.width())
        );
        if pow != 0 {
            assert_eq!((&a).mod_power_of_2_mul(Natural::ONE, pow), a);
        }
        assert_eq!(
            (&a).mod_power_of_2_mul((&c).mod_power_of_2_mul(&c, pow), pow),
            (&b).mod_power_of_2_mul(&c, pow)
        );
        assert_eq!(
            (&a).mod_power_of_2_add(&a, pow).mod_power_of_2_mul(&c, pow),
            (&b).mod_power_of_2_add(&b, pow)
        );
    });

    unsigned_matrix_unsigned_unsigned_triple_gen_var_2().test_properties(|(a, c, pow)| {
        // The `u64` elements multiply as their `Natural` counterparts do.
        assert_eq!(
            NaturalMatrix::from(a.clone()).mod_power_of_2_mul(Natural::from(c), pow),
            NaturalMatrix::from(a.mod_power_of_2_mul(c, pow))
        );
    });
}
