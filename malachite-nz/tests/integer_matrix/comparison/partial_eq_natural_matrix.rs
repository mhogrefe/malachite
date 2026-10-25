// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::integer_matrix_natural_matrix_pair_gen;

#[test]
fn test_partial_eq_natural_matrix() {
    let test = |s, t, out| {
        let m = IntegerMatrix::from_str(s).unwrap();
        let n = NaturalMatrix::from_str(t).unwrap();
        assert_eq!(m == n, out);
        assert_eq!(n == m, out);
    };
    test("[]_{0,0}", "[]_{0,0}", true);
    test("[]_{0,3}", "[]_{0,3}", true);
    // Matrices with no elements are equal only when their shapes match.
    test("[]_{0,3}", "[]_{3,0}", false);
    test("[]_{0,0}", "[0]", false);
    test("[1]", "[1]", true);
    test("[1, 2; 3, 4]", "[1, 2; 3, 4]", true);
    test("[1, 2; 3, 4]", "[1, 3; 2, 4]", false);
    // The same elements in a different shape are not equal.
    test("[1, 2, 3, 4]", "[1, 2; 3, 4]", false);
    test(
        "[123456789012345678901234567890; 0]",
        "[123456789012345678901234567890; 0]",
        true,
    );
    // A negative element matches nothing, including its absolute value.
    test("[-1]", "[1]", false);
    test("[1, 2; 3, -4]", "[1, 2; 3, 4]", false);
}

// Comparing with a converted matrix is the reference the direct comparison is checked against.
#[allow(clippy::cmp_owned, clippy::op_ref)]
#[test]
fn partial_eq_natural_matrix_properties() {
    integer_matrix_natural_matrix_pair_gen().test_properties(|(m, n)| {
        let eq = m == n;
        assert_eq!(n == m, eq);
        // Extra refs for type inference: with `IntegerMatrix: PartialEq<NaturalMatrix>` in scope,
        // `m == n` would look for that impl.
        assert_eq!(&m == &IntegerMatrix::from(n.clone()), eq);
        if eq {
            assert_eq!((m.height(), m.width()), (n.height(), n.width()));
            assert!(m.elements_ref().iter().all(|x| *x >= 0u32));
        }

        let n_m = IntegerMatrix::from(n.clone());
        assert!(n_m == n);
        assert!(n == n_m);
        // Transposing is a bijection, so it preserves both equality and inequality.
        assert_eq!(m.transpose() == n.transpose(), eq);
    });
}
