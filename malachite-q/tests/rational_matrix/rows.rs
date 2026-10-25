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
use malachite_q::test_util::generators::rational_matrix_gen;

fn rows_to_string(m: &RationalMatrix, reverse: bool) -> String {
    if reverse {
        prefix_to_string(m.rows().rev().map(|xs| prefix_to_string(xs, 100)), 100)
    } else {
        prefix_to_string(m.rows().map(|xs| prefix_to_string(xs, 100)), 100)
    }
}

#[test]
fn test_rows() {
    let test = |s, out, out_rev| {
        let m = RationalMatrix::from_str(s).unwrap();
        assert_eq!(rows_to_string(&m, false), out);
        assert_eq!(rows_to_string(&m, true), out_rev);
        assert_eq!(u64::exact_from(m.rows().len()), m.height());
    };
    test("[]_{0,0}", "[]", "[]");
    test("[]_{3,0}", "[[], [], []]", "[[], [], []]");
    test("[]_{0,4}", "[]", "[]");
    test("[5]", "[[5]]", "[[5]]");
    test(
        "[1, 2, 3; 4, 5, 6]",
        "[[1, 2, 3], [4, 5, 6]]",
        "[[4, 5, 6], [1, 2, 3]]",
    );
    test(
        "[1, 2; 3, 4; 5, 6]",
        "[[1, 2], [3, 4], [5, 6]]",
        "[[5, 6], [3, 4], [1, 2]]",
    );
    test(
        "[1/2, -2; 3, 4/3]",
        "[[1/2, -2], [3, 4/3]]",
        "[[3, 4/3], [1/2, -2]]",
    );
}

#[test]
fn test_rows_mut() {
    // Multiply each row by its position in the iteration, counting from 1, going forward or
    // backward.
    let test = |s, out, out_rev| {
        let mut m = RationalMatrix::from_str(s).unwrap();
        for (xs, k) in m.rows_mut().zip(1u32..) {
            for x in xs {
                *x *= Rational::from(k);
            }
        }
        assert_eq!(m.to_string(), out);

        let mut m = RationalMatrix::from_str(s).unwrap();
        for (xs, k) in m.rows_mut().rev().zip(1u32..) {
            for x in xs {
                *x *= Rational::from(k);
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
        "[1, 2, 3; 8, 10, 12]",
        "[2, 4, 6; 4, 5, 6]",
    );
    test(
        "[1, 2; 3, 4; 5, 6]",
        "[1, 2; 6, 8; 15, 18]",
        "[3, 6; 6, 8; 5, 6]",
    );
    test("[1/2, -2; 3, 4/3]", "[1/2, -2; 6, 8/3]", "[1, -4; 3, 4/3]");
}

#[test]
fn rows_properties() {
    rational_matrix_gen().test_properties(|mut m| {
        let n = m.height();
        assert_eq!(u64::exact_from(m.rows().len()), n);
        assert_eq!(u64::exact_from(m.rows_mut().len()), n);
        assert!(m.rows().zip(0..).all(|(xs, k)| xs.eq(m.row_iter(k))));
        assert!(
            m.rows()
                .rev()
                .zip((0..n).rev())
                .all(|(xs, k)| xs.eq(m.row_iter(k)))
        );
        // Flattening the rows gives the elements.
        assert!(m.rows().flatten().eq(m.elements_ref().iter()));

        // Taking from both ends meets in the middle, with the length always exact.
        let mut it = m.rows();
        let mut k = 0;
        let mut l = n;
        while k < l {
            assert_eq!(u64::exact_from(it.len()), l - k);
            if (k + l) % 2 == 0 {
                assert!(it.next().unwrap().eq(m.row_iter(k)));
                k += 1;
            } else {
                l -= 1;
                assert!(it.next_back().unwrap().eq(m.row_iter(l)));
            }
        }
        assert!(it.next().is_none());
        assert!(it.next_back().is_none());

        let old = m.clone();
        assert!(
            m.rows_mut()
                .zip(old.rows())
                .all(|(xs, ys)| xs.map(|x| &*x).eq(ys))
        );
        assert!(
            m.rows_mut()
                .rev()
                .zip(old.rows().rev())
                .all(|(xs, ys)| xs.map(|x| &*x).eq(ys))
        );

        // Changing every element through `rows_mut` is the same as changing `elements`.
        for xs in m.rows_mut() {
            for x in xs {
                *x += Rational::ONE;
            }
        }
        let expected = RationalMatrix::from_owned_elements(
            old.height(),
            old.width(),
            old.elements_ref()
                .iter()
                .map(|x| x + Rational::ONE)
                .collect(),
        );
        assert_eq!(m, expected);
        for xs in m.rows_mut().rev() {
            for x in xs.rev() {
                *x -= Rational::ONE;
            }
        }
        assert_eq!(m, old);
    });
}
