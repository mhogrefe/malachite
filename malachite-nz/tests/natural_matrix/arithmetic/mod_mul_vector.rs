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
    natural_matrix_natural_pair_gen_var_2, natural_matrix_natural_vector_natural_triple_gen_var_1,
    natural_vector_natural_pair_gen_var_2,
};
use malachite_nz::test_util::natural_matrix::arithmetic::mod_mul_vector::*;

#[test]
fn test_mod_mul_vector() {
    let test = |s, t, m, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let v = NaturalVector::from_str(t).unwrap();
        let m = Natural::from_str(m).unwrap();
        let w = (&a).mod_mul(&v, &m);
        assert_eq!(w.to_string(), out);
        // All eight combinations of value and reference.
        assert_eq!((&a).mod_mul(&v, m.clone()), w);
        assert_eq!((&a).mod_mul(v.clone(), &m), w);
        assert_eq!((&a).mod_mul(v.clone(), m.clone()), w);
        assert_eq!(a.clone().mod_mul(&v, &m), w);
        assert_eq!(a.clone().mod_mul(&v, m.clone()), w);
        assert_eq!(a.clone().mod_mul(v.clone(), &m), w);
        assert_eq!(a.mod_mul(v, m), w);
    };
    test("[]_{0,0}", "()", "3", "()");
    test("[]_{2,0}", "()", "3", "(0, 0)");
    test("[]_{0,3}", "(1, 2, 0)", "3", "()");
    test("[0, 0]", "(0, 0)", "1", "(0)");
    test("[5]", "(3)", "7", "(1)");
    test("[1, 2, 3; 4, 5, 6]", "(1, 0, 2)", "7", "(0, 2)");
    test("[1, 2, 3; 4, 5, 6]", "(1, 0, 2)", "100", "(7, 16)");
    test(
        "[18446744073709551615, 18446744073709551615]",
        "(18446744073709551615, 1)",
        "18446744073709551616",
        "(0)",
    );
    test(
        "[18446744073709551556, 5]",
        "(3, 18446744073709551556)",
        "18446744073709551557",
        "(18446744073709551549)",
    );
    test(
        "[1000000000000000000000000, 1]",
        "(1000000000000000000000000, 1)",
        "1000000000000000000000000000000000000000007",
        "(999999999999999999999999999999999993000008)",
    );
}

#[test]
#[should_panic]
fn mod_mul_vector_fail_1() {
    // The vector's dimension is not the width of the matrix.
    let _ = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap().mod_mul(
        &NaturalVector::from_str("(1)").unwrap(),
        Natural::from(5u32),
    );
}

#[test]
#[should_panic]
fn mod_mul_vector_fail_2() {
    // The vector's dimension is not the width of the matrix.
    let _ = (&NaturalMatrix::from_str("[]_{2,0}").unwrap()).mod_mul(
        NaturalVector::from_str("(1)").unwrap(),
        &Natural::from(5u32),
    );
}

#[test]
#[should_panic]
fn mod_mul_vector_fail_3() {
    // An element of the matrix is not reduced.
    let _ = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap().mod_mul(
        NaturalVector::from_str("(1, 1)").unwrap(),
        Natural::from(4u32),
    );
}

#[test]
#[should_panic]
fn mod_mul_vector_fail_4() {
    // An element of the vector is not reduced.
    let _ = (&NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap()).mod_mul(
        &NaturalVector::from_str("(1, 8)").unwrap(),
        &Natural::from(5u32),
    );
}

#[test]
#[should_panic]
fn mod_mul_vector_fail_5() {
    // The modulus is zero.
    let _ = NaturalMatrix::from_str("[]_{0,0}")
        .unwrap()
        .mod_mul(NaturalVector::from_str("()").unwrap(), Natural::ZERO);
}

#[test]
fn mod_mul_vector_properties() {
    natural_matrix_natural_vector_natural_triple_gen_var_1().test_properties(|(a, v, m)| {
        let w = (&a).mod_mul(&v, &m);
        // The forms agree.
        assert_eq!((&a).mod_mul(&v, m.clone()), w);
        assert_eq!((&a).mod_mul(v.clone(), &m), w);
        assert_eq!((&a).mod_mul(v.clone(), m.clone()), w);
        assert_eq!(a.clone().mod_mul(&v, &m), w);
        assert_eq!(a.clone().mod_mul(&v, m.clone()), w);
        assert_eq!(a.clone().mod_mul(v.clone(), &m), w);
        assert_eq!(a.clone().mod_mul(v.clone(), m.clone()), w);
        assert!(w.mod_is_reduced(&m));
        assert_eq!(w.dimension(), a.height());
        assert_eq!(natural_matrix_mod_mul_vector_naive(&a, &v, &m), w);
        assert_eq!((&a * &v).mod_op(&m), w);
        // Element `i` is the dot product of row `i` with the vector.
        for i in 0..a.height() {
            assert_eq!(w[usize::exact_from(i)], a.row(i).mod_dot(&v, &m));
        }
        // Multiplying on the right by the matrix is multiplying on the left by its transpose.
        assert_eq!((&v).mod_mul(&a.transpose(), &m), w);
        // The product is linear in the matrix.
        assert_eq!((&a).mod_add(&a, &m).mod_mul(&v, &m), (&w).mod_add(&w, &m));
    });

    natural_matrix_natural_pair_gen_var_2().test_properties(|(a, m)| {
        // The zero vector gives the zero vector, and (when 1 is reduced) a standard basis vector
        // picks out a column.
        assert_eq!(
            (&a).mod_mul(&NaturalVector::zero(a.width()), &m),
            NaturalVector::zero(a.height())
        );
        if m != 1u32 {
            for j in 0..a.width() {
                assert_eq!(
                    (&a).mod_mul(&NaturalVector::standard_basis_vector(a.width(), j), &m),
                    a.column(j)
                );
            }
        }
    });

    natural_vector_natural_pair_gen_var_2().test_properties(|(v, m)| {
        // The identity matrix (when 1 is reduced) leaves a vector unchanged.
        if m != 1u32 {
            assert_eq!(NaturalMatrix::identity(v.dimension()).mod_mul(&v, &m), v);
        }
    });

    unsigned_matrix_unsigned_vector_unsigned_triple_gen_var_3().test_properties(|(a, v, m)| {
        // The `u64` elements multiply as their `Natural` counterparts do.
        assert_eq!(
            NaturalMatrix::from(a.clone())
                .mod_mul(NaturalVector::from(v.clone()), Natural::from(m)),
            NaturalVector::from(a.mod_mul(v, m))
        );
    });
}
