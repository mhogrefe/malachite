// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::{
    unsigned_matrix_gen, unsigned_matrix_unsigned_pair_gen_var_3,
};
use malachite_base::unsigned_matrix::UnsignedMatrix;

#[test]
fn test_column() {
    let test = |s, j: u64, out| {
        let m = UnsignedMatrix::<u64>::from_str(s).unwrap();
        let v = m.column(j);
        assert_eq!(v.to_string(), out);
    };
    test("[]_{0,4}", 0, "()");
    test("[]_{0,4}", 1, "()");
    test("[]_{0,4}", 2, "()");
    test("[]_{0,4}", 3, "()");
    test("[5]", 0, "(5)");
    test("[1, 2, 3; 4, 5, 6]", 0, "(1, 4)");
    test("[1, 2, 3; 4, 5, 6]", 1, "(2, 5)");
    test("[1, 2, 3; 4, 5, 6]", 2, "(3, 6)");
    test("[1, 2; 3, 4; 5, 6]", 0, "(1, 3, 5)");
    test("[1, 2; 3, 4; 5, 6]", 1, "(2, 4, 6)");
}

#[test]
#[should_panic]
fn column_fail_1() {
    let m = UnsignedMatrix::<u64>::from_str("[]_{0,0}").unwrap();
    let _ = m.column(0);
}

#[test]
#[should_panic]
fn column_fail_2() {
    let m = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    let _ = m.column(3);
}

#[test]
#[should_panic]
fn column_fail_3() {
    let m = UnsignedMatrix::<u64>::from_str("[]_{4,0}").unwrap();
    let _ = m.column(0);
}

#[test]
fn column_properties() {
    unsigned_matrix_unsigned_pair_gen_var_3().test_properties(|(m, j)| {
        let v = m.column(j);
        assert_eq!(u64::exact_from(v.elements.len()), m.height());
        assert!(v.elements.iter().eq(m.column_iter(j)));
        assert_eq!(m.transpose().row(j), v);
    });

    unsigned_matrix_gen().test_properties(|m| {
        // The columns rebuild the matrix, unless it has no columns and so loses its height.
        if m.width() != 0 {
            let columns = (0..m.width()).map(|j| m.column(j)).collect::<Vec<_>>();
            assert_eq!(UnsignedMatrix::from_columns(&columns), m);
        }
    });
}
