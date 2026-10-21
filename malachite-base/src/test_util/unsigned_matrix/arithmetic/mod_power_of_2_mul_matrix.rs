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
use crate::unsigned_matrix::arithmetic::mod_mul_matrix::ProductShape;
use crate::unsigned_matrix::arithmetic::mod_power_of_2_mul_matrix::classical::*;
use crate::unsigned_matrix::arithmetic::mod_power_of_2_mul_matrix::strassen::*;
use alloc::vec::Vec;

/// The product $AB$ modulo $2^k$, one modular operation at a time.
pub fn unsigned_matrix_mod_power_of_2_mul_matrix_naive<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    b: &UnsignedMatrix<T>,
    pow: u64,
) -> UnsignedMatrix<T> {
    assert_eq!(a.width(), b.height());
    let mut elements = Vec::new();
    for i in 0..a.height() {
        for j in 0..b.width() {
            let mut sum = T::ZERO;
            for (k, &x) in a.row_iter(i).enumerate() {
                let y = b.elements[k * usize::exact_from(b.width()) + usize::exact_from(j)];
                sum.mod_power_of_2_add_assign(x.mod_power_of_2_mul(y, pow), pow);
            }
            elements.push(sum);
        }
    }
    UnsignedMatrix::from_owned_elements(a.height(), b.width(), elements)
}

// The product $AB$ modulo $2^k$ with the given algorithm for the product in wrapping arithmetic.
fn with_algorithm<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    b: &UnsignedMatrix<T>,
    pow: u64,
    f: impl Fn(&[T], &[T], ProductShape) -> Vec<T>,
) -> UnsignedMatrix<T> {
    assert_eq!(a.width(), b.height());
    let shape = ProductShape {
        ar: usize::exact_from(a.height()),
        br: usize::exact_from(a.width()),
        bc: usize::exact_from(b.width()),
    };
    let mut elements = f(&a.elements, &b.elements, shape);
    for x in &mut elements {
        x.mod_power_of_2_assign(pow);
    }
    UnsignedMatrix::from_owned_elements(a.height(), b.width(), elements)
}

/// The product $AB$ modulo $2^k$, with the classical algorithm.
pub fn unsigned_matrix_mod_power_of_2_mul_matrix_classical<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    b: &UnsignedMatrix<T>,
    pow: u64,
) -> UnsignedMatrix<T> {
    with_algorithm(a, b, pow, matrix_wrapping_mul_classical)
}

/// The product $AB$ modulo $2^k$, with Strassen's algorithm at the top level.
pub fn unsigned_matrix_mod_power_of_2_mul_matrix_strassen<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    b: &UnsignedMatrix<T>,
    pow: u64,
) -> UnsignedMatrix<T> {
    with_algorithm(a, b, pow, matrix_wrapping_mul_strassen)
}
