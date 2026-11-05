// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::natural_matrix::linear_algebra::mod_gram_schmidt::NaturalModRing;
use crate::natural_matrix::linear_algebra::mod_power_of_2_gram_schmidt::NaturalModPowerOf2Ring;
use malachite_base::matrix::Matrix;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::unsigned_matrix::linear_algebra::gram_schmidt::gram_schmidt_modified;

// The modified Gram-Schmidt process, always in `Natural`s, whatever the size of the modulus.
pub fn mod_gram_schmidt_modified(a: &NaturalMatrix, m: &Natural) -> Option<NaturalMatrix> {
    Some(NaturalMatrix::from_owned_elements(
        a.height(),
        a.width(),
        gram_schmidt_modified(
            &NaturalModRing { m },
            &a.elements,
            usize::exact_from(a.height()),
            usize::exact_from(a.width()),
        )?,
    ))
}

// The modified Gram-Schmidt process, always in `Natural`s, whatever the size of the modulus.
pub fn mod_power_of_2_gram_schmidt_modified(a: &NaturalMatrix, pow: u64) -> Option<NaturalMatrix> {
    Some(NaturalMatrix::from_owned_elements(
        a.height(),
        a.width(),
        gram_schmidt_modified(
            &NaturalModPowerOf2Ring { pow },
            &a.elements,
            usize::exact_from(a.height()),
            usize::exact_from(a.width()),
        )?,
    ))
}
