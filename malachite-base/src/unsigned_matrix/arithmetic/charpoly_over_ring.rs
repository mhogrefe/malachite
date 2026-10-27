// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2015 William Hart
//
//      Copyright © 2022, 2026 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::logic::traits::NotAssign;
use crate::unsigned_matrix::arithmetic::berkowitz_over_ring::{
    BerkowitzRing, UnsignedModBerkowitzRing,
};
use crate::unsigned_vector::arithmetic::mod_dot::dot_sum_strided;
use alloc::vec;
use alloc::vec::Vec;

/// A ring of integers modulo a positive $m$ with the arithmetic that Danilevsky's method and the
/// reduction to Hessenberg form need, beyond what Berkowitz's algorithm needs, so that they serve
/// `UnsignedMatrix`s and malachite-nz's `NaturalMatrix`s.
#[doc(hidden)]
pub trait CharpolyRing: BerkowitzRing {
    fn is_zero(&self, x: &Self::Element) -> bool;

    fn add_assign(&self, x: &mut Self::Element, y: &Self::Element);

    // x += y * z.
    fn add_mul_assign(&self, x: &mut Self::Element, y: &Self::Element, z: &Self::Element);

    // The inverse of `x`, or `None` if `x` is not invertible.
    fn inverse(&self, x: &Self::Element) -> Option<Self::Element>;

    // The sum of the products of `xs[i]` and `ys[i * stride]`.
    fn dot_strided(
        &self,
        xs: &[Self::Element],
        ys: &[Self::Element],
        stride: usize,
    ) -> Self::Element;

    // Given `a` and `b`, not both 0, returns `(s, t, a / g, b / g)`, where g is the GCD of `a` and
    // `b` as integers and `s a + t b = g`, so that `s (a / g) + t (b / g) = 1`.
    fn xgcd(
        &self,
        a: &Self::Element,
        b: &Self::Element,
    ) -> (Self::Element, Self::Element, Self::Element, Self::Element);
}

impl<T: PrimitiveUnsigned> CharpolyRing for UnsignedModBerkowitzRing<T> {
    #[inline]
    fn is_zero(&self, x: &T) -> bool {
        *x == T::ZERO
    }

    #[inline]
    fn add_assign(&self, x: &mut T, y: &T) {
        x.mod_add_assign(*y, self.m);
    }

    #[inline]
    fn add_mul_assign(&self, x: &mut T, y: &T, z: &T) {
        x.mod_add_assign(y.mod_mul(*z, self.m), self.m);
    }

    #[inline]
    fn inverse(&self, x: &T) -> Option<T> {
        x.mod_inverse(self.m)
    }

    #[inline]
    fn dot_strided(&self, xs: &[T], ys: &[T], stride: usize) -> T {
        self.data
            .reduce_sum(dot_sum_strided(xs, ys, stride, &self.data))
    }

    // The extended Euclidean algorithm. The cofactors alternate in sign, so only their magnitudes
    // are tracked, and they are bounded by `a / g` and `b / g`, so they fit; the signs are applied
    // modulo m at the end.
    fn xgcd(&self, a: &T, b: &T) -> (T, T, T, T) {
        let m = self.m;
        let (mut r_0, mut r_1) = (*a, *b);
        let (mut s_0, mut s_1) = (T::ONE, T::ZERO);
        let (mut t_0, mut t_1) = (T::ZERO, T::ONE);
        let mut odd = false;
        while r_1 != T::ZERO {
            let q = r_0 / r_1;
            (r_0, r_1) = (r_1, r_0 - q * r_1);
            (s_0, s_1) = (s_1, s_0 + q * s_1);
            (t_0, t_1) = (t_1, t_0 + q * t_1);
            odd.not_assign();
        }
        // Now s = (-1)^k s_0 and t = (-1)^(k + 1) t_0, where k is odd if `odd` is true.
        let (s, t) = (s_0 % m, t_0 % m);
        if odd {
            (s.mod_neg(m), t, *a / r_0, *b / r_0)
        } else {
            (s, t.mod_neg(m), *a / r_0, *b / r_0)
        }
    }
}

