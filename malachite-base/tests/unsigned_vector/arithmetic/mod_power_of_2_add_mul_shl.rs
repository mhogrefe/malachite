// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2AddMul, ModPowerOf2AddMulShl, ModPowerOf2AddMulShlAssign, ModPowerOf2IsReduced,
    ModPowerOf2SubMulShl,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::*;
use malachite_base::unsigned_vector::UnsignedVector;
use std::panic::catch_unwind;

#[test]
fn test_mod_power_of_2_add_mul_shl() {
    let test = |s, t, c: u8, bits, pow, out| {
        let u = UnsignedVector::<u8>::from_str(s).unwrap();
        let v = UnsignedVector::<u8>::from_str(t).unwrap();
        let w = (&u).mod_power_of_2_add_mul_shl(&v, c, bits, pow);
        assert_eq!(w.to_string(), out);
        assert_eq!(
            u.clone()
                .mod_power_of_2_add_mul_shl(v.clone(), c, bits, pow),
            w
        );
        assert_eq!(u.clone().mod_power_of_2_add_mul_shl(&v, c, bits, pow), w);
        let mut x = u.clone();
        x.mod_power_of_2_add_mul_shl_assign(v.clone(), c, bits, pow);
        assert_eq!(x, w);
        let mut x = u;
        x.mod_power_of_2_add_mul_shl_assign(&v, c, bits, pow);
        assert_eq!(x, w);
    };
    test("()", "()", 5, 1, 3, "()");
    test("(0, 0)", "(0, 0)", 0, 0, 0, "(0, 0)");
    test("(5, 1, 3)", "(2, 7, 4)", 0, 1, 4, "(5, 1, 3)");
    test("(5, 1, 3)", "(2, 7, 4)", 3, 0, 4, "(11, 6, 15)");
    test("(5, 1, 3)", "(2, 7, 4)", 3, 1, 4, "(1, 11, 11)");
    test("(5, 1, 3)", "(2, 7, 4)", 3, 4, 4, "(5, 1, 3)");
    test("(255, 1)", "(255, 2)", 255, 3, 8, "(7, 241)");
}

fn mod_power_of_2_add_mul_shl_fail_helper<T: PrimitiveUnsigned>() {
    let v = UnsignedVector::<T> {
        elements: vec![T::ONE],
    };
    let big = UnsignedVector::<T> {
        elements: vec![T::exact_from(8u8)],
    };
    let long = UnsignedVector::<T> {
        elements: vec![T::ONE, T::ONE],
    };
    // An element of either vector is not reduced.
    assert_panic!((&big).mod_power_of_2_add_mul_shl(&v, T::ONE, 1, 3));
    assert_panic!((&v).mod_power_of_2_add_mul_shl(&big, T::ONE, 1, 3));
    // The scalar is not reduced.
    assert_panic!((&v).mod_power_of_2_add_mul_shl(&v, T::exact_from(16u8), 1, 4));
    // The power is too large.
    assert_panic!((&v).mod_power_of_2_add_mul_shl(&v, T::ONE, 1, T::WIDTH + 1));
    // The dimensions differ.
    assert_panic!((&v).mod_power_of_2_add_mul_shl(&long, T::ONE, 1, 3));
    assert_panic!({
        let mut x = big.clone();
        x.mod_power_of_2_add_mul_shl_assign(&v, T::ONE, 1, 3);
    });
}

#[test]
fn mod_power_of_2_add_mul_shl_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_add_mul_shl_fail_helper);
}

fn mod_power_of_2_add_mul_shl_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_vector_unsigned_unsigned_unsigned_quintuple_gen_var_1::<T>()
        .test_properties(|(u, v, c, bits, pow)| {
            let w = (&u).mod_power_of_2_add_mul_shl(&v, c, bits, pow);
            assert!(w.mod_power_of_2_is_reduced(pow));
            // The forms agree.
            assert_eq!(
                u.clone()
                    .mod_power_of_2_add_mul_shl(v.clone(), c, bits, pow),
                w
            );
            assert_eq!(u.clone().mod_power_of_2_add_mul_shl(&v, c, bits, pow), w);
            let mut x = u.clone();
            x.mod_power_of_2_add_mul_shl_assign(v.clone(), c, bits, pow);
            assert_eq!(x, w);
            let mut x = u.clone();
            x.mod_power_of_2_add_mul_shl_assign(&v, c, bits, pow);
            assert_eq!(x, w);

            // Element by element it is the scalar mod_power_of_2_add_mul_shl, and it is
            // `mod_power_of_2_add_mul` by the shifted scalar.
            for ((&x, &y), &z) in u.elements.iter().zip(&v.elements).zip(&w.elements) {
                assert_eq!(z, x.mod_power_of_2_add_mul_shl(y, c, bits, pow));
            }
            assert_eq!(
                (&u).mod_power_of_2_add_mul(&v, c.mod_power_of_2_shl(bits, pow), pow),
                w
            );
            // Negating the scalar gives the opposite operation, which undoes this one.
            assert_eq!(
                (&u).mod_power_of_2_sub_mul_shl(&v, c.mod_power_of_2_neg(pow), bits, pow),
                w
            );
            assert_eq!((&w).mod_power_of_2_sub_mul_shl(&v, c, bits, pow), u);
            // Shifting by at least `pow` changes nothing.
            if bits >= pow {
                assert_eq!(w, u);
            }
        });

    unsigned_vector_unsigned_vector_unsigned_unsigned_quadruple_gen_var_1::<T>().test_properties(
        |(u, v, c, pow)| {
            // With no shift, this is `mod_power_of_2_add_mul`.
            assert_eq!(
                (&u).mod_power_of_2_add_mul_shl(&v, c, 0, pow),
                (&u).mod_power_of_2_add_mul(&v, c, pow)
            );
        },
    );
}

#[test]
fn mod_power_of_2_add_mul_shl_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_add_mul_shl_properties_helper);
}
