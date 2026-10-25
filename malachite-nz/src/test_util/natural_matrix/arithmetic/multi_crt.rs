// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_matrix::NaturalMatrix;
use crate::platform::Limb;
use crate::test_util::natural_vector::arithmetic::multi_crt::natural_vector_multi_crt_naive;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

// The residue matrices' elements, as vectors.
pub(crate) fn residue_vectors(residues: &[UnsignedMatrix<Limb>]) -> Vec<UnsignedVector<Limb>> {
    residues
        .iter()
        .map(|rs| UnsignedVector::from_elements(rs.elements_ref()))
        .collect()
}

// Combines each element's residues separately with `Natural::multi_crt`, so nothing that depends
// only on the moduli is shared between elements.
pub fn natural_matrix_multi_crt_naive(
    moduli: &[Limb],
    residues: &[UnsignedMatrix<Limb>],
) -> Option<NaturalMatrix> {
    let v = natural_vector_multi_crt_naive(moduli, &residue_vectors(residues))?;
    Some(NaturalMatrix::from_owned_elements(
        residues[0].height(),
        residues[0].width(),
        v.elements,
    ))
}
