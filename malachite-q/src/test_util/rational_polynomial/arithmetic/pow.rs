// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use malachite_base::num::arithmetic::traits::Pow;
use malachite_nz::test_util::integer_polynomial::arithmetic::pow::pow_naive as integer_pow_naive;

// Raises the numerator to the power `e` by `e` schoolbook multiplications over the integers, and
// the denominator by `Natural` powering, and reduces the quotient once.
pub fn pow_naive(p: &RationalPolynomial, e: u64) -> RationalPolynomial {
    RationalPolynomial::from_numerator_and_denominator(
        integer_pow_naive(p.numerator_ref(), e),
        p.denominator_ref().pow(e),
    )
}
