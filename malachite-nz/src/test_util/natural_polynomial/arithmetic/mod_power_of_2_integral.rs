// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2Inverse, ModPowerOf2Mul, Parity,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;

// Whether the integral modulo 2^pow of `p` is defined: whether every nonzero coefficient belongs to
// an even power of x.
pub fn mod_power_of_2_integral_is_defined(p: &NaturalPolynomial, pow: u64) -> bool {
    pow == 0 || (0..p.len()).all(|i| i.even() || *p.coefficient(i) == 0u32)
}

// Integrates a polynomial modulo 2^pow coefficient by coefficient, inverting each index separately.
// It returns `None` if the integral is not defined. Nothing is shared with the implementation.
pub fn mod_power_of_2_integral_naive(p: &NaturalPolynomial, pow: u64) -> Option<NaturalPolynomial> {
    let mut coefficients = Vec::with_capacity(usize::try_from(p.len()).unwrap() + 1);
    if p.len() != 0 {
        coefficients.push(Natural::ZERO);
    }
    for i in 0..p.len() {
        let c = p.coefficient(i);
        if *c == 0u32 {
            coefficients.push(Natural::ZERO);
            continue;
        }
        let k = Natural::from(i + 1).mod_power_of_2(pow);
        if k == 0u32 {
            return None;
        }
        let inverse = k.mod_power_of_2_inverse(pow)?;
        coefficients.push(c.mod_power_of_2_mul(inverse, pow));
    }
    Some(NaturalPolynomial::from_coefficients_asc(coefficients))
}
