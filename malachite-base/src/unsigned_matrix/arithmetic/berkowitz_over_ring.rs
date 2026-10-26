// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2023 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::arithmetic::mod_dot::{ModData, column_sum, dot_sum};
use alloc::vec;
use alloc::vec::Vec;

/// A ring with the arithmetic that `charpoly_berkowitz_over_ring` needs, so that it serves the
/// matrices of `UnsignedMatrix`s and malachite-nz's `NaturalMatrix`s modulo a number, and
/// malachite-nz's `IntegerMatrix`s. The dot products are the operations worth specializing: each is
/// accumulated exactly and reduced once.
#[doc(hidden)]
pub trait BerkowitzRing {
    type Element: Clone;

    fn zero(&self) -> Self::Element;

    fn one(&self) -> Self::Element;

    fn neg(&self, x: &Self::Element) -> Self::Element;

    fn add(&self, x: &Self::Element, y: &Self::Element) -> Self::Element;

    fn sub_assign(&self, x: &mut Self::Element, y: &Self::Element);

    fn mul(&self, x: &Self::Element, y: &Self::Element) -> Self::Element;

    // The sum of the products of `xs[i]` and `ys[i]`.
    fn dot(&self, xs: &[Self::Element], ys: &[Self::Element]) -> Self::Element;

    // The sum of the products of `xs[i]` and `ys[ys.len() - 1 - i]`.
    fn dot_rev(&self, xs: &[Self::Element], ys: &[Self::Element]) -> Self::Element;
}

/// The integers modulo a positive `T`, with the data for accumulating dot products of up to a given
/// number of terms.
#[doc(hidden)]
pub struct UnsignedModBerkowitzRing<T: PrimitiveUnsigned> {
    m: T,
    data: ModData<T>,
}

impl<T: PrimitiveUnsigned> UnsignedModBerkowitzRing<T> {
    // The ring modulo `m`, for dot products of up to `terms` terms.
    pub(crate) fn new(m: T, terms: usize) -> Self {
        Self {
            m,
            data: ModData::new(m, terms),
        }
    }
}

impl<T: PrimitiveUnsigned> BerkowitzRing for UnsignedModBerkowitzRing<T> {
    type Element = T;

    #[inline]
    fn zero(&self) -> T {
        T::ZERO
    }

    #[inline]
    fn one(&self) -> T {
        if self.m == T::ONE { T::ZERO } else { T::ONE }
    }

    #[inline]
    fn neg(&self, x: &T) -> T {
        x.mod_neg(self.m)
    }

    #[inline]
    fn add(&self, x: &T, y: &T) -> T {
        x.mod_add(*y, self.m)
    }

    #[inline]
    fn sub_assign(&self, x: &mut T, y: &T) {
        x.mod_sub_assign(*y, self.m);
    }

    #[inline]
    fn mul(&self, x: &T, y: &T) -> T {
        x.mod_mul(*y, self.m)
    }

    #[inline]
    fn dot(&self, xs: &[T], ys: &[T]) -> T {
        self.data.reduce_sum(dot_sum(xs, ys, &self.data))
    }

    #[inline]
    fn dot_rev(&self, xs: &[T], ys: &[T]) -> T {
        self.data.reduce_sum(column_sum(xs, ys, &self.data))
    }
}

/// The coefficients of the characteristic polynomial of the n by n matrix with elements `xs`, in
/// `ring`, from the constant coefficient up, with Berkowitz's division-free algorithm.
///
/// This is equivalent to `_gr_mat_charpoly_berkowitz` from `gr_mat/charpoly_berkowitz.c`, FLINT
/// 3.6.0, and to `_nmod_mat_charpoly_berkowitz` from `nmod_mat/charpoly.c`, which has the same
/// steps.
#[doc(hidden)]
pub fn charpoly_berkowitz_over_ring<R: BerkowitzRing>(
    ring: &R,
    xs: &[R::Element],
    n: usize,
) -> Vec<R::Element> {
    let entry = |i: usize, j: usize| &xs[i * n + j];
    let mut cp = vec![ring.zero(); n + 1];
    match n {
        0 => cp[0] = ring.one(),
        1 => {
            cp[0] = ring.neg(entry(0, 0));
            cp[1] = ring.one();
        }
        2 => {
            cp[0] = ring.mul(entry(0, 0), entry(1, 1));
            ring.sub_assign(&mut cp[0], &ring.mul(entry(0, 1), entry(1, 0)));
            cp[1] = ring.neg(&ring.add(entry(0, 0), entry(1, 1)));
            cp[2] = ring.one();
        }
        _ => {
            // `a` holds n - 1 rows of length n, and `big_a` is FLINT's `A`, the last row of
            // FLINT's `a`.
            let mut scratch = vec![ring.zero(); n * n];
            let (a, big_a) = scratch.split_at_mut((n - 1) * n);
            cp[0] = ring.neg(entry(0, 0));
            for t in 1..n {
                for (i, x) in a[..=t].iter_mut().enumerate() {
                    *x = entry(i, t).clone();
                }
                big_a[0] = entry(t, t).clone();
                for k in 1..t {
                    let (previous, current) = a.split_at_mut(k * n);
                    let previous = &previous[(k - 1) * n..][..=t];
                    for (i, x) in current[..=t].iter_mut().enumerate() {
                        *x = ring.dot(&xs[i * n..][..=t], previous);
                    }
                    big_a[k] = current[t].clone();
                }
                big_a[t] = ring.dot(&xs[t * n..][..=t], &a[(t - 1) * n..][..=t]);
                for k in 0..=t {
                    let s = ring.dot_rev(&big_a[..k], &cp[..k]);
                    ring.sub_assign(&mut cp[k], &s);
                    ring.sub_assign(&mut cp[k], &big_a[k]);
                }
            }
            // Shift all coefficients up by one.
            cp.rotate_right(1);
            cp[0] = ring.one();
            cp.reverse();
        }
    }
    cp
}
