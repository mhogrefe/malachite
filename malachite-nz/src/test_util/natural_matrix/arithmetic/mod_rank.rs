// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModInverse, ModMul, ModSubAssign};
use malachite_base::num::conversion::traits::ExactFrom;

/// The rank of `a` modulo the prime `m`, by Gaussian elimination, one element at a time.
pub fn natural_matrix_mod_rank_naive(a: &NaturalMatrix, m: &Natural) -> u64 {
    let height = usize::exact_from(a.height());
    let width = usize::exact_from(a.width());
    let mut xs = a.elements.clone();
    let mut rank = 0;
    for col in 0..width {
        let Some(p) = (rank..height).find(|&i| xs[i * width + col] != 0u32) else {
            continue;
        };
        for j in 0..width {
            xs.swap(rank * width + j, p * width + j);
        }
        let inverse = (&xs[rank * width + col]).mod_inverse(m).unwrap();
        for i in rank + 1..height {
            let c = (&xs[i * width + col]).mod_mul(&inverse, m);
            for j in 0..width {
                let x = (&c).mod_mul(&xs[rank * width + j], m);
                xs[i * width + j].mod_sub_assign(x, m);
            }
        }
        rank += 1;
    }
    u64::exact_from(rank)
}
