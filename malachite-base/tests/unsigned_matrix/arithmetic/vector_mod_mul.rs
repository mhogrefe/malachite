// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModAdd, ModDot, ModIsReduced, ModMul};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::{
    unsigned_matrix_unsigned_pair_gen_var_5,
    unsigned_matrix_unsigned_vector_unsigned_triple_gen_var_4,
    unsigned_vector_unsigned_pair_gen_var_5,
};
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_mul_vector::*;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;
use std::panic::catch_unwind;

#[test]
fn test_vector_mod_mul() {
    let test = |s, t, m: u8, out| {
        let v = UnsignedVector::<u8>::from_str(s).unwrap();
        let a = UnsignedMatrix::<u8>::from_str(t).unwrap();
        let w = (&v).mod_mul(&a, m);
        assert_eq!(w.to_string(), out);
        assert_eq!(v.clone().mod_mul(a.clone(), m), w);
        assert_eq!(v.clone().mod_mul(&a, m), w);
        assert_eq!((&v).mod_mul(a, m), w);
    };
    test("()", "[]_{0,0}", 3, "()");
    test("()", "[]_{0,2}", 3, "(0, 0)");
    test("(1, 2, 0)", "[]_{3,0}", 3, "()");
    test("(0, 0)", "[0; 0]", 1, "(0)");
    test("(3)", "[5]", 7, "(1)");
    test("(1, 2)", "[1, 2, 3; 4, 5, 6]", 7, "(2, 5, 1)");
    test("(1, 2)", "[1, 2, 3; 4, 5, 6]", 100, "(9, 12, 15)");
    test("(254, 254)", "[254; 254]", 255, "(2)");
    test("(201, 202, 203)", "[200; 100; 50]", 251, "(19)");
}

// With `u8` elements and a modulus close to 2^8, the column sums need three words, and with more
// than 128 rows the accumulators are reduced partway through.
#[test]
fn test_vector_mod_mul_many_rows() {
    let test = |x: &str, y: &str, n: usize, m: u8, out| {
        let v = UnsignedVector::<u8>::from_str(&format!("({})", vec![x; n].join(", "))).unwrap();
        let a = UnsignedMatrix::<u8>::from_str(&format!("[{}]", vec![y; n].join("; "))).unwrap();
        let w = (&v).mod_mul(&a, m);
        assert_eq!(w.to_string(), out);
        assert_eq!(unsigned_vector_mod_mul_unsigned_matrix_naive(&v, &a, m), w);
    };
    test("254", "254", 200, 255, "(200)");
    test("7", "9", 300, 251, "(75)");
}

fn vector_mod_mul_fail_helper<T: PrimitiveUnsigned>() {
    let a = UnsignedMatrix::<T>::from_str("[1, 2; 3, 4]").unwrap();
    // The vector's dimension is not the height of the matrix.
    assert_panic!(
        UnsignedVector::<T>::from_str("(1)")
            .unwrap()
            .mod_mul(&a, T::exact_from(5u8))
    );
    assert_panic!(UnsignedVector::<T>::from_str("(1)").unwrap().mod_mul(
        UnsignedMatrix::<T>::from_str("[]_{0,2}").unwrap(),
        T::exact_from(5u8)
    ));
    // An element of the matrix is not reduced.
    assert_panic!(
        UnsignedVector::<T>::from_str("(1, 1)")
            .unwrap()
            .mod_mul(&a, T::exact_from(4u8))
    );
    // An element of the vector is not reduced.
    assert_panic!(
        UnsignedVector::<T>::from_str("(1, 8)")
            .unwrap()
            .mod_mul(&a, T::exact_from(5u8))
    );
    // The modulus is zero.
    assert_panic!(
        UnsignedVector::<T>::from_str("()")
            .unwrap()
            .mod_mul(UnsignedMatrix::<T>::from_str("[]_{0,0}").unwrap(), T::ZERO)
    );
}

#[test]
fn vector_mod_mul_fail() {
    apply_fn_to_unsigneds!(vector_mod_mul_fail_helper);
}

#[test]
fn vector_mod_mul_properties() {
    unsigned_matrix_unsigned_vector_unsigned_triple_gen_var_4().test_properties(|(a, v, m)| {
        let w = (&v).mod_mul(&a, m);
        // The forms agree.
        assert_eq!(v.clone().mod_mul(a.clone(), m), w);
        assert_eq!(v.clone().mod_mul(&a, m), w);
        assert_eq!((&v).mod_mul(a.clone(), m), w);
        assert!(w.mod_is_reduced(&m));
        assert_eq!(w.dimension(), a.width());
        assert_eq!(unsigned_vector_mod_mul_unsigned_matrix_naive(&v, &a, m), w);
        // Element `j` is the dot product of the vector with column `j`.
        for j in 0..a.width() {
            assert_eq!(w[usize::exact_from(j)], (&v).mod_dot(&a.column(j), m));
        }
        // Multiplying on the left by the matrix is multiplying on the right by its transpose.
        assert_eq!(a.transpose().mod_mul(&v, m), w);
        // The product is linear in the vector.
        assert_eq!((&v).mod_add(&v, m).mod_mul(&a, m), (&w).mod_add(&w, m));
    });

    unsigned_matrix_unsigned_pair_gen_var_5().test_properties(|(a, m)| {
        // The zero vector gives the zero vector, and (when 1 is reduced) a standard basis vector
        // picks out a row.
        assert_eq!(
            UnsignedVector::<u64>::zero(a.height()).mod_mul(&a, m),
            UnsignedVector::<u64>::zero(a.width())
        );
        if m != 1 {
            for i in 0..a.height() {
                assert_eq!(
                    UnsignedVector::<u64>::standard_basis_vector(a.height(), i).mod_mul(&a, m),
                    a.row(i)
                );
            }
        }
    });

    unsigned_vector_unsigned_pair_gen_var_5::<u64>().test_properties(|(v, m)| {
        // The identity matrix (when 1 is reduced) leaves a vector unchanged.
        if m != 1 {
            assert_eq!(
                (&v).mod_mul(UnsignedMatrix::<u64>::identity(v.dimension()), m),
                v
            );
        }
    });
}
