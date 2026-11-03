// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::test_util::natural_polynomial::modular::mod_power_of_2_evaluate::*;
use alloc::vec::Vec;

// Evaluates a polynomial at each of `xs` modulo 2^pow with `mod_power_of_2_evaluate_naive`.
pub fn mod_power_of_2_evaluate_many_naive(
    p: &NaturalPolynomial,
    xs: &[Natural],
    pow: u64,
) -> Vec<Natural> {
    xs.iter()
        .map(|x| mod_power_of_2_evaluate_naive(p, x, pow))
        .collect()
}
