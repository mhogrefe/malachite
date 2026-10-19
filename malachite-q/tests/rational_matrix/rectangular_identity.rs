// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::min;
use malachite_base::matrix::Matrix;
use malachite_base::test_util::generators::unsigned_pair_gen_var_53;
use malachite_base::vector::Vector;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::rational_vector::RationalVector;

#[test]
fn test_rectangular_identity() {
    let test = |height, width, out| {
        let m = RationalMatrix::rectangular_identity(height, width);
        assert!(m.is_valid());
        assert_eq!(m.to_string(), out);
        assert_eq!((m.height(), m.width()), (height, width));
    };
    test(0, 0, "[]_{0,0}");
    test(0, 3, "[]_{0,3}");
    test(3, 0, "[]_{3,0}");
    test(1, 1, "[1]");
    test(1, 3, "[1, 0, 0]");
    test(3, 1, "[1; 0; 0]");
    test(2, 2, "[1, 0; 0, 1]");
    test(2, 3, "[1, 0, 0; 0, 1, 0]");
    test(3, 2, "[1, 0; 0, 1; 0, 0]");
    test(4, 2, "[1, 0; 0, 1; 0, 0; 0, 0]");
}

#[test]
#[should_panic]
fn rectangular_identity_fail() {
    // The number of elements does not fit in a `usize`.
    RationalMatrix::rectangular_identity(u64::MAX, 2);
}

#[test]
fn rectangular_identity_properties() {
    unsigned_pair_gen_var_53().test_properties(|(height, width)| {
        let m = RationalMatrix::rectangular_identity(height, width);
        assert!(m.is_valid());
        assert_eq!((m.height(), m.width()), (height, width));
        assert!(m.is_rectangular_identity());
        assert_eq!(m.is_identity(), height == width);
        let size = min(height, width);
        assert_eq!(m.is_zero(), size == 0);
        assert!(m.is_diagonal());
        assert!(m.is_upper_triangular());
        assert!(m.is_lower_triangular());
        assert_eq!(
            m.transpose(),
            RationalMatrix::rectangular_identity(width, height)
        );
        assert_eq!(m.window(0..size, 0..size), RationalMatrix::identity(size));
        if height == width {
            assert_eq!(m, RationalMatrix::identity(height));
        }
        // Row `i` is the `i`th standard basis vector, or zero once the rows outrun the columns.
        for i in 0..height {
            assert_eq!(
                m.row(i),
                if i < width {
                    RationalVector::standard_basis_vector(width, i)
                } else {
                    RationalVector::zero(width)
                }
            );
        }
    });
}
