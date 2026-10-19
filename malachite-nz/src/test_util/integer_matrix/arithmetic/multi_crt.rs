// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;
use crate::platform::Limb;
use crate::test_util::integer_vector::arithmetic::multi_crt::*;
use crate::test_util::natural_matrix::arithmetic::multi_crt::residue_vectors;
use malachite_base::matrix::Matrix;
use malachite_base::unsigned_matrix::UnsignedMatrix;

// Combines each element's residues separately with `Integer::multi_balanced_crt`, so nothing that
// depends only on the moduli is shared between elements.
pub fn integer_matrix_multi_balanced_crt_naive(
    moduli: &[Limb],
    residues: &[UnsignedMatrix<Limb>],
) -> Option<IntegerMatrix> {
    let v = integer_vector_multi_balanced_crt_naive(moduli, &residue_vectors(residues))?;
    Some(IntegerMatrix::from_owned_elements(
        residues[0].height(),
        residues[0].width(),
        v.elements,
    ))
}
