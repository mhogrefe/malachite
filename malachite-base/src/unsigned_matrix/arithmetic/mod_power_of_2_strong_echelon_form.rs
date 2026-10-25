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

use crate::num::arithmetic::traits::{
    ModPowerOf2IsReduced, ModPowerOf2StrongEchelonForm, ModPowerOf2StrongEchelonFormAssign,
};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::logic::traits::TrailingZeros;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::mod_strong_echelon_form::{pad, swap_rows, two_rows};
use alloc::vec::Vec;

// The multiplier s with s·p ≡ b (mod 2^pow), given p = 2^v·u with u odd, the inverse
// `u_inverse` of u modulo 2^pow, and b, which is divisible by 2^v.
fn quotient<T: PrimitiveUnsigned>(b: T, v: u64, u_inverse: T, pow: u64) -> T {
    (b >> v).mod_power_of_2_mul(u_inverse, pow)
}

// Replaces `ys` with `ys` - s·`xs` modulo 2^pow.
fn sub_mul_row<T: PrimitiveUnsigned>(ys: &mut [T], s: T, xs: &[T], pow: u64) {
    for (y, &x) in ys.iter_mut().zip(xs) {
        *y = y.mod_power_of_2_sub(s.mod_power_of_2_mul(x, pow), pow);
    }
}

// The 2-adic valuation v of the nonzero `x`, and the inverse modulo 2^pow of x/2^v.
fn valuation_and_unit_inverse<T: PrimitiveUnsigned>(x: T, pow: u64) -> (u64, T) {
    let v = TrailingZeros::trailing_zeros(x);
    (v, (x >> v).mod_power_of_2_inverse(pow).unwrap())
}

// Puts the `height` by `width` matrix with elements `xs`, which are reduced modulo 2^pow, into
// strong echelon form modulo 2^pow. `height` must be at least `width`.
//
// The algorithm is that of Fieker and Hofmann, specialized to the chain ring Z/2^pow, in which
// every nonzero element is a power of 2 times a unit, so that x divides y exactly when the 2-adic
// valuation of x is at most that of y: the pivot of each column is an element of smallest
// valuation, which divides the others, so the extended-GCD transformations of the general algorithm
// become subtractions; the units are inverted modulo 2^pow; and the elements above each pivot 2^v
// are reduced by shifting.
//
// This is equivalent to `nmod_mat_strong_echelon_form` from `nmod_mat/strong_echelon_form.c`, FLINT
// 3.6.0, with modulus 2^pow.
pub(crate) fn strong_echelon_form_mod_power_of_2_in_place<T: PrimitiveUnsigned>(
    xs: &mut [T],
    height: usize,
    width: usize,
    pow: u64,
) {
    if height == 0 || width == 0 {
        return;
    }
    assert!(height >= width);
    let n = height;
    let w = width;
    // Reduce to upper triangular form, choosing the pivot of each column with the smallest
    // valuation.
    let mut row = 0;
    let mut col = 0;
    while row < n && col < w {
        let Some(best) = (row..n)
            .filter(|&i| xs[i * w + col] != T::ZERO)
            .min_by_key(|&i| TrailingZeros::trailing_zeros(xs[i * w + col]))
        else {
            col += 1;
            continue;
        };
        swap_rows(xs, w, row, best);
        let (v, u_inverse) = valuation_and_unit_inverse(xs[row * w + col], pow);
        for i in row + 1..n {
            let (pivot_row, other_row) = two_rows(xs, w, row, i);
            let b = other_row[col];
            if b != T::ZERO {
                let s = quotient(b, v, u_inverse, pow);
                sub_mul_row(&mut other_row[col..], s, &pivot_row[col..], pow);
            }
        }
        row += 1;
        col += 1;
    }
    let mut extra_row = Vec::with_capacity(w);
    for col in 0..w {
        if xs[col * w + col] == T::ZERO {
            extra_row.clear();
            extra_row.extend_from_slice(&xs[col * w..(col + 1) * w]);
        } else {
            // Make the diagonal element a power of 2, and reduce the elements above it.
            let (v, u_inverse) = valuation_and_unit_inverse(xs[col * w + col], pow);
            let (above, pivot_and_below) = xs.split_at_mut(col * w);
            let pivot_row = &mut pivot_and_below[..w];
            for x in &mut pivot_row[col..] {
                *x = x.mod_power_of_2_mul(u_inverse, pow);
            }
            for (row, above_row) in above.chunks_exact_mut(w).enumerate() {
                let q = above_row[col] >> v;
                if q != T::ZERO {
                    sub_mul_row(&mut above_row[row..], q, &pivot_row[row..], pow);
                }
            }
            if v == 0 {
                continue;
            }
            // The row times 2^(pow - v) has a zero in this column, and may have nonzero elements
            // further right.
            extra_row.clear();
            extra_row.extend(
                pivot_row
                    .iter()
                    .map(|&x| x.mod_power_of_2_shl(pow - v, pow)),
            );
        }
        // Combine the extra row into the rows below.
        for row in col + 1..w {
            let this_row = &mut xs[row * w..(row + 1) * w];
            let a = this_row[row];
            let b = extra_row[row];
            if b == T::ZERO {
                continue;
            }
            if a == T::ZERO || TrailingZeros::trailing_zeros(b) < TrailingZeros::trailing_zeros(a) {
                this_row[row..].swap_with_slice(&mut extra_row[row..]);
            }
            let (v, u_inverse) = valuation_and_unit_inverse(this_row[row], pow);
            let s = quotient(extra_row[row], v, u_inverse, pow);
            sub_mul_row(&mut extra_row[row..], s, &this_row[row..], pow);
        }
    }
}

