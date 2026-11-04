// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::test_util::unsigned_polynomial::modular::mod_gcd::NaivePowerOf2Modulus;
use crate::test_util::unsigned_polynomial::modular::mod_lcm::lcm_naive;
use crate::unsigned_polynomial::UnsignedPolynomial;

pub fn mod_power_of_2_lcm_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> Option<UnsignedPolynomial<T>> {
    lcm_naive(p, q, &NaivePowerOf2Modulus(pow))
}
