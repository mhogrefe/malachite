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
use malachite_base::num::arithmetic::traits::ModPowerOf2;
use malachite_base::polynomial::Polynomial;

// Peels off the low `bits` bits of `n` as the next coefficient, and shifts them out, until nothing
// is left. Nothing is shared with the implementation.
pub fn bit_unpack_naive(n: &Natural, bits: u64) -> NaturalPolynomial {
    assert_ne!(bits, 0);
    let mut n = n.clone();
    let mut coefficients = Vec::new();
    while n != 0u32 {
        coefficients.push((&n).mod_power_of_2(bits));
        n >>= bits;
    }
    NaturalPolynomial::from_coefficients_asc(coefficients)
}
