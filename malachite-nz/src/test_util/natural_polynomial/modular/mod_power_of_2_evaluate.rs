// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::test_util::natural_polynomial::arithmetic::evaluate::evaluate_naive;
use malachite_base::num::modular::traits::ModPowerOf2;

// Evaluates a polynomial at x in full and then reduces the value modulo 2^pow. The modular
// evaluation never forms the full value, so this checks it independently.
pub fn mod_power_of_2_evaluate_naive(p: &NaturalPolynomial, x: &Natural, pow: u64) -> Natural {
    evaluate_naive(p, x).mod_power_of_2(pow)
}
