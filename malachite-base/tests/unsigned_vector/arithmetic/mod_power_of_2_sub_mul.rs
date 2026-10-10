// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2AddMul, ModPowerOf2IsReduced, ModPowerOf2Mul, ModPowerOf2Neg, ModPowerOf2Sub,
    ModPowerOf2SubMul, ModPowerOf2SubMulAssign,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_vector_unsigned_unsigned_triple_gen_var_1,
    unsigned_vector_unsigned_vector_unsigned_triple_gen_var_1,
    unsigned_vector_unsigned_vector_unsigned_unsigned_quadruple_gen_var_1,
};
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;
use std::panic::catch_unwind;

#[test]
fn test_mod_power_of_2_sub_mul() {
    let test = |s, t, c: u8, pow, out| {
        let u = UnsignedVector::<u8>::from_str(s).unwrap();
        let v = UnsignedVector::<u8>::from_str(t).unwrap();
        let w = (&u).mod_power_of_2_sub_mul(&v, c, pow);
        assert_eq!(w.to_string(), out);
        assert_eq!(u.clone().mod_power_of_2_sub_mul(v.clone(), c, pow), w);
        assert_eq!(u.clone().mod_power_of_2_sub_mul(&v, c, pow), w);
        let mut x = u.clone();
        x.mod_power_of_2_sub_mul_assign(v.clone(), c, pow);
        assert_eq!(x, w);
        let mut x = u;
        x.mod_power_of_2_sub_mul_assign(&v, c, pow);
        assert_eq!(x, w);
    };
    test("()", "()", 5, 3, "()");
    test("(0, 0)", "(0, 0)", 0, 0, "(0, 0)");
    test("(5, 1, 3)", "(2, 7, 4)", 0, 3, "(5, 1, 3)");
    test("(5, 1, 3)", "(2, 7, 4)", 1, 3, "(3, 2, 7)");
    test("(5, 1, 3)", "(2, 7, 4)", 3, 3, "(7, 4, 7)");
    test("(255, 1)", "(255, 2)", 255, 8, "(254, 3)");
    test("(200, 0)", "(100, 255)", 3, 8, "(156, 3)");
}

fn mod_power_of_2_sub_mul_fail_helper<T: PrimitiveUnsigned>() {
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
    assert_panic!((&big).mod_power_of_2_sub_mul(&v, T::ONE, 3));
    assert_panic!((&v).mod_power_of_2_sub_mul(&big, T::ONE, 3));
    // The scalar is not reduced.
    assert_panic!((&v).mod_power_of_2_sub_mul(&v, T::exact_from(16u8), 4));
    // The power is too large.
    assert_panic!((&v).mod_power_of_2_sub_mul(&v, T::ONE, T::WIDTH + 1));
    // The dimensions differ.
    assert_panic!((&v).mod_power_of_2_sub_mul(&long, T::ONE, 3));
    assert_panic!({
        let mut x = big.clone();
        x.mod_power_of_2_sub_mul_assign(&v, T::ONE, 3);
    });
    assert_panic!({
        let mut x = v.clone();
        x.mod_power_of_2_sub_mul_assign(long.clone(), T::ONE, 3);
    });
}

#[test]
fn mod_power_of_2_sub_mul_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_sub_mul_fail_helper);
}

fn mod_power_of_2_sub_mul_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_vector_unsigned_unsigned_quadruple_gen_var_1::<T>().test_properties(
        |(u, v, c, pow)| {
            let w = (&u).mod_power_of_2_sub_mul(&v, c, pow);
            assert!(w.mod_power_of_2_is_reduced(pow));
            // The forms agree.
            assert_eq!(u.clone().mod_power_of_2_sub_mul(v.clone(), c, pow), w);
            assert_eq!(u.clone().mod_power_of_2_sub_mul(&v, c, pow), w);
            let mut x = u.clone();
            x.mod_power_of_2_sub_mul_assign(v.clone(), c, pow);
            assert_eq!(x, w);
            let mut x = u.clone();
            x.mod_power_of_2_sub_mul_assign(&v, c, pow);
            assert_eq!(x, w);

            // It is the unfused combination, element by element the scalar mod_power_of_2_sub_mul,
            // and the dimension is unchanged.
            assert_eq!(
                w,
                (&u).mod_power_of_2_sub(&(&v).mod_power_of_2_mul(c, pow), pow)
            );
            assert_eq!(w.dimension(), u.dimension());
            for ((&x, &y), &z) in u.elements.iter().zip(&v.elements).zip(&w.elements) {
                assert_eq!(z, x.mod_power_of_2_sub_mul(y, c, pow));
            }
            // Negating the scalar gives the opposite operation, which undoes this one.
            assert_eq!(
                (&u).mod_power_of_2_add_mul(&v, c.mod_power_of_2_neg(pow), pow),
                w
            );
            assert_eq!((&w).mod_power_of_2_add_mul(&v, c, pow), u);
            // Negating everything negates the result.
            assert_eq!(
                (&u).mod_power_of_2_neg(pow).mod_power_of_2_sub_mul(
                    (&v).mod_power_of_2_neg(pow),
                    c,
                    pow
                ),
                (&w).mod_power_of_2_neg(pow)
            );
        },
    );

    unsigned_vector_unsigned_vector_unsigned_triple_gen_var_1::<T>().test_properties(
        |(u, v, pow)| {
            // A zero scalar changes nothing, and a scalar of 1 leaves the difference.
            assert_eq!((&u).mod_power_of_2_sub_mul(&v, T::ZERO, pow), u);
            if pow != 0 {
                assert_eq!(
                    (&u).mod_power_of_2_sub_mul(&v, T::ONE, pow),
                    (&u).mod_power_of_2_sub(&v, pow)
                );
            }
        },
    );

    unsigned_vector_unsigned_unsigned_triple_gen_var_1::<T>().test_properties(|(v, c, pow)| {
        // Starting from the zero vector gives the scalar product, negated.
        assert_eq!(
            UnsignedVector::zero(v.dimension()).mod_power_of_2_sub_mul(&v, c, pow),
            (&v).mod_power_of_2_mul(c, pow).mod_power_of_2_neg(pow)
        );
    });
}

#[test]
fn mod_power_of_2_sub_mul_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_sub_mul_properties_helper);
}
