// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_matrix::RationalMatrix;
use crate::rational_matrix::linear_algebra::similarity_transform::RationalRing;
use malachite_base::matrix::Matrix;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::unsigned_matrix::linear_algebra::gram_schmidt::gram_schmidt_modified;

// The modified Gram-Schmidt process, with fractions throughout.
pub fn gram_schmidt_modified_rational(a: &RationalMatrix) -> RationalMatrix {
    RationalMatrix::from_owned_elements(
        a.height(),
        a.width(),
        gram_schmidt_modified(
            &RationalRing,
            &a.elements,
            usize::exact_from(a.height()),
            usize::exact_from(a.width()),
        )
        .unwrap(),
    )
}
