// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::matrix::Matrix;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::test_util::unsigned_matrix::arithmetic::mod_strong_echelon_form::howell_rows_naive;
use crate::unsigned_matrix::UnsignedMatrix;
use alloc::vec::Vec;

/// The Howell form of `a` modulo `m`, computed from the definition by enumerating the row span. It
/// is only practical for small matrices and moduli.
pub fn unsigned_matrix_mod_howell_form_naive<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    m: T,
) -> UnsignedMatrix<T> {
    let w = a.width();
    let h = a.height().max(w);
    let mut elements: Vec<T> = howell_rows_naive(a, m)
        .into_iter()
        .flatten()
        .flatten()
        .collect();
    elements.resize(usize::exact_from(h * w), T::ZERO);
    UnsignedMatrix::from_owned_elements(h, w, elements)
}
