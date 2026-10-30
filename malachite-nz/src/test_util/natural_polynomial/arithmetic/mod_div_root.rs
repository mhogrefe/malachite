// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::test_util::natural_polynomial::arithmetic::mod_div_rem::mod_div_rem_naive;
use malachite_base::num::arithmetic::traits::Mod;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::polynomial::Polynomial;

// Divides `p` by $x - c$ modulo m by naive long division.
pub fn mod_div_root_naive(
    p: &NaturalPolynomial,
    c: &Natural,
    m: &Natural,
) -> (NaturalPolynomial, Natural) {
    let neg = if *c == 0u32 { Natural::ZERO } else { m - c };
    let divisor = NaturalPolynomial::from_coefficients_asc(vec![neg, Natural::ONE.mod_op(m)]);
    let (q, r) = mod_div_rem_naive(p, &divisor, m).unwrap();
    (q, r.coefficient(0).clone())
}
