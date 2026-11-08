// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::matrix::Matrix;
use crate::num::conversion::traits::ExactFrom;
use crate::num::linear_algebra::traits::ModDeterminant;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_vector::UnsignedVector;
use crate::vector::Vector;
use alloc::vec::Vec;

// The generalized cross product of the vectors modulo `m`, with element `j` found as the
// determinant modulo `m` of the matrix whose rows are the vectors followed by the `j`th standard
// basis vector, since the product's dot product with any vector `x` is the determinant of the
// vectors followed by `x`.
pub fn mod_generalized_cross_by_determinants(
    xs: &[UnsignedVector<u64>],
    m: u64,
) -> UnsignedVector<u64> {
    let n = xs.len() + 1;
    let size = u64::exact_from(n);
    UnsignedVector::from_owned_elements(
        (0..n)
            .map(|j| {
                let mut elements: Vec<u64> =
                    xs.iter().flat_map(|x| x.elements.iter().copied()).collect();
                elements.extend((0..n).map(|k| u64::from(k == j) % m));
                UnsignedMatrix::from_owned_elements(size, size, elements).mod_determinant(m)
            })
            .collect(),
    )
}
