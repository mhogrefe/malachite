// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2014 Abhinav Baid
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::modular::traits::ModMulPrecomputed;
use crate::unsigned_vector::modular::mod_dot::{ModData, dot_sum};
use alloc::vec::Vec;
use core::marker::PhantomData;

/// The arithmetic the Gram-Schmidt process needs: a commutative ring in which some elements have
/// inverses.
///
/// This is not part of the public API; it is public so that `malachite-nz` and `malachite-q` can
/// orthogonalize their matrices.
#[doc(hidden)]
pub trait GramSchmidtRing {
    type Element: Clone;

    fn is_zero(&self, x: &Self::Element) -> bool;

    fn mul(&self, x: &Self::Element, y: &Self::Element) -> Self::Element;

    /// The inverse of `x`, which is not 0, or `None` if it has none.
    fn inverse(&self, x: &Self::Element) -> Option<Self::Element>;

    /// The sum of the products of `xs[i]` and `ys[i]`, where `xs` and `ys` have the same length.
    fn dot(&self, xs: &[Self::Element], ys: &[Self::Element]) -> Self::Element;

    /// Subtracts `c` times `ys[i]` from each `xs[i]`, where `xs` and `ys` have the same length.
    fn sub_mul_assign(&self, xs: &mut [Self::Element], c: &Self::Element, ys: &[Self::Element]);
}

/// The integers modulo a positive $m$ that fits in a `T`. Dot products are summed exactly, in one,
/// two, or three words, and reduced once.
///
/// This is not part of the public API; it is public so that `malachite-nz` can orthogonalize
/// `NaturalMatrix`es modulo small numbers.
#[doc(hidden)]
pub struct ModWordRing<T: PrimitiveUnsigned> {
    m: T,
    mul_data: <T as ModMulPrecomputed<T, T>>::Data,
    dot_data: ModData<T>,
}

impl<T: PrimitiveUnsigned> ModWordRing<T> {
    /// The ring for orthogonalizing vectors of `terms` elements, which must be positive, modulo
    /// `m`.
    pub fn new(m: T, terms: usize) -> Self {
        Self {
            m,
            mul_data: T::precompute_mod_mul_data(&m),
            dot_data: ModData::new(m, terms),
        }
    }
}

impl<T: PrimitiveUnsigned> GramSchmidtRing for ModWordRing<T> {
    type Element = T;

    #[inline]
    fn is_zero(&self, x: &T) -> bool {
        *x == T::ZERO
    }

    #[inline]
    fn mul(&self, x: &T, y: &T) -> T {
        x.mod_mul_precomputed(*y, self.m, &self.mul_data)
    }

    #[inline]
    fn inverse(&self, x: &T) -> Option<T> {
        x.mod_inverse(self.m)
    }

    #[inline]
    fn dot(&self, xs: &[T], ys: &[T]) -> T {
        self.dot_data.reduce_sum(dot_sum(xs, ys, &self.dot_data))
    }

    fn sub_mul_assign(&self, xs: &mut [T], c: &T, ys: &[T]) {
        for (x, &y) in xs.iter_mut().zip(ys) {
            x.mod_sub_assign(c.mod_mul_precomputed(y, self.m, &self.mul_data), self.m);
        }
    }
}

/// The integers modulo $2^k$, where $k$ is at most the width of `T`. Arithmetic wraps, and results
/// are reduced by masking.
///
/// This is not part of the public API; it is public so that `malachite-nz` can orthogonalize
/// `NaturalMatrix`es modulo small powers of 2.
#[doc(hidden)]
pub struct ModPowerOf2WordRing<T: PrimitiveUnsigned> {
    pow: u64,
    phantom: PhantomData<T>,
}

impl<T: PrimitiveUnsigned> ModPowerOf2WordRing<T> {
    pub const fn new(pow: u64) -> Self {
        Self {
            pow,
            phantom: PhantomData,
        }
    }
}

