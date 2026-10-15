// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_vector::IntegerVector;
use crate::natural_vector::NaturalVector;
use malachite_base::num::arithmetic::traits::{Cross, ModPowerOf2};
use malachite_base::vector::Vector;

// The exact cross product, as integers, reduced modulo 2^pow afterwards.
pub fn natural_vector_mod_power_of_2_cross_naive(
    v: &NaturalVector,
    w: &NaturalVector,
    pow: u64,
) -> NaturalVector {
    let v =
        IntegerVector::from_owned_elements(v.elements_ref().iter().map(Integer::from).collect());
    let w =
        IntegerVector::from_owned_elements(w.elements_ref().iter().map(Integer::from).collect());
    NaturalVector::from_owned_elements(
        v.cross(w)
            .into_elements()
            .into_iter()
            .map(|x| x.mod_power_of_2(pow))
            .collect(),
    )
}
