// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2015 Tommy Hofmann
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::mod_div::gcdinv;
use crate::num::arithmetic::traits::{
    ExtendedGcd, ModIsReduced, ModStrongEchelonForm, ModStrongEchelonFormAssign,
};
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingFrom;
use crate::unsigned_matrix::UnsignedMatrix;
use alloc::vec::Vec;

// Swaps rows `i` and `j` of the matrix with elements `xs` and width `width`.
pub(crate) fn swap_rows<T>(xs: &mut [T], width: usize, i: usize, j: usize) {
    if i != j {
        for k in 0..width {
            xs.swap(i * width + k, j * width + k);
        }
    }
}

// Returns whether column `col` has a nonzero element in row `start_row` or below, swapping the
// first such row into row `start_row` if it is below.
//
// This is equivalent to `_nmod_mat_pivot` from `nmod_mat/strong_echelon_form.c`, FLINT 3.6.0, which
// also reports whether a swap was made.
fn pivot<T: PrimitiveUnsigned>(
    xs: &mut [T],
    height: usize,
    width: usize,
    start_row: usize,
    col: usize,
) -> bool {
    if let Some(j) = (start_row..height).find(|&j| xs[j * width + col] != T::ZERO) {
        swap_rows(xs, width, j, start_row);
        true
    } else {
        false
    }
}

// Rows `i` and `j`, where `i < j`, of the matrix with elements `xs` and width `width`.
pub(crate) fn two_rows<T>(xs: &mut [T], width: usize, i: usize, j: usize) -> (&mut [T], &mut [T]) {
    let (top, bottom) = xs.split_at_mut(j * width);
    (&mut top[i * width..(i + 1) * width], &mut bottom[..width])
}

// Returns `(c, n)` where `c` is the largest divisor of `a` all of whose prime factors divide `b`,
// and `n = a / c`.
//
// This is equivalent to `_n_ppio` from `nmod_mat/strong_echelon_form.c`, FLINT 3.6.0.
fn ppio<T: PrimitiveUnsigned>(a: T, b: T) -> (T, T) {
    let mut c = a.gcd(b);
    let mut n = a / c;
    let mut g = c.gcd(n);
    while g != T::ONE {
        c *= g;
        n /= g;
        g = c.gcd(n);
    }
    (c, n)
}

// This is equivalent to `_n_stab` from `nmod_mat/strong_echelon_form.c`, FLINT 3.6.0.
fn stab<T: PrimitiveUnsigned>(a: T, b: T, m: T) -> T {
    let g = a.gcd(b);
    let b = g.gcd(m);
    ppio(m / b, a / b).1
}

// Returns a unit `u` modulo `m` such that `ua` is the canonical associate of `a`: the divisor of
// `m` that generates the same ideal of $\mathbb{Z}/m\mathbb{Z}$ as `a`.
//
// This is equivalent to `_n_unit` from `nmod_mat/strong_echelon_form.c`, FLINT 3.6.0.
fn unit<U: PrimitiveUnsigned + WrappingFrom<S>, S: PrimitiveSigned + WrappingFrom<U>>(
    a: U,
    m: U,
) -> U {
    let (g, s) = gcdinv::<U, S>(a, m);
    if g == U::ONE {
        s
    } else {
        let l = m / g;
        let d = stab(s, l, m);
        s.mod_add((d % m).mod_mul(l % m, m), m)
    }
}

// Returns `Some(q)` with `qa = b` modulo `m` if there is such a `q`, and `None` otherwise.
//
// This is equivalent to `_n_is_divisible` from `nmod_mat/strong_echelon_form.c`, FLINT 3.6.0.
fn is_divisible<U: PrimitiveUnsigned + WrappingFrom<S>, S: PrimitiveSigned + WrappingFrom<U>>(
    b: U,
    a: U,
    m: U,
) -> Option<U> {
    let (g, e) = gcdinv::<U, S>(a, m);
    if b % g == U::ZERO {
        Some(e.mod_mul(b / g, m))
    } else {
        None
    }
}

// The residue modulo `m` of a cofactor `x` of the extended GCD of two elements less than `m`, which
// therefore has absolute value less than `m`.
fn signed_to_residue<U: PrimitiveUnsigned + WrappingFrom<S>, S: PrimitiveSigned>(x: S, m: U) -> U {
    if x >= S::ZERO {
        U::wrapping_from(x)
    } else {
        m - U::wrapping_from(x).wrapping_neg()
    }
}

// Returns `(g, s, t, u, v)`, where `g = gcd(a, b)`, `sa + tb = g` modulo `m`, `u = -b / g` and `v =
// a / g` modulo `m`, so that the transformation `(x, y) -> (sx + ty, ux + vy)` of a pair of rows is
// invertible modulo `m`. `g` is 0 if `a` and `b` are both 0.
//
// FLINT computes `s` and `t` with `n_xgcd`, which returns nonnegative cofactors with `as - bt = g`,
// and then negates `t`; here the signed cofactors of `extended_gcd` are reduced instead. Any such
// cofactors give an invertible transformation, and the strong echelon form is unique, so the result
// does not depend on the choice.
fn xgcd_transform<
    U: PrimitiveUnsigned + ExtendedGcd<U, Gcd = U, Cofactor = S> + WrappingFrom<S>,
    S: PrimitiveSigned,
