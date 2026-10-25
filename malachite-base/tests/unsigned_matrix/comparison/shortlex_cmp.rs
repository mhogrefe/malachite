// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::test_util::common::test_custom_cmp_helper;
use malachite_base::test_util::generators::{
    unsigned_matrix_gen, unsigned_matrix_pair_gen, unsigned_matrix_triple_gen,
};
use malachite_base::test_util::unsigned_matrix::comparison::shortlex_cmp::*;
use malachite_base::unsigned_matrix::{
    ShortlexUnsignedMatrix, ShortlexUnsignedMatrixRef, UnsignedMatrix,
};
use malachite_base::unsigned_vector::ShortlexUnsignedVectorRef;

fn shortlex(m: &UnsignedMatrix<u64>, n: &UnsignedMatrix<u64>) -> Ordering {
    ShortlexUnsignedMatrixRef(m).cmp(&ShortlexUnsignedMatrixRef(n))
}

#[test]
fn test_shortlex_cmp() {
    // In ascending order: by number of elements, then height, then width, and then by the elements
    // in row-major order.
    test_custom_cmp_helper::<UnsignedMatrix<u64>, _>(
        &[
            "[]_{0,0}",
            "[]_{0,1}",
            "[]_{0,5}",
            "[]_{1,0}",
            "[]_{3,0}",
            "[0]",
            "[1]",
            "[100]",
            "[18446744073709551615]",
            "[0, 0]",
            "[0, 1]",
            "[5, 5]",
            "[0; 0]",
            "[1; 0]",
            "[0, 0, 0]",
            "[0; 0; 0]",
            "[1, 2, 3, 4]",
            "[0, 0; 0, 0]",
            "[0, 0; 0, 1]",
            "[0; 0; 0; 0]",
        ],
        shortlex,
    );
}

#[test]
fn test_shortlex_cmp_vs_lexicographic() {
    // The order agrees with the lexicographic order of the elements only at equal shapes.
    let test = |s, t, shortlex_out, lex_out| {
        let m = UnsignedMatrix::<u64>::from_str(s).unwrap();
        let n = UnsignedMatrix::<u64>::from_str(t).unwrap();
        assert_eq!(shortlex(&m, &n), shortlex_out, "shortlex {s} vs {t}");
        assert_eq!(
            m.elements_ref().cmp(n.elements_ref()),
            lex_out,
            "lexicographic {s} vs {t}"
        );
    };
    // The number of elements decides first, whatever the elements are.
    test("[5]", "[0, 0]", Less, Greater);
    // Then the height, even when the elements are the same.
    test("[1, 2, 3]", "[1; 2; 3]", Less, Equal);
    test("[1, 2; 3, 4]", "[1, 2, 3, 4]", Greater, Equal);
    // Matrices with no elements are ordered by height, then width.
    test("[]_{0,5}", "[]_{1,0}", Less, Equal);
    test("[]_{0,2}", "[]_{0,1}", Greater, Equal);
    // At equal shapes, the elements decide, in row-major order.
    test("[0, 1; 0, 0]", "[0, 0; 1, 0]", Greater, Greater);
    test("[3, 4]", "[3, 4]", Equal, Equal);
}

#[test]
fn shortlex_cmp_properties() {
    unsigned_matrix_pair_gen().test_properties(|(m, n)| {
        let c = shortlex(&m, &n);
        // Comparison is antisymmetric, and agrees with `Eq`.
        assert_eq!(shortlex(&n, &m), c.reverse());
        assert_eq!(m == n, c == Equal);

        // The default comparison gives the same answer as an explicit walk.
        assert_eq!(unsigned_matrix_shortlex_cmp_naive(&m, &n), c);

        // The owned wrapper and the borrowing one agree, and both agree with `as_ref`.
        let owned = ShortlexUnsignedMatrix(m.clone());
        let other_owned = ShortlexUnsignedMatrix(n.clone());
        assert_eq!(owned.cmp(&other_owned), c);
        assert_eq!(owned.as_ref().cmp(&other_owned.as_ref()), c);
        assert_eq!(owned.partial_cmp(&other_owned), Some(c));

        // Shortlex is lexicographic order on (number of elements, height, width, elements).
        assert_eq!(
            (
                m.elements_ref().len(),
                m.height(),
                m.width(),
                m.elements_ref()
            )
                .cmp(&(
                    n.elements_ref().len(),
                    n.height(),
                    n.width(),
                    n.elements_ref()
                )),
            c
        );

        // Matrices with one row compare as their rows do under the shortlex order of vectors.
        if m.height() == 1 && n.height() == 1 {
            assert_eq!(
                ShortlexUnsignedVectorRef(&m.row(0)).cmp(&ShortlexUnsignedVectorRef(&n.row(0))),
                c
            );
        }
    });

    unsigned_matrix_gen().test_properties(|m| {
        // Reflexivity, and the 0-by-0 matrix is the least of them all.
        assert_eq!(shortlex(&m, &m), Equal);
        assert!(
            ShortlexUnsignedMatrix(m) >= ShortlexUnsignedMatrix(UnsignedMatrix::<u64>::zero(0, 0))
        );
    });

    unsigned_matrix_triple_gen().test_properties(|(m, n, p)| {
        // Transitivity, in the two forms that make the order a total one.
        if shortlex(&m, &n) == Less && shortlex(&n, &p) == Less {
            assert_eq!(shortlex(&m, &p), Less);
        } else if shortlex(&m, &n) == Greater && shortlex(&n, &p) == Greater {
            assert_eq!(shortlex(&m, &p), Greater);
        }
    });
}
