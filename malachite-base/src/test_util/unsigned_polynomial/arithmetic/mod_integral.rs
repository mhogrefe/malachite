// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::Polynomial;
use crate::unsigned_polynomial::UnsignedPolynomial;
use alloc::vec::Vec;

// The index `k` reduced modulo `m`, computed in a `u128`.
fn index_mod_u128<T: PrimitiveUnsigned>(k: u64, m: T) -> T {
    let m: u128 = m.wrapping_into();
    T::wrapping_from(u128::from(k) % m)
}

// Whether the integral modulo `m` of `p` is defined: whether every k for which the coefficient of
// x^(k-1) is nonzero is a unit modulo `m`, checked one at a time with a GCD.
pub fn mod_integral_is_defined<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, m: T) -> bool {
    (2..=p.len()).all(|k| p.coefficient(k - 1) == T::ZERO || index_mod_u128(k, m).gcd(m) == T::ONE)
}

// Integrates a polynomial modulo `m` coefficient by coefficient, inverting each index separately.
// It returns `None` if the integral is not defined. Nothing is shared with the implementation.
pub fn mod_integral_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    m: T,
) -> Option<UnsignedPolynomial<T>> {
    if p.len() == 0 {
        return Some(UnsignedPolynomial::from_coefficients_asc(Vec::new()));
    }
    let mut coefficients = Vec::with_capacity(usize::try_from(p.len()).unwrap() + 1);
    coefficients.push(T::ZERO);
    for i in 0..p.len() {
        let c = p.coefficient(i);
        // A zero coefficient stays zero, whether or not its index is a unit.
        if c == T::ZERO {
            coefficients.push(T::ZERO);
            continue;
        }
        let k = index_mod_u128(i + 1, m);
        let inverse = if m == T::ONE {
            T::ZERO
        } else if k == T::ZERO {
            return None;
        } else {
            k.mod_inverse(m)?
        };
        coefficients.push(c.mod_mul(inverse, m));
    }
    Some(UnsignedPolynomial::from_coefficients_asc(coefficients))
}
