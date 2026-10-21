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
use crate::unsigned_matrix::arithmetic::mod_mul_matrix::classical::matrix_mod_mul_classical_op;
use crate::unsigned_matrix::arithmetic::mod_mul_matrix::strassen::matrix_mod_mul_strassen;
use crate::unsigned_matrix::arithmetic::mod_mul_matrix::{ModMulOp, ProductShape};
use alloc::vec;
use alloc::vec::Vec;

/// The product $AB$ modulo $m$, one modular operation at a time.
pub fn unsigned_matrix_mod_mul_matrix_naive<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    b: &UnsignedMatrix<T>,
    m: T,
) -> UnsignedMatrix<T> {
    assert_eq!(a.width(), b.height());
    let mut elements = Vec::new();
    for i in 0..a.height() {
        for j in 0..b.width() {
            let mut sum = T::ZERO;
            for (k, &x) in a.row_iter(i).enumerate() {
                let y = b.elements[k * usize::exact_from(b.width()) + usize::exact_from(j)];
                sum.mod_add_assign(x.mod_mul(y, m), m);
            }
            elements.push(sum);
        }
    }
    UnsignedMatrix::from_owned_elements(a.height(), b.width(), elements)
}

// The shape of the product of `a` and `b`.
fn shape<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, b: &UnsignedMatrix<T>) -> ProductShape {
    assert_eq!(a.width(), b.height());
    ProductShape {
        ar: usize::exact_from(a.height()),
        br: usize::exact_from(a.width()),
        bc: usize::exact_from(b.width()),
    }
}

/// The product $AB$ modulo $m$, with the classical algorithm.
pub fn unsigned_matrix_mod_mul_matrix_classical<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    b: &UnsignedMatrix<T>,
    m: T,
) -> UnsignedMatrix<T> {
    let shape = shape(a, b);
    let mut elements = vec![T::ZERO; shape.ar * shape.bc];
    matrix_mod_mul_classical_op(
        &mut elements,
        ModMulOp::Set,
        &a.elements,
        &b.elements,
        shape,
        m,
    );
    UnsignedMatrix::from_owned_elements(a.height(), b.width(), elements)
}

/// $C + AB$ modulo $m$, with the classical algorithm.
pub fn unsigned_matrix_mod_add_mul_matrix_classical<T: PrimitiveUnsigned>(
    c: &UnsignedMatrix<T>,
    a: &UnsignedMatrix<T>,
    b: &UnsignedMatrix<T>,
    m: T,
) -> UnsignedMatrix<T> {
    let shape = shape(a, b);
    let mut elements = c.elements.clone();
    matrix_mod_mul_classical_op(
        &mut elements,
        ModMulOp::Add,
        &a.elements,
        &b.elements,
        shape,
        m,
    );
    UnsignedMatrix::from_owned_elements(a.height(), b.width(), elements)
}

/// The product $AB$ modulo $m$, with Strassen's algorithm.
pub fn unsigned_matrix_mod_mul_matrix_strassen<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    b: &UnsignedMatrix<T>,
    m: T,
) -> UnsignedMatrix<T> {
    let shape = shape(a, b);
    UnsignedMatrix::from_owned_elements(
        a.height(),
        b.width(),
        matrix_mod_mul_strassen(&a.elements, &b.elements, shape, m),
    )
}
