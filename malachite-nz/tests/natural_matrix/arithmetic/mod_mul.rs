// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModAdd, ModIsReduced, ModMul, ModMulAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_unsigned_triple_gen_var_3;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_natural_natural_triple_gen_var_1;

#[test]
fn test_mod_mul() {
    let test = |s, c, m, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let c = Natural::from_str(c).unwrap();
        let m = Natural::from_str(m).unwrap();
        // All eight combinations of value and reference, and in place with all four.
        let b = (&a).mod_mul(&c, &m);
        assert_eq!(b.to_string(), out);
        assert_eq!((&a).mod_mul(&c, m.clone()), b);
        assert_eq!((&a).mod_mul(c.clone(), &m), b);
        assert_eq!((&a).mod_mul(c.clone(), m.clone()), b);
        assert_eq!(a.clone().mod_mul(&c, &m), b);
        assert_eq!(a.clone().mod_mul(&c, m.clone()), b);
        assert_eq!(a.clone().mod_mul(c.clone(), &m), b);
        assert_eq!(a.clone().mod_mul(c.clone(), m.clone()), b);
        let mut x = a.clone();
        x.mod_mul_assign(&c, &m);
        assert_eq!(x, b);
        let mut x = a.clone();
        x.mod_mul_assign(&c, m.clone());
        assert_eq!(x, b);
        let mut x = a.clone();
        x.mod_mul_assign(c.clone(), &m);
        assert_eq!(x, b);
        let mut x = a;
        x.mod_mul_assign(c, m);
        assert_eq!(x, b);
    };
    test("[]_{0,0}", "5", "7", "[]_{0,0}");
    test("[0, 0]", "0", "1", "[0, 0]");
    test("[]_{3,0}", "5", "7", "[]_{3,0}");
    test("[5, 1; 3, 2]", "3", "7", "[1, 3; 2, 6]");
    test("[5, 1, 3]", "0", "7", "[0, 0, 0]");
    test("[5, 1, 3]", "1", "7", "[5, 1, 3]");
    test("[5, 1, 3]", "3", "7", "[1, 3, 2]");
    test(
        "[18446744073709551615, 2]",
        "18446744073709551615",
        "18446744073709551616",
        "[1, 18446744073709551614]",
    );
    test(
        "[123456789012345678901234567890, 7]",
        "2",
        "1000000000000000000000000000000",
        "[246913578024691357802469135780, 14]",
    );
}

#[test]
#[should_panic]
fn mod_mul_fail_1() {
    // The modulus is zero, even though there is no element to reduce.
    let _ = NaturalMatrix::from_str("[]_{0,0}")
        .unwrap()
        .mod_mul(Natural::ZERO, Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_mul_fail_2() {
    // An element is not reduced.
    let _ = (&NaturalMatrix::from_str("[7]").unwrap()).mod_mul(Natural::ONE, Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_mul_fail_3() {
    // The scalar is not reduced.
    let _ = NaturalMatrix::from_str("[1]")
        .unwrap()
        .mod_mul(&Natural::from(7u32), &Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_mul_assign_fail() {
    let mut a = NaturalMatrix::from_str("[1]").unwrap();
    a.mod_mul_assign(Natural::from(7u32), Natural::from(7u32));
}

#[test]
fn mod_mul_properties() {
    natural_matrix_natural_natural_triple_gen_var_1().test_properties(|(a, c, m)| {
        let b = (&a).mod_mul(&c, &m);
        // The forms agree.
        assert_eq!((&a).mod_mul(&c, m.clone()), b);
        assert_eq!((&a).mod_mul(c.clone(), &m), b);
        assert_eq!((&a).mod_mul(c.clone(), m.clone()), b);
        assert_eq!(a.clone().mod_mul(&c, &m), b);
        assert_eq!(a.clone().mod_mul(c.clone(), m.clone()), b);
        let mut x = a.clone();
        x.mod_mul_assign(&c, &m);
        assert_eq!(x, b);
        let mut x = a.clone();
        x.mod_mul_assign(c.clone(), m.clone());
        assert_eq!(x, b);

        // It commutes with transposition.

        assert_eq!(a.transpose().mod_mul(&c, &m), b.transpose());

        // The result is reduced, and is the ordinary product reduced.
        assert!(b.mod_is_reduced(&m));
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        assert_eq!((&a * &c) % &m, b);
        for (x, y) in a.elements.iter().zip(&b.elements) {
            assert_eq!(*y, x.mod_mul(&c, &m));
        }

        // Multiplying by 0 gives the zero matrix, by 1 (when 1 is reduced) changes nothing,
        // multiplying twice is multiplying by the square, and it distributes over modular addition.
        assert_eq!(
            (&a).mod_mul(Natural::ZERO, &m),
            NaturalMatrix::zero(a.height(), a.width())
        );
        if m != 1u32 {
            assert_eq!((&a).mod_mul(Natural::ONE, &m), a);
        }
        assert_eq!((&a).mod_mul((&c).mod_mul(&c, &m), &m), (&b).mod_mul(&c, &m));
        assert_eq!((&a).mod_add(&a, &m).mod_mul(&c, &m), (&b).mod_add(&b, &m));
    });

    unsigned_matrix_unsigned_unsigned_triple_gen_var_3().test_properties(|(a, c, m)| {
        // The `u64` elements multiply as their `Natural` counterparts do.
        assert_eq!(
            NaturalMatrix::from(a.clone()).mod_mul(Natural::from(c), Natural::from(m)),
            NaturalMatrix::from(a.mod_mul(c, m))
        );
    });
}