>(
    a: U,
    b: U,
    m: U,
) -> (U, U, U, U, U) {
    let (g, x, y) = a.extended_gcd(b);
    if g == U::ZERO {
        return (g, U::ZERO, U::ZERO, U::ZERO, U::ZERO);
    }
    (
        g,
        signed_to_residue(x, m),
        signed_to_residue(y, m),
        (b / g).mod_neg(m),
        a / g,
    )
}

// Puts the `height` by `width` matrix with elements `xs`, which are reduced modulo `m`, into strong
// echelon form. `height` must be at least `width`.
//
// This is equivalent to `nmod_mat_strong_echelon_form` from `nmod_mat/strong_echelon_form.c`, FLINT
// 3.6.0.
pub(crate) fn strong_echelon_form_in_place<
    U: PrimitiveUnsigned + ExtendedGcd<U, Gcd = U, Cofactor = S> + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    xs: &mut [U],
    height: usize,
    width: usize,
    m: U,
) {
    if height == 0 || width == 0 {
        return;
    }
    assert!(height >= width);
    let n = height;
    let w = width;
    let data = U::precompute_mod_mul_data(&m);
    let mul = |x: U, y: U| x.mod_mul_precomputed(y, m, &data);
    let mut extra_row = Vec::with_capacity(w);
    let mut row = 0;
    let mut col = 0;
    while row < n && col < w {
        if !pivot(xs, n, w, row, col) {
            col += 1;
            continue;
        }
        for i in row + 1..n {
            let (pivot_row, other_row) = two_rows(xs, w, row, i);
            if other_row[col] == U::ZERO {
                continue;
            }
            if let Some(s) = is_divisible::<U, S>(other_row[col], pivot_row[col], m) {
                for (y, &x) in other_row[col..].iter_mut().zip(&pivot_row[col..]) {
                    *y = y.mod_sub(mul(s, x), m);
                }
            } else {
                let (_, s, t, u, v) = xgcd_transform(pivot_row[col], other_row[col], m);
                for (x, y) in pivot_row[col..].iter_mut().zip(&mut other_row[col..]) {
                    (*x, *y) = (
                        mul(s, *x).mod_add(mul(t, *y), m),
                        mul(u, *x).mod_add(mul(v, *y), m),
                    );
                }
            }
        }
        row += 1;
        col += 1;
    }
    for col in 0..w {
        if xs[col * w + col] == U::ZERO {
            extra_row.clear();
            extra_row.extend_from_slice(&xs[col * w..(col + 1) * w]);
        } else {
            let u = unit::<U, S>(xs[col * w + col], m);
            let (above, pivot_and_below) = xs.split_at_mut(col * w);
            let pivot_row = &mut pivot_and_below[..w];
            for x in &mut pivot_row[col..] {
                *x = mul(u, *x);
            }
            let d = pivot_row[col];
            for (row, above_row) in above.chunks_exact_mut(w).enumerate() {
                let q = above_row[col] / d;
                for (x, &y) in above_row[row..].iter_mut().zip(&pivot_row[row..]) {
                    *x = x.mod_sub(mul(q, y), m);
                }
            }
            let g = m.gcd(d);
            if g == U::ONE {
                continue;
            }
            let g = m / g;
            extra_row.clear();
            extra_row.extend(pivot_row.iter().map(|&x| mul(g, x)));
        }
        for row in col + 1..w {
            let this_row = &mut xs[row * w..(row + 1) * w];
            let (g, s, t, u, v) = xgcd_transform(this_row[row], extra_row[row], m);
            if g == U::ZERO {
                continue;
            }
            for (x, y) in this_row[row..].iter_mut().zip(&mut extra_row[row..]) {
                (*x, *y) = (
                    mul(s, *x).mod_add(mul(t, *y), m),
                    mul(u, *x).mod_add(mul(v, *y), m),
                );
            }
        }
    }
}

fn assert_arguments<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, m: T) {
    assert_ne!(m, T::ZERO, "m cannot be zero");
    assert!(
        a.mod_is_reduced(&m),
        "the matrix must be reduced mod m, but {a} has an element >= {m}"
    );
}

// Appends zero rows to `a` so that it has at least as many rows as columns. Zero rows do not change
// the row span.
pub(crate) fn pad<T: PrimitiveUnsigned>(a: &mut UnsignedMatrix<T>) {
    if a.height < a.width {
        a.elements.resize(a.width * a.width, T::ZERO);
        a.height = a.width;
    }
}

