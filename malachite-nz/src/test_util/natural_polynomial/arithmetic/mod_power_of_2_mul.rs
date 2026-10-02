// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::platform::Limb;
use crate::test_util::natural_polynomial::arithmetic::mul::{
    mul_naive, natural_generated_coefficients,
};
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::ModPowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;

// Multiplies by schoolbook multiplication and then reduces the product's coefficients.
pub fn mod_power_of_2_mul_naive(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    pow: u64,
) -> NaturalPolynomial {
    mul_naive(p, q).mod_power_of_2(pow)
}

// `len` coefficients reduced modulo $2^k$, where $k$ is `pow`, for exercising the multiplication
// algorithms at chosen lengths and coefficient sizes: $2^{k + \text{W}} - (i + 1)$ reduced, so that
// they are all different and most have all their bits set.
pub fn natural_mod_power_of_2_generated_coefficients(len: usize, pow: u64) -> Vec<Natural> {
    natural_generated_coefficients(len, pow + Limb::WIDTH)
        .into_iter()
        .map(|x| x.mod_power_of_2(pow))
        .collect()
}
