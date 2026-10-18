// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{Mod, ModAdd, ModDot, ModIsReduced, ModMul};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::*;
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::{
    natural_matrix_natural_pair_gen_var_2, natural_matrix_natural_vector_natural_triple_gen_var_2,
    natural_vector_natural_pair_gen_var_2,
};
use malachite_nz::test_util::natural_matrix::arithmetic::mod_mul_vector::*;

#[test]
fn test_vector_mod_mul() {
    let test = |s, t, m, out| {
        let v = NaturalVector::from_str(s).unwrap();
        let a = NaturalMatrix::from_str(t).unwrap();
        let m = Natural::from_str(m).unwrap();
        let w = (&v).mod_mul(&a, &m);
        assert_eq!(w.to_string(), out);
        // All eight combinations of value and reference.
        assert_eq!((&v).mod_mul(&a, m.clone()), w);
        assert_eq!((&v).mod_mul(a.clone(), &m), w);
        assert_eq!((&v).mod_mul(a.clone(), m.clone()), w);
        assert_eq!(v.clone().mod_mul(&a, &m), w);
        assert_eq!(v.clone().mod_mul(&a, m.clone()), w);
        assert_eq!(v.clone().mod_mul(a.clone(), &m), w);
        assert_eq!(v.mod_mul(a, m), w);
    };
    test("()", "[]_{0,0}", "3", "()");
    test("()", "[]_{0,2}", "3", "(0, 0)");
    test("(1, 2, 0)", "[]_{3,0}", "3", "()");
    test("(0, 0)", "[0; 0]", "1", "(0)");
    test("(3)", "[5]", "7", "(1)");
    test("(1, 2)", "[1, 2, 3; 4, 5, 6]", "7", "(2, 5, 1)");
    test("(1, 2)", "[1, 2, 3; 4, 5, 6]", "100", "(9, 12, 15)");
    test(
        "(18446744073709551615, 1)",
        "[18446744073709551615; 18446744073709551615]",
        "18446744073709551616",
        "(0)",
    );
    test(
        "(3, 18446744073709551556)",
        "[18446744073709551556; 5]",
        "18446744073709551557",
        "(18446744073709551549)",
    );
    test(
        "(1000000000000000000000000, 1)",
        "[1000000000000000000000000; 1]",
        "1000000000000000000000000000000000000000007",
        "(999999999999999999999999999999999993000008)",
    );
}

#[test]
#[should_panic]
fn vector_mod_mul_fail_1() {
    // The vector's dimension is not the height of the matrix.
    let _ = NaturalVector::from_str("(1)").unwrap().mod_mul(
        &NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap(),
        Natural::from(5u32),
    );
}

#[test]
#[should_panic]
fn vector_mod_mul_fail_2() {
    // The vector's dimension is not the height of the matrix.
    let _ = (&NaturalVector::from_str("(1)").unwrap()).mod_mul(
        NaturalMatrix::from_str("[]_{0,2}").unwrap(),
        &Natural::from(5u32),
    );
}

#[test]
#[should_panic]
fn vector_mod_mul_fail_3() {
    // An element of the matrix is not reduced.
    let _ = NaturalVector::from_str("(1, 1)").unwrap().mod_mul(
        NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap(),
        Natural::from(4u32),
    );
}

#[test]
#[should_panic]
fn vector_mod_mul_fail_4() {
    // An element of the vector is not reduced.
    let _ = (&NaturalVector::from_str("(1, 8)").unwrap()).mod_mul(
        &NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap(),
        &Natural::from(5u32),
    );
}

#[test]
#[should_panic]
fn vector_mod_mul_fail_5() {
    // The modulus is zero.
    let _ = NaturalVector::from_str("()")
        .unwrap()
        .mod_mul(NaturalMatrix::from_str("[]_{0,0}").unwrap(), Natural::ZERO);
}

#[test]
fn vector_mod_mul_properties() {
    natural_matrix_natural_vector_natural_triple_gen_var_2().test_properties(|(a, v, m)| {
        let w = (&v).mod_mul(&a, &m);
        // The forms agree.
        assert_eq!((&v).mod_mul(&a, m.clone()), w);
        assert_eq!((&v).mod_mul(a.clone(), &m), w);
        assert_eq!((&v).mod_mul(a.clone(), m.clone()), w);
        assert_eq!(v.clone().mod_mul(&a, &m), w);
        assert_eq!(v.clone().mod_mul(&a, m.clone()), w);
        assert_eq!(v.clone().mod_mul(a.clone(), &m), w);
        assert_eq!(v.clone().mod_mul(a.clone(), m.clone()), w);
        assert!(w.mod_is_reduced(&m));
        assert_eq!(w.dimension(), a.width());
        assert_eq!(natural_vector_mod_mul_natural_matrix_naive(&v, &a, &m), w);
        assert_eq!((&v * &a).mod_op(&m), w);
        // Element `j` is the dot product of the vector with column `j`.
        for j in 0..a.width() {
            assert_eq!(w[usize::exact_from(j)], (&v).mod_dot(&a.column(j), &m));
        }
        // Multiplying on the left by the matrix is multiplying on the right by its transpose.
        assert_eq!(a.transpose().mod_mul(&v, &m), w);
        // The product is linear in the vector.
        assert_eq!((&v).mod_add(&v, &m).mod_mul(&a, &m), (&w).mod_add(&w, &m));
    });

    natural_matrix_natural_pair_gen_var_2().test_properties(|(a, m)| {
        // The zero vector gives the zero vector, and (when 1 is reduced) a standard basis vector
        // picks out a row.
        assert_eq!(
            NaturalVector::zero(a.height()).mod_mul(&a, &m),
            NaturalVector::zero(a.width())
        );
        if m != 1u32 {
            for i in 0..a.height() {
                assert_eq!(
                    NaturalVector::standard_basis_vector(a.height(), i).mod_mul(&a, &m),
                    a.row(i)
                );
            }
        }
    });

    natural_vector_natural_pair_gen_var_2().test_properties(|(v, m)| {
        // The identity matrix (when 1 is reduced) leaves a vector unchanged.
        if m != 1u32 {
            assert_eq!((&v).mod_mul(NaturalMatrix::identity(v.dimension()), &m), v);
        }
    });

    unsigned_matrix_unsigned_vector_unsigned_triple_gen_var_4().test_properties(|(a, v, m)| {
        // The `u64` elements multiply as their `Natural` counterparts do.
        assert_eq!(
            NaturalVector::from(v.clone())
                .mod_mul(NaturalMatrix::from(a.clone()), Natural::from(m)),
            NaturalVector::from(v.mod_mul(a, m))
        );
    });
}
