// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::platform::Limb;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::Crt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::unsigned_matrix::UnsignedMatrix;

// Combines each element separately with `Natural::crt`, so the inverse of `m1` modulo `m2` is
// recomputed for every element. Coprimality is checked by combining zeros, so that a matrix with no
// elements is checked too.
pub fn natural_matrix_crt_naive(
    a: &NaturalMatrix,
    m1: &Natural,
    b: &UnsignedMatrix<Limb>,
    m2: Limb,
) -> Option<NaturalMatrix> {
    let m2 = Natural::from(m2);
    Natural::ZERO.crt(m1, Natural::ZERO, &m2)?;
    Some(NaturalMatrix::from_owned_elements(
        a.height(),
        a.width(),
        a.elements
            .iter()
            .zip(b.elements_ref())
            .map(|(x, &r)| x.crt(m1, Natural::from(r), &m2))
            .collect::<Option<_>>()?,
    ))
}
