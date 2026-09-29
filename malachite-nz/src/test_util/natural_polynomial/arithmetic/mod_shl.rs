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
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::Polynomial;

// Shifts every coefficient, read one at a time through `coefficient`, left without a modulus, then
// reduces it modulo `m`, and normalizes the result. Nothing is shared with the implementation.
pub fn mod_shl_naive<T: PrimitiveUnsigned>(
    p: &NaturalPolynomial,
    bits: T,
    m: &Natural,
) -> NaturalPolynomial
where
    u64: ExactFrom<T>,
{
    let bits = u64::exact_from(bits);
    NaturalPolynomial::from_coefficients_asc(
        (0..p.len())
            .map(|i| (p.coefficient(i) << bits) % m)
            .collect::<Vec<Natural>>(),
    )
}
