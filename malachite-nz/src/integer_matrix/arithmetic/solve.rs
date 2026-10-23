// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2011 Fredrik Johansson
//
//      Copyright © 2020, 2021 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::integer_matrix::arithmetic::fraction_free_lu_decomposition::*;
use crate::natural_matrix::arithmetic::mod_determinant::permutation_is_odd;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    AddMulAssign, DivExactAssign, DivRem, DivisibleBy, ModPowerOf2Inverse, MulSubMul,
    MulSubMulAssign, NegAssign, SubMulAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::SignificantBits;

// Replaces `x` with `x / d` and returns true if `d` divides `x`; otherwise, returns false.
//
// This is `fmpz_divides` from `fmpz/divides.c`, FLINT 3.6.0, for a nonzero `d`.
fn divide_assign(x: &mut Integer, d: &Integer) -> bool {
    let (q, r) = (&*x).div_rem(d);
    if r == 0u32 {
        *x = q;
        true
    } else {
        false
    }
}

// The pivot column of each row of the `height`-by-`width` packed fraction-free LU decomposition
// `fflu`, or `width` if the row has none.
fn pivot_columns(fflu: &[Integer], height: usize, width: usize) -> Vec<usize> {
    let mut pivots = Vec::with_capacity(height);
    let mut l = 0;
    for i in 0..height {
        while l < width && fflu[i * width + l] == 0u32 {
            l += 1;
        }
        pivots.push(l);
        l += 1;
    }
    pivots
}

// The parts of a packed fraction-free LU decomposition that the substitutions read.
struct Substitution<'a> {
    fflu: &'a [Integer],
    height: usize,
    width: usize,
    pivots: &'a [usize],
    rank: usize,
    k: usize,
}