// Replaces `p` with `p` times `b`, both monic polynomials given by their coefficients from the
// constant coefficient up, in place: from the highest coefficient down, each new coefficient
// depends only on old ones at the same or lower indices, which are still unchanged.
fn mul_monic_assign<R: CharpolyRing>(ring: &R, p: &mut Vec<R::Element>, b: &[R::Element]) {
    let p_len = p.len();
    let i = b.len() - 1;
    p.resize(p_len + i, ring.zero());
    for k in (0..p_len + i).rev() {
        // The leading coefficient of `b` is 1.
        let mut s = if k >= i {
            p[k - i].clone()
        } else {
            ring.zero()
        };
        for (j, y) in b[..i].iter().enumerate() {
            if k >= j && k - j < p_len {
                ring.add_mul_assign(&mut s, &p[k - j], y);
            }
        }
        p[k] = s;
    }
}

/// The coefficients of the characteristic polynomial of the n by n matrix with elements `xs`, in
/// `ring`, from the constant coefficient up, by Danilevsky's method, which reduces the matrix to
/// Frobenius normal form with similarity transforms; or `None` if a pivot is not invertible, which
/// cannot happen if the modulus is prime.
///
/// This is equivalent to `_nmod_mat_charpoly_danilevsky` from `nmod_mat/charpoly.c`, FLINT 3.6.0,
/// and to `_gr_mat_charpoly_danilevsky` from `gr_mat/charpoly_danilevsky.c`, which has the same
/// steps.
#[doc(hidden)]
pub fn charpoly_danilevsky_over_ring<R: CharpolyRing>(
    ring: &R,
    xs: &[R::Element],
    n: usize,
) -> Option<Vec<R::Element>> {
    if n == 0 {
        return Some(vec![ring.one()]);
    }
    if n == 1 {
        return Some(vec![ring.neg(&xs[0]), ring.one()]);
    }
    let stride = n;
    let mut n = n;
    let mut a = xs.to_vec();
    let mut p = vec![ring.one()];
    // `w` is FLINT's `W`, a copy of a row, and `v` is FLINT's `V`, that row times a scalar.
    let mut scratch = vec![ring.zero(); n << 1];
    let (w, v) = scratch.split_at_mut(n);
    let mut i = 1;
    while i < n {
        while ring.is_zero(&a[(n - i) * stride + n - i - 1]) {
            let mut k = 1;
            while k < n - i && ring.is_zero(&a[(n - i) * stride + n - i - k - 1]) {
                k += 1;
            }
            if k == n - i {
                let mut b = vec![ring.zero(); i + 1];
                b[i] = ring.one();
                for k in 1..=i {
                    b[k - 1] = ring.neg(&a[(n - i) * stride + n - k]);
                }
                mul_monic_assign(ring, &mut p, &b);
                n -= i;
                i = 1;
                if n == 1 {
                    mul_monic_assign(ring, &mut p, &[ring.neg(&a[0]), ring.one()]);
                    return Some(p);
                }
            } else {
                let (r_1, r_2) = (n - i - k - 1, n - i - 1);
                for j in 0..stride {
                    a.swap(r_1 * stride + j, r_2 * stride + j);
                }
                for j in 1..=n - i + 1 {
                    a.swap((j - 1) * stride + r_1, (j - 1) * stride + r_2);
                }
            }
        }
        let h_inverse = ring.inverse(&ring.neg(&a[(n - i) * stride + n - i - 1]))?;
        let row = (n - i) * stride;
        w[..n].clone_from_slice(&a[row..row + n]);
        for (x, y) in v[..n].iter_mut().zip(&w[..n]) {
            *x = ring.mul(y, &h_inverse);
        }
        let h = ring.neg(&h_inverse);
        for j in 1..=n - i {
            let row = &mut a[(j - 1) * stride..(j - 1) * stride + n];
            let (left, right) = row.split_at_mut(n - i - 1);
            let (c, right) = right.split_first_mut().unwrap();
            for (x, y) in left.iter_mut().zip(&v[..n - i - 1]) {
                ring.add_mul_assign(x, c, y);
            }
            for (x, y) in right.iter_mut().zip(&v[n - i..n]) {
                ring.add_mul_assign(x, c, y);
            }
            *c = ring.mul(c, &h);
        }
        for j in 1..n - i {
            let x = ring.dot_strided(&w[..n - i], &a[j - 1..], stride);
            a[(n - i - 1) * stride + j - 1] = x;
        }
        for j in n - i..n {
            let mut x = ring.dot_strided(&w[..n - i], &a[j - 1..], stride);
            ring.add_assign(&mut x, &w[j]);
            a[(n - i - 1) * stride + j - 1] = x;
        }
        let x = ring.dot_strided(&w[..n - i], &a[n - 1..], stride);
        a[(n - i - 1) * stride + n - 1] = x;
        i += 1;
    }
    let mut b = vec![ring.zero(); n + 1];
    b[n] = ring.one();
    for i in 1..=n {
        b[i - 1] = ring.neg(&a[n - i]);
    }
    mul_monic_assign(ring, &mut p, &b);
    Some(p)
}

