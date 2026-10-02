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
use malachite_base::num::arithmetic::traits::{Gcd, ModInverse, ModMul};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;

// Whether the integral modulo `m` of `p` is defined: whether every k from 2 to the length of `p` is
// a unit modulo `m`, checked one at a time with a GCD.
pub fn mod_integral_is_defined(p: &NaturalPolynomial, m: &Natural) -> bool {
    (2..=p.len()).all(|k| (Natural::from(k) % m).gcd(m) == 1u32)
}

// Integrates a polynomial modulo `m` coefficient by coefficient, inverting each index separately.
// It returns `None` if the integral is not defined. Nothing is shared with the implementation.
pub fn mod_integral_naive(p: &NaturalPolynomial, m: &Natural) -> Option<NaturalPolynomial> {
    if p.len() == 0 {
        return Some(NaturalPolynomial::ZERO);
    }
    let mut coefficients = Vec::with_capacity(usize::try_from(p.len()).unwrap() + 1);
    coefficients.push(Natural::ZERO);
    for i in 0..p.len() {
        let k = Natural::from(i + 1) % m;
        let inverse = if *m == 1u32 {
            Natural::ZERO
        } else if k == 0u32 {
            return None;
        } else {
            k.mod_inverse(m)?
        };
        coefficients.push(p.coefficient(i).mod_mul(inverse, m));
    }
    Some(NaturalPolynomial::from_coefficients_asc(coefficients))
}
