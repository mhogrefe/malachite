// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::test_util::generators::unsigned_gen_var_36;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::vector::Vector;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::natural_vector::NaturalVector;

#[test]
fn test_identity() {
    let test = |size, out| {
        let m = NaturalMatrix::identity(size);
        assert!(m.is_valid());
        assert_eq!(m.to_string(), out);
        assert_eq!((m.height(), m.width()), (size, size));
    };
    test(0, "[]_{0,0}");
    test(1, "[1]");
    test(2, "[1, 0; 0, 1]");
    test(3, "[1, 0, 0; 0, 1, 0; 0, 0, 1]");
}

#[test]
#[should_panic]
fn identity_fail() {
    // The number of elements does not fit in a `usize`.
    NaturalMatrix::identity(u64::MAX);
}

#[test]
fn identity_properties() {
    unsigned_gen_var_36().test_properties(|size| {
        let m = NaturalMatrix::identity(size);
        assert!(m.is_valid());
        assert!(m.is_square());
        assert_eq!(m.height(), size);
        assert_eq!(m.is_zero(), size == 0);
        assert_eq!(m.transpose(), m);
        // The rows and columns are the standard basis vectors, in order.
        for i in 0..size {
            assert_eq!(m.row(i), NaturalVector::standard_basis_vector(size, i));
            assert_eq!(m.column(i), NaturalVector::standard_basis_vector(size, i));
        }
        assert_eq!(
            NaturalMatrix::from(UnsignedMatrix::<u64>::identity(size)),
            m
        );
    });
}
