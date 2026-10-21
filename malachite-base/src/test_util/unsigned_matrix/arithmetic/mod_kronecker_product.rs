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
use crate::unsigned_matrix::UnsignedMatrix;
use alloc::vec::Vec;

/// The Kronecker product $A \otimes B$ modulo $m$, computed one element at a time from the
/// definition, with `ModMul`.
pub fn unsigned_matrix_mod_kronecker_product_naive<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    b: &UnsignedMatrix<T>,
    m: T,
) -> UnsignedMatrix<T> {
    let p = usize::exact_from(b.height());
    let q = usize::exact_from(b.width());
    let height = usize::exact_from(a.height()) * p;
    let width = usize::exact_from(a.width()) * q;
    let mut elements = Vec::with_capacity(height * width);
    for r in 0..height {
        for c in 0..width {
            elements.push(a[(r / p, c / q)].mod_mul(b[(r % p, c % q)], m));
        }
    }
    UnsignedMatrix::from_owned_elements(a.height() * b.height(), a.width() * b.width(), elements)
}
