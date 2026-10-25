// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_q::Rational;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::generators::{
    rational_matrix_gen, rational_matrix_unsigned_unsigned_triple_gen,
    rational_matrix_unsigned_unsigned_triple_gen_var_1,
};

#[test]
fn test_index() {
    let test = |s, i: usize, j: usize, out| {
        let m = RationalMatrix::from_str(s).unwrap();
        assert_eq!(m[(i, j)].to_string(), out);
    };
    test("[5]", 0, 0, "5");
    test("[1, 2, 3; 4, 5, 6]", 0, 0, "1");
    test("[1, 2, 3; 4, 5, 6]", 0, 2, "3");
    test("[1, 2, 3; 4, 5, 6]", 1, 0, "4");
    test("[1, 2, 3; 4, 5, 6]", 1, 2, "6");
    test("[1, 2; 3, 4; 5, 6]", 2, 1, "6");
    test("[1, 2; 3, 4; 5, -6]", 2, 1, "-6");
    test("[1/2, 2; 3, -4/5]", 1, 1, "-4/5");
    test("[1/2, 2; 3, -4/5]", 0, 0, "1/2");
}

#[test]
#[should_panic]
fn index_fail_1() {
    let m = RationalMatrix::from_str("[]_{0,0}").unwrap();
    let _ = &m[(0, 0)];
}

#[test]
#[should_panic]
fn index_fail_2() {
    let m = RationalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    let _ = &m[(2, 0)];
}

// - the column index is out of range, even though i * width + j is a valid position in `elements`
#[test]
#[should_panic]
fn index_fail_3() {
    let m = RationalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    let _ = &m[(0, 3)];
}

#[test]
#[should_panic]
fn index_fail_4() {
    let m = RationalMatrix::from_str("[]_{0,4}").unwrap();
    let _ = &m[(0, 0)];
}

#[test]
fn test_index_mut() {
    let test = |s, i: usize, j: usize, x: i32, out| {
        let mut m = RationalMatrix::from_str(s).unwrap();
        m[(i, j)] = Rational::from(x);
        assert_eq!(m.to_string(), out);
    };
    test("[5]", 0, 0, 7, "[7]");
    test("[1, 2, 3; 4, 5, 6]", 0, 1, 10, "[1, 10, 3; 4, 5, 6]");
    test("[1, 2, 3; 4, 5, 6]", 1, 0, -1, "[1, 2, 3; -1, 5, 6]");
    test("[1, 2; 3, 4; 5, 6]", 2, 1, 0, "[1, 2; 3, 4; 5, 0]");
    let mut m = RationalMatrix::from_str("[1/2, 2; 3, 4]").unwrap();
    m[(1, 0)] = Rational::from_signeds(-5, 7);
    assert_eq!(m.to_string(), "[1/2, 2; -5/7, 4]");
}

#[test]
#[should_panic]
fn index_mut_fail_1() {
    let mut m = RationalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    m[(2, 0)] = Rational::ZERO;
}

#[test]
#[should_panic]
fn index_mut_fail_2() {
    let mut m = RationalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    m[(0, 3)] = Rational::ZERO;
}

#[test]
fn test_get() {
    let test = |s, i: u64, j: u64, out: Option<i32>| {
        let mut m = RationalMatrix::from_str(s).unwrap();
        let out = out.map(Rational::from);
        assert_eq!(m.get(i, j), out.as_ref());
        assert_eq!(m.get_mut(i, j).map(|x| x.clone()), out);
    };
    test("[]_{0,0}", 0, 0, None);
    test("[]_{0,4}", 0, 0, None);
    test("[]_{4,0}", 0, 0, None);
    test("[5]", 0, 0, Some(5));
    test("[5]", 0, 1, None);
    test("[5]", 1, 0, None);
    test("[1, 2, 3; 4, 5, 6]", 1, 2, Some(6));
    test("[1, 2, 3; 4, 5, 6]", 0, 3, None);
    test("[1, 2, 3; 4, 5, 6]", 2, 0, None);
    test("[1, 2, 3; 4, 5, 6]", u64::MAX, u64::MAX, None);
    test("[1/2, 2/3]", 0, 2, None);
}

#[test]
fn index_properties() {
    rational_matrix_unsigned_unsigned_triple_gen_var_1().test_properties(|(m, i, j)| {
        let x = &m[(i, j)];
        assert_eq!(x, &m.elements_ref()[i * usize::exact_from(m.width()) + j]);
        assert_eq!(m.get(u64::exact_from(i), u64::exact_from(j)), Some(x));
        assert_eq!(&m.transpose()[(j, i)], x);

        // Changing one element through `IndexMut` changes only that element.
        let mut n = m.clone();
        n[(i, j)] += Rational::ONE;
        assert!(n.is_valid());
        assert_eq!((n.height(), n.width()), (m.height(), m.width()));
        assert_eq!(n[(i, j)], x + Rational::ONE);
        for k in 0..usize::exact_from(m.height()) {
            for l in 0..usize::exact_from(m.width()) {
                if (k, l) != (i, j) {
                    assert_eq!(n[(k, l)], m[(k, l)]);
                }
            }
        }
        *n.get_mut(u64::exact_from(i), u64::exact_from(j)).unwrap() -= Rational::ONE;
        assert_eq!(n, m);
    });

    rational_matrix_gen().test_properties(|m| {
        // Indexing every position, row by row, reads back the elements.
        let mut xs = Vec::new();
        for i in 0..usize::exact_from(m.height()) {
            for j in 0..usize::exact_from(m.width()) {
                xs.push(m[(i, j)].clone());
            }
        }
        assert_eq!(xs, m.elements_ref());
    });
}

#[test]
fn get_properties() {
    rational_matrix_unsigned_unsigned_triple_gen().test_properties(|(mut m, i, j)| {
        let x = m.get(i, j).cloned();
        assert_eq!(x.is_some(), i < m.height() && j < m.width());
        if let Some(x) = &x {
            assert_eq!(&m[(usize::exact_from(i), usize::exact_from(j))], x);
        }
        assert_eq!(m.get_mut(i, j).map(|y| y.clone()), x);
        assert_eq!(m.transpose().get(j, i).cloned(), x);
    });
}
