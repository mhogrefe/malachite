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

use crate::integer::Integer;
use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::platform::Limb;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    AddMul, DivExact, DivExactAssign, DivModPrecomputed, DivisibleBy, ExtendedGcd, Gcd, Mod,
    ModAdd, ModIsReduced, ModMul, ModNeg, ModStrongEchelonForm, ModStrongEchelonFormAssign,
    ModSubAssign, UnsignedAbs,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::unsigned_matrix::UnsignedMatrix;

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
// This is equivalent to `_fmpz_mat_pivot` from `fmpz_mat/strong_echelon_form_mod.c`, FLINT 3.6.0,
// which also reports whether a swap was made.
fn pivot(xs: &mut [Natural], height: usize, width: usize, start_row: usize, col: usize) -> bool {
    if let Some(j) = (start_row..height).find(|&j| xs[j * width + col] != 0u32) {
        swap_rows(xs, width, j, start_row);
        true
    } else {
        false
    }
}

// Rows `i` and `j`, where `i < j`, of the matrix with elements `xs` and width `width`.
fn two_rows(
    xs: &mut [Natural],
    width: usize,
    i: usize,
    j: usize,
) -> (&mut [Natural], &mut [Natural]) {
    let (top, bottom) = xs.split_at_mut(j * width);
    (&mut top[i * width..(i + 1) * width], &mut bottom[..width])
}

// This is equivalent to `_fmpz_ppio` from `fmpz_mat/strong_echelon_form_mod.c`, FLINT 3.6.0.
fn ppio(a: &Natural, b: &Natural) -> (Natural, Natural) {
    let mut c = a.gcd(b);
    let mut n = a.div_exact(&c);
    let mut g = (&c).gcd(&n);
    while g != 1u32 {
        c *= &g;
        n.div_exact_assign(&g);
        g = (&c).gcd(&n);
    }
    (c, n)
}

// This is equivalent to `_fmpz_stab` from `fmpz_mat/strong_echelon_form_mod.c`, FLINT 3.6.0.
fn stab(a: &Natural, b: &Natural, m: &Natural) -> Natural {
    let g = a.gcd(b);
    let gg = g.gcd(m);
    let bb = m.div_exact(&gg);
    let aa = a.div_exact(&gg);
    ppio(&bb, &aa).1
}

fn integer_to_residue(x: Integer, m: &Natural) -> Natural {
    x.mod_op(Integer::from(m)).unsigned_abs()
}

// This is equivalent to `_fmpz_unit` from `fmpz_mat/strong_echelon_form_mod.c`, FLINT 3.6.0.
fn unit(a: &Natural, m: &Natural) -> Natural {
    let (g, s, _) = a.extended_gcd(m);
    if g == 1u32 {
        integer_to_residue(s, m)
    } else {
        let l = m.div_exact(&g);
        let s = integer_to_residue(s, m);
        let d = stab(&s, &l, m);
        s.add_mul(d, l) % m
    }
}

// This is equivalent to `_fmpz_is_divisible_mod` from `fmpz_mat/strong_echelon_form_mod.c`, FLINT
// 3.6.0.
fn is_divisible(b: &Natural, a: &Natural, m: &Natural) -> Option<Natural> {
    let (g, e, _) = a.extended_gcd(m);
    if b.divisible_by(&g) {
        Some(integer_to_residue(e * Integer::from(b.div_exact(&g)), m))
    } else {
        None
    }
}

// Returns `(g, s, t, u, v)`, where `g = gcd(a, b)`, `sa + tb = g`, `u = -b / g`, and `v = a / g`,
// all modulo `m`; `g` is 0 if `a` and `b` are both 0.
fn xgcd_transform(
    a: &Natural,
    b: &Natural,
    m: &Natural,
) -> (Natural, Natural, Natural, Natural, Natural) {
    let (g, s, t) = a.extended_gcd(b);
    if g == 0u32 {
        return (
            g,
            Natural::ZERO,
            Natural::ZERO,
            Natural::ZERO,
            Natural::ZERO,
        );
    }
    let u = b.div_exact(&g).mod_neg(m);
    let v = a.div_exact(&g);
    (g, integer_to_residue(s, m), integer_to_residue(t, m), u, v)
}

