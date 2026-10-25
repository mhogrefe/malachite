// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    ModPowerOf2IsReduced, ModPowerOf2SmithNormalForm, ModPowerOf2SmithNormalFormAssign,
};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::logic::traits::TrailingZeros;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::mod_strong_echelon_form::{swap_rows, two_rows};
use alloc::vec::Vec;

// Swaps columns `i` and `j` of the `width`-wide matrix with elements `xs`.
pub(crate) fn swap_columns<T>(xs: &mut [T], width: usize, i: usize, j: usize) {
    if i != j {
        for row in xs.chunks_exact_mut(width) {
            row.swap(i, j);
        }
    }
}

// Replaces `ys` with `ys` - s·`xs` modulo 2^pow.
pub(crate) fn sub_mul_row<T: PrimitiveUnsigned>(ys: &mut [T], s: T, xs: &[T], pow: u64) {
    for (y, &x) in ys.iter_mut().zip(xs) {
        *y = y.mod_power_of_2_sub(s.mod_power_of_2_mul(x, pow), pow);
    }
}

// Subtracts s times column `j` from column `k`, modulo 2^pow, in the `width`-wide matrix with
// elements `xs`.
fn sub_mul_column<T: PrimitiveUnsigned>(
    xs: &mut [T],
    width: usize,
    k: usize,
    s: T,
    j: usize,
    pow: u64,
) {
    for row in xs.chunks_exact_mut(width) {
        let x = s.mod_power_of_2_mul(row[j], pow);
        row[k].mod_power_of_2_sub_assign(x, pow);
    }
}

