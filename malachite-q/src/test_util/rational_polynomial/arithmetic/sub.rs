// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_polynomial::RationalPolynomial;
use alloc::vec::Vec;
use core::cmp::max;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;

// Subtracts two polynomials coefficient by coefficient, as `Rational`s, reading each coefficient
// through `coefficient`, which gives zero past the degree. Nothing is shared with the
// implementation.
pub fn sub_naive(p: &RationalPolynomial, q: &RationalPolynomial) -> RationalPolynomial {
    let len = max(p.len(), q.len());
    RationalPolynomial::from_coefficients_asc(
        (0..len)
            .map(|i| p.coefficient(i) - q.coefficient(i))
            .collect::<Vec<Rational>>(),
    )
}

// Subtracts two polynomials over the common denominator $ab$, as $(Ab - Ba)/(ab)$, and reduces the
// result only at the end, without taking the GCD of the denominators first.
pub fn sub_cross_multiply(p: &RationalPolynomial, q: &RationalPolynomial) -> RationalPolynomial {
    let a = Integer::from(p.denominator_ref());
    let b = Integer::from(q.denominator_ref());
    let xs = p.numerator_ref().coefficients_asc();
    let ys = q.numerator_ref().coefficients_asc();
    let numerator = (0..max(xs.len(), ys.len()))
        .map(|i| {
            xs.get(i).unwrap_or(&Integer::ZERO) * &b - ys.get(i).unwrap_or(&Integer::ZERO) * &a
        })
        .collect::<Vec<Integer>>();
    RationalPolynomial::from_numerator_and_denominator(
        IntegerPolynomial::from_coefficients_asc(numerator),
        p.denominator_ref() * q.denominator_ref(),
    )
}