// Replaces `(x, y)` by `(sx + ty, ux + vy)`, modulo `m`.
fn transform_pair(
    x: &Natural,
    y: &Natural,
    (s, t, u, v): (&Natural, &Natural, &Natural, &Natural),
    m: &Natural,
) -> (Natural, Natural) {
    (
        s.mod_mul(x, m).mod_add(t.mod_mul(y, m), m),
        u.mod_mul(x, m).mod_add(v.mod_mul(y, m), m),
    )
}

// Puts the `height` by `width` matrix with elements `xs`, which are reduced modulo `m`, into strong
// echelon form. `height` must be at least `width`. The elements stay reduced throughout, so the
// reductions modulo `m` that FLINT applies to its signed intermediate values are not needed.
//
// This is equivalent to `fmpz_mat_strong_echelon_form_mod` from
// `fmpz_mat/strong_echelon_form_mod.c`, FLINT 3.6.0, without its final pass. That pass reduces each
// element above a nonzero diagonal element `d` by multiples of `d`'s row, but each such element is
// already less than `d`: the reduction at `d`'s step of the second loop leaves it so, and no later
// step changes it or `d`, since they only change later rows, and columns of earlier rows from their
// own diagonal onwards.
pub(crate) fn strong_echelon_form_in_place(
    xs: &mut [Natural],
    height: usize,
    width: usize,
    m: &Natural,
) {
    if height == 0 || width == 0 {
        return;
    }
    assert!(height >= width);
    let n = height;
    let w = width;
    let mut extra_row: Vec<Natural> = Vec::with_capacity(w);
    let mut row = 0;
    let mut col = 0;
    while row < n && col < w {
        if !pivot(xs, n, w, row, col) {
            col += 1;
            continue;
        }
        for i in row + 1..n {
            let (pivot_row, other_row) = two_rows(xs, w, row, i);
            if other_row[col] == 0u32 {
                continue;
            }
            if let Some(s) = is_divisible(&other_row[col], &pivot_row[col], m) {
                for (y, x) in other_row[col..].iter_mut().zip(&pivot_row[col..]) {
                    y.mod_sub_assign((&s).mod_mul(x, m), m);
                }
            } else {
                let (_, s, t, u, v) = xgcd_transform(&pivot_row[col], &other_row[col], m);
                for (x, y) in pivot_row[col..].iter_mut().zip(&mut other_row[col..]) {
                    (*x, *y) = transform_pair(x, y, (&s, &t, &u, &v), m);
                }
            }
        }
        row += 1;
        col += 1;
    }
    for col in 0..w {
        if xs[col * w + col] == 0u32 {
            extra_row.clear();
            extra_row.extend_from_slice(&xs[col * w..(col + 1) * w]);
        } else {
            let u = unit(&xs[col * w + col], m);
            let (above, pivot_and_below) = xs.split_at_mut(col * w);
            let pivot_row = &mut pivot_and_below[..w];
            for x in &mut pivot_row[col..] {
                *x = (&u).mod_mul(&*x, m);
            }
            let d = &pivot_row[col];
            let data = Natural::precompute_div_mod_data(d);
            for (row, above_row) in above.chunks_exact_mut(w).enumerate() {
                let q = (&above_row[col]).div_mod_precomputed(d, &data).0;
                for (x, y) in above_row[row..].iter_mut().zip(&pivot_row[row..]) {
                    x.mod_sub_assign((&q).mod_mul(y, m), m);
                }
            }
            let g = m.gcd(d);
            if g == 1u32 {
                continue;
            }
            let g = m.div_exact(&g);
            extra_row.clear();
            extra_row.extend(pivot_row.iter().map(|x| (&g).mod_mul(x, m)));
        }
        for row in col + 1..w {
            let this_row = &mut xs[row * w..(row + 1) * w];
            let (g, s, t, u, v) = xgcd_transform(&this_row[row], &extra_row[row], m);
            if g == 0u32 {
                continue;
            }
            for (x, y) in this_row[row..].iter_mut().zip(&mut extra_row[row..]) {
                (*x, *y) = transform_pair(x, y, (&s, &t, &u, &v), m);
            }
        }
    }
}

