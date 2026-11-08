// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Cross, Dot, GeneralizedCross};
use malachite_base::num::basic::traits::{OneHalf, Two};
use malachite_base::num::linear_algebra::traits::{AreLinearlyIndependent, Determinant};
use malachite_base::vector::Vector;
use malachite_q::Rational;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::rational_matrix_gen_var_5;
use malachite_q::test_util::rational_vector::arithmetic::generalized_cross::*;
use std::panic::catch_unwind;

fn vectors(xs: &[&str]) -> Vec<RationalVector> {
    xs.iter()
        .map(|s| RationalVector::from_str(s).unwrap())
        .collect()
}

#[test]
fn test_generalized_cross() {
    let test = |xs: &[&str], out: &str| {
        let xs = vectors(xs);
        let w = RationalVector::generalized_cross(&xs);
        assert_eq!(w.to_string(), out);
        assert_eq!(generalized_cross_by_determinants(&xs), w);
    };
    test(&[], "(1)");
    test(&["(1/2, 3)"], "(-3, 1/2)");
    test(&["(1/2, 0, 0)", "(0, 1/3, 0)"], "(0, 0, 1/6)");
    test(&["(1/2, 1/3, 1/4)", "(1, 2/3, 1/2)"], "(0, 0, 0)");
    test(&["(1, -2, 3)", "(4, 5, -6)"], "(-3, 18, 13)");
    test(
        &["(1/2, 1, 0, 0)", "(0, 1/3, 1, 0)", "(0, 0, 1/5, 1)"],
        "(-1, 1/2, -1/6, 1/30)",
    );
}

#[test]
fn generalized_cross_fail() {
    assert_panic!(RationalVector::generalized_cross(&vectors(&["(1, 2, 3)"])));
}

#[test]
fn generalized_cross_properties() {
    rational_matrix_gen_var_5().test_properties(|a| {
        let xs: Vec<RationalVector> = a
            .rows()
            .map(|row| RationalVector::from_elements(row.as_slice()))
            .collect();
        let n = xs.len() + 1;
        let w = RationalVector::generalized_cross(&xs);
        assert_eq!(w.elements.len(), n);
        assert_eq!(generalized_cross_by_determinants(&xs), w);
        for x in &xs {
            assert_eq!(x.dot(&w), 0u32);
        }
        let mut rows = xs.clone();
        rows.push(w.clone());
        assert_eq!(RationalMatrix::from_rows(&rows).determinant(), (&w).dot(&w));
        assert_eq!(
            RationalVector::are_linearly_independent(&xs),
            w.elements.iter().any(|e| *e != 0u32)
        );
        if n == 3 {
            assert_eq!((&xs[0]).cross(&xs[1]), w);
        }
        // Scaling a vector scales w by the same factor.
        if let Some(first) = xs.first() {
            let mut scaled = xs.clone();
            scaled[0] = first * &Rational::ONE_HALF;
            assert_eq!(
                RationalVector::generalized_cross(&scaled) * Rational::TWO,
                w
            );
        }
    });
}
