// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_polynomial::RationalPolynomial;
use crate::test_util::rational_polynomial::arithmetic::evaluate::{
    evaluate_integer_polynomial_naive, evaluate_rational_polynomial_naive,
};
use alloc::vec::Vec;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;

// Evaluates a polynomial at each of `xs` with `evaluate_integer_polynomial_naive`.
pub fn evaluate_many_integer_polynomial_naive(
    p: &IntegerPolynomial,
    xs: &[Rational],
) -> Vec<Rational> {
    xs.iter()
        .map(|x| evaluate_integer_polynomial_naive(p, x))
        .collect()
}

// Evaluates a polynomial at each of `xs` with `evaluate_rational_polynomial_naive`.
pub fn evaluate_many_rational_polynomial_naive(
    p: &RationalPolynomial,
    xs: &[Rational],
) -> Vec<Rational> {
    xs.iter()
        .map(|x| evaluate_rational_polynomial_naive(p, x))
        .collect()
}

// Evaluates a polynomial at each of the `Integer`s `xs` with `evaluate_rational_polynomial_naive`.
pub fn evaluate_many_rational_polynomial_at_integers_naive(
    p: &RationalPolynomial,
    xs: &[Integer],
) -> Vec<Rational> {
    xs.iter()
        .map(|x| evaluate_rational_polynomial_naive(p, &Rational::from(x)))
        .collect()
}