fn assert_arguments(a: &NaturalMatrix, m: &Natural) {
    assert_ne!(*m, 0u32, "m cannot be zero");
    assert!(
        a.mod_is_reduced(m),
        "the matrix must be reduced mod m, but {a} has an element >= {m}"
    );
}

// Appends zero rows to `a` so that it has at least as many rows as columns. Zero rows do not change
// the row span.
pub(crate) fn pad(a: &mut NaturalMatrix) {
    if a.height < a.width {
        a.elements.resize(a.width * a.width, Natural::ZERO);
        a.height = a.width;
    }
}

// The elements of a matrix whose elements all fit in a limb, as an `UnsignedMatrix` of limbs.
pub(crate) fn to_limb_matrix(a: &NaturalMatrix) -> UnsignedMatrix<Limb> {
    UnsignedMatrix::from_owned_elements(
        a.height(),
        a.width(),
        a.elements.iter().map(Limb::wrapping_from).collect(),
    )
}

fn mod_strong_echelon_form_assign_helper(a: &mut NaturalMatrix, m: &Natural) {
    assert_arguments(a, m);
    if let Natural(Small(small_m)) = *m {
        *a = NaturalMatrix::from(to_limb_matrix(a).mod_strong_echelon_form(small_m));
    } else {
        pad(a);
        strong_echelon_form_in_place(&mut a.elements, a.height, a.width, m);
    }
}

impl ModStrongEchelonForm<Natural> for NaturalMatrix {
    type Output = Self;

    /// Computes the strong echelon form of a [`NaturalMatrix`] modulo a [`Natural`] $m$, taking the
    /// matrix by value and the modulus by value.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The strong echelon form of an
    /// $h \times w$ matrix $A$ modulo $m$ is the $\max(h, w) \times w$ matrix with the same nonzero
    /// rows as the Howell form of $A$ (see
    /// [`ModHowellForm`](malachite_base::num::arithmetic::traits::ModHowellForm)), but with each
    /// nonzero row in the row whose index is the column of its pivot (its first nonzero element),
    /// so that the pivots are on the diagonal, and with zero rows everywhere else. Like the Howell
    /// form, it is unique.
    ///
    /// A matrix with fewer rows than columns is first given zero rows until it is square, which
    /// leaves its row span unchanged. The strong echelon form is computed with the algorithm of
    /// Fieker and Hofmann, which reduces the matrix to upper triangular form with unimodular row
    /// operations, makes each diagonal element a divisor of $m$, reduces the elements above it, and
    /// then, for each diagonal element $d$ with $d \ne 1$, combines the multiple $(m/d)$ of its
    /// row, which may have a nonzero element further right, into the rows below.
    ///
    /// When $m$ fits in a limb, the form is computed as for [`UnsignedMatrix`]es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(h, w, n) = O(hw^2 n \log n \log\log n)$
    ///
    /// $M(h, w, n) = O(hwn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the larger of the matrix's height and
    /// width, $w$ is its width, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of the matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModStrongEchelonForm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[4, 2; 2, 0]").unwrap();
    /// assert_eq!(
    ///     a.mod_strong_echelon_form(Natural::from(8u32)).to_string(),
    ///     "[2, 0; 0, 2]"
    /// );
    ///
    /// // A matrix with fewer rows than columns may gain nonzero rows: here 2 * [2, 1] = [0, 2].
    /// let a = NaturalMatrix::from_str("[2, 1]").unwrap();
    /// assert_eq!(
    ///     a.mod_strong_echelon_form(Natural::from(4u32)).to_string(),
    ///     "[2, 1; 0, 2]"
    /// );
    ///
    /// // Each nonzero row is in the row whose index is the column of its pivot.
    /// let a = NaturalMatrix::from_str("[0, 2; 0, 0; 0, 0]").unwrap();
    /// assert_eq!(
    ///     a.mod_strong_echelon_form(Natural::from(4u32)).to_string(),
    ///     "[0, 0; 0, 2; 0, 0]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[3, 6; 6, 3]").unwrap();
    /// assert_eq!(
    ///     a.mod_strong_echelon_form(Natural::from(9u32)).to_string(),
    ///     "[3, 6; 0, 0]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[100000000000000000005, 2; 10000000000000000000, 25]")
    ///     .unwrap();
    /// let m = Natural::from_str("1000000000000000000000").unwrap();
    /// assert_eq!(a.mod_strong_echelon_form(m).to_string(), "[5, 2; 0, 25]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_strong_echelon_form_mod` from
    /// `fmpz_mat/strong_echelon_form_mod.c`, FLINT 3.6.0, except that a matrix with fewer rows than
    /// columns is given zero rows, which FLINT does not allow, and that it panics if the matrix is
    /// not reduced.
    #[inline]
    fn mod_strong_echelon_form(mut self, m: Natural) -> Self {
        mod_strong_echelon_form_assign_helper(&mut self, &m);
        self
    }
}

