// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Sebastian Pancratz
//
//      Copyright © 2012 William Hart
//
//      Copyright © 2014 Alex J. Best
//
//      Copyright © 2015, 2017 Tommy Hofmann
//
//      Copyright © 2021 Albin Ahlbäck
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::integer_matrix::arithmetic::hermite_normal_form_pernet_stein::hnf_pernet_stein;
use crate::integer_vector::arithmetic::max_bits::vec_max_bits;
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::natural_matrix::arithmetic::mod_strong_echelon_form::swap_rows;
use core::cmp::Ordering::*;
use malachite_base::num::arithmetic::traits::{
    AddMul, DivExact, DivExactAssign, DivRound, DivisibleBy, ExtendedGcd, HermiteNormalForm,
    HermiteNormalFormAssign, Mod, ModAssign, ModStrongEchelonForm, NegAssign, SaturatingSubAssign,
    Sign, SubMulAssign,
};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::rounding_modes::RoundingMode::*;

// Returns the GCD d of x and y, where x < y, and a cofactor a in [0, y) with ax ≡ d (mod y). When
// x is 0, the cofactor is 0.
//
// This is equivalent to `fmpz_gcdinv` from `fmpz/gcdinv.c`, FLINT 3.6.0.
fn gcd_and_cofactor(x: &Natural, y: &Natural) -> (Natural, Natural) {
    if *x == 0u32 {
        return (y.clone(), Natural::ZERO);
    }
    let (d, a, _) = x.extended_gcd(y);
    let a = if a < 0u32 { a + Integer::from(y) } else { a };
    (d, Natural::exact_from(a))
}

// Returns the GCD d of x and y, where x < y, the cofactor a in [0, y) of `gcd_and_cofactor`, and
// the cofactor b = (d - ax)/y.
fn word_gcd_and_cofactors(x: u64, y: u64) -> (u64, u64, Integer) {
    if x == 0 {
        return (y, 0, Integer::ONE);
    }
    let (d, a, _) = x.extended_gcd(y);
    let a = if a < 0 {
        y - a.unsigned_abs()
    } else {
        a.unsigned_abs()
    };
    // ax ≡ d (mod y) and a ≥ 1, so ax ≥ x ≥ d and b = -(ax - d)/y.
    let b = (u128::from(a) * u128::from(x) - u128::from(d)) / u128::from(y);
    (d, a, -Integer::from(u64::wrapping_from(b)))
}

// Returns the GCD d of f and g, as an `Integer`, and cofactors a and b with af + bg = d. When |f|
// ≠ |g| and neither is 0, the cofactor of the one with the smaller absolute value has its
// absolute value reduced modulo the larger absolute value, and the other cofactor is determined by
// it.
//
// This is equivalent to `fmpz_xgcd` from `fmpz/xgcd.c`, FLINT 3.6.0.
crate_test_fn! {
extended_gcd_reduced_cofactor(f: &Integer, g: &Integer) -> (Integer, Integer, Integer) {
    if *f == 0u32 {
        let b = match g.sign() {
            Greater => Integer::ONE,
            Less => Integer::NEGATIVE_ONE,
            Equal => Integer::ZERO,
        };
        return (Integer::from(g.unsigned_abs_ref()), Integer::ZERO, b);
    }
    let f1 = f.unsigned_abs_ref();
    let g1 = g.unsigned_abs_ref();
    if f1 == g1 {
        return if *f > 0u32 {
            (f.clone(), Integer::ONE, Integer::ZERO)
        } else {
            (-f, Integer::NEGATIVE_ONE, Integer::ZERO)
        };
    }
    let (d, mut a, mut b) = if let (Ok(f1), Ok(g1)) = (u64::try_from(f1), u64::try_from(g1)) {
        // Both fit in a word.
        let (d, a, b) = if f1 < g1 {
            let (d, a, b) = word_gcd_and_cofactors(f1, g1);
            (d, Integer::from(a), b)
        } else {
            let (d, b, a) = word_gcd_and_cofactors(g1, f1);
            (d, a, Integer::from(b))
        };
        (Natural::from(d), a, b)
    } else if f1 < g1 {
        let (d, a) = gcd_and_cofactor(f1, g1);
        let b = (Integer::from(&d) - Integer::from(&a * f1)).div_exact(Integer::from(g1));
        (d, Integer::from(a), b)
    } else {
        let (d, b) = gcd_and_cofactor(g1, f1);
        let a = (Integer::from(&d) - Integer::from(&b * g1)).div_exact(Integer::from(f1));
        (d, a, Integer::from(b))
    };
    if *f < 0u32 {
        a.neg_assign();
    }
    if *g < 0u32 {
        b.neg_assign();
    }
    (Integer::from(d), a, b)
}}

