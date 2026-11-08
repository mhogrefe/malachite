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
use malachite_q::test_util::generators::rational_matrix_unsigned_unsigned_triple_gen_var_1;

#[test]
fn test_without_row_and_column() {
    let test = |s, i, j, out| {
        let m = RationalMatrix::from_str(s).unwrap();
        let w = m.without_row_and_column(i, j);
        assert!(w.is_valid());
        assert_eq!(w.to_string(), out);

        let w_alt = m.into_without_row_and_column(i, j);
        assert!(w_alt.is_valid());
        assert_eq!(w_alt, w);
    };
    test("[7]", 0, 0, "[]_{0,0}");
    test("[1, 2, 3]", 0, 1, "[]_{0,2}");
    test("[1; 2; 3]", 1, 0, "[]_{2,0}");
    test(
        "[1/2, -2, 3; 4, 5/3, -6; 7, -8, 9]",
        0,
        0,
        "[5/3, -6; -8, 9]",
    );
    test("[1/2, -2, 3; 4, 5/3, -6; 7, -8, 9]", 0, 1, "[4, -6; 7, 9]");
    test(
        "[1/2, -2, 3; 4, 5/3, -6; 7, -8, 9]",
        1,
        2,
        "[1/2, -2; 7, -8]",
    );
    test("[1/2, -2, 3; 4, 5/3, -6; 7, -8, 9]", 1, 1, "[1/2, 3; 7, 9]");
    test(
        "[1/2, -2, 3; 4, 5/3, -6; 7, -8, 9]",
        2,
        2,
        "[1/2, -2; 4, 5/3]",
    );
    // - not square
    test("[1/2, -2, 3, -4; 5, 6, -7, 8]", 0, 1, "[5, -7, 8]");
    test("[1/2, -2, 3, -4; 5, 6, -7, 8]", 1, 3, "[1/2, -2, 3]");
}

#[test]
#[should_panic]
fn without_row_and_column_fail_1() {
    let m = RationalMatrix::from_str("[1/2, -2, 3; 4, 5/3, -6; 7, -8, 9]").unwrap();
    let _ = m.without_row_and_column(3, 0);
}

#[test]
#[should_panic]
fn without_row_and_column_fail_2() {
    let m = RationalMatrix::from_str("[1/2, -2, 3; 4, 5/3, -6; 7, -8, 9]").unwrap();
    let _ = m.without_row_and_column(0, 3);
}

#[test]
#[should_panic]
fn into_without_row_and_column_fail_1() {
    let m = RationalMatrix::from_str("[1/2, -2, 3; 4, 5/3, -6; 7, -8, 9]").unwrap();
    let _ = m.into_without_row_and_column(3, 0);
}

#[test]
#[should_panic]
fn into_without_row_and_column_fail_2() {
    let m = RationalMatrix::from_str("[1/2, -2, 3; 4, 5/3, -6; 7, -8, 9]").unwrap();
    let _ = m.into_without_row_and_column(0, 3);
}

#[test]
#[should_panic]
fn without_row_and_column_fail_3() {
    let m = RationalMatrix::from_str("[]_{0,0}").unwrap();
    let _ = m.without_row_and_column(0, 0);
}

#[test]
fn without_row_and_column_properties() {
    rational_matrix_unsigned_unsigned_triple_gen_var_1().test_properties(|(m, i, j)| {
        let (i, j) = (u64::try_from(i).unwrap(), u64::try_from(j).unwrap());
        let w = m.without_row_and_column(i, j);
        assert!(w.is_valid());
        assert_eq!((w.height(), w.width()), (m.height() - 1, m.width() - 1));
        for r in 0..w.height() {
            for c in 0..w.width() {
                assert_eq!(
                    w.get(r, c),
                    m.get(r + u64::from(r >= i), c + u64::from(c >= j))
                );
            }
        }

        let w_alt = m.clone().into_without_row_and_column(i, j);
        assert!(w_alt.is_valid());
        assert_eq!(w_alt, w);

        assert_eq!(m.transpose().without_row_and_column(j, i), w.transpose());
    });
}
