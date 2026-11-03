// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::test_util::rational_polynomial::arithmetic::from_roots::from_roots_coefficients_naive;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::Polynomial;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;

// The monic product of the factors $x - p/q$ over the rationals, multiplied by the product of the
// denominators $q$, which clears every denominator.
pub fn from_rational_roots_naive(roots: &[Rational]) -> IntegerPolynomial {
    let denominator: Natural = roots.iter().map(Rational::denominator_ref).product();
    let denominator = Rational::from(denominator);
    IntegerPolynomial::from_coefficients_asc(
        from_roots_coefficients_naive(roots)
            .into_iter()
            .map(|c| Integer::exact_from(c * &denominator))
            .collect(),
    )
}
