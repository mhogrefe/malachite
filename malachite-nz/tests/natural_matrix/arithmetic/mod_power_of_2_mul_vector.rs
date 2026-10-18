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
    natural_matrix_natural_vector_unsigned_triple_gen_var_1,
    natural_matrix_unsigned_pair_gen_var_4, natural_vector_unsigned_pair_gen_var_3,
};
use malachite_nz::test_util::natural_matrix::arithmetic::mod_power_of_2_mul_vector::*;

#[test]
fn test_mod_power_of_2_mul_vector() {
    let test = |s, t, pow, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let v = NaturalVector::from_str(t).unwrap();
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
    test(
        "[18446744073709551615, 18446744073709551615]",
        "(18446744073709551615, 1)",
        64,
        "(0)",
    );
    test(
        "[18446744073709551615, 18446744073709551615]",
        "(18446744073709551615, 1)",
        65,
        "(18446744073709551616)",
    );
    test(
        "[1000000000000, 1]",
        "(1000000000000, 1)",
        100,
        "(1000000000000000000000001)",
    );
    test(
        "[1000000000000, 1]",
        "(1000000000000, 1)",
        70,
        "(38897252352625999873)",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_vector_fail_1() {
    // The vector's dimension is not the width of the matrix.
    let _ = NaturalMatrix::from_str("[1, 2; 3, 4]")
        .unwrap()
        .mod_power_of_2_mul(&NaturalVector::from_str("(1)").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_vector_fail_2() {
    // The vector's dimension is not the width of the matrix.
    let _ = (&NaturalMatrix::from_str("[]_{2,0}").unwrap())
        .mod_power_of_2_mul(NaturalVector::from_str("(1)").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_vector_fail_3() {
    // An element of the matrix is not reduced.
    let _ = NaturalMatrix::from_str("[1, 2; 3, 4]")
        .unwrap()
        .mod_power_of_2_mul(NaturalVector::from_str("(1, 1)").unwrap(), 2);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_vector_fail_4() {
    // An element of the vector is not reduced.
    let _ = (&NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap())
        .mod_power_of_2_mul(&NaturalVector::from_str("(1, 8)").unwrap(), 3);
}

#[test]
fn mod_power_of_2_mul_vector_properties() {
    natural_matrix_natural_vector_unsigned_triple_gen_var_1().test_properties(|(a, v, pow)| {
        let w = (&a).mod_power_of_2_mul(&v, pow);
        // The forms agree.
        assert_eq!(a.clone().mod_power_of_2_mul(v.clone(), pow), w);
        assert_eq!(a.clone().mod_power_of_2_mul(&v, pow), w);
        assert_eq!((&a).mod_power_of_2_mul(v.clone(), pow), w);
        assert!(w.mod_power_of_2_is_reduced(pow));
        assert_eq!(w.dimension(), a.height());
        assert_eq!(
            natural_matrix_mod_power_of_2_mul_vector_naive(&a, &v, pow),
            w
        );
        assert_eq!((&a * &v).mod_power_of_2(pow), w);
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

    natural_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        // The zero vector gives the zero vector, and (when 1 is reduced) a standard basis vector
        // picks out a column.
        assert_eq!(
            (&a).mod_power_of_2_mul(&NaturalVector::zero(a.width()), pow),
            NaturalVector::zero(a.height())
        );
        if pow != 0 {
            for j in 0..a.width() {
                assert_eq!(
                    (&a).mod_power_of_2_mul(
                        &NaturalVector::standard_basis_vector(a.width(), j),
                        pow
                    ),
                    a.column(j)
                );
            }
        }
    });

    natural_vector_unsigned_pair_gen_var_3().test_properties(|(v, pow)| {
        // The identity matrix (when 1 is reduced) leaves a vector unchanged.
        if pow != 0 {
            assert_eq!(
                NaturalMatrix::identity(v.dimension()).mod_power_of_2_mul(&v, pow),
                v
            );
        }
    });

    unsigned_matrix_unsigned_vector_unsigned_triple_gen_var_1().test_properties(|(a, v, pow)| {
        // The `u64` elements multiply as their `Natural` counterparts do.
        assert_eq!(
            NaturalMatrix::from(a.clone()).mod_power_of_2_mul(NaturalVector::from(v.clone()), pow),
            NaturalVector::from(a.mod_power_of_2_mul(v, pow))
        );
    });
}
