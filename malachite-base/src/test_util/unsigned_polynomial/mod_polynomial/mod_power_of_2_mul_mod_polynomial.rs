// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::test_util::unsigned_polynomial::modular::mod_power_of_2_div_rem::*;
use crate::test_util::unsigned_polynomial::modular::mod_power_of_2_mul::*;
use crate::unsigned_polynomial::UnsignedPolynomial;

// The product of `a` and `b` modulo `f`, with coefficients modulo $2^k$, where $k$ is `pow`: the
// schoolbook product, reduced by schoolbook division. `f` must be nonzero with an odd leading
// coefficient, unless $k$ is 0.
pub fn mod_power_of_2_mul_mod_polynomial_naive<T: PrimitiveUnsigned>(
    a: &UnsignedPolynomial<T>,
    b: &UnsignedPolynomial<T>,
    f: &UnsignedPolynomial<T>,
    pow: u64,
) -> UnsignedPolynomial<T> {
    mod_power_of_2_div_rem_naive(&mod_power_of_2_mul_polynomial_naive(a, b, pow), f, pow)
        .unwrap()
        .1
}
