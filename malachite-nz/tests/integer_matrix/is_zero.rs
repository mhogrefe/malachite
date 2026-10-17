// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::test_util::generators::unsigned_quadruple_gen_var_14;
use malachite_base::vector::Vector;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::test_util::generators::integer_matrix_gen;

#[test]
fn test_is_zero() {
    let test = |s, out| {
        assert_eq!(IntegerMatrix::from_str(s).unwrap().is_zero(), out);
    };
    test("[]_{0,0}", true);
    test("[]_{3,0}", true);
    test("[0]", true);
    test("[0, 0; 0, 0]", true);
    test("[0, 1; 0, 0]", false);
    test("[5]", false);
    test("[0, -1; 0, 0]", false);
}

#[test]
fn is_zero_properties() {
    integer_matrix_gen().test_properties(|m| {
        let zero = m.is_zero();
        // It is equality with the zero matrix of the same shape, and holds iff every row is zero.
        assert_eq!(zero, m == IntegerMatrix::zero(m.height(), m.width()));
        assert_eq!(zero, (0..m.height()).all(|i| m.row(i).is_zero()));
        assert_eq!(m.transpose().is_zero(), zero);
        let mut n = m.clone();
        n.set_zero();
        assert!(n.is_zero());
    });

    unsigned_quadruple_gen_var_14().test_properties(|(height, width, i, j)| {
        // A standard basis matrix is never zero.
        assert!(!IntegerMatrix::standard_basis_matrix(height, width, i, j).is_zero());
    });
}
