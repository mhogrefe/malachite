// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::arithmetic::traits::PowerOf2;

// Multiplies by $2^k$ instead of shifting. Shifting instead would make this the fused `sub_mul_shl`
// it is the reference for.
#[cfg_attr(dylint_lib = "malachite_lints", expect(mul_div_by_power_of_2))]
pub fn sub_mul_shl_naive(x: &Rational, y: &Rational, z: &Rational, bits: u64) -> Rational {
    x - y * z * Rational::power_of_2(bits)
}
