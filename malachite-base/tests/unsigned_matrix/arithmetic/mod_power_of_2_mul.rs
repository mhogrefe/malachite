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
    ModPowerOf2Add, ModPowerOf2IsReduced, ModPowerOf2Mul, ModPowerOf2MulAssign,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::unsigned_matrix_unsigned_unsigned_triple_gen_var_2;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use std::panic::catch_unwind;

#[test]
fn test_mod_power_of_2_mul() {
    let test = |s, c: u8, pow, out| {
        let a = UnsignedMatrix::<u8>::from_str(s).unwrap();
        let b = (&a).mod_power_of_2_mul(c, pow);
        assert_eq!(b.to_string(), out);
        assert_eq!(a.clone().mod_power_of_2_mul(c, pow), b);
        let mut x = a;
        x.mod_power_of_2_mul_assign(c, pow);
        assert_eq!(x, b);
    };
    test("[]_{0,0}", 5, 3, "[]_{0,0}");
    test("[0, 0]", 0, 0, "[0, 0]");
    test("[]_{3,0}", 5, 3, "[]_{3,0}");
    test("[5, 1; 3, 2]", 3, 3, "[7, 3; 1, 6]");
    test("[5, 1, 3]", 0, 3, "[0, 0, 0]");
    test("[5, 1, 3]", 1, 3, "[5, 1, 3]");
    test("[5, 1, 3]", 3, 3, "[7, 3, 1]");
    test("[255, 1]", 255, 8, "[1, 255]");
}

fn mod_power_of_2_mul_fail_helper<T: PrimitiveUnsigned>() {
    let a = UnsignedMatrix::<T>::from_str("[8]").unwrap();
    // An element is not reduced.
    assert_panic!((&a).mod_power_of_2_mul(T::ONE, 3));
    // The scalar is not reduced.
    assert_panic!((&a).mod_power_of_2_mul(T::exact_from(16u8), 4));
    // The power is too large.
    assert_panic!((&a).mod_power_of_2_mul(T::ONE, T::WIDTH + 1));
    assert_panic!({
        let mut b = a.clone();
        b.mod_power_of_2_mul_assign(T::ONE, 3);
    });
}

#[test]
fn mod_power_of_2_mul_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_mul_fail_helper);
}

#[test]
fn mod_power_of_2_mul_properties() {
    unsigned_matrix_unsigned_unsigned_triple_gen_var_2().test_properties(|(a, c, pow)| {
        let b = (&a).mod_power_of_2_mul(c, pow);
        assert_eq!(a.clone().mod_power_of_2_mul(c, pow), b);
        let mut x = a.clone();
        x.mod_power_of_2_mul_assign(c, pow);
        assert_eq!(x, b);

        // It commutes with transposition.
        assert_eq!(a.transpose().mod_power_of_2_mul(c, pow), b.transpose());
        // The result is reduced, and element by element this is the scalar operation.
        assert!(b.mod_power_of_2_is_reduced(pow));
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        for (&x, &y) in a.elements_ref().iter().zip(b.elements_ref()) {
            assert_eq!(y, x.mod_power_of_2_mul(c, pow));
        }

        // Multiplying by 0 gives the zero matrix, by 1 (when 1 is reduced) changes nothing,
        // multiplying twice is multiplying by the square, and it distributes over modular addition.
        assert_eq!(
            (&a).mod_power_of_2_mul(0, pow),
            UnsignedMatrix::<u64>::zero(a.height(), a.width())
        );
        if pow != 0 {
            assert_eq!((&a).mod_power_of_2_mul(1, pow), a);
        }
        assert_eq!(
            (&a).mod_power_of_2_mul(c.mod_power_of_2_mul(c, pow), pow),
            (&b).mod_power_of_2_mul(c, pow)
        );
        assert_eq!(
            (&a).mod_power_of_2_add(&a, pow).mod_power_of_2_mul(c, pow),
            (&b).mod_power_of_2_add(&b, pow)
        );
    });
}
