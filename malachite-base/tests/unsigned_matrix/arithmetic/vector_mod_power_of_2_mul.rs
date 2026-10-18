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
    unsigned_matrix_unsigned_vector_unsigned_triple_gen_var_2,
    unsigned_vector_unsigned_pair_gen_var_4,
};
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_power_of_2_mul_vector::*;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;
use std::panic::catch_unwind;

#[test]
fn test_vector_mod_power_of_2_mul() {
    let test = |s, t, pow, out| {
        let v = UnsignedVector::<u8>::from_str(s).unwrap();
        let a = UnsignedMatrix::<u8>::from_str(t).unwrap();
        let w = (&v).mod_power_of_2_mul(&a, pow);
        assert_eq!(w.to_string(), out);
        assert_eq!(v.clone().mod_power_of_2_mul(a.clone(), pow), w);
        assert_eq!(v.clone().mod_power_of_2_mul(&a, pow), w);
        assert_eq!((&v).mod_power_of_2_mul(a, pow), w);
    };
    test("()", "[]_{0,0}", 3, "()");
    test("()", "[]_{0,2}", 3, "(0, 0)");
    test("(1, 2, 3)", "[]_{3,0}", 2, "()");
    test("(0, 0)", "[0; 0]", 0, "(0)");
    test("(3)", "[5]", 3, "(7)");
    test("(1, 2)", "[1, 2, 3; 4, 5, 6]", 3, "(1, 4, 7)");
    test("(1, 2)", "[1, 2, 3; 4, 5, 6]", 8, "(9, 12, 15)");
    test("(255, 255)", "[255; 255]", 8, "(2)");
}

fn vector_mod_power_of_2_mul_fail_helper<T: PrimitiveUnsigned>() {
    let a = UnsignedMatrix::<T>::from_str("[1, 2; 3, 4]").unwrap();
    // The vector's dimension is not the height of the matrix.
    assert_panic!(
        UnsignedVector::<T>::from_str("(1)")
            .unwrap()
            .mod_power_of_2_mul(&a, 3)
    );
    assert_panic!(
        UnsignedVector::<T>::from_str("(1)")
            .unwrap()
            .mod_power_of_2_mul(UnsignedMatrix::<T>::from_str("[]_{0,2}").unwrap(), 3)
    );
    // An element of the matrix is not reduced.
    assert_panic!(
        UnsignedVector::<T>::from_str("(1, 1)")
            .unwrap()
            .mod_power_of_2_mul(&a, 2)
    );
    // An element of the vector is not reduced.
    assert_panic!(
        UnsignedVector::<T>::from_str("(1, 8)")
            .unwrap()
            .mod_power_of_2_mul(&a, 3)
    );
    // The power is too large.
    assert_panic!(
        UnsignedVector::<T>::from_str("(1, 1)")
            .unwrap()
            .mod_power_of_2_mul(&a, T::WIDTH + 1)
    );
}

#[test]
fn vector_mod_power_of_2_mul_fail() {
    apply_fn_to_unsigneds!(vector_mod_power_of_2_mul_fail_helper);
}

#[test]
fn vector_mod_power_of_2_mul_properties() {
    unsigned_matrix_unsigned_vector_unsigned_triple_gen_var_2().test_properties(|(a, v, pow)| {
        let w = (&v).mod_power_of_2_mul(&a, pow);
        // The forms agree.
        assert_eq!(v.clone().mod_power_of_2_mul(a.clone(), pow), w);
        assert_eq!(v.clone().mod_power_of_2_mul(&a, pow), w);
        assert_eq!((&v).mod_power_of_2_mul(a.clone(), pow), w);
        assert!(w.mod_power_of_2_is_reduced(pow));
        assert_eq!(w.dimension(), a.width());
        assert_eq!(
            unsigned_vector_mod_power_of_2_mul_unsigned_matrix_naive(&v, &a, pow),
            w
        );
        // Element `j` is the dot product of the vector with column `j`.
        for j in 0..a.width() {
            assert_eq!(
                w[usize::exact_from(j)],
                (&v).mod_power_of_2_dot(&a.column(j), pow)
            );
        }
        // Multiplying on the left by the matrix is multiplying on the right by its transpose.
        assert_eq!(a.transpose().mod_power_of_2_mul(&v, pow), w);
        // The product is linear in the vector.
        assert_eq!(
            (&v).mod_power_of_2_add(&v, pow).mod_power_of_2_mul(&a, pow),
            (&w).mod_power_of_2_add(&w, pow)
        );
    });

    unsigned_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        // The zero vector gives the zero vector, and (when 1 is reduced) a standard basis vector
        // picks out a row.
        assert_eq!(
            UnsignedVector::<u64>::zero(a.height()).mod_power_of_2_mul(&a, pow),
            UnsignedVector::<u64>::zero(a.width())
        );
        if pow != 0 {
            for i in 0..a.height() {
                assert_eq!(
                    UnsignedVector::<u64>::standard_basis_vector(a.height(), i)
                        .mod_power_of_2_mul(&a, pow),
                    a.row(i)
                );
            }
        }
    });

    unsigned_vector_unsigned_pair_gen_var_4::<u64>().test_properties(|(v, pow)| {
        // The identity matrix (when 1 is reduced) leaves a vector unchanged.
        if pow != 0 {
            assert_eq!(
                (&v).mod_power_of_2_mul(UnsignedMatrix::<u64>::identity(v.dimension()), pow),
                v
            );
        }
    });
}