impl Substitution<'_> {
    const fn lu(&self, i: usize, j: usize) -> &Integer {
        &self.fflu[i * self.width + j]
    }

    // The pivot of row `i`, which must have one.
    const fn pivot(&self, i: usize) -> &Integer {
        self.lu(i, self.pivots[i])
    }

    // Whether row `i` has a pivot, and whether row `i - 1` does, so that step `i` of the forward
    // substitution updates the rows below and divides by the previous pivot.
    const fn step(&self, i: usize) -> (bool, bool) {
        (
            i < self.width && self.pivots[i] < self.width,
            i > 0 && i < self.width && self.pivots[i - 1] < self.width,
        )
    }

    // Runs the fraction-free forward substitution on the rows of `y` from step `start` on, and
    // returns false if a division is not exact.
    fn forward(&self, y: &mut [Integer], start: usize) -> bool {
        let k = self.k;
        for i in start..self.height.saturating_sub(1) {
            let (has_pivot, divide) = self.step(i);
            let (top, bottom) = y.split_at_mut((i + 1) * k);
            let y_i = &top[i * k..];
            for (j, y_j) in bottom.chunks_exact_mut(k).enumerate() {
                if has_pivot {
                    let multiplier = self.lu(i + 1 + j, self.pivots[i]);
                    let pivot = self.pivot(i);
                    for (z, w) in y_j.iter_mut().zip(y_i) {
                        z.mul_sub_mul_assign(pivot, multiplier, w);
                    }
                }
                if divide {
                    let den = self.pivot(i - 1);
                    for z in y_j {
                        if !divide_assign(z, den) {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    // The fraction-free back substitution, from the forward-substituted rows `y`: the elements of
    // X.
    fn back(&self, y: &[Integer]) -> Vec<Integer> {
        let k = self.k;
        let width = self.width;
        let mut x = vec![Integer::ZERO; width * k];
        let mut l = self.rank;
        for i in (0..width).rev() {
            if l > 0 && i == self.pivots[l - 1] {
                l -= 1;
                let last_pivot = self.pivot(self.rank - 1);
                let (head, tail) = x.split_at_mut((i + 1) * k);
                let x_i = &mut head[i * k..];
                for (z, w) in x_i.iter_mut().zip(&y[l * k..(l + 1) * k]) {
                    *z = w * last_pivot;
                }
                for (j, x_j) in (self.pivots[l] + 1..width).zip(tail.chunks_exact(k)) {
                    let c = self.lu(l, j);
                    if *c != 0u32 {
                        for (z, w) in x_i.iter_mut().zip(x_j) {
                            z.sub_mul_assign(w, c);
                        }
                    }
                }
                // The first r rows of PAX = dPB form a system whose matrix, in the pivot columns,
                // has full rank and determinant +-d, so by Cramer's rule every element of X is an
                // integer and this division is exact. (FLINT checks it anyway.)
                let pivot = self.pivot(l);
                for z in x_i {
                    z.div_exact_assign(pivot);
                }
            }
        }
        x
    }
}

// The quotient of `z` by the nonzero `d`, if it is exact and at most `SMALL_MAX` in absolute value,
// as `Ok`; `Err(None)` if it is not exact; or `Err(Some(q))` if it is exact but larger. The
// quotient is computed by multiplying by the inverse modulo 2^64 of the odd part of `d`, and
// checked by multiplying back.
fn small_exact_quotient(z: i128, d: i64) -> Result<i64, Option<i128>> {
    let bound = u128::from(SMALL_MAX.unsigned_abs()) * u128::from(d.unsigned_abs());
    if z.unsigned_abs() <= bound {
        let shift = d.trailing_zeros();
        let inverse = (d >> shift)
            .cast_unsigned()
            .mod_power_of_2_inverse(u64::WIDTH)
            .unwrap();
        let q = u64::wrapping_from(z >> shift)
            .wrapping_mul(inverse)
            .cast_signed();
        if i128::from(q) * i128::from(d) == z {
            Ok(q)
        } else {
            Err(None)
        }
    } else if z.divisible_by(i128::from(d)) {
        Err(Some(z / i128::from(d)))
    } else {
        Err(None)
    }
}

// How the small-element forward substitution ended.
enum SmallForward {
    Finished,
    NotExact,
    // A step produced elements too large for the path; the step is complete, the large elements are
    // listed with their indices (their places hold 0), and the next step is `next_step`.
    TooLarge {
        next_step: usize,
        large: Vec<(usize, i128)>,
    },
}

// The forward substitution of `Substitution::forward` on word-sized elements: `lu` holds the
// elements of the decomposition and `ys` the rows of PB, all at most `SMALL_MAX` in absolute value.
fn forward_small(s: &Substitution<'_>, lu: &[i64], ys: &mut [i64]) -> SmallForward {
    let k = s.k;
    let width = s.width;
    let mut large = Vec::new();
    for i in 0..s.height.saturating_sub(1) {
        let (has_pivot, divide) = s.step(i);
        let pivot = if has_pivot { lu[i * width + s.pivots[i]] } else { 0 };
        let den = if divide { lu[(i - 1) * width + s.pivots[i - 1]] } else { 1 };
        let (top, bottom) = ys.split_at_mut((i + 1) * k);
        let y_i = &top[i * k..];
        for (j, y_j) in bottom.chunks_exact_mut(k).enumerate() {
            let multiplier = if has_pivot {
                lu[(i + 1 + j) * width + s.pivots[i]]
            } else {
                0
            };
            let row_start = (i + 1 + j) * k;
            for (c, (z, &w)) in y_j.iter_mut().zip(y_i).enumerate() {
                let product = if has_pivot {
                    i128::from(*z) * i128::from(pivot) - i128::from(multiplier) * i128::from(w)
                } else {
                    i128::from(*z)
                };
                if divide {
                    match small_exact_quotient(product, den) {
                        Ok(q) => *z = q,
                        Err(None) => return SmallForward::NotExact,
                        Err(Some(q)) => {
                            *z = 0;
                            large.push((row_start + c, q));
                        }
                    }
                } else if product.unsigned_abs() <= SMALL_MAX_U128 {
                    *z = i64::wrapping_from(product);
                } else {
                    *z = 0;
                    large.push((row_start + c, product));
                }
            }
        }
        if !large.is_empty() {
            return SmallForward::TooLarge {
                next_step: i + 1,
                large,
            };
        }
    }
    SmallForward::Finished
}

// The back substitution of `Substitution::back` on word-sized elements, or `None` if an element or
// a sum outgrows the path.
fn back_small(s: &Substitution<'_>, lu: &[i64], ys: &[i64]) -> Option<Vec<i64>> {
    let k = s.k;
    let width = s.width;
    let pivot = |i: usize| lu[i * width + s.pivots[i]];
    let last_pivot = i128::from(pivot(s.rank - 1));
    let mut x = vec![0; width * k];
    let mut l = s.rank;
    for i in (0..width).rev() {
        if l > 0 && i == s.pivots[l - 1] {
            l -= 1;
            let d = pivot(l);
            let (head, tail) = x.split_at_mut((i + 1) * k);
            let x_i = &mut head[i * k..];
            for (c, z) in x_i.iter_mut().enumerate() {
                let mut sum = i128::from(ys[l * k + c]) * last_pivot;
                for (j, x_j) in (s.pivots[l] + 1..width).zip(tail.chunks_exact(k)) {
                    let a = lu[l * width + j];
                    if a != 0 {
                        sum = sum.checked_sub(i128::from(x_j[c]) * i128::from(a))?;
                    }
                }
                *z = small_exact_quotient(sum, d).ok()?;
            }
        }
    }
    Some(x)
}

// Whether the small-element path is likely to finish: the elements of X are about as large as the
// last pivot times the largest element of B, times the width, so the path is only tried if that
// estimate fits in the path's 62 bits. Checking the pivot first rejects most hopeless cases without
// reading B.
fn small_path_is_promising(last_pivot: &Integer, b: &[Integer], width: usize) -> bool {
    const SMALL_BITS: u64 = 62;
    let width_bits = u64::wrapping_from(width).significant_bits();
    let pivot_bits = last_pivot.significant_bits();
    pivot_bits + width_bits <= SMALL_BITS
        && b.iter().all(|x| pivot_bits + x.significant_bits() + width_bits <= SMALL_BITS)
}

// Solves PAX = d PB, where `fflu` is the packed fraction-free LU decomposition of the
// `height`-by-`width` matrix A from `fflu_in_place_with_options`, `perm` is its permutation P, d is
// its last pivot, and B is the `height`-by-`k` matrix with elements `b`. Returns the elements of
// the `width`-by-`k` matrix X, whose rows for non-pivot columns are zero, or `None` if a division
// in the forward substitution is not exact, which can only happen if the system has no solution.
// When A has rank r, the first r rows of PAX = d PB hold; the others must be checked by the caller.
// If `allow_small` is true, the matrix is not too small, and every element of `fflu` and `b` is at
// most `SMALL_MAX` in absolute value, the substitutions start with word-sized arithmetic.
//
// This is equivalent to `fmpz_mat_solve_fflu_precomp` from `fmpz_mat/solve_fflu_precomp.c`, FLINT
// 3.6.0.
pub(crate) fn solve_fflu_precomp_with_options(
    perm: &[usize],
    fflu: &[Integer],
    height: usize,
    width: usize,
    b: &[Integer],
    k: usize,
    allow_small: bool,
) -> Option<Vec<Integer>> {
    if k == 0 {
        return Some(Vec::new());
    }
    let pivots = pivot_columns(fflu, height, width);
    let s = Substitution {
        fflu,
        height,
        width,
        pivots: &pivots,
        rank: pivots.iter().filter(|&&p| p < width).count(),
        k,
    };
    // PB, whose rows are transformed together.
    let permute = |xs: &[Integer]| {
        let mut y = Vec::with_capacity(height * k);
        for &p in perm {
            y.extend_from_slice(&xs[p * k..(p + 1) * k]);
        }
        y
    };
    if allow_small
        && height.min(width) >= SMALL_PATH_MIN_DIMENSION
        && s.rank != 0
        && small_path_is_promising(s.pivot(s.rank - 1), b, width)
        && let Some(lu) = small_elements(fflu)
        && let Some(b) = small_elements(b)
    {
        let mut ys = Vec::with_capacity(height * k);
        for &p in perm {
            ys.extend_from_slice(&b[p * k..(p + 1) * k]);
        }
        let to_integers = |ys: &[i64]| ys.iter().map(|&y| Integer::from(y)).collect::<Vec<_>>();
        match forward_small(&s, &lu, &mut ys) {
            SmallForward::NotExact => return None,
            SmallForward::TooLarge { next_step, large } => {
                let mut y = to_integers(&ys);
                for (index, q) in large {
                    y[index] = Integer::from(q);
                }
                return if s.forward(&mut y, next_step) {
                    Some(s.back(&y))
                } else {
                    None
                };
            }
            SmallForward::Finished => {
                return Some(match back_small(&s, &lu, &ys) {
                    Some(x) => to_integers(&x),
                    None => s.back(&to_integers(&ys)),
                });
            }
        }
    }
    let mut y = permute(b);
    if s.forward(&mut y, 0) {
        Some(s.back(&y))
    } else {
        None
    }
}

// `solve_fflu_precomp_with_options` with the small-element path allowed.
crate_test_fn! {
solve_fflu_precomp(
    perm: &[usize],
    fflu: &[Integer],
    height: usize,
    width: usize,
    b: &[Integer],
    k: usize,
) -> Option<Vec<Integer>> {
    solve_fflu_precomp_with_options(perm, fflu, height, width, b, k, true)
}}

crate_test_fn! {
solve_fflu(a: &IntegerMatrix, b: &IntegerMatrix) -> Option<(IntegerMatrix, Integer)> {
    assert_eq!(a.height, a.width, "the matrix must be square");
    assert_eq!(
        a.height, b.height,
        "the height of the matrix must equal the height of the right-hand side"
    );
    let n = a.height;
    let k = b.width;
    if n == 0 || k == 0 {
        return Some((
            IntegerMatrix {
                elements: vec![Integer::ZERO; n * k],
                height: n,
                width: k,
            },
            Integer::ONE,
        ));
    }
    let mut lu = a.elements.clone();
    let mut perm: Vec<usize> = (0..n).collect();
    let (rank, mut den) = fflu_in_place_with_options(&mut lu, n, n, &mut perm, true, true);
    if rank != n {
        return None;
    }
    let mut x = solve_fflu_precomp(&perm, &lu, n, n, &b.elements, k).unwrap();
    if permutation_is_odd(&perm) {
        den.neg_assign();
        for y in &mut x {
            y.neg_assign();
        }
    }
    Some((
        IntegerMatrix {
            elements: x,
            height: n,
            width: k,
        },
        den,
    ))
}}

// Solves AX = dB by Cramer's rule for a square `a` of dimension at most 3, returning X and d, the
// determinant of `a`, or `None` if `a` is singular.
//
// This is equivalent to `fmpz_mat_solve_cramer` from `fmpz_mat/solve_cramer.c`, FLINT 3.6.0.
crate_test_fn! {
solve_cramer(a: &IntegerMatrix, b: &IntegerMatrix) -> Option<(IntegerMatrix, Integer)> {
    assert_eq!(a.height, a.width, "the matrix must be square");
    assert_eq!(
        a.height, b.height,
        "the height of the matrix must equal the height of the right-hand side"
    );
    let n = a.height;
    let k = b.width;
    let aa = |i: usize, j: usize| &a.elements[i * n + j];
    let bb = |i: usize, j: usize| &b.elements[i * k + j];
    let fmms = |w: &Integer, x: &Integer, y: &Integer, z: &Integer| w.mul_sub_mul(x, y, z);
    let mut x = vec![Integer::ZERO; n * k];
    let den = match n {
        _ if n == 0 || k == 0 => Integer::ONE,
        1 => {
            let den = aa(0, 0).clone();
            if den == 0u32 {
                return None;
            }
            x.clone_from_slice(&b.elements);
            den
        }
        2 => {
            let den = fmms(aa(0, 0), aa(1, 1), aa(0, 1), aa(1, 0));
            if den == 0u32 {
                return None;
            }
            for i in 0..k {
                x[i] = fmms(aa(1, 1), bb(0, i), aa(0, 1), bb(1, i));
                x[k + i] = fmms(aa(0, 0), bb(1, i), aa(1, 0), bb(0, i));
            }
            den
        }
        3 => {
            let t17 = fmms(aa(1, 0), aa(2, 1), aa(1, 1), aa(2, 0));
            let t16 = fmms(aa(1, 2), aa(2, 0), aa(1, 0), aa(2, 2));
            let t15 = fmms(aa(1, 1), aa(2, 2), aa(1, 2), aa(2, 1));
            let mut den = &t15 * aa(0, 0);
            den.add_mul_assign(&t16, aa(0, 1));
            den.add_mul_assign(&t17, aa(0, 2));
            if den == 0u32 {
                return None;
            }
            for i in 0..k {
                let t14 = fmms(aa(2, 0), bb(1, i), aa(1, 0), bb(2, i));
                let t13 = fmms(aa(2, 1), bb(1, i), aa(1, 1), bb(2, i));
                let t12 = fmms(aa(2, 2), bb(1, i), aa(1, 2), bb(2, i));
                let mut x0 = &t15 * bb(0, i);
                x0.add_mul_assign(&t13, aa(0, 2));
                x0.sub_mul_assign(&t12, aa(0, 1));
                let mut x1 = &t16 * bb(0, i);
                x1.add_mul_assign(&t12, aa(0, 0));
                x1.sub_mul_assign(&t14, aa(0, 2));
                let mut x2 = &t17 * bb(0, i);
                x2.add_mul_assign(&t14, aa(0, 1));
                x2.sub_mul_assign(&t13, aa(0, 0));
                x[i] = x0;
                x[k + i] = x1;
                x[(k << 1) + i] = x2;
            }
            den
        }
        _ => panic!("Cramer's rule is only implemented for dimensions up to 3"),
    };
    Some((
        IntegerMatrix {
            elements: x,
            height: n,
            width: k,
        },
        den,
    ))
}}
