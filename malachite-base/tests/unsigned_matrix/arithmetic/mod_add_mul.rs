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
    ModAdd, ModAddMul, ModAddMulAssign, ModIsReduced, ModMul, ModNeg, ModSubMul,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_11,
    unsigned_matrix_unsigned_matrix_unsigned_unsigned_quadruple_gen_var_3,
    unsigned_matrix_unsigned_unsigned_triple_gen_var_11,
};
use malachite_base::unsigned_matrix::UnsignedMatrix;
use std::panic::catch_unwind;

#[test]
fn test_mod_add_mul() {
    let test = |s, t, c: u8, m: u8, out| {
        let u = UnsignedMatrix::<u8>::from_str(s).unwrap();
        let v = UnsignedMatrix::<u8>::from_str(t).unwrap();
        let b = (&u).mod_add_mul(&v, c, m);
        assert_eq!(b.to_string(), out);
        assert_eq!(u.clone().mod_add_mul(v.clone(), c, m), b);
        assert_eq!(u.clone().mod_add_mul(&v, c, m), b);
        let mut x = u.clone();
        x.mod_add_mul_assign(v.clone(), c, m);
        assert_eq!(x, b);
        let mut x = u;
        x.mod_add_mul_assign(&v, c, m);
        assert_eq!(x, b);
    };
    test("[]_{0,0}", "[]_{0,0}", 5, 7, "[]_{0,0}");
    test("[0, 0]", "[0, 0]", 0, 1, "[0, 0]");
    test("[5, 1, 3]", "[2, 6, 4]", 0, 7, "[5, 1, 3]");
    test("[5, 1, 3]", "[2, 6, 4]", 1, 7, "[0, 0, 0]");
    test("[5, 1, 3]", "[2, 6, 4]", 6, 7, "[3, 2, 6]");
    test("[5, 1, 3]", "[2, 6, 4]", 3, 7, "[4, 5, 1]");
    test("[254, 1]", "[254, 253]", 254, 255, "[0, 3]");
    test("[200, 0]", "[100, 254]", 3, 255, "[245, 252]");
}

fn mod_add_mul_fail_helper<T: PrimitiveUnsigned>() {
    let seven = T::exact_from(7u8);
    let v = UnsignedMatrix::<T>::from_owned_elements(1, 1, vec![T::ONE]);
    let big = UnsignedMatrix::<T>::from_owned_elements(1, 1, vec![seven]);
    let long = UnsignedMatrix::<T>::from_owned_elements(1, 2, vec![T::ONE, T::ONE]);
    // An element of either matrix is not reduced.
    assert_panic!((&big).mod_add_mul(&v, T::ONE, seven));
    assert_panic!((&v).mod_add_mul(&big, T::ONE, seven));
    // The scalar is not reduced.
    assert_panic!((&v).mod_add_mul(&v, seven, seven));
    // The modulus is zero.
    assert_panic!((&v).mod_add_mul(&v, T::ZERO, T::ZERO));
    // The dimensions differ.
    assert_panic!((&v).mod_add_mul(&long, T::ONE, seven));
    assert_panic!({
        let mut x = big.clone();
        x.mod_add_mul_assign(&v, T::ONE, seven);
    });
    assert_panic!({
        let mut x = v.clone();
        x.mod_add_mul_assign(long.clone(), T::ONE, seven);
    });
}

#[test]
fn mod_add_mul_fail() {
    apply_fn_to_unsigneds!(mod_add_mul_fail_helper);
}

fn mod_add_mul_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_matrix_unsigned_matrix_unsigned_unsigned_quadruple_gen_var_3::<T>().test_properties(
        |(u, v, c, m)| {
            let b = (&u).mod_add_mul(&v, c, m);
            assert!(b.mod_is_reduced(&m));
            // The forms agree.
            assert_eq!(u.clone().mod_add_mul(v.clone(), c, m), b);
            assert_eq!(u.clone().mod_add_mul(&v, c, m), b);
            let mut x = u.clone();
            x.mod_add_mul_assign(v.clone(), c, m);
            assert_eq!(x, b);
            let mut x = u.clone();
            x.mod_add_mul_assign(&v, c, m);
            assert_eq!(x, b);

            // It is the unfused combination, element by element the scalar mod_add_mul, and the
            // shape is unchanged.
            assert_eq!(b, (&u).mod_add(&(&v).mod_mul(c, m), m));
            assert_eq!((b.height(), b.width()), (u.height(), u.width()));
            for ((&x, &y), &z) in u
                .elements_ref()
                .iter()
                .zip(v.elements_ref())
                .zip(b.elements_ref())
            {
                assert_eq!(z, x.mod_add_mul(y, c, m));
            }
            // Negating the scalar gives the opposite operation, which undoes this one.
            assert_eq!((&u).mod_sub_mul(&v, c.mod_neg(m), m), b);
            assert_eq!((&b).mod_sub_mul(&v, c, m), u);
            // Negating everything negates the result.
            assert_eq!(
                (&u).mod_neg(m).mod_add_mul((&v).mod_neg(m), c, m),
                (&b).mod_neg(m)
            );
        },
    );

    unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_11::<T>().test_properties(
        |(u, v, m)| {
            // A zero scalar changes nothing, and a scalar of 1 leaves the sum.
            assert_eq!((&u).mod_add_mul(&v, T::ZERO, m), u);
            if m != T::ONE {
                assert_eq!((&u).mod_add_mul(&v, T::ONE, m), (&u).mod_add(&v, m));
            }
        },
    );

    unsigned_matrix_unsigned_unsigned_triple_gen_var_11::<T>().test_properties(|(v, c, m)| {
        // Starting from the zero matrix gives the scalar product.
        assert_eq!(
            UnsignedMatrix::zero(v.height(), v.width()).mod_add_mul(&v, c, m),
            (&v).mod_mul(c, m)
        );
    });
}

#[test]
fn mod_add_mul_properties() {
    apply_fn_to_unsigneds!(mod_add_mul_properties_helper);
}
