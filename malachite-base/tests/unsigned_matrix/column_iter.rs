// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::iterators::prefix_to_string;
use malachite_base::matrix::Matrix;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::{
    unsigned_matrix_gen, unsigned_matrix_unsigned_pair_gen_var_3,
};
use malachite_base::unsigned_matrix::UnsignedMatrix;

#[test]
fn test_column_iter() {
    let test = |s, j: u64, out| {
        let mut m = UnsignedMatrix::<u64>::from_str(s).unwrap();
        assert_eq!(prefix_to_string(m.column_iter(j), 100), out);
        assert_eq!(prefix_to_string(m.column_iter_mut(j), 100), out);
    };
    test("[]_{0,4}", 3, "[]");
    test("[5]", 0, "[5]");
    test("[1, 2, 3; 4, 5, 6]", 0, "[1, 4]");
    test("[1, 2, 3; 4, 5, 6]", 1, "[2, 5]");
    test("[1, 2, 3; 4, 5, 6]", 2, "[3, 6]");
    test("[1, 2; 3, 4; 5, 6]", 0, "[1, 3, 5]");
    test("[1, 2; 3, 4; 5, 6]", 1, "[2, 4, 6]");
}

#[test]
fn test_column_iter_mut() {
    let test = |s, j: u64, out| {
        let mut m = UnsignedMatrix::<u64>::from_str(s).unwrap();
        for x in m.column_iter_mut(j) {
            *x ^= 1;
        }
        assert_eq!(m.to_string(), out);
    };
    test("[]_{0,4}", 3, "[]_{0,4}");
    test("[5]", 0, "[4]");
    test("[1, 2, 3; 4, 5, 6]", 0, "[0, 2, 3; 5, 5, 6]");
    test("[1, 2, 3; 4, 5, 6]", 1, "[1, 3, 3; 4, 4, 6]");
    test("[1, 2, 3; 4, 5, 6]", 2, "[1, 2, 2; 4, 5, 7]");
    test("[1, 2; 3, 4; 5, 6]", 0, "[0, 2; 2, 4; 4, 6]");
    test("[1, 2; 3, 4; 5, 6]", 1, "[1, 3; 3, 5; 5, 7]");
}

#[test]
#[should_panic]
fn column_iter_fail_1() {
    let m = UnsignedMatrix::<u64>::from_str("[]_{0,0}").unwrap();
    let _ = m.column_iter(0);
}

#[test]
#[should_panic]
fn column_iter_fail_2() {
    let m = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    let _ = m.column_iter(3);
}

#[test]
#[should_panic]
fn column_iter_fail_3() {
    let m = UnsignedMatrix::<u64>::from_str("[]_{4,0}").unwrap();
    let _ = m.column_iter(0);
}

#[test]
#[should_panic]
fn column_iter_mut_fail_1() {
    let mut m = UnsignedMatrix::<u64>::from_str("[]_{0,0}").unwrap();
    let _ = m.column_iter_mut(0);
}

#[test]
#[should_panic]
fn column_iter_mut_fail_2() {
    let mut m = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    let _ = m.column_iter_mut(3);
}

#[test]
fn column_iter_properties() {
    unsigned_matrix_unsigned_pair_gen_var_3().test_properties(|(mut m, j)| {
        let n = m.height();
        assert_eq!(u64::exact_from(m.column_iter(j).len()), n);
        assert!(
            m.column_iter(j)
                .eq((0..m.height()).map(|i| m.get(i, j).unwrap()))
        );
        assert!(
            m.column_iter(j)
                .rev()
                .eq((0..m.height()).map(|i| m.get(i, j).unwrap()).rev())
        );
        assert!(m.column_iter(j).eq(m.transpose().row_iter(j)));

        // Changing the elements through `column_iter_mut` changes only that column.
        let old = m.clone();
        assert_eq!(u64::exact_from(m.column_iter_mut(j).len()), n);
        assert!(m.column_iter_mut(j).map(|x| &*x).eq(old.column_iter(j)));
        for x in m.column_iter_mut(j) {
            *x ^= 1;
        }
        assert!(m.is_valid());
        assert_eq!((m.height(), m.width()), (old.height(), old.width()));
        assert!(
            m.column_iter(j)
                .zip(old.column_iter(j))
                .all(|(x, y)| x != y)
        );
        for k in 0..m.width() {
            if k != j {
                assert!(m.column_iter(k).eq(old.column_iter(k)));
            }
        }
        for x in m.column_iter_mut(j).rev() {
            *x ^= 1;
        }
        assert_eq!(m, old);
    });

    unsigned_matrix_gen().test_properties(|m| {
        // The columns, in order, are the elements of the transpose.
        assert!(
            (0..m.width())
                .flat_map(|j| m.column_iter(j))
                .eq(m.transpose().elements.iter())
        );
    });
}