// Mutable references to elements `i` and `j`, which are distinct, of `xs`.
pub(crate) const fn two_elements(
    xs: &mut [Integer],
    i: usize,
    j: usize,
) -> (&mut Integer, &mut Integer) {
    if i < j {
        let (left, right) = xs.split_at_mut(j);
        (&mut left[i], &mut right[0])
    } else {
        let (left, right) = xs.split_at_mut(i);
        (&mut right[0], &mut left[j])
    }
}

// Mutable references to rows `i` and `j`, which are distinct, of the `width`-wide matrix with
// elements `xs`.
pub(crate) fn two_rows(
    xs: &mut [Integer],
    width: usize,
    i: usize,
    j: usize,
) -> (&mut [Integer], &mut [Integer]) {
    if i < j {
        let (top, bottom) = xs.split_at_mut(j * width);
        (&mut top[i * width..(i + 1) * width], &mut bottom[..width])
    } else {
        let (top, bottom) = xs.split_at_mut(i * width);
        (&mut bottom[..width], &mut top[j * width..(j + 1) * width])
    }
}

// The unimodular transformation of two rows x and y that replaces their leading elements x₀ and
// y₀ with gcd(x₀, y₀) and 0: x becomes ux + vy and y becomes (x₀/d)y - (y₀/d)x, where d =
// gcd(x₀, y₀) = ux₀ + vy₀.
pub(crate) struct RowCombination {
    u: Integer,
    v: Integer,
    x_cofactor: Integer,
    y_cofactor: Integer,
}

impl RowCombination {
    // The combination for rows with leading elements `x0` and `y0`, not both zero, with the
    // cofactors of `extended_gcd_reduced_cofactor`.
    pub(crate) fn new(x0: &Integer, y0: &Integer) -> Self {
        let (gcd, u, v) = extended_gcd_reduced_cofactor(x0, y0);
        let x_cofactor = x0.div_exact(&gcd);
        let y_cofactor = y0.div_exact(&gcd);
        Self {
            u,
            v,
            x_cofactor,
            y_cofactor,
        }
    }

    // Applies the combination to one element `x` of the first row and the corresponding element `y`
    // of the second.
    pub(crate) fn apply_to_pair(&self, x: &mut Integer, y: &mut Integer) {
        let b = (&self.u * &*x).add_mul(&self.v, &*y);
        *y *= &self.x_cofactor;
        y.sub_mul_assign(&self.y_cofactor, &*x);
        *x = b;
    }

    // Applies the combination to the rows `xs` and `ys`.
    pub(crate) fn apply(&self, xs: &mut [Integer], ys: &mut [Integer]) {
        for (x, y) in xs.iter_mut().zip(ys.iter_mut()) {
            self.apply_to_pair(x, y);
        }
    }

    // Applies the combination to columns `x_col` and `y_col`, which are distinct, of the rows from
    // `start_row` on of the `width`-wide matrix with elements `xs`.
    pub(crate) fn apply_to_columns(
        &self,
        xs: &mut [Integer],
        width: usize,
        start_row: usize,
        x_col: usize,
        y_col: usize,
    ) {
        for row in xs.chunks_exact_mut(width).skip(start_row) {
            let (x, y) = two_elements(row, x_col, y_col);
            self.apply_to_pair(x, y);
        }
    }
}

// Combines rows `i` and `k` of the `width`-wide matrix with elements `xs`, from column `j` on, so
// that the element of row `k` in column `j` becomes 0 and that of row `i` the GCD of the two.
// Returns the combination, so that it can be applied to other rows.
pub(crate) fn combine_rows(
    xs: &mut [Integer],
    width: usize,
    i: usize,
    k: usize,
    j: usize,
) -> RowCombination {
    let (x, y) = two_rows(xs, width, i, k);
    let c = RowCombination::new(&x[j], &y[j]);
    c.apply(&mut x[j..], &mut y[j..]);
    c
}

// Replaces x with x - qy.
pub(crate) fn sub_mul_row(xs: &mut [Integer], q: &Integer, ys: &[Integer]) {
    for (x, y) in xs.iter_mut().zip(ys) {
        x.sub_mul_assign(q, y);
    }
}

