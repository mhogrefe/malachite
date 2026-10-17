// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::{
    natural_matrix_unsigned_natural_vector_triple_gen_var_2, natural_matrix_unsigned_pair_gen_var_3,
};

#[test]
fn test_set_column() {
    let test = |s, j: u64, v, out| {
        let v = NaturalVector::from_str(v).unwrap();
        let mut m = NaturalMatrix::from_str(s).unwrap();
        m.set_column(j, &v);
        assert!(m.is_valid());
        assert_eq!(m.to_string(), out);

        let mut m = NaturalMatrix::from_str(s).unwrap();
        m.set_column_owned(j, v);
        assert!(m.is_valid());
        assert_eq!(m.to_string(), out);
    };
    test("[]_{0,4}", 0, "()", "[]_{0,4}");
    test("[]_{0,4}", 1, "()", "[]_{0,4}");
    test("[]_{0,4}", 2, "()", "[]_{0,4}");
    test("[]_{0,4}", 3, "()", "[]_{0,4}");
    test("[5]", 0, "(7)", "[7]");
    test("[1, 2, 3; 4, 5, 6]", 0, "(7, 8)", "[7, 2, 3; 8, 5, 6]");
    test("[1, 2, 3; 4, 5, 6]", 1, "(7, 8)", "[1, 7, 3; 4, 8, 6]");
    test("[1, 2, 3; 4, 5, 6]", 2, "(7, 8)", "[1, 2, 7; 4, 5, 8]");
    test("[1, 2; 3, 4; 5, 6]", 0, "(7, 8, 9)", "[7, 2; 8, 4; 9, 6]");
    test("[1, 2; 3, 4; 5, 6]", 1, "(7, 8, 9)", "[1, 7; 3, 8; 5, 9]");
}

#[test]
#[should_panic]
fn set_column_fail_1() {
    let mut m = NaturalMatrix::from_str("[]_{0,0}").unwrap();
    m.set_column(0, &NaturalVector::from_str("()").unwrap());
}

#[test]
#[should_panic]
fn set_column_fail_2() {
    let mut m = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    m.set_column(3, &NaturalVector::from_str("(7, 8)").unwrap());
}

#[test]
#[should_panic]
fn set_column_fail_3() {
    let mut m = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    m.set_column(0, &NaturalVector::from_str("(7)").unwrap());
}

#[test]
#[should_panic]
fn set_column_fail_4() {
    let mut m = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    m.set_column(0, &NaturalVector::from_str("(7, 8, 9)").unwrap());
}

#[test]
#[should_panic]
fn set_column_fail_5() {
    let mut m = NaturalMatrix::from_str("[]_{0,4}").unwrap();
    m.set_column(0, &NaturalVector::from_str("(7)").unwrap());
}

#[test]
#[should_panic]
fn set_column_owned_fail_1() {
    let mut m = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    m.set_column_owned(3, NaturalVector::from_str("(7, 8)").unwrap());
}

#[test]
#[should_panic]
fn set_column_owned_fail_2() {
    let mut m = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    m.set_column_owned(0, NaturalVector::from_str("(7)").unwrap());
}

#[test]
fn set_column_properties() {
    natural_matrix_unsigned_natural_vector_triple_gen_var_2().test_properties(|(mut m, j, v)| {
        let old = m.clone();
        m.set_column(j, &v);
        assert!(m.is_valid());
        assert_eq!((m.height(), m.width()), (old.height(), old.width()));
        assert_eq!(m.column(j), v);
        for k in 0..m.width() {
            if k != j {
                assert!(m.column_iter(k).eq(old.column_iter(k)));
            }
        }

        let mut n = old.clone();
        n.set_column_owned(j, v.clone());
        assert_eq!(n, m);

        let mut n = old.transpose();
        n.set_row(j, &v);
        assert_eq!(n.transpose(), m);

        // Setting the old column back restores the matrix.
        m.set_column(j, &old.column(j));
        assert_eq!(m, old);
    });

    natural_matrix_unsigned_pair_gen_var_3().test_properties(|(m, j)| {
        // Setting a column to itself changes nothing.
        let mut n = m.clone();
        n.set_column(j, &m.column(j));
        assert_eq!(n, m);
    });
}
