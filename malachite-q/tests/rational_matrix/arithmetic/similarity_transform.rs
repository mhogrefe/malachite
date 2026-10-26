// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    Determinant, SimilarityTransform, SimilarityTransformAssign, Square, Trace,
};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::test_util::generators::integer_matrix_unsigned_integer_triple_gen_var_1;
use malachite_q::Rational;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::generators::rational_matrix_unsigned_rational_triple_gen_var_1;
use malachite_q::test_util::rational_matrix::arithmetic::similarity_transform::*;
use std::panic::catch_unwind;

// Checks every combination of value and reference, and the definition, and returns the result.
fn verify(a: &RationalMatrix, r: u64, d: &Rational) -> RationalMatrix {
    let b = a.similarity_transform(r, d);
    assert!(b.is_valid());
    assert_eq!(a.similarity_transform(r, d.clone()), b);
    assert_eq!(a.clone().similarity_transform(r, d), b);
    assert_eq!(a.clone().similarity_transform(r, d.clone()), b);
    let mut c = a.clone();
    c.similarity_transform_assign(r, d);
    assert_eq!(c, b);
    let mut c = a.clone();
    c.similarity_transform_assign(r, d.clone());
    assert_eq!(c, b);
    assert_eq!(
        rational_matrix_similarity_transform_by_definition(a, r, d),
        b
    );
    b
}

#[test]
fn test_similarity_transform() {
    let test = |s, r, d, out| {
        let a = RationalMatrix::from_str(s).unwrap();
        let d = Rational::from_str(d).unwrap();
        assert_eq!(verify(&a, r, &d).to_string(), out);
    };
    // - n = 1: nothing to mix
    test("[7/2]", 0, "100", "[7/2]");
    // - d = 0
    test("[1/2, 1; 3, 1/5]", 0, "0", "[1/2, 1; 3, 1/5]");
    // - n = 2 and r = 1: nothing to mix
    test("[1/2, 1; 3, 1/5]", 1, "7/3", "[1/2, 1; 3, 1/5]");
    // - r = 0
    test(
        "[1/2, 1/3, 1; 2, 3/4, 5; -1, 0, 2/3]",
        0,
        "1/2",
        "[0, -1/24, -11/6; 2, 7/4, 6; -1, -1/2, 1/6]",
    );
    // - r > 0: column r - 1 is left out
    test(
        "[1/2, 1/3, 1; 2, 3/4, 5; -1, 0, 2/3]",
        2,
        "-3",
        "[-5/2, 1/3, 1; -13, 3/4, 5; -21/2, 1, 11/3]",
    );
    // - an integer matrix, as for IntegerMatrix
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 10]",
        2,
        "5",
        "[16, 2, 3; 34, 5, 6; -23, -2, -5]",
    );
}

#[test]
fn similarity_transform_fail() {
    // - not square
    let a = RationalMatrix::from_str("[1/2, 2]").unwrap();
    assert_panic!(a.similarity_transform(0, Rational::ZERO));
    // - r too large
    let a = RationalMatrix::from_str("[1/2, 2; 3, 4]").unwrap();
    assert_panic!(a.similarity_transform(2, Rational::ZERO));
    // - the 0 by 0 matrix has no row index
    let a = RationalMatrix::from_str("[]_{0,0}").unwrap();
    assert_panic!(a.similarity_transform(0, Rational::ZERO));
}

#[test]
#[should_panic]
fn similarity_transform_assign_fail() {
    let mut a = RationalMatrix::from_str("[1/2, 2; 3, 4]").unwrap();
    a.similarity_transform_assign(2, Rational::ZERO);
}

#[test]
fn similarity_transform_properties() {
    rational_matrix_unsigned_rational_triple_gen_var_1().test_properties(|(a, r, d)| {
        let b = verify(&a, r, &d);
        // A similar matrix has the same invariants.
        assert_eq!((&b).determinant(), (&a).determinant());
        assert_eq!(b.to_trace(), a.to_trace());
        // The transform with -d undoes it.
        assert_eq!(b.similarity_transform(r, -&d), a);
        // A transform with d = 0 does nothing.
        assert_eq!((&a).similarity_transform(r, Rational::ZERO), a);
        // Conjugation respects products.
        assert_eq!(
            (&a).square().similarity_transform(r, &d),
            (&a).similarity_transform(r, &d).square()
        );
    });

    integer_matrix_unsigned_integer_triple_gen_var_1().test_properties(|(a, r, d)| {
        // The transform agrees with the transform of the IntegerMatrix.
        assert_eq!(
            RationalMatrix::from(&a).similarity_transform(r, Rational::from(&d)),
            RationalMatrix::from(a.similarity_transform(r, d))
        );
    });
}