impl ModStrongEchelonForm<&Natural> for NaturalMatrix {
    type Output = Self;

    /// Computes the strong echelon form of a [`NaturalMatrix`] modulo a [`Natural`] $m$, taking the
    /// matrix by value and the modulus by reference.
    ///
    /// See the documentation for the implementation that takes both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(h, w, n) = O(hw^2 n \log n \log\log n)$
    ///
    /// $M(h, w, n) = O(hwn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the larger of the matrix's height and
    /// width, $w$ is its width, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of the matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModStrongEchelonForm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[4, 2; 2, 0]").unwrap();
    /// assert_eq!(
    ///     a.mod_strong_echelon_form(&Natural::from(8u32)).to_string(),
    ///     "[2, 0; 0, 2]"
    /// );
    ///
    /// // A matrix with fewer rows than columns may gain nonzero rows: here 2 * [2, 1] = [0, 2].
    /// let a = NaturalMatrix::from_str("[2, 1]").unwrap();
    /// assert_eq!(
    ///     a.mod_strong_echelon_form(&Natural::from(4u32)).to_string(),
    ///     "[2, 1; 0, 2]"
    /// );
    ///
    /// // Each nonzero row is in the row whose index is the column of its pivot.
    /// let a = NaturalMatrix::from_str("[0, 2; 0, 0; 0, 0]").unwrap();
    /// assert_eq!(
    ///     a.mod_strong_echelon_form(&Natural::from(4u32)).to_string(),
    ///     "[0, 0; 0, 2; 0, 0]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[3, 6; 6, 3]").unwrap();
    /// assert_eq!(
    ///     a.mod_strong_echelon_form(&Natural::from(9u32)).to_string(),
    ///     "[3, 6; 0, 0]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[100000000000000000005, 2; 10000000000000000000, 25]")
    ///     .unwrap();
    /// let m = Natural::from_str("1000000000000000000000").unwrap();
    /// assert_eq!(a.mod_strong_echelon_form(&m).to_string(), "[5, 2; 0, 25]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_strong_echelon_form_mod` from
    /// `fmpz_mat/strong_echelon_form_mod.c`, FLINT 3.6.0, except that a matrix with fewer rows than
    /// columns is given zero rows, which FLINT does not allow, and that it panics if the matrix is
    /// not reduced.
    #[inline]
    fn mod_strong_echelon_form(mut self, m: &Natural) -> Self {
        mod_strong_echelon_form_assign_helper(&mut self, m);
        self
    }
}

