// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModAdd, ModIsReduced, ModMul, ModMulAssign};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::unsigned_vector_unsigned_unsigned_triple_gen_var_2;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;
use std::panic::catch_unwind;

#[test]
fn test_mod_mul() {
    let test = |s, c: u8, m: u8, out| {
        let v = UnsignedVector::<u8>::from_str(s).unwrap();
        let w = (&v).mod_mul(c, m);
        assert_eq!(w.to_string(), out);
        assert_eq!(v.clone().mod_mul(c, m), w);
        let mut x = v;
        x.mod_mul_assign(c, m);
        assert_eq!(x, w);
    };
    test("()", 5, 7, "()");
    test("(0, 0)", 0, 1, "(0, 0)");
    test("(5, 1, 3)", 0, 7, "(0, 0, 0)");
    test("(5, 1, 3)", 1, 7, "(5, 1, 3)");
    test("(5, 1, 3)", 3, 7, "(1, 3, 2)");
    test("(254, 2)", 254, 255, "(1, 253)");
}

fn mod_mul_fail_helper<T: PrimitiveUnsigned>() {
    let v = UnsignedVector::<T> {
        elements: vec![T::exact_from(7u8)],
    };
    // The modulus is zero, even with no element to reduce.
    assert_panic!(UnsignedVector::<T>::zero(0).mod_mul(T::ZERO, T::ZERO));
    // An element is not reduced.
    assert_panic!((&v).mod_mul(T::ONE, T::exact_from(7u8)));
    // The scalar is not reduced.
    assert_panic!((&v).mod_mul(T::exact_from(8u8), T::exact_from(8u8)));
    assert_panic!({
        let mut w = v.clone();
        w.mod_mul_assign(T::ONE, T::exact_from(7u8));
    });
}

#[test]
fn mod_mul_fail() {
    apply_fn_to_unsigneds!(mod_mul_fail_helper);
}

fn mod_mul_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_unsigned_triple_gen_var_2::<T>().test_properties(|(v, c, m)| {
        let w = (&v).mod_mul(c, m);
        assert_eq!(v.clone().mod_mul(c, m), w);
        let mut x = v.clone();
        x.mod_mul_assign(c, m);
        assert_eq!(x, w);

        // The result is reduced, and element by element this is the scalar operation.
        assert!(w.mod_is_reduced(&m));
        assert_eq!(w.dimension(), v.dimension());
        for (&x, &y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(y, x.mod_mul(c, m));
        }

        // Multiplying by 0 gives the zero vector, by 1 (when 1 is reduced) changes nothing,
        // multiplying twice is multiplying by the square, and it distributes over modular addition.
        assert_eq!(
            (&v).mod_mul(T::ZERO, m),
            UnsignedVector::zero(v.dimension())
        );
        if m != T::ONE {
            assert_eq!((&v).mod_mul(T::ONE, m), v);
        }
        assert_eq!((&v).mod_mul(c.mod_mul(c, m), m), (&w).mod_mul(c, m));
        assert_eq!((&v).mod_add(&v, m).mod_mul(c, m), (&w).mod_add(&w, m));
    });
}

#[test]
fn mod_mul_properties() {
    apply_fn_to_unsigneds!(mod_mul_properties_helper);
}
