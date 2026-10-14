// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::num::arithmetic::traits::Mod;
use malachite_base::polynomial::Compose;

// Composes two polynomials exactly and reduces the result modulo m afterwards, letting the
// coefficients grow along the way.
pub fn mod_compose_naive(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    m: &Natural,
) -> NaturalPolynomial {
    p.compose(q).mod_op(m)
}