impl ModStrongEchelonForm<Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Computes the strong echelon form of a [`NaturalMatrix`] modulo a [`Natural`] $m$, taking the
    /// matrix by reference and the modulus by value.
    ///
    /// See the documentation for the implementation that takes both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(h, w, n) = O(hw^2 n \log n \log\log n)$
    ///
    /// $M(h, w, n) = O(hwn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the larger of the matrix's height and
    /// width, $w$ is its width, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of the matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModStrongEchelonForm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[4, 2; 2, 0]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_strong_echelon_form(Natural::from(8u32))
    ///         .to_string(),
    ///     "[2, 0; 0, 2]"
    /// );
    ///
    /// // A matrix with fewer rows than columns may gain nonzero rows: here 2 * [2, 1] = [0, 2].
    /// let a = NaturalMatrix::from_str("[2, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_strong_echelon_form(Natural::from(4u32))
    ///         .to_string(),
    ///     "[2, 1; 0, 2]"
    /// );
    ///
    /// // Each nonzero row is in the row whose index is the column of its pivot.
    /// let a = NaturalMatrix::from_str("[0, 2; 0, 0; 0, 0]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_strong_echelon_form(Natural::from(4u32))
    ///         .to_string(),
    ///     "[0, 0; 0, 2; 0, 0]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[3, 6; 6, 3]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_strong_echelon_form(Natural::from(9u32))
    ///         .to_string(),
    ///     "[3, 6; 0, 0]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[100000000000000000005, 2; 10000000000000000000, 25]")
    ///     .unwrap();
    /// let m = Natural::from_str("1000000000000000000000").unwrap();
    /// assert_eq!((&a).mod_strong_echelon_form(m).to_string(), "[5, 2; 0, 25]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_strong_echelon_form_mod` from
    /// `fmpz_mat/strong_echelon_form_mod.c`, FLINT 3.6.0, except that a matrix with fewer rows than
    /// columns is given zero rows, which FLINT does not allow, and that it panics if the matrix is
    /// not reduced.
    #[inline]
    fn mod_strong_echelon_form(self, m: Natural) -> NaturalMatrix {
        self.clone().mod_strong_echelon_form(m)
    }
}

impl ModStrongEchelonForm<&Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Computes the strong echelon form of a [`NaturalMatrix`] modulo a [`Natural`] $m$, taking the
    /// matrix by reference and the modulus by reference.
    ///
    /// See the documentation for the implementation that takes both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(h, w, n) = O(hw^2 n \log n \log\log n)$
    ///
    /// $M(h, w, n) = O(hwn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the larger of the matrix's height and
    /// width, $w$ is its width, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of the matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModStrongEchelonForm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[4, 2; 2, 0]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_strong_echelon_form(&Natural::from(8u32))
    ///         .to_string(),
    ///     "[2, 0; 0, 2]"
    /// );
    ///
    /// // A matrix with fewer rows than columns may gain nonzero rows: here 2 * [2, 1] = [0, 2].
    /// let a = NaturalMatrix::from_str("[2, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_strong_echelon_form(&Natural::from(4u32))
    ///         .to_string(),
    ///     "[2, 1; 0, 2]"
    /// );
    ///
    /// // Each nonzero row is in the row whose index is the column of its pivot.
    /// let a = NaturalMatrix::from_str("[0, 2; 0, 0; 0, 0]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_strong_echelon_form(&Natural::from(4u32))
    ///         .to_string(),
    ///     "[0, 0; 0, 2; 0, 0]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[3, 6; 6, 3]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_strong_echelon_form(&Natural::from(9u32))
    ///         .to_string(),
    ///     "[3, 6; 0, 0]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[100000000000000000005, 2; 10000000000000000000, 25]")
    ///     .unwrap();
    /// let m = Natural::from_str("1000000000000000000000").unwrap();
    /// assert_eq!(
    ///     (&a).mod_strong_echelon_form(&m).to_string(),
    ///     "[5, 2; 0, 25]"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_strong_echelon_form_mod` from
    /// `fmpz_mat/strong_echelon_form_mod.c`, FLINT 3.6.0, except that a matrix with fewer rows than
    /// columns is given zero rows, which FLINT does not allow, and that it panics if the matrix is
    /// not reduced.
    #[inline]
    fn mod_strong_echelon_form(self, m: &Natural) -> NaturalMatrix {
        self.clone().mod_strong_echelon_form(m)
    }
}

