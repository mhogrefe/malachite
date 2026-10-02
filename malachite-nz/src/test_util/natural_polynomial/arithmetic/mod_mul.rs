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
use malachite_base::num::arithmetic::traits::Mod;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::Polynomial;

// Multiplies by schoolbook multiplication and then reduces the product's coefficients modulo `m`.
pub fn mod_mul_naive(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    m: &Natural,
) -> NaturalPolynomial {
    NaturalPolynomial::from_coefficients_asc(
        mul_naive(p, q)
            .coefficients_asc()
            .iter()
            .map(|x| x.mod_op(m))
            .collect(),
    )
}

// `len` coefficients reduced modulo `m`, for exercising the multiplication algorithms at chosen
// lengths and moduli: $2^{k + \text{W}} - (i + 1)$ reduced, where $k$ is the number of bits of $m$,
// so that they are all different and most are large.
pub fn natural_mod_generated_coefficients(len: usize, m: &Natural) -> Vec<Natural> {
    natural_generated_coefficients(len, m.significant_bits() + Limb::WIDTH)
        .into_iter()
        .map(|x| x.mod_op(m))
        .collect()
}

// Moduli for exercising the multiplication algorithms: small ones, ones near the size of a limb,
// and ones too large for a limb.
pub fn natural_test_moduli() -> Vec<Natural> {
    [1u32, 2, 3, 7, 1000]
        .into_iter()
        .map(Natural::from)
        .chain([
            Natural::from(Limb::MAX >> 1),
            Natural::from((Limb::MAX >> 1) + 2),
            Natural::from(Limb::MAX - 1),
            Natural::from(Limb::MAX),
        ])
        .collect()
}
