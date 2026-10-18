// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::generators::{
    rational_matrix_rational_matrix_unsigned_unsigned_quadruple_gen_var_1,
    rational_matrix_unsigned_unsigned_unsigned_unsigned_quintuple_gen_var_1,
};

#[test]
fn test_set_window() {
    let test = |s, i, j, w, out| {
        let w = RationalMatrix::from_str(w).unwrap();
        let mut m = RationalMatrix::from_str(s).unwrap();
        m.set_window(i, j, &w);
        assert!(m.is_valid());
        assert_eq!(m.to_string(), out);

        let mut m_alt = RationalMatrix::from_str(s).unwrap();
        m_alt.set_window_owned(i, j, w);
        assert!(m_alt.is_valid());
        assert_eq!(m_alt, m);
    };
    test("[]_{0,0}", 0, 0, "[]_{0,0}", "[]_{0,0}");
    test("[]_{0,3}", 0, 1, "[]_{0,2}", "[]_{0,3}");
    test("[]_{3,0}", 1, 0, "[]_{2,0}", "[]_{3,0}");
    test("[7]", 0, 0, "[8]", "[8]");
    test("[7]", 1, 0, "[]_{0,1}", "[7]");
    test("[7]", 0, 1, "[]_{1,0}", "[7]");
    test(
        "[1/2, 2, 3; 4, 5/3, 6; 7, 8, -9]",
        1,
        1,
        "[0, -1/2; 1/3, 0]",
        "[1/2, 2, 3; 4, 0, -1/2; 7, 1/3, 0]",
    );
    // - a whole row
    test(
        "[1/2, 2, 3; 4, 5/3, 6; 7, 8, -9]",
        1,
        0,
        "[10, -1/20, 30]",
        "[1/2, 2, 3; 10, -1/20, 30; 7, 8, -9]",
    );
    // - a whole column
    test(
        "[1/2, 2, 3; 4, 5/3, 6; 7, 8, -9]",
        0,
        2,
        "[11; -1/12; 13]",
        "[1/2, 2, 11; 4, 5/3, -1/12; 7, 8, 13]",
    );
    test(
        "[1/2, 2, 3; 4, 5/3, 6; 7, 8, -9]",
        2,
        0,
        "[0]",
        "[1/2, 2, 3; 4, 5/3, 6; 0, 8, -9]",
    );
    // - the whole matrix
    test(
        "[1/2, 2, 3; 4, 5/3, 6; 7, 8, -9]",
        0,
        0,
        "[5, 6/7, 7; 8, 9, -10; 11, 12, 13/2]",
        "[5, 6/7, 7; 8, 9, -10; 11, 12, 13/2]",
    );
    test(
        "[1/2, 2, 3; 4, 5/3, 6; 7, 8, -9]",
        2,
        1,
        "[20, 1/21]",
        "[1/2, 2, 3; 4, 5/3, 6; 7, 20, 1/21]",
    );
    // - an empty window in the corner
    test(
        "[1/2, 2, 3; 4, 5/3, 6; 7, 8, -9]",
        3,
        3,
        "[]_{0,0}",
        "[1/2, 2, 3; 4, 5/3, 6; 7, 8, -9]",
    );
    test(
        "[1/2, 2, 3; 4, 5/3, 6; 7, 8, -9]",
        1,
        2,
        "[]_{2,0}",
        "[1/2, 2, 3; 4, 5/3, 6; 7, 8, -9]",
    );
}

#[test]
#[should_panic]
fn set_window_fail_1() {
    let mut m = RationalMatrix::from_str("[1/2, 2, 3; 4, 5/3, 6; 7, 8, -9]").unwrap();
    let w = RationalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    m.set_window(2, 0, &w);
}

#[test]
#[should_panic]
fn set_window_fail_2() {
    let mut m = RationalMatrix::from_str("[1/2, 2, 3; 4, 5/3, 6; 7, 8, -9]").unwrap();
    let w = RationalMatrix::from_str("[1, 2, 3]").unwrap();
    m.set_window(0, 1, &w);
}

#[test]
#[should_panic]
fn set_window_fail_3() {
    let mut m = RationalMatrix::from_str("[1/2, 2, 3; 4, 5/3, 6; 7, 8, -9]").unwrap();
    let w = RationalMatrix::from_str("[]_{0,0}").unwrap();
    m.set_window(4, 0, &w);
}

#[test]
#[should_panic]
fn set_window_fail_4() {
    let mut m = RationalMatrix::from_str("[1/2, 2, 3; 4, 5/3, 6; 7, 8, -9]").unwrap();
    let w = RationalMatrix::from_str("[]_{0,0}").unwrap();
    m.set_window(0, 4, &w);
}

