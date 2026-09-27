// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::IntegerPolynomial;
use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::num::arithmetic::traits::Mod;

// Adds the polynomials over the integers and then reduces every coefficient of the result modulo m.
// Nothing is shared with the implementation.
pub fn mod_add_naive(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    m: &Natural,
) -> NaturalPolynomial {
    (IntegerPolynomial::from(p.clone()) + IntegerPolynomial::from(q.clone())).mod_op(m)
}
