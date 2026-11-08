// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Lcm;
use malachite_base::num::basic::traits::One;
use malachite_base::num::linear_algebra::traits::AreLinearlyIndependent;
use malachite_base::vector::Vector;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::rational_matrix_gen;
use std::panic::catch_unwind;

fn vectors(xs: &[&str]) -> Vec<RationalVector> {
    xs.iter()
        .map(|s| RationalVector::from_str(s).unwrap())
        .collect()
}

#[test]
fn test_are_linearly_independent() {
    let test = |xs: &[&str], out: bool| {
        assert_eq!(RationalVector::are_linearly_independent(&vectors(xs)), out);
    };
    test(&[], true);
    test(&["(0, 0)"], false);
    test(&["(1/2, 1)", "(1, 1/2)"], true);
    test(&["(1/2, 1)", "(1, 2)"], false);
    test(&["(1/3, -1/7)"], true);
}

#[test]
fn are_linearly_independent_fail() {
    assert_panic!(RationalVector::are_linearly_independent(&vectors(&[
        "(1, 2)", "(1)"
    ])));
}

#[test]
fn are_linearly_independent_properties() {
    rational_matrix_gen().test_properties(|matrix| {
        let xs: Vec<RationalVector> = matrix
            .rows()
            .map(|row| RationalVector::from_elements(row.as_slice()))
            .collect();
        let independent = RationalVector::are_linearly_independent(&xs);
        // Scaling each vector by the lcm of its denominators gives integer vectors with the same
        // span.
        let integers: Vec<IntegerVector> = xs
            .iter()
            .map(|x| {
                let l = x
                    .elements_ref()
                    .iter()
                    .fold(Natural::ONE, |l, e| l.lcm(e.denominator_ref()));
                let l = Rational::from(l);
                IntegerVector::from_owned_elements(
                    x.elements_ref()
                        .iter()
                        .map(|e| (e * &l).try_into().unwrap())
                        .collect(),
                )
            })
            .collect();
        assert_eq!(
            IntegerVector::are_linearly_independent(&integers),
            independent
        );
    });
}
