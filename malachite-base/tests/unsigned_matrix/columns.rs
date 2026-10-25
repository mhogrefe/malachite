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
use malachite_base::test_util::generators::unsigned_matrix_gen;
use malachite_base::unsigned_matrix::UnsignedMatrix;

fn columns_to_string(m: &UnsignedMatrix<u64>, reverse: bool) -> String {
    if reverse {
        prefix_to_string(m.columns().rev().map(|xs| prefix_to_string(xs, 100)), 100)
    } else {
        prefix_to_string(m.columns().map(|xs| prefix_to_string(xs, 100)), 100)
    }
}

#[test]
fn test_columns() {
    let test = |s, out, out_rev| {
        let m = UnsignedMatrix::<u64>::from_str(s).unwrap();
        assert_eq!(columns_to_string(&m, false), out);
        assert_eq!(columns_to_string(&m, true), out_rev);
        assert_eq!(u64::exact_from(m.columns().len()), m.width());
    };
    test("[]_{0,0}", "[]", "[]");
    test("[]_{3,0}", "[]", "[]");
    test("[]_{0,4}", "[[], [], [], []]", "[[], [], [], []]");
    test("[5]", "[[5]]", "[[5]]");
    test(
        "[1, 2, 3; 4, 5, 6]",
        "[[1, 4], [2, 5], [3, 6]]",
        "[[3, 6], [2, 5], [1, 4]]",
    );
    test(
        "[1, 2; 3, 4; 5, 6]",
        "[[1, 3, 5], [2, 4, 6]]",
        "[[2, 4, 6], [1, 3, 5]]",
    );
}

#[test]
fn test_columns_mut() {
    // Multiply each column by its position in the iteration, counting from 1, going forward or
    // backward.
    let test = |s, out, out_rev| {
        let mut m = UnsignedMatrix::<u64>::from_str(s).unwrap();
        for (xs, k) in m.columns_mut().zip(1u32..) {
            for x in xs {
                *x *= u64::from(k);
            }
        }
        assert_eq!(m.to_string(), out);

        let mut m = UnsignedMatrix::<u64>::from_str(s).unwrap();
        for (xs, k) in m.columns_mut().rev().zip(1u32..) {
            for x in xs {
                *x *= u64::from(k);
            }
        }
        assert_eq!(m.to_string(), out_rev);
    };
    test("[]_{0,0}", "[]_{0,0}", "[]_{0,0}");
    test("[]_{3,0}", "[]_{3,0}", "[]_{3,0}");
    test("[]_{0,4}", "[]_{0,4}", "[]_{0,4}");
    test("[5]", "[5]", "[5]");
    test(
        "[1, 2, 3; 4, 5, 6]",
        "[1, 4, 9; 4, 10, 18]",
        "[3, 4, 3; 12, 10, 6]",
    );
    test(
        "[1, 2; 3, 4; 5, 6]",
        "[1, 4; 3, 8; 5, 12]",
        "[2, 2; 6, 4; 10, 6]",
    );
}

#[test]
fn columns_properties() {
    unsigned_matrix_gen().test_properties(|mut m| {
        let n = m.width();
        assert_eq!(u64::exact_from(m.columns().len()), n);
        assert_eq!(u64::exact_from(m.columns_mut().len()), n);
        assert!(m.columns().zip(0..).all(|(xs, k)| xs.eq(m.column_iter(k))));
        assert!(
            m.columns()
                .rev()
                .zip((0..n).rev())
                .all(|(xs, k)| xs.eq(m.column_iter(k)))
        );
        // Flattening the columns gives the elements of the transpose.
        assert!(
            m.columns()
                .flatten()
                .eq(m.transpose().elements_ref().iter())
        );

        // Taking from both ends meets in the middle, with the length always exact.
        let mut it = m.columns();
        let mut k = 0;
        let mut l = n;
        while k < l {
            assert_eq!(u64::exact_from(it.len()), l - k);
            if (k + l) % 2 == 0 {
                assert!(it.next().unwrap().eq(m.column_iter(k)));
                k += 1;
            } else {
                l -= 1;
                assert!(it.next_back().unwrap().eq(m.column_iter(l)));
            }
        }
        assert!(it.next().is_none());
        assert!(it.next_back().is_none());

        let old = m.clone();
        assert!(
            m.columns_mut()
                .zip(old.columns())
                .all(|(xs, ys)| xs.map(|x| &*x).eq(ys))
        );
        assert!(
            m.columns_mut()
                .rev()
                .zip(old.columns().rev())
                .all(|(xs, ys)| xs.map(|x| &*x).eq(ys))
        );

        // Changing every element through `columns_mut` is the same as changing `elements`.
        for xs in m.columns_mut() {
            for x in xs {
                *x ^= 1;
            }
        }
        let expected = UnsignedMatrix::from_owned_elements(
            old.height(),
            old.width(),
            old.elements_ref().iter().map(|x| x ^ 1).collect(),
        );
        assert_eq!(m, expected);
        for xs in m.columns_mut().rev() {
            for x in xs.rev() {
                *x ^= 1;
            }
        }
        assert_eq!(m, old);
    });
}
