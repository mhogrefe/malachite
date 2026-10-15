// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_vector::RationalVector;
use alloc::vec::Vec;
use malachite_base::num::basic::traits::Zero;
use malachite_base::vector::Vector;

// The sign of the permutation (i, j, k) of (0, 1, 2): 1 if even, -1 if odd, and 0 if two indices
// coincide.
fn levi_civita(i: usize, j: usize, k: usize) -> i8 {
    if i == j || j == k || i == k {
        0
    } else if (i, j, k) == (0, 1, 2) || (i, j, k) == (1, 2, 0) || (i, j, k) == (2, 0, 1) {
        1
    } else {
        -1
    }
}

// The cross product of two 3-dimensional vectors as $c_i = \sum_{j, k} \epsilon_{ijk} v_j w_k$,
// summing over all nine pairs of indices with the Levi-Civita symbol. Nothing is shared with the
// implementation.
pub fn rational_vector_cross_naive(v: &RationalVector, w: &RationalVector) -> RationalVector {
    let xs = v.elements_ref();
    let ys = w.elements_ref();
    assert!(xs.len() == 3 && ys.len() == 3);
    RationalVector::from_owned_elements(
        (0..3)
            .map(|i| {
                let mut sum = Rational::ZERO;
                for (j, x) in xs.iter().enumerate() {
                    for (k, y) in ys.iter().enumerate() {
                        match levi_civita(i, j, k) {
                            1 => sum += x * y,
                            -1 => sum -= x * y,
                            _ => {}
                        }
                    }
                }
                sum
            })
            .collect::<Vec<_>>(),
    )
}
