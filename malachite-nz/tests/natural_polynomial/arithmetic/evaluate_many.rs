// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::polynomial::arithmetic::traits::{Evaluate, EvaluateMany};
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::natural_polynomial_natural_vec_pair_gen;
use malachite_nz::test_util::natural_polynomial::arithmetic::evaluate_many::*;

#[test]
fn test_evaluate_many() {
    let test = |s, xs: &[u32], out: &[u32]| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let xs: Vec<Natural> = xs.iter().map(|&x| Natural::from(x)).collect();
        let out: Vec<Natural> = out.iter().map(|&y| Natural::from(y)).collect();
        assert_eq!((&p).evaluate_many(&xs), out);
        assert_eq!(evaluate_many_naive(&p, &xs), out);
    };
    test("5*x^2+3*x+7", &[], &[]);
    test("0", &[0, 5], &[0, 0]);
    test("5*x^2+3*x+7", &[0, 1, 2, 3], &[7, 15, 33, 61]);
}

#[test]
fn evaluate_many_properties() {
    natural_polynomial_natural_vec_pair_gen().test_properties(|(p, xs)| {
        let ys = (&p).evaluate_many(&xs);
        assert_eq!(ys.len(), xs.len());
        for (x, y) in xs.iter().zip(&ys) {
            assert_eq!((&p).evaluate(x), *y);
        }
        assert_eq!(evaluate_many_naive(&p, &xs), ys);
    });
}
