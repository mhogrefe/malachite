// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::polynomial::arithmetic::traits::{Evaluate, EvaluateMany};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::integer_polynomial_integer_vec_pair_gen;
use malachite_nz::test_util::integer_polynomial::arithmetic::evaluate_many::evaluate_many_naive;

#[test]
fn test_evaluate_many() {
    let test = |s, xs: &[i32], out: &[i32]| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let xs: Vec<Integer> = xs.iter().map(|&x| Integer::from(x)).collect();
        let out: Vec<Integer> = out.iter().map(|&y| Integer::from(y)).collect();
        assert_eq!((&p).evaluate_many(&xs), out);
        assert_eq!(evaluate_many_naive(&p, &xs), out);
    };
    test("x^2-3*x+2", &[], &[]);
    test("0", &[-1, 5], &[0, 0]);
    test("x^2-3*x+2", &[-1, 0, 1, 2, 3], &[6, 2, 0, 0, 2]);
}

#[test]
fn evaluate_many_properties() {
    integer_polynomial_integer_vec_pair_gen().test_properties(|(p, xs)| {
        let ys = (&p).evaluate_many(&xs);
        assert_eq!(ys.len(), xs.len());
        for (x, y) in xs.iter().zip(&ys) {
            assert_eq!((&p).evaluate(x), *y);
        }
        assert_eq!(evaluate_many_naive(&p, &xs), ys);
    });
}
