// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;
use alloc::vec::Vec;

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

// The cross product of two 3-dimensional vectors modulo `m` as $c_i = \sum_{j, k} \epsilon_{ijk}
// v_j w_k$, summing over all nine pairs of indices with the Levi-Civita symbol and reducing after
// every operation. Nothing is shared with the implementation.
pub fn unsigned_vector_mod_cross_naive<T: PrimitiveUnsigned>(
    v: &UnsignedVector<T>,
    w: &UnsignedVector<T>,
    m: T,
) -> UnsignedVector<T> {
    let xs = &v.elements;
    let ys = &w.elements;
    assert!(xs.len() == 3 && ys.len() == 3);
    UnsignedVector {
        elements: (0..3)
            .map(|i| {
                let mut sum = T::ZERO;
                for (j, &x) in xs.iter().enumerate() {
                    for (k, &y) in ys.iter().enumerate() {
                        let product = x.mod_mul(y, m);
                        match levi_civita(i, j, k) {
                            1 => sum = sum.mod_add(product, m),
                            -1 => sum = sum.mod_sub(product, m),
                            _ => {}
                        }
                    }
                }
                sum
            })
            .collect::<Vec<T>>(),
    }
}
