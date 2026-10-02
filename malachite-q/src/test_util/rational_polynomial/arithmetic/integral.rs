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
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;

// Integrates a polynomial coefficient by coefficient, as `Rational`s, dividing the coefficient of
// $x^i$ by $i + 1$. Nothing is shared with the implementation.
pub fn integral_naive(p: &RationalPolynomial) -> RationalPolynomial {
    if *p == RationalPolynomial::ZERO {
        return RationalPolynomial::ZERO;
    }
    let mut coefficients = Vec::with_capacity(usize::try_from(p.len()).unwrap() + 1);
    coefficients.push(Rational::ZERO);
    for i in 0..p.len() {
        coefficients.push(p.coefficient(i) / Rational::from(i + 1));
    }
    RationalPolynomial::from_coefficients_asc(coefficients)
}
