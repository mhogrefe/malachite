// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModAddMul, ModAddMulShl, ModAddMulShlAssign, ModIsReduced, ModShl, ModSubMulShl,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::*;
use malachite_base::unsigned_vector::UnsignedVector;
use std::panic::catch_unwind;

#[test]
fn test_mod_add_mul_shl() {
    let test = |s, t, c: u8, bits, m: u8, out| {
        let u = UnsignedVector::<u8>::from_str(s).unwrap();
        let v = UnsignedVector::<u8>::from_str(t).unwrap();
        let w = (&u).mod_add_mul_shl(&v, c, bits, m);
        assert_eq!(w.to_string(), out);
        assert_eq!(u.clone().mod_add_mul_shl(v.clone(), c, bits, m), w);
        assert_eq!(u.clone().mod_add_mul_shl(&v, c, bits, m), w);
        let mut x = u.clone();
        x.mod_add_mul_shl_assign(v.clone(), c, bits, m);
        assert_eq!(x, w);
        let mut x = u;
        x.mod_add_mul_shl_assign(&v, c, bits, m);
        assert_eq!(x, w);
    };
    test("()", "()", 5, 1, 7, "()");
    test("(0, 0)", "(0, 0)", 0, 0, 1, "(0, 0)");
    test("(5, 1, 3)", "(2, 6, 4)", 0, 1, 7, "(5, 1, 3)");
    test("(5, 1, 3)", "(2, 6, 4)", 3, 0, 7, "(4, 5, 1)");
    test("(5, 1, 3)", "(2, 6, 4)", 3, 2, 7, "(1, 3, 2)");
    test("(5, 1, 3)", "(2, 6, 4)", 6, 100, 7, "(1, 3, 2)");
    test("(254, 1)", "(254, 253)", 254, 9, 255, "(1, 5)");
}

fn mod_add_mul_shl_fail_helper<T: PrimitiveUnsigned + ModShl<u64, T, Output = T>>() {
    let seven = T::exact_from(7u8);
    let v = UnsignedVector::<T> {
        elements: vec![T::ONE],
    };
    let big = UnsignedVector::<T> {
        elements: vec![seven],
    };
    let long = UnsignedVector::<T> {
        elements: vec![T::ONE, T::ONE],
    };
    // An element of either vector is not reduced.
    assert_panic!((&big).mod_add_mul_shl(&v, T::ONE, 1, seven));
    assert_panic!((&v).mod_add_mul_shl(&big, T::ONE, 1, seven));
    // The scalar is not reduced.
    assert_panic!((&v).mod_add_mul_shl(&v, seven, 1, seven));
    // The modulus is zero.
    assert_panic!((&v).mod_add_mul_shl(&v, T::ZERO, 1, T::ZERO));
    // The dimensions differ.
    assert_panic!((&v).mod_add_mul_shl(&long, T::ONE, 1, seven));
    assert_panic!({
        let mut x = big.clone();
        x.mod_add_mul_shl_assign(&v, T::ONE, 1, seven);
    });
}

#[test]
fn mod_add_mul_shl_fail() {
    apply_fn_to_unsigneds!(mod_add_mul_shl_fail_helper);
}

fn mod_add_mul_shl_properties_helper<T: PrimitiveUnsigned + ModShl<u64, T, Output = T>>() {
    unsigned_vector_unsigned_vector_unsigned_unsigned_unsigned_quintuple_gen_var_2::<T>()
        .test_properties(|(u, v, c, bits, m)| {
            let w = (&u).mod_add_mul_shl(&v, c, bits, m);
            assert!(w.mod_is_reduced(&m));
            // The forms agree.
            assert_eq!(u.clone().mod_add_mul_shl(v.clone(), c, bits, m), w);
            assert_eq!(u.clone().mod_add_mul_shl(&v, c, bits, m), w);
            let mut x = u.clone();
            x.mod_add_mul_shl_assign(v.clone(), c, bits, m);
            assert_eq!(x, w);
            let mut x = u.clone();
            x.mod_add_mul_shl_assign(&v, c, bits, m);
            assert_eq!(x, w);

            // Element by element it is the scalar mod_add_mul_shl, and it is `mod_add_mul` by the
            // shifted scalar.
            for ((&x, &y), &z) in u.elements.iter().zip(&v.elements).zip(&w.elements) {
                assert_eq!(z, x.mod_add_mul_shl(y, c, bits, m));
            }
            assert_eq!((&u).mod_add_mul(&v, c.mod_shl(bits, m), m), w);
            // Negating the scalar gives the opposite operation, which undoes this one.
            assert_eq!((&u).mod_sub_mul_shl(&v, c.mod_neg(m), bits, m), w);
            assert_eq!((&w).mod_sub_mul_shl(&v, c, bits, m), u);
        });

    unsigned_vector_unsigned_vector_unsigned_unsigned_quadruple_gen_var_2::<T>().test_properties(
        |(u, v, c, m)| {
            // With no shift, this is `mod_add_mul`.
            assert_eq!(
                (&u).mod_add_mul_shl(&v, c, 0, m),
                (&u).mod_add_mul(&v, c, m)
            );
        },
    );
}

#[test]
fn mod_add_mul_shl_properties() {
    apply_fn_to_unsigneds!(mod_add_mul_shl_properties_helper);
}
