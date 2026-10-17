// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::test_util::generators::unsigned_quadruple_gen_var_14;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::vector::Vector;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::natural_vector::NaturalVector;

#[test]
fn test_standard_basis_matrix() {
    let test = |height, width, i, j, out| {
        let m = NaturalMatrix::standard_basis_matrix(height, width, i, j);
        assert!(m.is_valid());
        assert_eq!(m.to_string(), out);
    };
    test(1, 1, 0, 0, "[1]");
    test(2, 3, 0, 2, "[0, 0, 1; 0, 0, 0]");
    test(2, 3, 1, 0, "[0, 0, 0; 1, 0, 0]");
    test(3, 1, 2, 0, "[0; 0; 1]");
}

#[test]
#[should_panic]
fn standard_basis_matrix_fail_1() {
    // The row index equals the height.
    NaturalMatrix::standard_basis_matrix(2, 3, 2, 0);
}

#[test]
#[should_panic]
fn standard_basis_matrix_fail_2() {
    // The column index equals the width.
    NaturalMatrix::standard_basis_matrix(2, 3, 0, 3);
}

#[test]
#[should_panic]
fn standard_basis_matrix_fail_3() {
    // A matrix with no rows has no positions.
    NaturalMatrix::standard_basis_matrix(0, 3, 0, 0);
}

#[test]
#[should_panic]
fn standard_basis_matrix_fail_4() {
    // A matrix with no columns has no positions.
    NaturalMatrix::standard_basis_matrix(3, 0, 0, 0);
}

#[test]
fn standard_basis_matrix_properties() {
    unsigned_quadruple_gen_var_14().test_properties(|(height, width, i, j)| {
        let m = NaturalMatrix::standard_basis_matrix(height, width, i, j);
        assert!(m.is_valid());
        assert_eq!((m.height(), m.width()), (height, width));
        assert!(!m.is_zero());
        assert_eq!(*m.get(i, j).unwrap(), 1u32);
        // Row `i` is the standard basis vector with its 1 at `j`, and every other row is zero.
        for k in 0..height {
            if k == i {
                assert_eq!(m.row(k), NaturalVector::standard_basis_vector(width, j));
            } else {
                assert!(m.row(k).is_zero());
            }
        }
        assert_eq!(m.column(j), NaturalVector::standard_basis_vector(height, i));
        assert_eq!(
            m.transpose(),
            NaturalMatrix::standard_basis_matrix(width, height, j, i)
        );
        let mut n = m.clone();
        n.set_zero();
        assert_eq!(n, NaturalMatrix::zero(height, width));
        assert_eq!(
            NaturalMatrix::from(UnsignedMatrix::<u64>::standard_basis_matrix(
                height, width, i, j
            )),
            m
        );
    });
}
