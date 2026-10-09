// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_vector::IntegerVector;
use crate::platform::Limb;
use crate::test_util::natural_vector::arithmetic::multi_crt::naturals_and_columns;
use malachite_base::unsigned_vector::UnsignedVector;

// Combines each element's residues separately with `Integer::multi_balanced_crt`, so nothing that
// depends only on the moduli is shared between elements.
pub fn integer_vector_multi_balanced_crt_naive(
    moduli: &[Limb],
    residues: &[UnsignedVector<Limb>],
) -> Option<IntegerVector> {
    let (moduli, columns) = naturals_and_columns(moduli, residues)?;
    Some(IntegerVector {
        elements: columns
            .iter()
            .map(|column| Integer::multi_balanced_crt(&moduli, column))
            .collect::<Option<_>>()?,
    })
}