#[test]
#[should_panic]
fn set_window_fail_5() {
    let mut m = RationalMatrix::from_str("[1/2, 2, 3; 4, 5/3, 6; 7, 8, -9]").unwrap();
    let w = RationalMatrix::from_str("[]_{0,0}").unwrap();
    m.set_window(u64::MAX, 0, &w);
}

#[test]
#[should_panic]
fn set_window_fail_6() {
    let mut m = RationalMatrix::from_str("[1/2, 2, 3; 4, 5/3, 6; 7, 8, -9]").unwrap();
    let w = RationalMatrix::from_str("[]_{0,0}").unwrap();
    m.set_window(0, u64::MAX, &w);
}

#[test]
#[should_panic]
fn set_window_owned_fail_1() {
    let mut m = RationalMatrix::from_str("[1/2, 2, 3; 4, 5/3, 6; 7, 8, -9]").unwrap();
    let w = RationalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    m.set_window_owned(2, 0, w);
}

#[test]
#[should_panic]
fn set_window_owned_fail_2() {
    let mut m = RationalMatrix::from_str("[1/2, 2, 3; 4, 5/3, 6; 7, 8, -9]").unwrap();
    let w = RationalMatrix::from_str("[1, 2, 3]").unwrap();
    m.set_window_owned(0, 1, w);
}

#[test]
#[should_panic]
fn set_window_owned_fail_3() {
    let mut m = RationalMatrix::from_str("[1/2, 2, 3; 4, 5/3, 6; 7, 8, -9]").unwrap();
    let w = RationalMatrix::from_str("[]_{0,0}").unwrap();
    m.set_window_owned(4, 0, w);
}

#[test]
#[should_panic]
fn set_window_owned_fail_4() {
    let mut m = RationalMatrix::from_str("[1/2, 2, 3; 4, 5/3, 6; 7, 8, -9]").unwrap();
    let w = RationalMatrix::from_str("[]_{0,0}").unwrap();
    m.set_window_owned(0, 4, w);
}

#[test]
#[should_panic]
fn set_window_owned_fail_5() {
    let mut m = RationalMatrix::from_str("[1/2, 2, 3; 4, 5/3, 6; 7, 8, -9]").unwrap();
    let w = RationalMatrix::from_str("[]_{0,0}").unwrap();
    m.set_window_owned(u64::MAX, 0, w);
}

#[test]
#[should_panic]
fn set_window_owned_fail_6() {
    let mut m = RationalMatrix::from_str("[1/2, 2, 3; 4, 5/3, 6; 7, 8, -9]").unwrap();
    let w = RationalMatrix::from_str("[]_{0,0}").unwrap();
    m.set_window_owned(0, u64::MAX, w);
}

#[test]
fn set_window_properties() {
    rational_matrix_rational_matrix_unsigned_unsigned_quadruple_gen_var_1().test_properties(
        |(m, w, i, j)| {
            let (window_height, window_width) = (w.height(), w.width());
            let mut m_new = m.clone();
            m_new.set_window(i, j, &w);
            assert!(m_new.is_valid());
            assert_eq!((m_new.height(), m_new.width()), (m.height(), m.width()));
            assert_eq!(m_new.window(i..i + window_height, j..j + window_width), w);
            for r in 0..m.height() {
                for c in 0..m.width() {
                    if !(i..i + window_height).contains(&r) || !(j..j + window_width).contains(&c) {
                        assert_eq!(m_new.get(r, c), m.get(r, c));
                    }
                }
            }

            let mut m_alt = m.clone();
            m_alt.set_window_owned(i, j, w.clone());
            assert!(m_alt.is_valid());
            assert_eq!(m_alt, m_new);

            m_alt.set_window(i, j, &m.window(i..i + window_height, j..j + window_width));
            assert_eq!(m_alt, m);

            let mut t = m.transpose();
            t.set_window(j, i, &w.transpose());
            assert_eq!(t.transpose(), m_new);
        },
    );

    rational_matrix_unsigned_unsigned_unsigned_unsigned_quintuple_gen_var_1().test_properties(
        |(m, r1, r2, c1, c2)| {
            let mut m_alt = m.clone();
            m_alt.set_window(r1, c1, &m.window(r1..r2, c1..c2));
            assert_eq!(m_alt, m);

            let mut m_alt = m.clone();
            m_alt.set_window_owned(r1, c1, m.window(r1..r2, c1..c2));
            assert_eq!(m_alt, m);
        },
    );
}