// Replaces rows `p` and `i`, which are 0 before column `j`, of the n by n matrix with elements `xs`
// with `s` times row `p` plus `t` times row `i` and `-v` times row `p` plus `u` times row `i`,
// where `s u + t v = 1`, and then columns `p` and `i` with `u` times column `p` plus `v` times
// column `i` and `-t` times column `p` plus `s` times column `i`: the row operation is invertible,
// and the column operation is its inverse, so the matrix stays similar to the original.
#[allow(clippy::too_many_arguments)]
fn unimodular_similarity<R: CharpolyRing>(
    ring: &R,
    xs: &mut [R::Element],
    n: usize,
    j: usize,
    p: usize,
    i: usize,
    (s, t, u, v): (&R::Element, &R::Element, &R::Element, &R::Element),
) {
    let combine = |x: &R::Element, y: &R::Element, a: &R::Element, b: &R::Element| {
        let mut z = ring.mul(x, a);
        ring.add_mul_assign(&mut z, y, b);
        z
    };
    let minus_v = ring.neg(v);
    let minus_t = ring.neg(t);
    for c in j..n {
        let (x, y) = (&xs[p * n + c], &xs[i * n + c]);
        let new_p = combine(x, y, s, t);
        let new_i = combine(x, y, &minus_v, u);
        xs[p * n + c] = new_p;
        xs[i * n + c] = new_i;
    }
    for r in 0..n {
        let (x, y) = (&xs[r * n + p], &xs[r * n + i]);
        let new_p = combine(x, y, u, v);
        let new_i = combine(x, y, &minus_t, s);
        xs[r * n + p] = new_p;
        xs[r * n + i] = new_i;
    }
}

