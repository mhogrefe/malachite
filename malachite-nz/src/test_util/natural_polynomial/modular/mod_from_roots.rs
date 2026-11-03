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
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::modular::traits::Mod;
use malachite_base::polynomial::Polynomial;

// Builds the product of the factors $x - r$ over the integers and then reduces its coefficients
// modulo m. The modular construction never forms the integer product, so this checks it
// independently.
pub fn mod_from_roots_naive(roots: &[Natural], m: &Natural) -> NaturalPolynomial {
    let roots: Vec<Integer> = roots.iter().map(Integer::from).collect();
    let m = Integer::from(m);
    NaturalPolynomial::from_coefficients_asc(
        from_roots_coefficients_naive(&roots)
            .into_iter()
            .map(|c| Natural::exact_from(c.mod_op(&m)))
            .collect(),
    )
}
