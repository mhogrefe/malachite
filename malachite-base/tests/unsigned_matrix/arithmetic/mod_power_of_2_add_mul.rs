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
    ModPowerOf2Add, ModPowerOf2AddMul, ModPowerOf2AddMulAssign, ModPowerOf2IsReduced,
    ModPowerOf2Mul, ModPowerOf2Neg, ModPowerOf2SubMul,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_10,
    unsigned_matrix_unsigned_matrix_unsigned_unsigned_quadruple_gen_var_2,
    unsigned_matrix_unsigned_unsigned_triple_gen_var_10,
};
use malachite_base::unsigned_matrix::UnsignedMatrix;
use std::panic::catch_unwind;

#[test]
fn test_mod_power_of_2_add_mul() {
    let test = |s, t, c: u8, pow, out| {
        let u = UnsignedMatrix::<u8>::from_str(s).unwrap();
        let v = UnsignedMatrix::<u8>::from_str(t).unwrap();
        let b = (&u).mod_power_of_2_add_mul(&v, c, pow);
        assert_eq!(b.to_string(), out);
        assert_eq!(u.clone().mod_power_of_2_add_mul(v.clone(), c, pow), b);
        assert_eq!(u.clone().mod_power_of_2_add_mul(&v, c, pow), b);
        let mut x = u.clone();
        x.mod_power_of_2_add_mul_assign(v.clone(), c, pow);
        assert_eq!(x, b);
        let mut x = u;
        x.mod_power_of_2_add_mul_assign(&v, c, pow);
        assert_eq!(x, b);
    };
    test("[]_{0,0}", "[]_{0,0}", 5, 3, "[]_{0,0}");
    test("[0, 0]", "[0, 0]", 0, 0, "[0, 0]");
    test("[5, 1, 3]", "[2, 7, 4]", 0, 3, "[5, 1, 3]");
    test("[5, 1, 3]", "[2, 7, 4]", 1, 3, "[7, 0, 7]");
    test("[5, 1, 3]", "[2, 7, 4]", 3, 3, "[3, 6, 7]");
    test("[255, 1]", "[255, 2]", 255, 8, "[0, 255]");
    test("[200, 0]", "[100, 255]", 3, 8, "[244, 253]");
}

fn mod_power_of_2_add_mul_fail_helper<T: PrimitiveUnsigned>() {
    let v = UnsignedMatrix::<T>::from_owned_elements(1, 1, vec![T::ONE]);
    let big = UnsignedMatrix::<T>::from_owned_elements(1, 1, vec![T::exact_from(8u8)]);
    let long = UnsignedMatrix::<T>::from_owned_elements(1, 2, vec![T::ONE, T::ONE]);
    // An element of either matrix is not reduced.
    assert_panic!((&big).mod_power_of_2_add_mul(&v, T::ONE, 3));
    assert_panic!((&v).mod_power_of_2_add_mul(&big, T::ONE, 3));
    // The scalar is not reduced.
    assert_panic!((&v).mod_power_of_2_add_mul(&v, T::exact_from(16u8), 4));
    // The power is too large.
    assert_panic!((&v).mod_power_of_2_add_mul(&v, T::ONE, T::WIDTH + 1));
    // The dimensions differ.
    assert_panic!((&v).mod_power_of_2_add_mul(&long, T::ONE, 3));
    assert_panic!({
        let mut x = big.clone();
        x.mod_power_of_2_add_mul_assign(&v, T::ONE, 3);
    });
    assert_panic!({
        let mut x = v.clone();
        x.mod_power_of_2_add_mul_assign(long.clone(), T::ONE, 3);
    });
}

#[test]
fn mod_power_of_2_add_mul_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_add_mul_fail_helper);
}

fn mod_power_of_2_add_mul_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_matrix_unsigned_matrix_unsigned_unsigned_quadruple_gen_var_2::<T>().test_properties(
        |(u, v, c, pow)| {
            let b = (&u).mod_power_of_2_add_mul(&v, c, pow);
            assert!(b.mod_power_of_2_is_reduced(pow));
            // The forms agree.
            assert_eq!(u.clone().mod_power_of_2_add_mul(v.clone(), c, pow), b);
            assert_eq!(u.clone().mod_power_of_2_add_mul(&v, c, pow), b);
            let mut x = u.clone();
            x.mod_power_of_2_add_mul_assign(v.clone(), c, pow);
            assert_eq!(x, b);
            let mut x = u.clone();
            x.mod_power_of_2_add_mul_assign(&v, c, pow);
            assert_eq!(x, b);

            // It is the unfused combination, element by element the scalar mod_power_of_2_add_mul,
            // and the shape is unchanged.
            assert_eq!(
                b,
                (&u).mod_power_of_2_add(&(&v).mod_power_of_2_mul(c, pow), pow)
            );
            assert_eq!((b.height(), b.width()), (u.height(), u.width()));
            for ((&x, &y), &z) in u
                .elements_ref()
                .iter()
                .zip(v.elements_ref())
                .zip(b.elements_ref())
            {
                assert_eq!(z, x.mod_power_of_2_add_mul(y, c, pow));
            }
            // Negating the scalar gives the opposite operation, which undoes this one.
            assert_eq!(
                (&u).mod_power_of_2_sub_mul(&v, c.mod_power_of_2_neg(pow), pow),
                b
            );
            assert_eq!((&b).mod_power_of_2_sub_mul(&v, c, pow), u);
            // Negating everything negates the result.
            assert_eq!(
                (&u).mod_power_of_2_neg(pow).mod_power_of_2_add_mul(
                    (&v).mod_power_of_2_neg(pow),
                    c,
                    pow
                ),
                (&b).mod_power_of_2_neg(pow)
            );
        },
    );

    unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_10::<T>().test_properties(
        |(u, v, pow)| {
            // A zero scalar changes nothing, and a scalar of 1 leaves the sum.
            assert_eq!((&u).mod_power_of_2_add_mul(&v, T::ZERO, pow), u);
            if pow != 0 {
                assert_eq!(
                    (&u).mod_power_of_2_add_mul(&v, T::ONE, pow),
                    (&u).mod_power_of_2_add(&v, pow)
                );
            }
        },
    );

    unsigned_matrix_unsigned_unsigned_triple_gen_var_10::<T>().test_properties(|(v, c, pow)| {
        // Starting from the zero matrix gives the scalar product.
        assert_eq!(
            UnsignedMatrix::zero(v.height(), v.width()).mod_power_of_2_add_mul(&v, c, pow),
            (&v).mod_power_of_2_mul(c, pow)
        );
    });
}

#[test]
fn mod_power_of_2_add_mul_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_add_mul_properties_helper);
}
