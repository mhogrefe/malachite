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
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_q::Rational;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::generators::{
    rational_matrix_gen, rational_matrix_unsigned_pair_gen_var_1,
};

#[test]
fn test_row_iter() {
    let test = |s, i: u64, out| {
        let mut m = RationalMatrix::from_str(s).unwrap();
        assert_eq!(prefix_to_string(m.row_iter(i), 100), out);
        assert_eq!(prefix_to_string(m.row_iter_mut(i), 100), out);
    };
    test("[]_{3,0}", 2, "[]");
    test("[5]", 0, "[5]");
    test("[1, 2, 3; 4, 5, 6]", 0, "[1, 2, 3]");
    test("[1, 2, 3; 4, 5, 6]", 1, "[4, 5, 6]");
    test("[1, 2; 3, 4; 5, 6]", 0, "[1, 2]");
    test("[1, 2; 3, 4; 5, 6]", 1, "[3, 4]");
    test("[1, 2; 3, 4; 5, 6]", 2, "[5, 6]");
    test("[1/2, -2; 3, 4/3]", 0, "[1/2, -2]");
    test("[1/2, -2; 3, 4/3]", 1, "[3, 4/3]");
}

#[test]
fn test_row_iter_mut() {
    let test = |s, i: u64, out| {
        let mut m = RationalMatrix::from_str(s).unwrap();
        for x in m.row_iter_mut(i) {
            *x += Rational::ONE;
        }
        assert_eq!(m.to_string(), out);
    };
    test("[]_{3,0}", 2, "[]_{3,0}");
    test("[5]", 0, "[6]");
    test("[1, 2, 3; 4, 5, 6]", 0, "[2, 3, 4; 4, 5, 6]");
    test("[1, 2, 3; 4, 5, 6]", 1, "[1, 2, 3; 5, 6, 7]");
    test("[1, 2; 3, 4; 5, 6]", 0, "[2, 3; 3, 4; 5, 6]");
    test("[1, 2; 3, 4; 5, 6]", 1, "[1, 2; 4, 5; 5, 6]");
    test("[1, 2; 3, 4; 5, 6]", 2, "[1, 2; 3, 4; 6, 7]");
    test("[1/2, -2; 3, 4/3]", 0, "[3/2, -1; 3, 4/3]");
    test("[1/2, -2; 3, 4/3]", 1, "[1/2, -2; 4, 7/3]");
}

#[test]
#[should_panic]
fn row_iter_fail_1() {
    let m = RationalMatrix::from_str("[]_{0,0}").unwrap();
    let _ = m.row_iter(0);
}

#[test]
#[should_panic]
fn row_iter_fail_2() {
    let m = RationalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    let _ = m.row_iter(2);
}

#[test]
#[should_panic]
fn row_iter_fail_3() {
    let m = RationalMatrix::from_str("[]_{0,4}").unwrap();
    let _ = m.row_iter(0);
}

#[test]
#[should_panic]
fn row_iter_mut_fail_1() {
    let mut m = RationalMatrix::from_str("[]_{0,0}").unwrap();
    let _ = m.row_iter_mut(0);
}

#[test]
#[should_panic]
fn row_iter_mut_fail_2() {
    let mut m = RationalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    let _ = m.row_iter_mut(2);
}

#[test]
fn row_iter_properties() {
    rational_matrix_unsigned_pair_gen_var_1().test_properties(|(mut m, i)| {
        let n = m.width();
        assert_eq!(u64::exact_from(m.row_iter(i).len()), n);
        assert!(
            m.row_iter(i)
                .eq((0..m.width()).map(|j| m.get(i, j).unwrap()))
        );
        assert!(
            m.row_iter(i).eq(m.elements_ref()
                [usize::exact_from(i * m.width())..usize::exact_from((i + 1) * m.width())]
                .iter())
        );
        assert!(
            m.row_iter(i)
                .rev()
                .eq((0..m.width()).map(|j| m.get(i, j).unwrap()).rev())
        );
        assert!(m.row_iter(i).eq(m.transpose().column_iter(i)));

        // Changing the elements through `row_iter_mut` changes only that row.
        let old = m.clone();
        assert_eq!(u64::exact_from(m.row_iter_mut(i).len()), n);
        assert!(m.row_iter_mut(i).map(|x| &*x).eq(old.row_iter(i)));
        for x in m.row_iter_mut(i) {
            *x += Rational::ONE;
        }
        assert!(m.is_valid());
        assert_eq!((m.height(), m.width()), (old.height(), old.width()));
        assert!(m.row_iter(i).zip(old.row_iter(i)).all(|(x, y)| x != y));
        for k in 0..m.height() {
            if k != i {
                assert!(m.row_iter(k).eq(old.row_iter(k)));
            }
        }
        for x in m.row_iter_mut(i).rev() {
            *x -= Rational::ONE;
        }
        assert_eq!(m, old);
    });

    rational_matrix_gen().test_properties(|m| {
        // The rows, in order, are the elements.
        assert!(
            (0..m.height())
                .flat_map(|i| m.row_iter(i))
                .eq(m.elements_ref().iter())
        );
    });
}
