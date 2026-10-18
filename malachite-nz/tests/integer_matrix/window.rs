// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::ops::Range;
use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::test_util::generators::{
    integer_matrix_gen, integer_matrix_unsigned_unsigned_unsigned_unsigned_quintuple_gen_var_1,
};

#[test]
fn test_window() {
    let test = |s, rows: Range<u64>, columns: Range<u64>, out| {
        let m = IntegerMatrix::from_str(s).unwrap();
        let w = m.window(rows.clone(), columns.clone());
        assert!(w.is_valid());
        assert_eq!(w.to_string(), out);

        let w_alt = m.into_window(rows, columns);
        assert!(w_alt.is_valid());
        assert_eq!(w_alt, w);
    };
    test("[]_{0,0}", 0..0, 0..0, "[]_{0,0}");
    test("[]_{0,3}", 0..0, 1..3, "[]_{0,2}");
    test("[]_{3,0}", 1..3, 0..0, "[]_{2,0}");
    test("[7]", 0..1, 0..1, "[7]");
    test("[7]", 0..1, 1..1, "[]_{1,0}");
    test("[7]", 1..1, 0..1, "[]_{0,1}");
    // - every column, every row
    test(
        "[1, -2, 3; 4, 5, -6; 7, -8, 9]",
        0..3,
        0..3,
        "[1, -2, 3; 4, 5, -6; 7, -8, 9]",
    );
    // - every column, a prefix of the rows
    test(
        "[1, -2, 3; 4, 5, -6; 7, -8, 9]",
        0..2,
        0..3,
        "[1, -2, 3; 4, 5, -6]",
    );
    // - every column, not a prefix of the rows
    test(
        "[1, -2, 3; 4, 5, -6; 7, -8, 9]",
        1..3,
        0..3,
        "[4, 5, -6; 7, -8, 9]",
    );
    test("[1, -2, 3; 4, 5, -6; 7, -8, 9]", 2..2, 0..3, "[]_{0,3}");
    // - not every column
    test(
        "[1, -2, 3; 4, 5, -6; 7, -8, 9]",
        1..3,
        0..2,
        "[4, 5; 7, -8]",
    );
    test("[1, -2, 3; 4, 5, -6; 7, -8, 9]", 0..3, 1..2, "[-2; 5; -8]");
    test("[1, -2, 3; 4, 5, -6; 7, -8, 9]", 1..2, 1..2, "[5]");
    test("[1, -2, 3; 4, 5, -6; 7, -8, 9]", 0..2, 2..3, "[3; -6]");
    test("[1, -2, 3; 4, 5, -6; 7, -8, 9]", 1..3, 1..1, "[]_{2,0}");
    test("[1, -2, 3, -4; 5, 6, -7, 8]", 0..2, 1..3, "[-2, 3; 6, -7]");
    test("[1, -2, 3, -4; 5, 6, -7, 8]", 1..2, 0..4, "[5, 6, -7, 8]");
    test("[1, -2, 3, -4; 5, 6, -7, 8]", 0..1, 2..4, "[3, -4]");
}

#[test]
#[should_panic]
fn window_fail_1() {
    let m = IntegerMatrix::from_str("[1, -2, 3; 4, 5, -6; 7, -8, 9]").unwrap();
    let _ = m.window(Range { start: 2, end: 1 }, 0..3);
}

#[test]
#[should_panic]
fn window_fail_2() {
    let m = IntegerMatrix::from_str("[1, -2, 3; 4, 5, -6; 7, -8, 9]").unwrap();
    let _ = m.window(0..4, 0..3);
}

#[test]
#[should_panic]
fn window_fail_3() {
    let m = IntegerMatrix::from_str("[1, -2, 3; 4, 5, -6; 7, -8, 9]").unwrap();
    let _ = m.window(0..3, Range { start: 2, end: 1 });
}

#[test]
#[should_panic]
fn window_fail_4() {
    let m = IntegerMatrix::from_str("[1, -2, 3; 4, 5, -6; 7, -8, 9]").unwrap();
    let _ = m.window(0..3, 0..4);
}

#[test]
#[should_panic]
fn into_window_fail_1() {
    let m = IntegerMatrix::from_str("[1, -2, 3; 4, 5, -6; 7, -8, 9]").unwrap();
    let _ = m.into_window(Range { start: 2, end: 1 }, 0..3);
}

#[test]
#[should_panic]
fn into_window_fail_2() {
    let m = IntegerMatrix::from_str("[1, -2, 3; 4, 5, -6; 7, -8, 9]").unwrap();
    let _ = m.into_window(0..4, 0..3);
}

#[test]
#[should_panic]
fn into_window_fail_3() {
    let m = IntegerMatrix::from_str("[1, -2, 3; 4, 5, -6; 7, -8, 9]").unwrap();
    let _ = m.into_window(0..3, Range { start: 2, end: 1 });
}

#[test]
#[should_panic]
fn into_window_fail_4() {
    let m = IntegerMatrix::from_str("[1, -2, 3; 4, 5, -6; 7, -8, 9]").unwrap();
    let _ = m.into_window(0..3, 0..4);
}

#[test]
fn window_properties() {
    integer_matrix_unsigned_unsigned_unsigned_unsigned_quintuple_gen_var_1().test_properties(
        |(m, r1, r2, c1, c2)| {
            let w = m.window(r1..r2, c1..c2);
            assert!(w.is_valid());
            assert_eq!((w.height(), w.width()), (r2 - r1, c2 - c1));
            for i in 0..w.height() {
                for j in 0..w.width() {
                    assert_eq!(w.get(i, j), m.get(r1 + i, c1 + j));
                }
            }

            let w_alt = m.clone().into_window(r1..r2, c1..c2);
            assert!(w_alt.is_valid());
            assert_eq!(w_alt, w);

            assert_eq!(m.transpose().window(c1..c2, r1..r2), w.transpose());
            assert_eq!(m.window(r1..r2, 0..m.width()).window(0..r2 - r1, c1..c2), w);
            assert_eq!(
                m.window(0..m.height(), c1..c2).window(r1..r2, 0..c2 - c1),
                w
            );
        },
    );

    integer_matrix_gen().test_properties(|m| {
        assert_eq!(m.window(0..m.height(), 0..m.width()), m);
        assert_eq!(m.clone().into_window(0..m.height(), 0..m.width()), m);
        for i in 0..m.height() {
            assert_eq!(
                m.window(i..i + 1, 0..m.width()).into_elements(),
                m.row(i).elements
            );
        }
        for j in 0..m.width() {
            assert_eq!(
                m.window(0..m.height(), j..j + 1).into_elements(),
                m.column(j).elements
            );
        }
    });
}
