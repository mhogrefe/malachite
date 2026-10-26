// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2015 William Hart
//
//      Copyright © 2021 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::arithmetic::mod_determinant::charpoly_berkowitz;
use crate::unsigned_vector::arithmetic::mod_dot::{ModData, dot_sum_strided};
use alloc::vec;
use alloc::vec::Vec;

// Replaces `p` with `p` times `b`, both monic polynomials given by their coefficients from the
// constant coefficient up, modulo `m`, in place: from the highest coefficient down, each new
// coefficient depends only on old ones at the same or lower indices, which are still unchanged.
fn mul_monic_assign<T: PrimitiveUnsigned>(p: &mut Vec<T>, b: &[T], m: T) {
    let p_len = p.len();
    let i = b.len() - 1;
    p.resize(p_len + i, T::ZERO);
    for k in (0..p_len + i).rev() {
        // The leading coefficient of `b` is 1.
        let mut s = if k >= i { p[k - i] } else { T::ZERO };
        for (j, &y) in b[..i].iter().enumerate() {
            if k >= j && k - j < p_len {
                s.mod_add_assign(p[k - j].mod_mul(y, m), m);
            }
        }
        p[k] = s;
    }
}

// The coefficients of the characteristic polynomial of the n by n matrix with elements `xs`, reduced
// modulo `m`, from the constant coefficient up, by Danilevsky's method, which reduces the matrix to
// Frobenius normal form with similarity transforms; or `None` if a pivot is not invertible modulo
// `m`, which cannot happen if `m` is prime.
//
// This is equivalent to `_nmod_mat_charpoly_danilevsky` from `nmod_mat/charpoly.c`, FLINT 3.6.0.
crate_test_fn! {
charpoly_danilevsky<T: PrimitiveUnsigned>(xs: &[T], n: usize, m: T) -> Option<Vec<T>> {
    if n == 0 {
        return Some(vec![T::ONE]);
    }
    if n == 1 {
        return Some(vec![xs[0].mod_neg(m), T::ONE]);
    }
    let stride = n;
    let mut n = n;
    let mut a = xs.to_vec();
    let d = ModData::new(m, n);
    let mut p = vec![T::ONE];
    let mut w = vec![T::ZERO; n];
    let mut v = vec![T::ZERO; n];
    let mut i = 1;
    while i < n {
        let mut h = a[(n - i) * stride + n - i - 1];
        while h == T::ZERO {
            let mut k = 1;
            while k < n - i && a[(n - i) * stride + n - i - k - 1] == T::ZERO {
                k += 1;
            }
            if k == n - i {
                let mut b = vec![T::ZERO; i + 1];
                b[i] = T::ONE;
                for k in 1..=i {
                    b[k - 1] = a[(n - i) * stride + n - k].mod_neg(m);
                }
                mul_monic_assign(&mut p, &b, m);
                n -= i;
                i = 1;
                if n == 1 {
                    mul_monic_assign(&mut p, &[a[0].mod_neg(m), T::ONE], m);
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
            h = a[(n - i) * stride + n - i - 1];
        }
        let h_inverse = h.mod_neg(m).mod_inverse(m)?;
        let row = (n - i) * stride;
        w[..n].copy_from_slice(&a[row..row + n]);
        for (x, &y) in v[..n].iter_mut().zip(&w[..n]) {
            *x = y.mod_mul(h_inverse, m);
        }
        let h = h_inverse.mod_neg(m);
        for j in 1..=n - i {
            let c = a[(j - 1) * stride + n - i - 1];
            let row = &mut a[(j - 1) * stride..(j - 1) * stride + n];
            for (x, &y) in row[..n - i - 1].iter_mut().zip(&v[..n - i - 1]) {
                x.mod_add_assign(c.mod_mul(y, m), m);
            }
            for (x, &y) in row[n - i..n].iter_mut().zip(&v[n - i..n]) {
                x.mod_add_assign(c.mod_mul(y, m), m);
            }
            row[n - i - 1] = c.mod_mul(h, m);
        }
        let column_dot = |a: &[T], j: usize| {
            d.reduce_sum(dot_sum_strided(&w[..n - i], &a[j..], stride, &d))
        };
        for j in 1..n - i {
            let x = column_dot(&a, j - 1);
            a[(n - i - 1) * stride + j - 1] = x;
        }
        for j in n - i..n {
            let x = column_dot(&a, j - 1).mod_add(w[j], m);
            a[(n - i - 1) * stride + j - 1] = x;
        }
        let x = column_dot(&a, n - 1);
        a[(n - i - 1) * stride + n - 1] = x;
        i += 1;
    }
    let mut b = vec![T::ZERO; n + 1];
    b[n] = T::ONE;
    for i in 1..=n {
        b[i - 1] = a[n - i].mod_neg(m);
    }
    mul_monic_assign(&mut p, &b, m);
    Some(p)
}}

// The coefficients of the characteristic polynomial of the n by n matrix with elements `xs`, reduced
// modulo `m`, from the constant coefficient up: by Danilevsky's method above dimension 8, if it
// succeeds, and otherwise by Berkowitz's division-free algorithm.
//
// This is equivalent to `nmod_mat_charpoly` from `nmod_mat/charpoly.c`, FLINT 3.6.0, without the
// path for prime moduli below 256 at dimension 32 and up, which computes the same polynomial.
#[doc(hidden)]
pub fn mod_charpoly<T: PrimitiveUnsigned>(xs: &[T], n: usize, m: T) -> Vec<T> {
    if n > 8
        && m != T::ONE
        && let Some(p) = charpoly_danilevsky(xs, n, m)
    {
        return p;
    }
    let mut cp = vec![T::ZERO; n + 1];
    charpoly_berkowitz(&mut cp, xs, n, m);
    cp
}
