// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::{
    natural_matrix_unsigned_natural_vector_triple_gen_var_1, natural_matrix_unsigned_pair_gen_var_2,
};

#[test]
fn test_set_row() {
    let test = |s, i: usize, v, out| {
        let v = NaturalVector::from_str(v).unwrap();
        let mut m = NaturalMatrix::from_str(s).unwrap();
        m.set_row(i, &v);
        assert!(m.is_valid());
        assert_eq!(m.to_string(), out);

        let mut m = NaturalMatrix::from_str(s).unwrap();
        m.set_row_owned(i, v);
        assert!(m.is_valid());
        assert_eq!(m.to_string(), out);
    };
    test("[]_{3,0}", 0, "()", "[]_{3,0}");
    test("[]_{3,0}", 1, "()", "[]_{3,0}");
    test("[]_{3,0}", 2, "()", "[]_{3,0}");
    test("[5]", 0, "(7)", "[7]");
    test("[1, 2, 3; 4, 5, 6]", 0, "(7, 8, 9)", "[7, 8, 9; 4, 5, 6]");
    test("[1, 2, 3; 4, 5, 6]", 1, "(7, 8, 9)", "[1, 2, 3; 7, 8, 9]");
    test("[1, 2; 3, 4; 5, 6]", 0, "(7, 8)", "[7, 8; 3, 4; 5, 6]");
    test("[1, 2; 3, 4; 5, 6]", 1, "(7, 8)", "[1, 2; 7, 8; 5, 6]");
    test("[1, 2; 3, 4; 5, 6]", 2, "(7, 8)", "[1, 2; 3, 4; 7, 8]");
}

#[test]
#[should_panic]
fn set_row_fail_1() {
    let mut m = NaturalMatrix::from_str("[]_{0,0}").unwrap();
    m.set_row(0, &NaturalVector::from_str("()").unwrap());
}

#[test]
#[should_panic]
fn set_row_fail_2() {
    let mut m = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    m.set_row(2, &NaturalVector::from_str("(7, 8, 9)").unwrap());
}

#[test]
#[should_panic]
fn set_row_fail_3() {
    let mut m = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    m.set_row(0, &NaturalVector::from_str("(7, 8)").unwrap());
}

#[test]
#[should_panic]
fn set_row_fail_4() {
    let mut m = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    m.set_row(0, &NaturalVector::from_str("(7, 8, 9, 10)").unwrap());
}

#[test]
#[should_panic]
fn set_row_fail_5() {
    let mut m = NaturalMatrix::from_str("[]_{3,0}").unwrap();
    m.set_row(0, &NaturalVector::from_str("(7)").unwrap());
}

#[test]
#[should_panic]
fn set_row_owned_fail_1() {
    let mut m = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    m.set_row_owned(2, NaturalVector::from_str("(7, 8, 9)").unwrap());
}

#[test]
#[should_panic]
fn set_row_owned_fail_2() {
    let mut m = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    m.set_row_owned(0, NaturalVector::from_str("(7, 8)").unwrap());
}

#[test]
fn set_row_properties() {
    natural_matrix_unsigned_natural_vector_triple_gen_var_1().test_properties(|(mut m, i, v)| {
        let old = m.clone();
        m.set_row(i, &v);
        assert!(m.is_valid());
        assert_eq!((m.height(), m.width()), (old.height(), old.width()));
        assert_eq!(m.row(i), v);
        for k in 0..m.height() {
            if k != i {
                assert!(m.row_iter(k).eq(old.row_iter(k)));
            }
        }

        let mut n = old.clone();
        n.set_row_owned(i, v.clone());
        assert_eq!(n, m);

        let mut n = old.transpose();
        n.set_column(i, &v);
        assert_eq!(n.transpose(), m);

        // Setting the old row back restores the matrix.
        m.set_row(i, &old.row(i));
        assert_eq!(m, old);
    });

    natural_matrix_unsigned_pair_gen_var_2().test_properties(|(m, i)| {
        // Setting a row to itself changes nothing.
        let mut n = m.clone();
        n.set_row(i, &m.row(i));
        assert_eq!(n, m);
    });
}