pub(crate) fn neg_row(xs: &mut [Integer]) {
    for x in xs {
        x.neg_assign();
    }
}

// Subtracts from row `i` of the `width`-wide matrix with elements `xs` the multiple of row `k`,
// from column `j` on, that brings its element in column `j` into [0, d), where d is the element of
// row `k` in column `j`, which is positive. Returns the multiplier, if it is nonzero.
pub(crate) fn reduce_row(
    xs: &mut [Integer],
    width: usize,
    i: usize,
    k: usize,
    j: usize,
) -> Option<Integer> {
    let (x, y) = two_rows(xs, width, i, k);
    let q = (&x[j]).div_round(&y[j], Floor).0;
    if q == 0u32 {
        None
    } else {
        sub_mul_row(&mut x[j..], &q, &y[j..]);
        Some(q)
    }
}

// The Hermite normal form of `a`, by the classical algorithm, which, column by column, repeatedly
// reduces the elements below the pivot by the one with the smallest absolute value until they are
// zero, and then reduces the elements above the pivot.
//
// This is equivalent to `fmpz_mat_hnf_classical` from `fmpz_mat/hnf_classical.c`, FLINT 3.6.0.
crate_test_fn! {
hnf_classical(a: &IntegerMatrix) -> IntegerMatrix {
    let (m, n) = (a.height, a.width);
    let mut h = a.clone();
    let xs = &mut h.elements;
    let mut j = 0;
    let mut k = 0;
    let mut l = n.saturating_sub(m);
    while n - j != l {
        if (k + 1..m).all(|i| xs[i * n + j] == 0u32) {
            if xs[k * n + j] < 0u32 {
                neg_row(&mut xs[k * n + j..(k + 1) * n]);
            }
            if xs[k * n + j] == 0u32 {
                l.saturating_sub_assign(1);
                j += 1;
                continue;
            }
            // Reduce the elements of column j above row k with row k.
            for i in 0..k {
                reduce_row(xs, n, i, k, j);
            }
        } else {
            // Find the nonzero element of column j below row k with the smallest absolute value.
            let mut i0 = k;
            for i in k + 1..m {
                let x = &xs[i * n + j];
                if *x != 0u32
                    && (i0 == k || x.unsigned_abs_ref() < xs[i0 * n + j].unsigned_abs_ref())
                {
                    i0 = i;
                }
            }
            swap_rows(xs, n, i0, k);
            if xs[k * n + j] < 0u32 {
                neg_row(&mut xs[k * n + j..(k + 1) * n]);
            }
            // Reduce the elements of column j below row k with row k, and stay in this column.
            for i in k + 1..m {
                reduce_row(xs, n, i, k, j);
            }
            continue;
        }
        j += 1;
        k += 1;
    }
    h
}}

// The Hermite normal form of `a`, by an algorithm which, column by column, combines consecutive
// rows with extended GCDs until only the last has a nonzero element there, moves that row up, and
// then reduces the elements above the pivot.
//
// This is equivalent to `fmpz_mat_hnf_xgcd` from `fmpz_mat/hnf_xgcd.c`, FLINT 3.6.0.
crate_test_fn! {
hnf_xgcd(a: &IntegerMatrix) -> IntegerMatrix {
    let (m, n) = (a.height, a.width);
    let mut h = a.clone();
    let xs = &mut h.elements;
    let mut j = 0;
    let mut k = 0;
    let mut l = n.saturating_sub(m);
    while n - j != l {
        for i in k + 1..m {
            // Reduce row i - 1 with row i.
            if xs[(i - 1) * n + j] == 0u32 {
                continue;
            }
            combine_rows(xs, n, i, i - 1, j);
        }
        swap_rows(xs, n, m - 1, k);
        if xs[k * n + j] < 0u32 {
            neg_row(&mut xs[k * n + j..(k + 1) * n]);
        }
        if xs[k * n + j] == 0u32 {
            l.saturating_sub_assign(1);
            j += 1;
            continue;
        }
        // Reduce the elements of column j above row k with row k.
        for i in (0..k).rev() {
            reduce_row(xs, n, i, k, j);
        }
        j += 1;
        k += 1;
    }
    h
}}

