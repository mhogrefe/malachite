// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use crate::natural::Natural;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, PowerOf2};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;

// Peels off the low `bits` bits of |n|, shifting them out, until nothing is left; each field whose
// value is at least 2^(bits - 1) stands for that value minus 2^bits, and adds 1 to the coefficient
// above. The coefficients are negated if n is. Nothing is shared with the implementation.
pub fn bit_unpack_naive(n: &Integer, bits: u64) -> IntegerPolynomial {
    assert_ne!(bits, 0);
    let half = Natural::power_of_2(bits - 1);
    let mut rest = n.unsigned_abs_ref().clone();
    let mut coefficients = Vec::new();
    let mut carry = Integer::ZERO;
    while rest != 0u32 {
        let field = (&rest).mod_power_of_2(bits);
        rest >>= bits;
        let negative = field >= half;
        let mut c = Integer::from(field) + &carry;
        if negative {
            c -= Integer::power_of_2(bits);
        }
        coefficients.push(c);
        carry = Integer::from(u32::from(negative));
    }
    if carry != 0u32 {
        coefficients.push(carry);
    }
    let p = IntegerPolynomial::from_coefficients_asc(coefficients);
    if *n < 0u32 { -p } else { p }
}