// Reduces the `height` by `width` matrix with elements `xs`, which are reduced modulo 2^pow, to the
// diagonal matrix diag(2^v_0, 2^v_1, ..., 2^v_(r-1), 0, ...) with v_0 <= v_1 <= ... by invertible
// row and column operations over the chain ring Z/2^pow, and returns (v_0, ..., v_(r-1)), where r
// is the number of nonzero diagonal elements. At each step, an element of smallest 2-adic valuation
// in the remaining submatrix is moved to the diagonal and divided by its unit part, so that it is a
// power of 2 dividing every other element; then its row and column are cleared. The row operations
// are also applied to the `rhs_width`-wide matrix with elements `rhs`, which has `height` rows, if
// it is present, and the column operations to the `width` by `width` matrix with elements `v`, if
// it is present.
pub(crate) fn smith_eliminate_mod_power_of_2<T: PrimitiveUnsigned>(
    xs: &mut [T],
    height: usize,
    width: usize,
    pow: u64,
    mut rhs: Option<(&mut [T], usize)>,
    mut v: Option<&mut [T]>,
) -> Vec<u64> {
    let (h, w) = (height, width);
    let mut valuations = Vec::new();
    for t in 0..h.min(w) {
        let Some((val, i, j)) = (t..h)
            .flat_map(|i| (t..w).map(move |j| (i, j)))
            .filter(|&(i, j)| xs[i * w + j] != T::ZERO)
            .map(|(i, j)| (TrailingZeros::trailing_zeros(xs[i * w + j]), i, j))
            .min()
        else {
            break;
        };
        swap_rows(xs, w, t, i);
        swap_columns(xs, w, t, j);
        if let Some((rhs, rhs_width)) = rhs.as_mut() {
            swap_rows(rhs, *rhs_width, t, i);
        }
        if let Some(v) = v.as_deref_mut() {
            swap_columns(v, w, t, j);
        }
        // Divide the pivot by its unit part.
        let u_inverse = (xs[t * w + t] >> val).mod_power_of_2_inverse(pow).unwrap();
        for x in &mut xs[t * w + t..(t + 1) * w] {
            *x = x.mod_power_of_2_mul(u_inverse, pow);
        }
        if let Some((rhs, rhs_width)) = rhs.as_mut() {
            for x in &mut rhs[t * *rhs_width..(t + 1) * *rhs_width] {
                *x = x.mod_power_of_2_mul(u_inverse, pow);
            }
        }
        // Clear the column below the pivot with row operations.
        for i in t + 1..h {
            let s = xs[i * w + t] >> val;
            if s != T::ZERO {
                let (pivot_row, other_row) = two_rows(xs, w, t, i);
                sub_mul_row(&mut other_row[t..], s, &pivot_row[t..], pow);
                if let Some((rhs, rhs_width)) = rhs.as_mut() {
                    let (pivot_row, other_row) = two_rows(rhs, *rhs_width, t, i);
                    sub_mul_row(other_row, s, pivot_row, pow);
                }
            }
        }
        // Clear the row to the right of the pivot with column operations; the column of the pivot
        // is zero below it, so only row t changes.
        for k in t + 1..w {
            let s = xs[t * w + k] >> val;
            if s != T::ZERO {
                xs[t * w + k] = T::ZERO;
                if let Some(v) = v.as_deref_mut() {
                    sub_mul_column(v, w, k, s, t, pow);
                }
            }
        }
        valuations.push(val);
    }
    valuations
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

impl<T: PrimitiveUnsigned> ModPowerOf2SmithNormalForm for UnsignedMatrix<T> {
    type Output = Self;
    /// Computes the Smith normal form of an [`UnsignedMatrix`] modulo $2^k$, taking it by value.
    ///
    /// The elements of the matrix must already be reduced modulo $2^k$. The Smith normal form of a
    /// matrix $A$ modulo $2^k$ is the unique diagonal matrix $UAV$, for $U$ and $V$ invertible
    /// modulo $2^k$, whose diagonal elements are powers of 2 in increasing order, followed by
    /// zeros.
    ///
    /// At each step, an element of smallest 2-adic valuation in the remaining submatrix is moved to
    /// the diagonal and divided by its unit part, so that it is a power of 2 dividing every other
    /// element of the submatrix; then its row and column are cleared.
    ///
    /// # Worst-case complexity
    /// $T(h, w) = O(hw\min(h, w))$
    ///
    /// $M(h, w) = O(hw)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, and $w$ is its
    /// width.
    ///
    /// # Panics
    /// Panics if `pow` is greater than the width of `T`, or if any element of the matrix is greater
    /// than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SmithNormalForm;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 4; 6, 0]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_smith_normal_form(3).to_string(),
    ///     "[2, 0; 0, 4]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[4, 2, 6; 2, 0, 4]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_smith_normal_form(3).to_string(),
    ///     "[2, 0, 0; 0, 2, 0]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_smith_normal_form(3).to_string(),
    ///     "[1, 0; 0, 2]"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_smith_normal_form(mut self, pow: u64) -> Self {
        self.mod_power_of_2_smith_normal_form_assign(pow);
        self
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2SmithNormalForm for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;
    /// Computes the Smith normal form of an [`UnsignedMatrix`] modulo $2^k$, taking it by
    /// reference.
    ///
    /// The elements of the matrix must already be reduced modulo $2^k$. The Smith normal form of a
    /// matrix $A$ modulo $2^k$ is the unique diagonal matrix $UAV$, for $U$ and $V$ invertible
    /// modulo $2^k$, whose diagonal elements are powers of 2 in increasing order, followed by
    /// zeros.
    ///
    /// At each step, an element of smallest 2-adic valuation in the remaining submatrix is moved to
    /// the diagonal and divided by its unit part, so that it is a power of 2 dividing every other
    /// element of the submatrix; then its row and column are cleared.
    ///
    /// # Worst-case complexity
    /// $T(h, w) = O(hw\min(h, w))$
    ///
    /// $M(h, w) = O(hw)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, and $w$ is its
    /// width.
    ///
    /// # Panics
    /// Panics if `pow` is greater than the width of `T`, or if any element of the matrix is greater
    /// than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SmithNormalForm;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 4; 6, 0]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_smith_normal_form(3).to_string(),
    ///     "[2, 0; 0, 4]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[4, 2, 6; 2, 0, 4]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_smith_normal_form(3).to_string(),
    ///     "[2, 0, 0; 0, 2, 0]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_smith_normal_form(3).to_string(),
    ///     "[1, 0; 0, 2]"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_smith_normal_form(self, pow: u64) -> UnsignedMatrix<T> {
        self.clone().mod_power_of_2_smith_normal_form(pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2SmithNormalFormAssign for UnsignedMatrix<T> {
    /// Replaces an [`UnsignedMatrix`] with its Smith normal form modulo $2^k$.
    ///
    /// The elements of the matrix must already be reduced modulo $2^k$. The Smith normal form of a
    /// matrix $A$ modulo $2^k$ is the unique diagonal matrix $UAV$, for $U$ and $V$ invertible
    /// modulo $2^k$, whose diagonal elements are powers of 2 in increasing order, followed by
    /// zeros.
    ///
    /// At each step, an element of smallest 2-adic valuation in the remaining submatrix is moved to
    /// the diagonal and divided by its unit part, so that it is a power of 2 dividing every other
    /// element of the submatrix; then its row and column are cleared.
    ///
    /// # Worst-case complexity
    /// $T(h, w) = O(hw\min(h, w))$
    ///
    /// $M(h, w) = O(hw)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, and $w$ is its
    /// width.
    ///
    /// # Panics
    /// Panics if `pow` is greater than the width of `T`, or if any element of the matrix is greater
    /// than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SmithNormalFormAssign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[2, 4; 6, 0]").unwrap();
    /// a.mod_power_of_2_smith_normal_form_assign(3);
    /// assert_eq!(a.to_string(), "[2, 0; 0, 4]");
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[4, 2, 6; 2, 0, 4]").unwrap();
    /// a.mod_power_of_2_smith_normal_form_assign(3);
    /// assert_eq!(a.to_string(), "[2, 0, 0; 0, 2, 0]");
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// a.mod_power_of_2_smith_normal_form_assign(3);
    /// assert_eq!(a.to_string(), "[1, 0; 0, 2]");
    /// ```
    fn mod_power_of_2_smith_normal_form_assign(&mut self, pow: u64) {
        assert_arguments(self, pow);
        smith_eliminate_mod_power_of_2(
            &mut self.elements,
            self.height,
            self.width,
            pow,
            None,
            None,
        );
    }
}