// Combines rows `j` and `k` of the `width`-wide matrix with elements `xs` so that the element of
// row `k` in column `j` becomes 0, given that the element of row `j` there is positive, applying
// the same transformation to the rows of `us`, which is `us_width` wide, if it is present.
fn minors_eliminate(
    xs: &mut [Integer],
    width: usize,
    us: Option<&mut [Integer]>,
    us_width: usize,
    j: usize,
    k: usize,
) {
    let (x, y) = two_rows(xs, width, j, k);
    // When the element of row j divides that of row k, a multiple of row j is subtracted instead.
    if (&y[j]).divisible_by(&x[j]) {
        let b = (&y[j]).div_exact(&x[j]);
        sub_mul_row(&mut y[j..], &b, &x[j..]);
        if let Some(us) = us {
            let (ux, uy) = two_rows(us, us_width, j, k);
            sub_mul_row(uy, &b, ux);
        }
        return;
    }
    let c = RowCombination::new(&x[j], &y[j]);
    c.apply(&mut x[j..], &mut y[j..]);
    if let Some(us) = us {
        let (ux, uy) = two_rows(us, us_width, j, k);
        c.apply(ux, uy);
    }
}

// Subtracts from row `i` of the `width`-wide matrix with elements `xs` the multiple of row `j` that
// brings its element in column `j` into [0, d), where d is the element of row `j` there, which is
// positive, applying the same transformation to the rows of `us`, which is `us_width` wide, if it
// is present.
fn minors_reduce(
    xs: &mut [Integer],
    width: usize,
    us: Option<&mut [Integer]>,
    us_width: usize,
    i: usize,
    j: usize,
) {
    if let Some(q) = reduce_row(xs, width, i, j, j)
        && let Some(us) = us
    {
        let (ux, uy) = two_rows(us, us_width, i, j);
        sub_mul_row(ux, &q, uy);
    }
}

// Replaces `h`, which is m by n with rank n, with its Hermite normal form, by the algorithm of
// Kannan and Bachem, which puts the leading principal minors into Hermite normal form in turn. The
// row operations are also applied to `u`, which is m by m, if it is present.
//
// This is equivalent to `fmpz_mat_hnf_minors` from `fmpz_mat/hnf_minors.c`, FLINT 3.6.0, and, with
// `u` present, to `fmpz_mat_hnf_minors_transform` from `fmpz_mat/hnf_minors_transform.c`, FLINT
// 3.6.0.
pub(crate) fn hnf_minors_in_place(h: &mut IntegerMatrix, u: Option<&mut IntegerMatrix>) {
    let (m, n) = (h.height, h.width);
    let xs = &mut h.elements;
    let mut us = u.map(|u| u.elements.as_mut_slice());
    // Put the kth leading principal minor into Hermite normal form.
    let mut k = 0;
    let mut l = m.saturating_sub(1);
    while k < n {
        for j in 0..k {
            if xs[k * n + j] != 0u32 {
                minors_eliminate(xs, n, us.as_deref_mut(), m, j, k);
            }
        }
        // If the diagonal element of row k is zero, swap row k with another row, starting with the
        // last.
        if xs[k * n + k] == 0u32 {
            swap_rows(xs, n, k, l);
            if let Some(us) = us.as_deref_mut() {
                swap_rows(us, m, k, l);
            }
            l -= 1;
            continue;
        }
        if xs[k * n + k] < 0u32 {
            neg_row(&mut xs[k * n + k..(k + 1) * n]);
            if let Some(us) = us.as_deref_mut() {
                neg_row(&mut us[k * m..(k + 1) * m]);
            }
        }
        // Reduce the elements above the diagonal in each row.
        for i in (0..k).rev() {
            for j in i + 1..=k {
                minors_reduce(xs, n, us.as_deref_mut(), m, i, j);
            }
        }
        l = m - 1;
        k += 1;
    }
    // Reduce the remaining rows.
    for k in n..m {
        for j in 0..n {
            minors_eliminate(xs, n, us.as_deref_mut(), m, j, k);
        }
        for i in (0..n).rev() {
            for j in i + 1..n {
                minors_reduce(xs, n, us.as_deref_mut(), m, i, j);
            }
        }
    }
}

// The Hermite normal form of `a`, which is m by n with rank n, by the algorithm of Kannan and
// Bachem.
//
// This is equivalent to `fmpz_mat_hnf_minors` from `fmpz_mat/hnf_minors.c`, FLINT 3.6.0.
crate_test_fn! {
hnf_minors(a: &IntegerMatrix) -> IntegerMatrix {
    let mut h = a.clone();
    hnf_minors_in_place(&mut h, None);
    h
}}

