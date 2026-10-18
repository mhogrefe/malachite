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
    ModPowerOf2Add, ModPowerOf2Dot, ModPowerOf2IsReduced, ModPowerOf2Mul,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::{
    unsigned_matrix_unsigned_pair_gen_var_4,
    unsigned_matrix_unsigned_vector_unsigned_triple_gen_var_1,
    unsigned_vector_unsigned_pair_gen_var_4,
};
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_power_of_2_mul_vector::*;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;
use std::panic::catch_unwind;

#[test]
fn test_mod_power_of_2_mul_vector() {
    let test = |s, t, pow, out| {
        let a = UnsignedMatrix::<u8>::from_str(s).unwrap();
        let v = UnsignedVector::<u8>::from_str(t).unwrap();
        let w = (&a).mod_power_of_2_mul(&v, pow);
        assert_eq!(w.to_string(), out);
        assert_eq!(a.clone().mod_power_of_2_mul(v.clone(), pow), w);
        assert_eq!(a.clone().mod_power_of_2_mul(&v, pow), w);
        assert_eq!((&a).mod_power_of_2_mul(v, pow), w);
    };
    test("[]_{0,0}", "()", 3, "()");
    test("[]_{2,0}", "()", 3, "(0, 0)");
    test("[]_{0,3}", "(1, 2, 3)", 2, "()");
    test("[0, 0]", "(0, 0)", 0, "(0)");
    test("[5]", "(3)", 3, "(7)");
    test("[1, 2, 3; 4, 5, 6]", "(1, 0, 2)", 3, "(7, 0)");
    test("[1, 2, 3; 4, 5, 6]", "(1, 0, 2)", 8, "(7, 16)");
    test("[255, 255]", "(255, 255)", 8, "(2)");
}

fn mod_power_of_2_mul_vector_fail_helper<T: PrimitiveUnsigned>() {
    let a = UnsignedMatrix::<T>::from_str("[1, 2; 3, 4]").unwrap();
    // The vector's dimension is not the width of the matrix.
    assert_panic!((&a).mod_power_of_2_mul(&UnsignedVector::<T>::from_str("(1)").unwrap(), 3));
    assert_panic!(
        UnsignedMatrix::<T>::from_str("[]_{2,0}")
            .unwrap()
            .mod_power_of_2_mul(UnsignedVector::<T>::from_str("(1)").unwrap(), 3)
    );
    // An element of the matrix is not reduced.
    assert_panic!((&a).mod_power_of_2_mul(&UnsignedVector::<T>::from_str("(1, 1)").unwrap(), 2));
    // An element of the vector is not reduced.
    assert_panic!((&a).mod_power_of_2_mul(&UnsignedVector::<T>::from_str("(1, 8)").unwrap(), 3));
    // The power is too large.
    assert_panic!((&a).mod_power_of_2_mul(
        &UnsignedVector::<T>::from_str("(1, 1)").unwrap(),
        T::WIDTH + 1
    ));
}

#[test]
fn mod_power_of_2_mul_vector_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_mul_vector_fail_helper);
}

#[test]
fn mod_power_of_2_mul_vector_properties() {
    unsigned_matrix_unsigned_vector_unsigned_triple_gen_var_1().test_properties(|(a, v, pow)| {
        let w = (&a).mod_power_of_2_mul(&v, pow);
        // The forms agree.
        assert_eq!(a.clone().mod_power_of_2_mul(v.clone(), pow), w);
        assert_eq!(a.clone().mod_power_of_2_mul(&v, pow), w);
        assert_eq!((&a).mod_power_of_2_mul(v.clone(), pow), w);
        assert!(w.mod_power_of_2_is_reduced(pow));
        assert_eq!(w.dimension(), a.height());
        assert_eq!(
            unsigned_matrix_mod_power_of_2_mul_vector_naive(&a, &v, pow),
            w
        );
        // Element `i` is the dot product of row `i` with the vector.
        for i in 0..a.height() {
            assert_eq!(
                w[usize::exact_from(i)],
                a.row(i).mod_power_of_2_dot(&v, pow)
            );
        }
        // Multiplying on the right by the matrix is multiplying on the left by its transpose.
        assert_eq!((&v).mod_power_of_2_mul(&a.transpose(), pow), w);
        // The product is linear in the matrix.
        assert_eq!(
            (&a).mod_power_of_2_add(&a, pow).mod_power_of_2_mul(&v, pow),
            (&w).mod_power_of_2_add(&w, pow)
        );
    });

    unsigned_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        // The zero vector gives the zero vector, and (when 1 is reduced) a standard basis vector
        // picks out a column.
        assert_eq!(
            (&a).mod_power_of_2_mul(&UnsignedVector::<u64>::zero(a.width()), pow),
            UnsignedVector::<u64>::zero(a.height())
        );
        if pow != 0 {
            for j in 0..a.width() {
                assert_eq!(
                    (&a).mod_power_of_2_mul(
                        &UnsignedVector::<u64>::standard_basis_vector(a.width(), j),
                        pow
                    ),
                    a.column(j)
                );
            }
        }
    });

    unsigned_vector_unsigned_pair_gen_var_4::<u64>().test_properties(|(v, pow)| {
        // The identity matrix (when 1 is reduced) leaves a vector unchanged.
        if pow != 0 {
            assert_eq!(
                UnsignedMatrix::<u64>::identity(v.dimension()).mod_power_of_2_mul(&v, pow),
                v
            );
        }
    });
}
