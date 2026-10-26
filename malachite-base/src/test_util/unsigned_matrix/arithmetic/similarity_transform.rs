// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2015 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::unsigned_matrix::arithmetic::similarity_over_ring::SimilarityRing;
use alloc::vec::Vec;

// The similarity transform of `fmpz_mat_similarity` and its `nmod_mat` and `fmpz_mod_mat`
// counterparts, in `ring`, on the n by n matrix with elements `xs`, translated loop for loop: one
// multiplication by `d` per element touched.
//
// This is equivalent to `nmod_mat_similarity` from `nmod_mat/similarity.c`, FLINT 3.6.0.
pub fn matrix_similarity_transform_flint_loops<R: SimilarityRing>(
    ring: &R,
    xs: &[R::Element],
    n: usize,
    r: usize,
    d: &R::Element,
) -> Vec<R::Element> {
    let mut xs = xs.to_vec();
    let indices = || (0..r.saturating_sub(1)).chain(r + 1..n);
    for i in 0..n {
        for j in indices() {
            let t = ring.mul(&xs[i * n + r], d);
            ring.add_assign(&mut xs[i * n + j], &t);
        }
    }
    for i in 0..n {
        for j in indices() {
            let t = ring.mul(&xs[j * n + i], d);
            ring.sub_assign(&mut xs[r * n + i], &t);
        }
    }
    xs
}