/// Replaces the n by n matrix with elements `xs`, in `ring`, with a similar upper Hessenberg
/// matrix, whose elements more than one below the diagonal are 0. For each column `j`, a nonzero
/// element below the diagonal is moved to row `j + 1` by swapping two rows and the corresponding
/// two columns. While the pivot is not invertible, each nonzero element below it is cleared by a
/// row operation of determinant 1 built from the extended GCD of the pivot and the element, which
/// leaves their GCD in the pivot's place. Once the pivot is invertible, the remaining elements are
/// cleared at once by subtracting multiples of row `j + 1`. Each row operation is followed by the
/// inverse column operation, so the matrix stays similar to the original; the inverse of the
/// subtractions adds a combination of the later columns to column `j + 1`, a dot product for each
/// row. Unlike the elimination in Danilevsky's method, this works for any modulus.
#[doc(hidden)]
pub fn hessenberg_over_ring<R: CharpolyRing>(ring: &R, xs: &mut [R::Element], n: usize) {
    for j in 0..n.saturating_sub(2) {
        let p = j + 1;
        if ring.is_zero(&xs[p * n + j]) {
            let Some(k) = (p + 1..n).find(|&k| !ring.is_zero(&xs[k * n + j])) else {
                continue;
            };
            for c in 0..n {
                xs.swap(k * n + c, p * n + c);
            }
            for r in 0..n {
                xs.swap(r * n + k, r * n + p);
            }
        }
        let mut inverse = ring.inverse(&xs[p * n + j]);
        let mut i = p + 1;
        while inverse.is_none() && i < n {
            if !ring.is_zero(&xs[i * n + j]) {
                let (s, t, u, v) = ring.xgcd(&xs[p * n + j], &xs[i * n + j]);
                unimodular_similarity(ring, xs, n, j, p, i, (&s, &t, &u, &v));
                inverse = ring.inverse(&xs[p * n + j]);
            }
            i += 1;
        }
        let Some(pivot_inverse) = inverse else {
            continue;
        };
        let multipliers: Vec<R::Element> = (i..n)
            .map(|k| ring.mul(&xs[k * n + j], &pivot_inverse))
            .collect();
        if multipliers.iter().all(|s| ring.is_zero(s)) {
            continue;
        }
        // Row `p` is 0 before column `j`, and the subtractions leave it unchanged.
        let pivot_row = xs[p * n + j..p * n + n].to_vec();
        for (k, s) in (i..n).zip(&multipliers) {
            if ring.is_zero(s) {
                continue;
            }
            for (x, y) in xs[k * n + j..k * n + n].iter_mut().zip(&pivot_row) {
                let z = ring.mul(s, y);
                ring.sub_assign(x, &z);
            }
        }
        for r in 0..n {
            let d = ring.dot(&multipliers, &xs[r * n + i..r * n + n]);
            ring.add_assign(&mut xs[r * n + p], &d);
        }
    }
}

/// The coefficients of the characteristic polynomial of the n by n upper Hessenberg matrix H with
/// elements `xs`, in `ring`, from the constant coefficient up. Expanding det(xI - H_m), for the
/// leading m by m submatrix H_m, along its last column gives p_m(x) = (x - h_{m-1,m-1}) p_{m-1}(x)
/// minus the sum, for 1 <= i < m, of h_{m-1-i,m-1} p_{m-1-i}(x) times the product of the
/// subdiagonal elements h_{j,j-1} for m - i <= j < m, so no division is needed.
#[doc(hidden)]
pub fn charpoly_of_hessenberg_over_ring<R: CharpolyRing>(
    ring: &R,
    xs: &[R::Element],
    n: usize,
) -> Vec<R::Element> {
    let mut ps: Vec<Vec<R::Element>> = Vec::with_capacity(n + 1);
    ps.push(vec![ring.one()]);
    for m in 1..=n {
        let c = m - 1;
        let previous = &ps[c];
        let mut p = vec![ring.zero(); m + 1];
        for (k, y) in previous.iter().enumerate() {
            ring.add_assign(&mut p[k + 1], y);
            let t = ring.mul(y, &xs[c * n + c]);
            ring.sub_assign(&mut p[k], &t);
        }
        let mut product = ring.one();
        for i in 1..m {
            product = ring.mul(&product, &xs[(c + 1 - i) * n + c - i]);
            if ring.is_zero(&product) {
                break;
            }
            let h = &xs[(c - i) * n + c];
            if ring.is_zero(h) {
                continue;
            }
            let factor = ring.mul(&product, h);
            for (k, y) in ps[c - i].iter().enumerate() {
                let t = ring.mul(y, &factor);
                ring.sub_assign(&mut p[k], &t);
            }
        }
        ps.push(p);
    }
    ps.pop().unwrap()
}

/// The coefficients of the characteristic polynomial of the n by n matrix with elements `xs`, in
/// `ring`, from the constant coefficient up, by reduction to Hessenberg form, which works for any
/// modulus.
#[doc(hidden)]
pub fn charpoly_hessenberg_over_ring<R: CharpolyRing>(
    ring: &R,
    xs: &[R::Element],
    n: usize,
) -> Vec<R::Element> {
    let mut a = xs.to_vec();
    hessenberg_over_ring(ring, &mut a, n);
    charpoly_of_hessenberg_over_ring(ring, &a, n)
}
