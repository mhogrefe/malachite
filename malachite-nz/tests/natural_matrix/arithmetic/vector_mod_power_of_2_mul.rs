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
    ModPowerOf2, ModPowerOf2Add, ModPowerOf2Dot, ModPowerOf2IsReduced, ModPowerOf2Mul,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::*;
use malachite_base::vector::Vector;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::{
    natural_matrix_natural_vector_unsigned_triple_gen_var_2,
    natural_matrix_unsigned_pair_gen_var_4, natural_vector_unsigned_pair_gen_var_3,
};
use malachite_nz::test_util::natural_matrix::arithmetic::mod_power_of_2_mul_vector::*;

#[test]
fn test_vector_mod_power_of_2_mul() {
    let test = |s, t, pow, out| {
        let v = NaturalVector::from_str(s).unwrap();
        let a = NaturalMatrix::from_str(t).unwrap();
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
    test(
        "(18446744073709551615, 1)",
        "[18446744073709551615; 18446744073709551615]",
        64,
        "(0)",
    );
    test(
        "(18446744073709551615, 1)",
        "[18446744073709551615; 18446744073709551615]",
        65,
        "(18446744073709551616)",
    );
    test(
        "(1000000000000, 1)",
        "[1000000000000; 1]",
        100,
        "(1000000000000000000000001)",
    );
    test(
        "(1000000000000, 1)",
        "[1000000000000; 1]",
        70,
        "(38897252352625999873)",
    );
}

#[test]
#[should_panic]
fn vector_mod_power_of_2_mul_fail_1() {
    // The vector's dimension is not the height of the matrix.
    let _ = NaturalVector::from_str("(1)")
        .unwrap()
        .mod_power_of_2_mul(&NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap(), 3);
}

#[test]
#[should_panic]
fn vector_mod_power_of_2_mul_fail_2() {
    // The vector's dimension is not the height of the matrix.
    let _ = (&NaturalVector::from_str("(1)").unwrap())
        .mod_power_of_2_mul(NaturalMatrix::from_str("[]_{0,2}").unwrap(), 3);
}

#[test]
#[should_panic]
fn vector_mod_power_of_2_mul_fail_3() {
    // An element of the matrix is not reduced.
    let _ = NaturalVector::from_str("(1, 1)")
        .unwrap()
        .mod_power_of_2_mul(NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap(), 2);
}

#[test]
#[should_panic]
fn vector_mod_power_of_2_mul_fail_4() {
    // An element of the vector is not reduced.
    let _ = (&NaturalVector::from_str("(1, 8)").unwrap())
        .mod_power_of_2_mul(&NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap(), 3);
}

#[test]
fn vector_mod_power_of_2_mul_properties() {
    natural_matrix_natural_vector_unsigned_triple_gen_var_2().test_properties(|(a, v, pow)| {
        let w = (&v).mod_power_of_2_mul(&a, pow);
        // The forms agree.
        assert_eq!(v.clone().mod_power_of_2_mul(a.clone(), pow), w);
        assert_eq!(v.clone().mod_power_of_2_mul(&a, pow), w);
        assert_eq!((&v).mod_power_of_2_mul(a.clone(), pow), w);
        assert!(w.mod_power_of_2_is_reduced(pow));
        assert_eq!(w.dimension(), a.width());
        assert_eq!(
            natural_vector_mod_power_of_2_mul_natural_matrix_naive(&v, &a, pow),
            w
        );
        assert_eq!((&v * &a).mod_power_of_2(pow), w);
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

    natural_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        // The zero vector gives the zero vector, and (when 1 is reduced) a standard basis vector
        // picks out a row.
        assert_eq!(
            NaturalVector::zero(a.height()).mod_power_of_2_mul(&a, pow),
            NaturalVector::zero(a.width())
        );
        if pow != 0 {
            for i in 0..a.height() {
                assert_eq!(
                    NaturalVector::standard_basis_vector(a.height(), i).mod_power_of_2_mul(&a, pow),
                    a.row(i)
                );
            }
        }
    });

    natural_vector_unsigned_pair_gen_var_3().test_properties(|(v, pow)| {
        // The identity matrix (when 1 is reduced) leaves a vector unchanged.
        if pow != 0 {
            assert_eq!(
                (&v).mod_power_of_2_mul(NaturalMatrix::identity(v.dimension()), pow),
                v
            );
        }
    });

    unsigned_matrix_unsigned_vector_unsigned_triple_gen_var_2().test_properties(|(a, v, pow)| {
        // The `u64` elements multiply as their `Natural` counterparts do.
        assert_eq!(
            NaturalVector::from(v.clone()).mod_power_of_2_mul(NaturalMatrix::from(a.clone()), pow),
            NaturalVector::from(v.mod_power_of_2_mul(a, pow))
        );
    });
}
