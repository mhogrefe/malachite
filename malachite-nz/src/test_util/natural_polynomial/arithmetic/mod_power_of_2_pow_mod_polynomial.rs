// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_polynomial::NaturalPolynomial;
use crate::test_util::natural_polynomial::arithmetic::mod_power_of_2_mul_mod_polynomial::*;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;

// The power of `a` with exponent `exp` modulo `f`, with coefficients modulo $2^k$, where $k$ is
// `pow`: `exp` naive multiplications modulo `f`, starting from 1. `f` must be nonzero with an odd
// leading coefficient, unless $k$ is 0.
pub fn mod_power_of_2_pow_mod_polynomial_naive(
    a: &NaturalPolynomial,
    exp: u64,
    f: &NaturalPolynomial,
    pow: u64,
) -> NaturalPolynomial {
    if pow == 0 || f.len() == 1 {
        return NaturalPolynomial::ZERO;
    }
    let mut r = NaturalPolynomial::one();
    for _ in 0..exp {
        r = mod_power_of_2_mul_mod_polynomial_naive(&r, a, f, pow);
    }
    r
}
