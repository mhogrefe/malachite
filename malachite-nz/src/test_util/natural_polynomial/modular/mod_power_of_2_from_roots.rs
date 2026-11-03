// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::test_util::integer_polynomial::arithmetic::from_roots::from_roots_coefficients_naive;
use alloc::vec::Vec;
use malachite_base::num::modular::traits::ModPowerOf2;
use malachite_base::polynomial::Polynomial;

// Builds the product of the factors $x - r$ over the integers and then reduces its coefficients
// modulo 2^pow. The modular construction never forms the integer product, so this checks it
// independently.
pub fn mod_power_of_2_from_roots_naive(roots: &[Natural], pow: u64) -> NaturalPolynomial {
    let roots: Vec<Integer> = roots.iter().map(Integer::from).collect();
    NaturalPolynomial::from_coefficients_asc(
        from_roots_coefficients_naive(&roots)
            .into_iter()
            .map(|c| c.mod_power_of_2(pow))
            .collect(),
    )
}
