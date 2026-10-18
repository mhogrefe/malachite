// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_matrix::RationalMatrix;
use crate::rational_vector::RationalVector;
use crate::test_util::rational_vector::conversion::to_numerators_and_denominator::*;
use malachite_base::matrix::Matrix;
use malachite_base::vector::Vector;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural::Natural;

/// Clears denominators without computing a least common multiple, by applying
/// [`rational_vector_to_numerators_and_denominator_naive`] to the elements in row-major order.
pub fn rational_matrix_to_numerators_and_denominator_naive(
    a: &RationalMatrix,
) -> (IntegerMatrix, Natural) {
    let (ns, d) = rational_vector_to_numerators_and_denominator_naive(
        &RationalVector::from_elements(&a.elements),
    );
    (
        IntegerMatrix::from_owned_elements(a.height(), a.width(), ns.elements),
        d,
    )
}