impl ModStrongEchelonFormAssign<Natural> for NaturalMatrix {
    /// Replaces a [`NaturalMatrix`] with its strong echelon form modulo a [`Natural`] $m$, taking
    /// the modulus by value.
    ///
    /// See the documentation for the [`ModStrongEchelonForm`] implementation that takes both
    /// arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(h, w, n) = O(hw^2 n \log n \log\log n)$
    ///
    /// $M(h, w, n) = O(hwn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the larger of the matrix's height and
    /// width, $w$ is its width, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of the matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModStrongEchelonFormAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[4, 2; 2, 0]").unwrap();
    /// a.mod_strong_echelon_form_assign(Natural::from(8u32));
    /// assert_eq!(a.to_string(), "[2, 0; 0, 2]");
    ///
    /// let mut a = NaturalMatrix::from_str("[2, 1]").unwrap();
    /// a.mod_strong_echelon_form_assign(Natural::from(4u32));
    /// assert_eq!(a.to_string(), "[2, 1; 0, 2]");
    ///
    /// let mut a = NaturalMatrix::from_str("[100000000000000000005, 2; 10000000000000000000, 25]")
    ///     .unwrap();
    /// let m = Natural::from_str("1000000000000000000000").unwrap();
    /// a.mod_strong_echelon_form_assign(m);
    /// assert_eq!(a.to_string(), "[5, 2; 0, 25]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_strong_echelon_form_mod` from
    /// `fmpz_mat/strong_echelon_form_mod.c`, FLINT 3.6.0, except that a matrix with fewer rows than
    /// columns is given zero rows, which FLINT does not allow, and that it panics if the matrix is
    /// not reduced.
    #[inline]
    fn mod_strong_echelon_form_assign(&mut self, m: Natural) {
        mod_strong_echelon_form_assign_helper(self, &m);
    }
}

impl ModStrongEchelonFormAssign<&Natural> for NaturalMatrix {
    /// Replaces a [`NaturalMatrix`] with its strong echelon form modulo a [`Natural`] $m$, taking
    /// the modulus by reference.
    ///
    /// See the documentation for the [`ModStrongEchelonForm`] implementation that takes both
    /// arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(h, w, n) = O(hw^2 n \log n \log\log n)$
    ///
    /// $M(h, w, n) = O(hwn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the larger of the matrix's height and
    /// width, $w$ is its width, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of the matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModStrongEchelonFormAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[4, 2; 2, 0]").unwrap();
    /// a.mod_strong_echelon_form_assign(&Natural::from(8u32));
    /// assert_eq!(a.to_string(), "[2, 0; 0, 2]");
    ///
    /// let mut a = NaturalMatrix::from_str("[2, 1]").unwrap();
    /// a.mod_strong_echelon_form_assign(&Natural::from(4u32));
    /// assert_eq!(a.to_string(), "[2, 1; 0, 2]");
    ///
    /// let mut a = NaturalMatrix::from_str("[100000000000000000005, 2; 10000000000000000000, 25]")
    ///     .unwrap();
    /// let m = Natural::from_str("1000000000000000000000").unwrap();
    /// a.mod_strong_echelon_form_assign(&m);
    /// assert_eq!(a.to_string(), "[5, 2; 0, 25]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_strong_echelon_form_mod` from
    /// `fmpz_mat/strong_echelon_form_mod.c`, FLINT 3.6.0, except that a matrix with fewer rows than
    /// columns is given zero rows, which FLINT does not allow, and that it panics if the matrix is
    /// not reduced.
    #[inline]
    fn mod_strong_echelon_form_assign(&mut self, m: &Natural) {
        mod_strong_echelon_form_assign_helper(self, m);
    }
}
