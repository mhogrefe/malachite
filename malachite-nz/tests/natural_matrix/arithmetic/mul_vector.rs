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
    natural_matrix_gen, natural_matrix_natural_vector_pair_gen_var_1, natural_vector_gen,
};
use malachite_nz::test_util::natural_matrix::arithmetic::mul_vector::*;

#[test]
fn test_mul_vector() {
    let test = |s, t, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let v = NaturalVector::from_str(t).unwrap();
        let w = &a * &v;
        assert_eq!(w.to_string(), out);
        assert_eq!(a.clone() * v.clone(), w);
        assert_eq!(a.clone() * &v, w);
        assert_eq!(&a * v, w);
    };
    test("[]_{0,0}", "()", "()");
    test("[]_{2,0}", "()", "(0, 0)");
    test("[]_{0,3}", "(1, 2, 3)", "()");
    test("[5]", "(3)", "(15)");
    test("[1, 2, 3; 4, 5, 6]", "(1, 0, 2)", "(7, 16)");
    test(
        "[1000000000000, 1]",
        "(1000000000000, 1)",
        "(1000000000000000000000001)",
    );
}

#[test]
#[should_panic]
fn mul_vector_fail_1() {
    // The vector's dimension is not the width of the matrix.
    let _ =
        NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap() * &NaturalVector::from_str("(1)").unwrap();
}

#[test]
#[should_panic]
fn mul_vector_fail_2() {
    // The vector's dimension is not the width of the matrix.
    let _ = &NaturalMatrix::from_str("[]_{2,0}").unwrap() * NaturalVector::from_str("(1)").unwrap();
}

#[test]
#[should_panic]
fn mul_vector_fail_3() {
    // The vector's dimension is not the width of the matrix.
    let _ =
        NaturalMatrix::from_str("[]_{0,3}").unwrap() * &NaturalVector::from_str("(1, 2)").unwrap();
}

#[test]
fn mul_vector_properties() {
    natural_matrix_natural_vector_pair_gen_var_1().test_properties(|(a, v)| {
        let w = &a * &v;
        assert_eq!(a.clone() * v.clone(), w);
        assert_eq!(a.clone() * &v, w);
        assert_eq!(&a * v.clone(), w);
        assert_eq!(w.dimension(), a.height());
        assert_eq!(natural_matrix_mul_vector_naive(&a, &v), w);
        // Element `i` is the dot product of row `i` with the vector.
        for i in 0..a.height() {
            assert_eq!(w[usize::exact_from(i)], a.row(i).dot(&v));
        }
        // Multiplying on the right by the matrix is multiplying on the left by its transpose.
        assert_eq!(&v * &a.transpose(), w);
        // The product is linear in the matrix.
        assert_eq!(&(&a + &a) * &v, &w + &w);
    });

    natural_matrix_gen().test_properties(|a| {
        // The zero vector gives the zero vector, and a standard basis vector picks out a column.
        assert_eq!(
            &a * &NaturalVector::zero(a.width()),
            NaturalVector::zero(a.height())
        );
        for j in 0..a.width() {
            assert_eq!(
                &a * &NaturalVector::standard_basis_vector(a.width(), j),
                a.column(j)
            );
        }
    });

    natural_vector_gen().test_properties(|v| {
        // The identity matrix leaves a vector unchanged.
        assert_eq!(&NaturalMatrix::identity(v.dimension()) * &v, v);
    });
}
