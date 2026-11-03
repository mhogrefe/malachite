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
use malachite_q::Rational;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::{
    integer_polynomial_rational_vec_pair_gen,
    rational_polynomial_integer_vec_pair_gen,
    rational_polynomial_rational_vec_pair_gen,
};
use malachite_q::test_util::rational_polynomial::arithmetic::evaluate_many::*;

fn rationals(xs: &[&str]) -> Vec<Rational> {
    xs.iter().map(|x| Rational::from_str(x).unwrap()).collect()
}

#[test]
fn test_integer_polynomial_evaluate_many_rational() {
    let test = |s, xs: &[&str], out: &[&str]| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let xs = rationals(xs);
        let out = rationals(out);
        assert_eq!((&p).evaluate_many(&xs), out);
        assert_eq!(evaluate_many_integer_polynomial_naive(&p, &xs), out);
    };
    test("x^2-3*x+2", &[], &[]);
    test("0", &["1/2", "-3"], &["0", "0"]);
    test("x^2-3*x+2", &["1/2", "-1", "3/2"], &["3/4", "6", "-1/4"]);
}

#[test]
fn integer_polynomial_evaluate_many_rational_properties() {
    integer_polynomial_rational_vec_pair_gen().test_properties(|(p, xs)| {
        let ys = (&p).evaluate_many(&xs);
        assert_eq!(ys.len(), xs.len());
        for (x, y) in xs.iter().zip(&ys) {
            assert!(y.is_valid());
            assert_eq!((&p).evaluate(x), *y);
        }
        assert_eq!(evaluate_many_integer_polynomial_naive(&p, &xs), ys);
    });
}

#[test]
fn test_rational_polynomial_evaluate_many() {
    let test = |s, xs: &[&str], out: &[&str]| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let xs = rationals(xs);
        let out = rationals(out);
        assert_eq!((&p).evaluate_many(&xs), out);
        assert_eq!(evaluate_many_rational_polynomial_naive(&p, &xs), out);
    };
    test("1/2*x^2+1/3", &[], &[]);
    test("0", &["1/2", "-3"], &["0", "0"]);
    test("1/2*x^2+1/3", &["0", "1", "-2/3"], &["1/3", "5/6", "5/9"]);
}

#[test]
fn rational_polynomial_evaluate_many_properties() {
    rational_polynomial_rational_vec_pair_gen().test_properties(|(p, xs)| {
        let ys = (&p).evaluate_many(&xs);
        assert_eq!(ys.len(), xs.len());
        for (x, y) in xs.iter().zip(&ys) {
            assert!(y.is_valid());
            assert_eq!((&p).evaluate(x), *y);
        }
        assert_eq!(evaluate_many_rational_polynomial_naive(&p, &xs), ys);
    });
}

#[test]
fn test_rational_polynomial_evaluate_many_integer() {
    let test = |s, xs: &[i32], out: &[&str]| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let xs: Vec<Integer> = xs.iter().map(|&x| Integer::from(x)).collect();
        let out = rationals(out);
        assert_eq!((&p).evaluate_many(&xs), out);
        assert_eq!(
            evaluate_many_rational_polynomial_at_integers_naive(&p, &xs),
            out
        );
    };
    test("1/2*x^2+1/3", &[], &[]);
    test("0", &[1, -3], &["0", "0"]);
    test("1/2*x^2+1/3", &[0, 1, 2], &["1/3", "5/6", "7/3"]);
}

#[test]
fn rational_polynomial_evaluate_many_integer_properties() {
    rational_polynomial_integer_vec_pair_gen().test_properties(|(p, xs)| {
        let ys = (&p).evaluate_many(&xs);
        assert_eq!(ys.len(), xs.len());
        for (x, y) in xs.iter().zip(&ys) {
            assert!(y.is_valid());
            assert_eq!((&p).evaluate(x), *y);
            // Evaluating at an `Integer` is evaluating at the equal `Rational`.
            assert_eq!((&p).evaluate(&Rational::from(x)), *y);
        }
        assert_eq!(
            evaluate_many_rational_polynomial_at_integers_naive(&p, &xs),
            ys
        );
    });
}
