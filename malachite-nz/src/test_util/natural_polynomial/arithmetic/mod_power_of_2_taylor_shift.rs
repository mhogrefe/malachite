// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::num::arithmetic::traits::ModPowerOf2;
use malachite_base::polynomial::TaylorShift;

// Shifts a polynomial exactly and reduces the result modulo `2^pow` afterwards, letting the
// coefficients grow by up to `n` times the size of `c` along the way.
pub fn mod_power_of_2_taylor_shift_naive(
    p: &NaturalPolynomial,
    c: &Natural,
    pow: u64,
) -> NaturalPolynomial {
    p.taylor_shift(c).mod_power_of_2(pow)
}
