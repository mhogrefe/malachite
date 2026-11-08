// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_matrix::RationalMatrix;
use crate::rational_vector::RationalVector;
use malachite_base::matrix::Matrix;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::linear_algebra::traits::Determinant;
use malachite_base::vector::Vector;

// The generalized cross product of the vectors, with element `j` found as the determinant of the
// matrix whose rows are the vectors followed by the `j`th standard basis vector, since the
// product's dot product with any vector `x` is the determinant of the vectors followed by `x`.
pub fn generalized_cross_by_determinants(xs: &[RationalVector]) -> RationalVector {
    let n = xs.len() + 1;
    let size = u64::exact_from(n);
    RationalVector::from_owned_elements(
        (0..n)
            .map(|j| {
                let mut elements: Vec<Rational> =
                    xs.iter().flat_map(|x| x.elements.iter().cloned()).collect();
                elements.extend((0..n).map(|k| Rational::from(u32::from(k == j))));
                RationalMatrix::from_owned_elements(size, size, elements).determinant()
            })
            .collect(),
    )
}
