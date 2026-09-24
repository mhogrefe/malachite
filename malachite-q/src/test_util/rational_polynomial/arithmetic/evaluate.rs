// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_polynomial::RationalPolynomial;
use malachite_base::num::arithmetic::traits::Pow;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::integer_polynomial::IntegerPolynomial;

// Evaluates an integer polynomial at a rational term by term, as the sum of $c_i x^i$ computed in
// `Rational`s, with each power of $x$ computed from scratch. This shares nothing with Horner's rule
// or the divide-and-conquer scheme, which work on numerators and denominators separately, so it can
// check both.
pub fn evaluate_integer_polynomial_naive(p: &IntegerPolynomial, x: &Rational) -> Rational {
    let mut sum = Rational::ZERO;
    for (i, c) in p.coefficients_asc().iter().enumerate() {
        sum += Rational::from(c) * x.pow(u64::exact_from(i));
    }
    sum
}

// Evaluates a rational polynomial at a rational term by term, as the sum of $c_i x^i$ over its
// `Rational` coefficients, with each power of $x$ computed from scratch. It never separates the
// numerator from the denominator, as the evaluation does.
pub fn evaluate_rational_polynomial_naive(p: &RationalPolynomial, x: &Rational) -> Rational {
    let mut sum = Rational::ZERO;
    for (i, c) in p.to_coefficients_asc().into_iter().enumerate() {
        sum += c * x.pow(u64::exact_from(i));
    }
    sum
}