macro_rules! impl_mod_strong_echelon_form {
    ($u:ident, $s:ident) => {
        impl ModStrongEchelonForm<$u> for UnsignedMatrix<$u> {
            type Output = Self;

            /// Computes the strong echelon form of an [`UnsignedMatrix`] modulo $m$, taking it by
            /// value.
            ///
            /// The elements of the matrix must already be reduced modulo $m$. The strong echelon
            /// form of an $h \times w$ matrix $A$ modulo $m$ is the $\max(h, w) \times w$ matrix
            /// with the same nonzero rows as the Howell form of $A$ (see
            /// [`ModHowellForm`](crate::num::arithmetic::traits::ModHowellForm)), but with each
            /// nonzero row in the row whose index is the column of its pivot (its first nonzero
            /// element), so that the pivots are on the diagonal, and with zero rows everywhere
            /// else. Like the Howell form, it is unique.
            ///
            /// A matrix with fewer rows than columns is first given zero rows until it is square,
            /// which leaves its row span unchanged. The strong echelon form is computed with the
            /// algorithm of Fieker and Hofmann, which reduces the matrix to upper triangular form
            /// with unimodular row operations, makes each diagonal element a divisor of $m$,
            /// reduces the elements above it, and then, for each diagonal element $d$ with $d \ne
            /// 1$, combines the multiple $(m/d)$ of its row, which may have a nonzero element
            /// further right, into the rows below.
            ///
            /// # Worst-case complexity
            /// $T(h, w) = O(hw^2)$
            ///
            /// $M(h, w) = O(hw)$
            ///
            /// where $T$ is time, $M$ is additional memory, $h$ is the larger of the matrix's
            /// height and width, and $w$ is its width.
            ///
            /// # Panics
            /// Panics if `m` is zero, or if any element of the matrix is greater than or equal to
            /// `m`.
            ///
            /// # Examples
            /// See [here](super::mod_strong_echelon_form#mod_strong_echelon_form).
            ///
            /// This is equivalent to `nmod_mat_strong_echelon_form` from
            /// `nmod_mat/strong_echelon_form.c`, FLINT 3.6.0, except that a matrix with fewer rows
            /// than columns is given zero rows, which FLINT does not allow, and that it panics if
            /// the matrix is not reduced.
            #[inline]
            fn mod_strong_echelon_form(mut self, m: $u) -> Self {
                self.mod_strong_echelon_form_assign(m);
                self
            }
        }

        impl ModStrongEchelonForm<$u> for &UnsignedMatrix<$u> {
            type Output = UnsignedMatrix<$u>;

            /// Computes the strong echelon form of an [`UnsignedMatrix`] modulo $m$, taking it by
            /// reference.
            ///
            /// See the documentation for the implementation that takes the matrix by value for
            /// details.
            ///
            /// # Worst-case complexity
            /// $T(h, w) = O(hw^2)$
            ///
            /// $M(h, w) = O(hw)$
            ///
            /// where $T$ is time, $M$ is additional memory, $h$ is the larger of the matrix's
            /// height and width, and $w$ is its width.
            ///
            /// # Panics
            /// Panics if `m` is zero, or if any element of the matrix is greater than or equal to
            /// `m`.
            ///
            /// # Examples
            /// See [here](super::mod_strong_echelon_form#mod_strong_echelon_form).
            ///
            /// This is equivalent to `nmod_mat_strong_echelon_form` from
            /// `nmod_mat/strong_echelon_form.c`, FLINT 3.6.0, except that a matrix with fewer rows
            /// than columns is given zero rows, which FLINT does not allow, and that it panics if
            /// the matrix is not reduced.
            #[inline]
            fn mod_strong_echelon_form(self, m: $u) -> UnsignedMatrix<$u> {
                self.clone().mod_strong_echelon_form(m)
            }
        }

        impl ModStrongEchelonFormAssign<$u> for UnsignedMatrix<$u> {
            /// Replaces an [`UnsignedMatrix`] with its strong echelon form modulo $m$.
            ///
            /// See the documentation for the [`ModStrongEchelonForm`] implementation that takes the
            /// matrix by value for details.
            ///
            /// # Worst-case complexity
            /// $T(h, w) = O(hw^2)$
            ///
            /// $M(h, w) = O(hw)$
            ///
            /// where $T$ is time, $M$ is additional memory, $h$ is the larger of the matrix's
            /// height and width, and $w$ is its width.
            ///
            /// # Panics
            /// Panics if `m` is zero, or if any element of the matrix is greater than or equal to
            /// `m`.
            ///
            /// # Examples
            /// See [here](super::mod_strong_echelon_form#mod_strong_echelon_form_assign).
            ///
            /// This is equivalent to `nmod_mat_strong_echelon_form` from
            /// `nmod_mat/strong_echelon_form.c`, FLINT 3.6.0, except that a matrix with fewer rows
            /// than columns is given zero rows, which FLINT does not allow, and that it panics if
            /// the matrix is not reduced.
            fn mod_strong_echelon_form_assign(&mut self, m: $u) {
                assert_arguments(self, m);
                pad(self);
                strong_echelon_form_in_place::<$u, $s>(
                    &mut self.elements,
                    self.height,
                    self.width,
                    m,
                );
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_mod_strong_echelon_form);
