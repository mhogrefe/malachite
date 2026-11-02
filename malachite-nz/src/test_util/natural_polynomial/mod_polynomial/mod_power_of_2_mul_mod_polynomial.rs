// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_polynomial::NaturalPolynomial;
use crate::test_util::natural_polynomial::modular::mod_power_of_2_div_rem::*;
use crate::test_util::natural_polynomial::modular::mod_power_of_2_mul::mod_power_of_2_mul_naive;

// The product of `a` and `b` modulo `f`, with coefficients modulo $2^k$, where $k$ is `pow`: the
// schoolbook product, reduced by schoolbook division. `f` must be nonzero with an odd leading
// coefficient, unless $k$ is 0.
pub fn mod_power_of_2_mul_mod_polynomial_naive(
    a: &NaturalPolynomial,
    b: &NaturalPolynomial,
    f: &NaturalPolynomial,
    pow: u64,
) -> NaturalPolynomial {
    mod_power_of_2_div_rem_naive(&mod_power_of_2_mul_naive(a, b, pow), f, pow)
        .unwrap()
        .1
}
