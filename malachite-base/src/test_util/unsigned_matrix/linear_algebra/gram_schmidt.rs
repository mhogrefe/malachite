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
use crate::unsigned_matrix::linear_algebra::gram_schmidt_over_ring::{
    GramSchmidtRing, ModPowerOf2WordRing, ModWordRing,
};
use alloc::vec::Vec;

// Column `j` of the `height` by `width` matrix with elements `xs`.
fn column<E: Clone>(xs: &[E], height: usize, width: usize, j: usize) -> Vec<E> {
    (0..height).map(|k| xs[k * width + j].clone()).collect()
}

// The Gram-Schmidt process applied to the columns of the `height` by `width` matrix with elements
// `xs`, by the modified algorithm: once column `j` is final, its projection is subtracted from
// every later column. In exact arithmetic this gives the same columns as the classical algorithm,
// and divides by the same squared lengths, so it gives up in the same cases: when a squared length
// it needs has no inverse, or is 0 for a nonzero column.
pub fn gram_schmidt_modified<R: GramSchmidtRing>(
    ring: &R,
    xs: &[R::Element],
    height: usize,
    width: usize,
) -> Option<Vec<R::Element>> {
    let mut bs = xs.to_vec();
    if height == 0 {
        return Some(bs);
    }
    for j in 0..width.saturating_sub(1) {
        let b_j = column(&bs, height, width, j);
        let den = ring.dot(&b_j, &b_j);
        if ring.is_zero(&den) {
            if b_j.iter().any(|x| !ring.is_zero(x)) {
                return None;
            }
            continue;
        }
        let inverse = ring.inverse(&den)?;
        for i in j + 1..width {
            let mut b_i = column(&bs, height, width, i);
            let mu = ring.mul(&ring.dot(&b_i, &b_j), &inverse);
            ring.sub_mul_assign(&mut b_i, &mu, &b_j);
            for (k, x) in b_i.into_iter().enumerate() {
                bs[k * width + i] = x;
            }
        }
    }
    Some(bs)
}

pub fn mod_gram_schmidt_modified<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    m: T,
) -> Option<UnsignedMatrix<T>> {
    let (height, width) = (a.height(), a.width());
    Some(UnsignedMatrix::from_owned_elements(
        height,
        width,
        gram_schmidt_modified(
            &ModWordRing::new(m, usize::exact_from(height).max(1)),
            &a.elements,
            usize::exact_from(height),
            usize::exact_from(width),
        )?,
    ))
}

pub fn mod_power_of_2_gram_schmidt_modified<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    pow: u64,
) -> Option<UnsignedMatrix<T>> {
    let (height, width) = (a.height(), a.width());
    Some(UnsignedMatrix::from_owned_elements(
        height,
        width,
        gram_schmidt_modified(
            &ModPowerOf2WordRing::new(pow),
            &a.elements,
            usize::exact_from(height),
            usize::exact_from(width),
        )?,
    ))
}
