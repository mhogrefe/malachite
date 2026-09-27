// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::IntegerPolynomial;
use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::num::arithmetic::traits::ModPowerOf2;
use malachite_base::polynomial::Polynomial;

// Subtracts the whole polynomials over the integers, reduces the result modulo 2^pow, and then
// truncates it. Nothing is shared with the implementation.
pub fn mod_power_of_2_sub_truncated_naive(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    len: u64,
    pow: u64,
) -> NaturalPolynomial {
    (IntegerPolynomial::from(p.clone()) - IntegerPolynomial::from(q.clone()))
        .mod_power_of_2(pow)
        .truncate(len)
}
