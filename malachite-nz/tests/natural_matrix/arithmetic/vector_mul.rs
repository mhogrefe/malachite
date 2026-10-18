// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::Dot;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::vector::Vector;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::{
    natural_matrix_gen, natural_matrix_natural_vector_pair_gen_var_2, natural_vector_gen,
};
use malachite_nz::test_util::natural_matrix::arithmetic::mul_vector::*;

#[test]
fn test_vector_mul() {
    let test = |s, t, out| {
        let v = NaturalVector::from_str(s).unwrap();
        let a = NaturalMatrix::from_str(t).unwrap();
        let w = &v * &a;
        assert_eq!(w.to_string(), out);
        assert_eq!(v.clone() * a.clone(), w);
        assert_eq!(v.clone() * &a, w);
        assert_eq!(&v * a, w);
    };
    test("()", "[]_{0,0}", "()");
    test("()", "[]_{0,3}", "(0, 0, 0)");
    test("(1, 2)", "[]_{2,0}", "()");
    test("(3)", "[5]", "(15)");
    test("(1, 2)", "[1, 2, 3; 4, 5, 6]", "(9, 12, 15)");
    test(
        "(1000000000000, 1)",
        "[1000000000000; 1]",
        "(1000000000000000000000001)",
    );
}

#[test]
#[should_panic]
fn vector_mul_fail_1() {
    // The vector's dimension is not the height of the matrix.
    let _ =
        NaturalVector::from_str("(1)").unwrap() * &NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
}

#[test]
#[should_panic]
fn vector_mul_fail_2() {
    // The vector's dimension is not the height of the matrix.
    let _ = &NaturalVector::from_str("(1)").unwrap() * NaturalMatrix::from_str("[]_{0,2}").unwrap();
}

#[test]
#[should_panic]
fn vector_mul_fail_3() {
    // The vector's dimension is not the height of the matrix.
    let _ =
        NaturalVector::from_str("(1, 2)").unwrap() * &NaturalMatrix::from_str("[]_{3,0}").unwrap();
}

#[test]
fn vector_mul_properties() {
    natural_matrix_natural_vector_pair_gen_var_2().test_properties(|(a, v)| {
        let w = &v * &a;
        assert_eq!(v.clone() * a.clone(), w);
        assert_eq!(v.clone() * &a, w);
        assert_eq!(&v * a.clone(), w);
        assert_eq!(w.dimension(), a.width());
        assert_eq!(natural_vector_mul_natural_matrix_naive(&v, &a), w);
        // Element `j` is the dot product of the vector with column `j`.
        for j in 0..a.width() {
            assert_eq!(w[usize::exact_from(j)], v.clone().dot(a.column(j)));
        }
        // Multiplying on the left by the matrix is multiplying on the right by its transpose.
        assert_eq!(&a.transpose() * &v, w);
        // The product is linear in the matrix.
        assert_eq!(&v * &(&a + &a), &w + &w);
    });

    natural_matrix_gen().test_properties(|a| {
        // The zero vector gives the zero vector, and a standard basis vector picks out a row.
        assert_eq!(
            &NaturalVector::zero(a.height()) * &a,
            NaturalVector::zero(a.width())
        );
        for i in 0..a.height() {
            assert_eq!(
                &NaturalVector::standard_basis_vector(a.height(), i) * &a,
                a.row(i)
            );
        }
    });

    natural_vector_gen().test_properties(|v| {
        // The identity matrix leaves a vector unchanged.
        assert_eq!(&v * &NaturalMatrix::identity(v.dimension()), v);
    });
}
