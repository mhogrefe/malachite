// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::test_util::natural_polynomial::arithmetic::mod_power_of_2_div_rem::*;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2Neg};
use malachite_base::num::basic::traits::One;
use malachite_base::polynomial::Polynomial;

// Divides `p` by $x - c$ modulo 2^k by naive long division.
pub fn mod_power_of_2_div_root_naive(
    p: &NaturalPolynomial,
    c: &Natural,
    pow: u64,
) -> (NaturalPolynomial, Natural) {
    let neg = c.mod_power_of_2_neg(pow);
    let divisor =
        NaturalPolynomial::from_coefficients_asc(vec![neg, Natural::ONE.mod_power_of_2(pow)]);
    let (q, r) = mod_power_of_2_div_rem_naive(p, &divisor, pow).unwrap();
    (q, r.coefficient(0).clone())
}
