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

/// The rank of `a` modulo the prime `m`, by Gaussian elimination, one element at a time.
pub fn unsigned_matrix_mod_rank_naive<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, m: T) -> u64 {
    let (height, width) = (usize::exact_from(a.height()), usize::exact_from(a.width()));
    let mut xs = a.elements.clone();
    let mut rank = 0;
    for col in 0..width {
        let Some(p) = (rank..height).find(|&i| xs[i * width + col] != T::ZERO) else {
            continue;
        };
        for j in 0..width {
            xs.swap(rank * width + j, p * width + j);
        }
        let inverse = xs[rank * width + col].mod_inverse(m).unwrap();
        for i in rank + 1..height {
            let c = xs[i * width + col].mod_mul(inverse, m);
            for j in 0..width {
                let x = c.mod_mul(xs[rank * width + j], m);
                xs[i * width + j].mod_sub_assign(x, m);
            }
        }
        rank += 1;
    }
    u64::exact_from(rank)
}