impl<T: PrimitiveUnsigned> GramSchmidtRing for ModPowerOf2WordRing<T> {
    type Element = T;

    #[inline]
    fn is_zero(&self, x: &T) -> bool {
        *x == T::ZERO
    }

    #[inline]
    fn mul(&self, x: &T, y: &T) -> T {
        x.wrapping_mul(*y).mod_power_of_2(self.pow)
    }

    #[inline]
    fn inverse(&self, x: &T) -> Option<T> {
        x.mod_power_of_2_inverse(self.pow)
    }

    fn dot(&self, xs: &[T], ys: &[T]) -> T {
        let mut sum = T::ZERO;
        for (&x, &y) in xs.iter().zip(ys) {
            sum.wrapping_add_assign(x.wrapping_mul(y));
        }
        sum.mod_power_of_2(self.pow)
    }

    fn sub_mul_assign(&self, xs: &mut [T], c: &T, ys: &[T]) {
        for (x, &y) in xs.iter_mut().zip(ys) {
            *x = x.wrapping_sub(c.wrapping_mul(y)).mod_power_of_2(self.pow);
        }
    }
}

// The elements of the `height` by `width` matrix with elements `xs` in row-major order, in
// column-major order.
fn transpose<E: Clone>(xs: &[E], height: usize, width: usize) -> Vec<E> {
    let mut ys = Vec::with_capacity(xs.len());
    for j in 0..width {
        for i in 0..height {
            ys.push(xs[i * width + j].clone());
        }
    }
    ys
}

/// The Gram-Schmidt process applied to the columns of the `height` by `width` matrix with elements
/// `xs`, in row-major order, without normalizing them; or `None` if a squared length it needs to
/// divide by has no inverse. Column $i$ of the result is column $i$ of the matrix minus its
/// projections onto the earlier columns of the result. A column whose squared length is 0 is
/// skipped if it is 0, as it always is over the rationals; a nonzero column whose squared length is
/// 0 makes the result `None`. The squared length of the last column is never divided by.
///
/// The columns are worked on in column-major order, so that each dot product reads consecutive
/// elements, and the inverse of each column's squared length is found once, when the column is
/// final.
///
/// This is not part of the public API; it is public so that `malachite-nz` and `malachite-q` can
/// orthogonalize their matrices.
///
/// This is equivalent to `fmpq_mat_gso` from `fmpq_mat/gso.c`, FLINT 3.6.0.
#[doc(hidden)]
pub fn gram_schmidt_in_ring<R: GramSchmidtRing>(
    ring: &R,
    xs: &[R::Element],
    height: usize,
    width: usize,
) -> Option<Vec<R::Element>> {
    if height == 0 || width == 0 {
        return Some(xs.to_vec());
    }
    let a = transpose(xs, height, width);
    let mut b = a.clone();
    // The inverse of the squared length of each final column but the last, or `None` for a column
    // that is 0.
    let mut inverses: Vec<Option<R::Element>> = Vec::with_capacity(width - 1);
    for i in 0..width {
        let (done, rest) = b.split_at_mut(i * height);
        let b_i = &mut rest[..height];
        let a_i = &a[i * height..(i + 1) * height];
        for (j, inverse) in inverses.iter().enumerate() {
            if let Some(inverse) = inverse {
                let b_j = &done[j * height..(j + 1) * height];
                let mu = ring.mul(&ring.dot(a_i, b_j), inverse);
                ring.sub_mul_assign(b_i, &mu, b_j);
            }
        }
        if i + 1 < width {
            let squared_length = ring.dot(b_i, b_i);
            inverses.push(if !ring.is_zero(&squared_length) {
                Some(ring.inverse(&squared_length)?)
            } else if b_i.iter().all(|x| ring.is_zero(x)) {
                None
            } else {
                return None;
            });
        }
    }
    Some(transpose(&b, width, height))
}