// The Hermite normal form of `a`, which is m by n with rank n, given `d`, a positive multiple of
// the determinant of the lattice spanned by its rows, by the algorithm of Domich, Kannan, and
// Trotter, which works modulo d and divides it by each pivot found.
//
// This is equivalent to `fmpz_mat_hnf_modular` from `fmpz_mat/hnf_modular.c`, FLINT 3.6.0.
crate_test_fn! {
hnf_modular(a: &IntegerMatrix, d: &Natural) -> IntegerMatrix {
    let (m, n) = (a.height, a.width);
    let mut h = a.clone();
    let xs = &mut h.elements;
    let mut r = Integer::from(d);
    for k in 0..n {
        let half_r = &r >> 1u32;
        if xs[k * n + k] == 0u32 {
            xs[k * n + k] = r.clone();
        }
        for i in k + 1..m {
            // Reduce row i with row k, modulo R.
            if xs[i * n + k] == 0u32 {
                continue;
            }
            let (x, y) = two_rows(xs, n, k, i);
            RowCombination::new(&x[k], &y[k]).apply(&mut x[k..], &mut y[k..]);
            // Reduce both rows to (-R/2, R/2].
            for z in x[k..].iter_mut().chain(y[k..].iter_mut()) {
                z.mod_assign(&r);
                if *z > half_r {
                    *z -= &r;
                }
            }
        }
        let (g, u, _) = extended_gcd_reduced_cofactor(&xs[k * n + k], &r);
        for x in &mut xs[k * n + k..(k + 1) * n] {
            *x *= &u;
            x.mod_assign(&r);
        }
        if xs[k * n + k] == 0u32 {
            xs[k * n + k] = r.clone();
        }
        // Reduce the elements of column k above row k with row k.
        for i in (0..k).rev() {
            reduce_row(xs, n, i, k, k);
        }
        r.div_exact_assign(g);
    }
    h
}}

// Replaces `a`, which is m by n with rank n, with its Hermite normal form, given `d`, a positive
// multiple of the largest elementary divisor of `a`, by the algorithm of Fieker and Hofmann: the
// strong echelon form of `a` modulo `d`, with each zero diagonal element replaced by `d`.
//
// This is equivalent to `fmpz_mat_hnf_modular_eldiv` from `fmpz_mat/hnf_modular_eldiv.c`, FLINT
// 3.6.0.
crate_test_fn! {
hnf_modular_eldiv(a: &mut IntegerMatrix, d: &Natural) {
    if a.height == 0 || a.width == 0 {
        return;
    }
    assert!(a.height >= a.width);
    let e: NaturalMatrix = (&*a).mod_op(d).mod_strong_echelon_form(d);
    *a = IntegerMatrix::from(e);
    let n = a.width;
    for i in 0..n {
        let x = &mut a.elements[i * n + i];
        if *x == 0u32 {
            *x = Integer::from(d);
        }
    }
}}

// The Hermite normal form of `a`: by the classical algorithm when `a` has few rows for the size of
// its elements, and by the algorithm of Pernet and Stein otherwise.
//
// This is equivalent to `fmpz_mat_hnf` from `fmpz_mat/hnf.c`, FLINT 3.6.0, except that the cutoffs
// were tuned for Malachite.
fn hnf_helper(a: &IntegerMatrix) -> IntegerMatrix {
    let b = vec_max_bits(&a.elements).0;
    let cutoff = match b {
        0..=2 => 60,
        3..=4 => 16,
        5..=8 => 12,
        9..=16 => 8,
        17..=32 => 5,
        33..=64 => 4,
        65..=128 => 3,
        _ => 2,
    };
    if a.height < cutoff {
        hnf_classical(a)
    } else {
        hnf_pernet_stein(a)
    }
}

impl HermiteNormalForm for IntegerMatrix {
    type Output = Self;

