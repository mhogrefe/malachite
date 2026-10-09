// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Add, ModPowerOf2IsReduced, ModPowerOf2Mul, ModPowerOf2MulAssign,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::unsigned_vector_unsigned_unsigned_triple_gen_var_1;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;
use std::panic::catch_unwind;

#[test]
fn test_mod_power_of_2_mul() {
    let test = |s, c: u8, pow, out| {
        let v = UnsignedVector::<u8>::from_str(s).unwrap();
        let w = (&v).mod_power_of_2_mul(c, pow);
        assert_eq!(w.to_string(), out);
        assert_eq!(v.clone().mod_power_of_2_mul(c, pow), w);
        let mut x = v;
        x.mod_power_of_2_mul_assign(c, pow);
        assert_eq!(x, w);
    };
    test("()", 5, 3, "()");
    test("(0, 0)", 0, 0, "(0, 0)");
    test("(5, 1, 3)", 0, 3, "(0, 0, 0)");
    test("(5, 1, 3)", 1, 3, "(5, 1, 3)");
    test("(5, 1, 3)", 3, 3, "(7, 3, 1)");
    test("(255, 1)", 255, 8, "(1, 255)");
}

fn mod_power_of_2_mul_fail_helper<T: PrimitiveUnsigned>() {
    let v = UnsignedVector::<T> {
        elements: vec![T::exact_from(8u8)],
    };
    // An element is not reduced.
    assert_panic!((&v).mod_power_of_2_mul(T::ONE, 3));
    // The scalar is not reduced.
    assert_panic!((&v).mod_power_of_2_mul(T::exact_from(16u8), 4));
    // The power is too large.
    assert_panic!((&v).mod_power_of_2_mul(T::ONE, T::WIDTH + 1));
    assert_panic!({
        let mut w = v.clone();
        w.mod_power_of_2_mul_assign(T::ONE, 3);
    });
}

#[test]
fn mod_power_of_2_mul_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_mul_fail_helper);
}

fn mod_power_of_2_mul_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_unsigned_triple_gen_var_1::<T>().test_properties(|(v, c, pow)| {
        let w = (&v).mod_power_of_2_mul(c, pow);
        assert_eq!(v.clone().mod_power_of_2_mul(c, pow), w);
        let mut x = v.clone();
        x.mod_power_of_2_mul_assign(c, pow);
        assert_eq!(x, w);

        // The result is reduced, and element by element this is the scalar operation.
        assert!(w.mod_power_of_2_is_reduced(pow));
        assert_eq!(w.dimension(), v.dimension());
        for (&x, &y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(y, x.mod_power_of_2_mul(c, pow));
        }

        // Multiplying by 0 gives the zero vector, by 1 (when 1 is reduced) changes nothing,
        // multiplying twice is multiplying by the square, and it distributes over modular addition.
        assert_eq!(
            (&v).mod_power_of_2_mul(T::ZERO, pow),
            UnsignedVector::zero(v.dimension())
        );
        if pow != 0 {
            assert_eq!((&v).mod_power_of_2_mul(T::ONE, pow), v);
        }
        assert_eq!(
            (&v).mod_power_of_2_mul(c.mod_power_of_2_mul(c, pow), pow),
            (&w).mod_power_of_2_mul(c, pow)
        );
        assert_eq!(
            (&v).mod_power_of_2_add(&v, pow).mod_power_of_2_mul(c, pow),
            (&w).mod_power_of_2_add(&w, pow)
        );
    });
}

#[test]
fn mod_power_of_2_mul_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_mul_properties_helper);
}
