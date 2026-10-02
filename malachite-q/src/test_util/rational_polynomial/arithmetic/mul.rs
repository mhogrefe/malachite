// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_polynomial::RationalPolynomial;
use alloc::vec;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;

// Multiplies two polynomials by schoolbook multiplication, as `Rational`s, reading each coefficient
// through `coefficient`. Nothing is shared with the implementation.
pub fn mul_naive(p: &RationalPolynomial, q: &RationalPolynomial) -> RationalPolynomial {
    let (n, m) = (p.len(), q.len());
    if n == 0 || m == 0 {
        return RationalPolynomial::ZERO;
    }
    let mut out = vec![Rational::ZERO; usize::try_from(n + m - 1).unwrap()];
    for i in 0..n {
        let c = p.coefficient(i);
        for j in 0..m {
            out[usize::try_from(i + j).unwrap()] += &c * q.coefficient(j);
        }
    }
    RationalPolynomial::from_coefficients_asc(out)
}

// Multiplies the numerators and the denominators and reduces the result only at the end, without
// computing the cross GCDs first.
pub fn mul_then_reduce(p: &RationalPolynomial, q: &RationalPolynomial) -> RationalPolynomial {
    RationalPolynomial::from_numerator_and_denominator(
        p.numerator_ref() * q.numerator_ref(),
        p.denominator_ref() * q.denominator_ref(),
    )
}