    /// Computes the Hermite normal form of an [`IntegerMatrix`], taking it by value. The Hermite
    /// normal form $H$ of an $h \times w$ matrix $A$ is the unique $h \times w$ matrix in row
    /// echelon form whose rows span the same lattice as those of $A$ (so that $H = UA$ for some
    /// unimodular $U$), whose pivots (the first nonzero elements of its nonzero rows) are positive,
    /// and whose elements above each pivot lie in $[0, p)$, where $p$ is the pivot. Its nonzero
    /// rows come first, and their number is the rank of $A$.
    ///
    /// Matrices with few rows for the size of their elements are reduced by the classical
    /// algorithm, which, column by column, reduces the elements below the pivot by the one of
    /// smallest absolute value until they are zero; the others by the algorithm of Pernet and
    /// Stein, which, modulo a word-sized prime, finds the pivot columns and a nonsingular square
    /// submatrix; finds that submatrix's Hermite normal form modulo its determinant; extends it to
    /// all the columns; and adds the remaining rows. Matrices with full column rank and few columns
    /// are instead reduced by the algorithm of Kannan and Bachem, which puts the leading principal
    /// minors into Hermite normal form in turn.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^4 (b + \log n)^2 + n^5 (b + \log n) \log (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^3 (b + \log n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum of the height and width of
    /// the matrix, and $b$ is the maximum number of significant bits of its elements: the
    /// determinants that bound the elements have $O(n(b + \log n))$ bits, adding each of $O(n)$
    /// rows takes $O(n^3)$ operations on such numbers, and the nonsingular systems are solved
    /// within the bounds of [`solve`](malachite_base::num::arithmetic::traits::Solve::solve).
    /// Matrices with fewer rows than a cutoff of at most 59 are reduced by the classical algorithm,
    /// whose intermediate elements can double in size with each pivot found; the bound holds for
    /// them only because their height is bounded by a constant.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::HermiteNormalForm;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.hermite_normal_form().to_string(), "[1, 0; 0, 2]");
    ///
    /// let a = IntegerMatrix::from_str("[6, 4; 4, 6; 9, 3]").unwrap();
    /// assert_eq!(a.hermite_normal_form().to_string(), "[1, 1; 0, 2; 0, 0]");
    ///
    /// // A matrix of rank 1
    /// let a = IntegerMatrix::from_str("[2, 1; 4, 2; 6, 3]").unwrap();
    /// assert_eq!(a.hermite_normal_form().to_string(), "[2, 1; 0, 0; 0, 0]");
    ///
    /// let a = IntegerMatrix::from_str("[0, 2, 1; 0, 4, 3]").unwrap();
    /// assert_eq!(a.hermite_normal_form().to_string(), "[0, 2, 0; 0, 0, 1]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_hnf` from `fmpz_mat/hnf.c`, FLINT 3.6.0, except that the
    /// cutoffs between the algorithms differ.
    #[inline]
    fn hermite_normal_form(self) -> Self {
        hnf_helper(&self)
    }
}

impl HermiteNormalForm for &IntegerMatrix {
    type Output = IntegerMatrix;

    /// Computes the Hermite normal form of an [`IntegerMatrix`], taking it by reference. The
    /// Hermite normal form $H$ of an $h \times w$ matrix $A$ is the unique $h \times w$ matrix in
    /// row echelon form whose rows span the same lattice as those of $A$ (so that $H = UA$ for some
    /// unimodular $U$), whose pivots (the first nonzero elements of its nonzero rows) are positive,
    /// and whose elements above each pivot lie in $[0, p)$, where $p$ is the pivot. Its nonzero
    /// rows come first, and their number is the rank of $A$.
    ///
    /// Matrices with few rows for the size of their elements are reduced by the classical
    /// algorithm, which, column by column, reduces the elements below the pivot by the one of
    /// smallest absolute value until they are zero; the others by the algorithm of Pernet and
    /// Stein, which, modulo a word-sized prime, finds the pivot columns and a nonsingular square
    /// submatrix; finds that submatrix's Hermite normal form modulo its determinant; extends it to
    /// all the columns; and adds the remaining rows. Matrices with full column rank and few columns
    /// are instead reduced by the algorithm of Kannan and Bachem, which puts the leading principal
    /// minors into Hermite normal form in turn.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^4 (b + \log n)^2 + n^5 (b + \log n) \log (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^3 (b + \log n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum of the height and width of
    /// the matrix, and $b$ is the maximum number of significant bits of its elements: the
    /// determinants that bound the elements have $O(n(b + \log n))$ bits, adding each of $O(n)$
    /// rows takes $O(n^3)$ operations on such numbers, and the nonsingular systems are solved
    /// within the bounds of [`solve`](malachite_base::num::arithmetic::traits::Solve::solve).
    /// Matrices with fewer rows than a cutoff of at most 59 are reduced by the classical algorithm,
    /// whose intermediate elements can double in size with each pivot found; the bound holds for
    /// them only because their height is bounded by a constant.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::HermiteNormalForm;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).hermite_normal_form().to_string(), "[1, 0; 0, 2]");
    ///
    /// let a = IntegerMatrix::from_str("[6, 4; 4, 6; 9, 3]").unwrap();
    /// assert_eq!((&a).hermite_normal_form().to_string(), "[1, 1; 0, 2; 0, 0]");
    ///
    /// // A matrix of rank 1
    /// let a = IntegerMatrix::from_str("[2, 1; 4, 2; 6, 3]").unwrap();
    /// assert_eq!((&a).hermite_normal_form().to_string(), "[2, 1; 0, 0; 0, 0]");
    ///
    /// let a = IntegerMatrix::from_str("[0, 2, 1; 0, 4, 3]").unwrap();
    /// assert_eq!((&a).hermite_normal_form().to_string(), "[0, 2, 0; 0, 0, 1]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_hnf` from `fmpz_mat/hnf.c`, FLINT 3.6.0, except that the
    /// cutoffs between the algorithms differ.
    #[inline]
    fn hermite_normal_form(self) -> IntegerMatrix {
        hnf_helper(self)
    }
}

