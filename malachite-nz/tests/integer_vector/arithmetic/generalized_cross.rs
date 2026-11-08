// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Cross, Dot, GeneralizedCross};
use malachite_base::num::linear_algebra::traits::{AreLinearlyIndependent, Determinant};
use malachite_base::vector::Vector;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::integer_matrix_gen_var_5;
use malachite_nz::test_util::integer_vector::arithmetic::generalized_cross::*;
use std::panic::catch_unwind;

fn vectors(xs: &[&str]) -> Vec<IntegerVector> {
    xs.iter()
        .map(|s| IntegerVector::from_str(s).unwrap())
        .collect()
}

#[test]
fn test_generalized_cross() {
    let test = |xs: &[&str], out: &str| {
        let xs = vectors(xs);
        let w = IntegerVector::generalized_cross(&xs);
        assert_eq!(w.to_string(), out);
        assert_eq!(generalized_cross_by_determinants(&xs), w);
    };
    // - no vectors
    test(&[], "(1)");
    // - n = 2: a quarter turn counterclockwise
    test(&["(3, 4)"], "(-4, 3)");
    test(&["(0, 0)"], "(0, 0)");
    // - n = 3: the cross product
    test(&["(1, 2, 3)", "(4, 5, 6)"], "(-3, 6, -3)");
    test(&["(1, 0, 0)", "(0, 1, 0)"], "(0, 0, 1)");
    // - linearly dependent vectors
    test(&["(1, 2, 3)", "(2, 4, 6)"], "(0, 0, 0)");
    test(
        &["(1, 2, 3, 4)", "(2, 4, 6, 8)", "(0, 1, 0, 1)"],
        "(0, 0, 0, 0)",
    );
    // - n = 4
    test(
        &["(1, 0, 0, 0)", "(0, 1, 0, 0)", "(0, 0, 1, 0)"],
        "(0, 0, 0, 1)",
    );
    test(
        &["(1, 2, 3, 4)", "(0, 1, -1, 2)", "(2, 0, 1, 1)"],
        "(5, 17, -1, -9)",
    );
}

#[test]
fn generalized_cross_fail() {
    assert_panic!(IntegerVector::generalized_cross(&vectors(&["(1, 2, 3)"])));
    assert_panic!(IntegerVector::generalized_cross(&vectors(&[
        "(1, 2, 3)",
        "(1, 2)"
    ])));
}

#[test]
fn generalized_cross_properties() {
    integer_matrix_gen_var_5().test_properties(|a| {
        let xs: Vec<IntegerVector> = a
            .rows()
            .map(|row| IntegerVector::from_elements(row.as_slice()))
            .collect();
        let n = xs.len() + 1;
        let w = IntegerVector::generalized_cross(&xs);
        assert_eq!(w.elements.len(), n);
        assert_eq!(generalized_cross_by_determinants(&xs), w);
        // w is perpendicular to every vector.
        for x in &xs {
            assert_eq!(x.dot(&w), 0u32);
        }
        // The vectors followed by w are positively oriented: their determinant is |w|^2.
        let mut rows = xs.clone();
        rows.push(w.clone());
        assert_eq!(IntegerMatrix::from_rows(&rows).determinant(), (&w).dot(&w));
        // w is zero exactly when the vectors are linearly dependent.
        assert_eq!(
            IntegerVector::are_linearly_independent(&xs),
            w.elements.iter().any(|e| *e != 0u32)
        );
        match n {
            2 => assert_eq!(w.elements, [-&xs[0].elements[1], xs[0].elements[0].clone()]),
            3 => assert_eq!((&xs[0]).cross(&xs[1]), w),
            _ => {}
        }
        // Swapping two vectors negates w.
        if xs.len() >= 2 {
            let mut swapped = xs.clone();
            swapped.swap(0, 1);
            assert_eq!(IntegerVector::generalized_cross(&swapped), -&w);
        }
    });
}
