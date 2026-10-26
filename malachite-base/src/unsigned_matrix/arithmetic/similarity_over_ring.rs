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

use crate::unsigned_matrix::arithmetic::permanent_over_ring::{PermanentModRing, PermanentRing};

/// A ring with the arithmetic that `similarity_transform_in_place` needs, so that it serves the
/// matrices of `UnsignedMatrix`s and malachite-nz's `NaturalMatrix`s modulo a number or a power of
/// 2, as well as malachite-nz's `IntegerMatrix`s and malachite-q's `RationalMatrix`s.
#[doc(hidden)]
pub trait SimilarityRing {
    type Element: Clone;

    fn zero(&self) -> Self::Element;

    fn is_zero(&self, x: &Self::Element) -> bool;

    fn add_assign(&self, x: &mut Self::Element, y: &Self::Element);

    fn sub_assign(&self, x: &mut Self::Element, y: &Self::Element);

    fn mul(&self, x: &Self::Element, y: &Self::Element) -> Self::Element;
}

impl<R: PermanentModRing> SimilarityRing for R {
    type Element = <R as PermanentRing>::Element;

    #[inline]
    fn zero(&self) -> Self::Element {
        PermanentRing::zero(self)
    }

    #[inline]
    fn is_zero(&self, x: &Self::Element) -> bool {
        PermanentRing::is_zero(self, x)
    }

    #[inline]
    fn add_assign(&self, x: &mut Self::Element, y: &Self::Element) {
        PermanentRing::add_assign(self, x, y);
    }

    #[inline]
    fn sub_assign(&self, x: &mut Self::Element, y: &Self::Element) {
        PermanentModRing::sub_assign(self, x, y);
    }

    #[inline]
    fn mul(&self, x: &Self::Element, y: &Self::Element) -> Self::Element {
        let mut z = x.clone();
        PermanentRing::mul_assign(self, &mut z, y);
        z
    }
}

/// Replaces the $n \times n$ matrix with elements `xs`, in `ring`, with $P^{-1}AP$, where $P$ is
/// the identity matrix with the elements of row `r` in the columns other than `r - 1` and `r`
/// replaced by `d`: first `d` times column `r` is added to each of those columns, and then `d`
/// times each of the corresponding rows is subtracted from row `r`. The caller checks that `r < n`.
///
/// This is equivalent to `fmpz_mat_similarity` from `fmpz_mat/similarity.c`, FLINT 3.6.0, and to
/// its `nmod_mat`, `fmpz_mod_mat`, and `fmpq_mat` counterparts, except that it multiplies by `d`
/// once per row in the first step and once per column in the second, rather than once per element.
#[doc(hidden)]
pub fn similarity_transform_in_place<R: SimilarityRing>(
    ring: &R,
    xs: &mut [R::Element],
    n: usize,
    r: usize,
    d: &R::Element,
) {
    if ring.is_zero(d) {
        return;
    }
    // The columns, and rows, that P mixes with column, and row, r.
    let mixed = |j: usize| j != r && j + 1 != r;
    // A := AP
    for row in xs.chunks_exact_mut(n) {
        if ring.is_zero(&row[r]) {
            continue;
        }
        let x = ring.mul(&row[r], d);
        for (j, y) in row.iter_mut().enumerate() {
            if mixed(j) {
                ring.add_assign(y, &x);
            }
        }
    }
    // A := P^{-1}A, where P^{-1} is P with -d in place of d
    for j in 0..n {
        let mut sum = ring.zero();
        for k in (0..n).filter(|&k| mixed(k)) {
            ring.add_assign(&mut sum, &xs[k * n + j]);
        }
        let t = ring.mul(&sum, d);
        ring.sub_assign(&mut xs[r * n + j], &t);
    }
}