impl HermiteNormalFormAssign for IntegerMatrix {
    /// Replaces an [`IntegerMatrix`] with its Hermite normal form. The Hermite normal form $H$ of
    /// an $h \times w$ matrix $A$ is the unique $h \times w$ matrix in row echelon form whose rows
    /// span the same lattice as those of $A$ (so that $H = UA$ for some unimodular $U$), whose
    /// pivots (the first nonzero elements of its nonzero rows) are positive, and whose elements
    /// above each pivot lie in $[0, p)$, where $p$ is the pivot. Its nonzero rows come first, and
    /// their number is the rank of $A$.
    ///
    /// Matrices with few rows for the size of their elements are reduced by the classical
    /// algorithm, which, column by column, reduces the elements below the pivot by the one of
    /// smallest absolute value until they are zero; the others by the algorithm of Pernet and
    /// Stein, which, modulo a word-sized prime, finds the pivot columns and a nonsingular square
    /// submatrix; finds that submatrix's Hermite normal form modulo its determinant; extends it to
    /// all the columns; and adds the remaining rows. Matrices with full column rank and few columns
    /// are instead reduced by the algorithm of Kannan and Bachem, which puts the leading principal
    /// minors into Hermite normal form in turn.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^4 (b + \log n)^2 + n^5 (b + \log n) \log (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^3 (b + \log n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum of the height and width of
    /// the matrix, and $b$ is the maximum number of significant bits of its elements: the
    /// determinants that bound the elements have $O(n(b + \log n))$ bits, adding each of $O(n)$
    /// rows takes $O(n^3)$ operations on such numbers, and the nonsingular systems are solved
    /// within the bounds of [`solve`](malachite_base::num::arithmetic::traits::Solve::solve).
    /// Matrices with fewer rows than a cutoff of at most 59 are reduced by the classical algorithm,
    /// whose intermediate elements can double in size with each pivot found; the bound holds for
    /// them only because their height is bounded by a constant.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::HermiteNormalFormAssign;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let mut a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// a.hermite_normal_form_assign();
    /// assert_eq!(a.to_string(), "[1, 0; 0, 2]");
    ///
    /// let mut a = IntegerMatrix::from_str("[6, 4; 4, 6; 9, 3]").unwrap();
    /// a.hermite_normal_form_assign();
    /// assert_eq!(a.to_string(), "[1, 1; 0, 2; 0, 0]");
    ///
    /// // A matrix of rank 1
    /// let mut a = IntegerMatrix::from_str("[2, 1; 4, 2; 6, 3]").unwrap();
    /// a.hermite_normal_form_assign();
    /// assert_eq!(a.to_string(), "[2, 1; 0, 0; 0, 0]");
    ///
    /// let mut a = IntegerMatrix::from_str("[0, 2, 1; 0, 4, 3]").unwrap();
    /// a.hermite_normal_form_assign();
    /// assert_eq!(a.to_string(), "[0, 2, 0; 0, 0, 1]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_hnf` from `fmpz_mat/hnf.c`, FLINT 3.6.0, with `H` aliasing
    /// `A`, except that the cutoffs between the algorithms differ.
    #[inline]
    fn hermite_normal_form_assign(&mut self) {
        *self = hnf_helper(self);
    }
}
