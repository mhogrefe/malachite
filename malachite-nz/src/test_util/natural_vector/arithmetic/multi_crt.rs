// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use crate::platform::Limb;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::basic::traits::Zero;
use malachite_base::unsigned_vector::UnsignedVector;

// The moduli as `Natural`s and, for each element, its residues as `Natural`s, one per modulus; or
// `None` if the moduli are unusable. Usability is checked by combining zeros, so that a
// 0-dimensional vector, which has no element to combine, is checked too.
pub(crate) fn naturals_and_columns(
    moduli: &[Limb],
    residues: &[UnsignedVector<Limb>],
) -> Option<(Vec<Natural>, Vec<Vec<Natural>>)> {
    let moduli: Vec<Natural> = moduli.iter().copied().map(Natural::from).collect();
    Natural::multi_crt(&moduli, &vec![Natural::ZERO; moduli.len()])?;
    let columns = (0..residues[0].elements.len())
        .map(|i| {
            residues
                .iter()
                .map(|rs| Natural::from(rs.elements[i]))
                .collect()
        })
        .collect();
    Some((moduli, columns))
}

// Combines each element's residues separately with `Natural::multi_crt`, so nothing that depends
// only on the moduli is shared between elements.
pub fn natural_vector_multi_crt_naive(
    moduli: &[Limb],
    residues: &[UnsignedVector<Limb>],
) -> Option<NaturalVector> {
    let (moduli, columns) = naturals_and_columns(moduli, residues)?;
    Some(NaturalVector {
        elements: columns
            .iter()
            .map(|column| Natural::multi_crt(&moduli, column))
            .collect::<Option<_>>()?,
    })
}
