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
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::unsigned_matrix_unsigned_unsigned_triple_gen_var_3;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use std::panic::catch_unwind;

#[test]
fn test_mod_mul() {
    let test = |s, c: u8, m: u8, out| {
        let a = UnsignedMatrix::<u8>::from_str(s).unwrap();
        let b = (&a).mod_mul(c, m);
        assert_eq!(b.to_string(), out);
        assert_eq!(a.clone().mod_mul(c, m), b);
        let mut x = a;
        x.mod_mul_assign(c, m);
        assert_eq!(x, b);
    };
    test("[]_{0,0}", 5, 7, "[]_{0,0}");
    test("[0, 0]", 0, 1, "[0, 0]");
    test("[]_{3,0}", 5, 7, "[]_{3,0}");
    test("[5, 1; 3, 2]", 3, 7, "[1, 3; 2, 6]");
    test("[5, 1, 3]", 0, 7, "[0, 0, 0]");
    test("[5, 1, 3]", 1, 7, "[5, 1, 3]");
    test("[5, 1, 3]", 3, 7, "[1, 3, 2]");
    test("[254, 2]", 254, 255, "[1, 253]");
}

fn mod_mul_fail_helper<T: PrimitiveUnsigned>() {
    let a = UnsignedMatrix::<T>::from_str("[7]").unwrap();
    // The modulus is zero, even with no element to reduce.
    assert_panic!(UnsignedMatrix::<T>::zero(0, 0).mod_mul(T::ZERO, T::ZERO));
    // An element is not reduced.
    assert_panic!((&a).mod_mul(T::ONE, T::exact_from(7u8)));
    // The scalar is not reduced.
    assert_panic!((&a).mod_mul(T::exact_from(8u8), T::exact_from(8u8)));
    assert_panic!({
        let mut b = a.clone();
        b.mod_mul_assign(T::ONE, T::exact_from(7u8));
    });
}

#[test]
fn mod_mul_fail() {
    apply_fn_to_unsigneds!(mod_mul_fail_helper);
}

#[test]
fn mod_mul_properties() {
    unsigned_matrix_unsigned_unsigned_triple_gen_var_3().test_properties(|(a, c, m)| {
        let b = (&a).mod_mul(c, m);
        assert_eq!(a.clone().mod_mul(c, m), b);
        let mut x = a.clone();
        x.mod_mul_assign(c, m);
        assert_eq!(x, b);

        // It commutes with transposition.

        assert_eq!(a.transpose().mod_mul(c, m), b.transpose());

        // The result is reduced, and element by element this is the scalar operation.
        assert!(b.mod_is_reduced(&m));
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        for (&x, &y) in a.elements.iter().zip(&b.elements) {
            assert_eq!(y, x.mod_mul(c, m));
        }

        // Multiplying by 0 gives the zero matrix, by 1 (when 1 is reduced) changes nothing,
        // multiplying twice is multiplying by the square, and it distributes over modular addition.
        assert_eq!(
            (&a).mod_mul(0, m),
            UnsignedMatrix::<u64>::zero(a.height(), a.width())
        );
        if m != 1 {
            assert_eq!((&a).mod_mul(1, m), a);
        }
        assert_eq!((&a).mod_mul(c.mod_mul(c, m), m), (&b).mod_mul(c, m));
        assert_eq!((&a).mod_add(&a, m).mod_mul(c, m), (&b).mod_add(&b, m));
    });
}