fn assert_arguments<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, pow: u64) {
    assert!(
        pow <= T::WIDTH,
        "pow must be at most the width of the element type, {}, but it is {pow}",
        T::WIDTH
    );
    assert!(
        a.mod_power_of_2_is_reduced(pow),
        "the matrix must be reduced mod 2^{pow}, but {a} has an element >= 2^{pow}"
    );
}

impl<T: PrimitiveUnsigned> ModPowerOf2StrongEchelonForm for UnsignedMatrix<T> {
    type Output = Self;
    /// Computes the strong echelon form of an [`UnsignedMatrix`] modulo $2^k$, taking it by value.
    ///
    /// The elements of the matrix must already be reduced modulo $2^k$. The strong echelon form of
    /// an $h \times w$ matrix $A$ modulo $2^k$ is the $\max(h, w) \times w$ matrix with the same
    /// nonzero rows as the Howell form of $A$ modulo $2^k$, but with each nonzero row in the row
    /// whose index is the column of its pivot, so that the pivots are on the diagonal, and with
    /// zero rows everywhere else. It is unique, and its pivots are powers of 2, with the elements
    /// above each pivot $2^v$ in $[0, 2^v)$.
    ///
    /// The algorithm is that of Fieker and Hofmann, specialized to the chain ring
    /// $\mathbb{Z}/2^k\mathbb{Z}$, in which every nonzero element is a power of 2 times a unit: the
    /// pivot of each column is an element of smallest 2-adic valuation, which divides the others,
    /// so the matrix is triangularized by subtracting multiples of rows; each pivot is then divided
    /// by its unit part and the elements above it are reduced; and, for each pivot $2^v \ne 1$, the
    /// row times $2^{k - v}$ is combined into the rows below.
    ///
    /// # Worst-case complexity
    /// $T(h, w) = O(hw^2)$
    ///
    /// $M(h, w) = O(hw)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the larger of the matrix's height and
    /// width, and $w$ is its width.
    ///
    /// # Panics
    /// Panics if `pow` is greater than the width of `T`, or if any element of the matrix is greater
    /// than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2StrongEchelonForm;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 4; 6, 0]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_strong_echelon_form(3).to_string(),
    ///     "[2, 0; 0, 4]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[4, 2, 6; 2, 0, 4]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_strong_echelon_form(3).to_string(),
    ///     "[2, 0, 4; 0, 2, 6; 0, 0, 0]"
    /// );
    ///
    /// // A matrix with more rows than columns
    /// let a = UnsignedMatrix::<u8>::from_str("[0, 2; 0, 4; 0, 6]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_strong_echelon_form(3).to_string(),
    ///     "[0, 0; 0, 2; 0, 0]"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_strong_echelon_form(mut self, pow: u64) -> Self {
        self.mod_power_of_2_strong_echelon_form_assign(pow);
        self
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2StrongEchelonForm for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;
    /// Computes the strong echelon form of an [`UnsignedMatrix`] modulo $2^k$, taking it by
    /// reference.
    ///
    /// The elements of the matrix must already be reduced modulo $2^k$. The strong echelon form of
    /// an $h \times w$ matrix $A$ modulo $2^k$ is the $\max(h, w) \times w$ matrix with the same
    /// nonzero rows as the Howell form of $A$ modulo $2^k$, but with each nonzero row in the row
    /// whose index is the column of its pivot, so that the pivots are on the diagonal, and with
    /// zero rows everywhere else. It is unique, and its pivots are powers of 2, with the elements
    /// above each pivot $2^v$ in $[0, 2^v)$.
    ///
    /// The algorithm is that of Fieker and Hofmann, specialized to the chain ring
    /// $\mathbb{Z}/2^k\mathbb{Z}$, in which every nonzero element is a power of 2 times a unit: the
    /// pivot of each column is an element of smallest 2-adic valuation, which divides the others,
    /// so the matrix is triangularized by subtracting multiples of rows; each pivot is then divided
    /// by its unit part and the elements above it are reduced; and, for each pivot $2^v \ne 1$, the
    /// row times $2^{k - v}$ is combined into the rows below.
    ///
    /// # Worst-case complexity
    /// $T(h, w) = O(hw^2)$
    ///
    /// $M(h, w) = O(hw)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the larger of the matrix's height and
    /// width, and $w$ is its width.
    ///
    /// # Panics
    /// Panics if `pow` is greater than the width of `T`, or if any element of the matrix is greater
    /// than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2StrongEchelonForm;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 4; 6, 0]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_strong_echelon_form(3).to_string(),
    ///     "[2, 0; 0, 4]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[4, 2, 6; 2, 0, 4]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_strong_echelon_form(3).to_string(),
    ///     "[2, 0, 4; 0, 2, 6; 0, 0, 0]"
    /// );
    ///
    /// // A matrix with more rows than columns
    /// let a = UnsignedMatrix::<u8>::from_str("[0, 2; 0, 4; 0, 6]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_strong_echelon_form(3).to_string(),
    ///     "[0, 0; 0, 2; 0, 0]"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_strong_echelon_form(self, pow: u64) -> UnsignedMatrix<T> {
        self.clone().mod_power_of_2_strong_echelon_form(pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2StrongEchelonFormAssign for UnsignedMatrix<T> {
    /// Replaces an [`UnsignedMatrix`] with its strong echelon form modulo $2^k$.
    ///
    /// The elements of the matrix must already be reduced modulo $2^k$. The strong echelon form of
    /// an $h \times w$ matrix $A$ modulo $2^k$ is the $\max(h, w) \times w$ matrix with the same
    /// nonzero rows as the Howell form of $A$ modulo $2^k$, but with each nonzero row in the row
    /// whose index is the column of its pivot, so that the pivots are on the diagonal, and with
    /// zero rows everywhere else. It is unique, and its pivots are powers of 2, with the elements
    /// above each pivot $2^v$ in $[0, 2^v)$.
    ///
    /// The algorithm is that of Fieker and Hofmann, specialized to the chain ring
    /// $\mathbb{Z}/2^k\mathbb{Z}$, in which every nonzero element is a power of 2 times a unit: the
    /// pivot of each column is an element of smallest 2-adic valuation, which divides the others,
    /// so the matrix is triangularized by subtracting multiples of rows; each pivot is then divided
    /// by its unit part and the elements above it are reduced; and, for each pivot $2^v \ne 1$, the
    /// row times $2^{k - v}$ is combined into the rows below.
    ///
    /// # Worst-case complexity
    /// $T(h, w) = O(hw^2)$
    ///
    /// $M(h, w) = O(hw)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the larger of the matrix's height and
    /// width, and $w$ is its width.
    ///
    /// # Panics
    /// Panics if `pow` is greater than the width of `T`, or if any element of the matrix is greater
    /// than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2StrongEchelonFormAssign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[2, 4; 6, 0]").unwrap();
    /// a.mod_power_of_2_strong_echelon_form_assign(3);
    /// assert_eq!(a.to_string(), "[2, 0; 0, 4]");
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[4, 2, 6; 2, 0, 4]").unwrap();
    /// a.mod_power_of_2_strong_echelon_form_assign(3);
    /// assert_eq!(a.to_string(), "[2, 0, 4; 0, 2, 6; 0, 0, 0]");
    ///
    /// // A matrix with more rows than columns
    /// let mut a = UnsignedMatrix::<u8>::from_str("[0, 2; 0, 4; 0, 6]").unwrap();
    /// a.mod_power_of_2_strong_echelon_form_assign(3);
    /// assert_eq!(a.to_string(), "[0, 0; 0, 2; 0, 0]");
    /// ```
    fn mod_power_of_2_strong_echelon_form_assign(&mut self, pow: u64) {
        assert_arguments(self, pow);
        pad(self);
        strong_echelon_form_mod_power_of_2_in_place(
            &mut self.elements,
            self.height,
            self.width,
            pow,
        );
    }
}
