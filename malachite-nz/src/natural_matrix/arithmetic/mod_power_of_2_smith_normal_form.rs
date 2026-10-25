// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::natural_matrix::arithmetic::mod_strong_echelon_form::{
    swap_rows, to_limb_matrix, two_rows,
};
use crate::platform::Limb;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Inverse, ModPowerOf2IsReduced, ModPowerOf2Mul, ModPowerOf2MulAssign,
    ModPowerOf2SmithNormalForm, ModPowerOf2SmithNormalFormAssign, ModPowerOf2SubAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;

// Swaps columns `i` and `j` of the `width`-wide matrix with elements `xs`.
pub(crate) fn swap_columns(xs: &mut [Natural], width: usize, i: usize, j: usize) {
    if i != j {
        for row in xs.chunks_exact_mut(width) {
            row.swap(i, j);
        }
    }
}

// Replaces `ys` with `ys` - s·`xs` modulo 2^pow.
fn sub_mul_row(ys: &mut [Natural], s: &Natural, xs: &[Natural], pow: u64) {
    for (y, x) in ys.iter_mut().zip(xs) {
        y.mod_power_of_2_sub_assign(s.mod_power_of_2_mul(x, pow), pow);
    }
}

// Reduces the `height` by `width` matrix with elements `xs`, which are reduced modulo 2^pow, to the
// diagonal matrix diag(2^v_0, ..., 2^v_(r-1), 0, ...) with v_0 <= ... <= v_(r-1) by invertible row
// and column operations over Z/2^pow, as for `UnsignedMatrix`s, applying the row operations to the
// `rhs_width`-wide matrix with elements `rhs`, if it is present, and the column operations to the
// `width` by `width` matrix with elements `v`, if it is present. Returns (v_0, ..., v_(r-1)).
pub(crate) fn smith_eliminate_mod_power_of_2(
    xs: &mut [Natural],
    height: usize,
    width: usize,
    pow: u64,
    mut rhs: Option<(&mut [Natural], usize)>,
    mut v: Option<&mut [Natural]>,
) -> Vec<u64> {
    let (h, w) = (height, width);
    let mut valuations = Vec::new();
    for t in 0..h.min(w) {
        let Some((val, i, j)) = (t..h)
            .flat_map(|i| (t..w).map(move |j| (i, j)))
            .filter_map(|(i, j)| xs[i * w + j].trailing_zeros().map(|val| (val, i, j)))
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
        let u_inverse = (&xs[t * w + t] >> val).mod_power_of_2_inverse(pow).unwrap();
        for x in &mut xs[t * w + t..(t + 1) * w] {
            x.mod_power_of_2_mul_assign(&u_inverse, pow);
        }
        if let Some((rhs, rhs_width)) = rhs.as_mut() {
            for x in &mut rhs[t * *rhs_width..(t + 1) * *rhs_width] {
                x.mod_power_of_2_mul_assign(&u_inverse, pow);
            }
        }
        // Clear the column below the pivot with row operations.
        for i in t + 1..h {
            let s = &xs[i * w + t] >> val;
            if s != 0u32 {
                let (pivot_row, other_row) = two_rows(xs, w, t, i);
                sub_mul_row(&mut other_row[t..], &s, &pivot_row[t..], pow);
                if let Some((rhs, rhs_width)) = rhs.as_mut() {
                    let (pivot_row, other_row) = two_rows(rhs, *rhs_width, t, i);
                    sub_mul_row(other_row, &s, pivot_row, pow);
                }
            }
        }
        // Clear the row to the right of the pivot with column operations; only row t changes.
        for k in t + 1..w {
            let s = &xs[t * w + k] >> val;
            if s != 0u32 {
                xs[t * w + k] = Natural::ZERO;
                if let Some(v) = v.as_deref_mut() {
                    for row in v.chunks_exact_mut(w) {
                        let x = (&s).mod_power_of_2_mul(&row[t], pow);
                        row[k].mod_power_of_2_sub_assign(x, pow);
                    }
                }
            }
        }
        valuations.push(val);
    }
    valuations
}

fn mod_power_of_2_smith_normal_form_assign_helper(a: &mut NaturalMatrix, pow: u64) {
    assert!(
        a.mod_power_of_2_is_reduced(pow),
        "the matrix must be reduced mod 2^{pow}, but {a} has an element >= 2^{pow}"
    );
    if pow <= Limb::WIDTH {
        *a = NaturalMatrix::from(to_limb_matrix(a).mod_power_of_2_smith_normal_form(pow));
    } else {
        let (h, w) = (a.height, a.width);
        smith_eliminate_mod_power_of_2(&mut a.elements, h, w, pow, None, None);
    }
}

impl ModPowerOf2SmithNormalForm for NaturalMatrix {
    type Output = Self;
    /// Computes the Smith normal form of an [`NaturalMatrix`] modulo $2^k$, taking it by value.
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
    /// $T(h, w, n) = O(hw\min(h, w) n \log n \log\log n)$
    ///
    /// $M(h, w, n) = O(hwn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if any element of the matrix is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SmithNormalForm;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[2, 4; 6, 0]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_smith_normal_form(3).to_string(),
    ///     "[2, 0; 0, 4]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[4, 2, 6; 2, 0, 4]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_smith_normal_form(3).to_string(),
    ///     "[2, 0, 0; 0, 2, 0]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_smith_normal_form(3).to_string(),
    ///     "[1, 0; 0, 2]"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_smith_normal_form(mut self, pow: u64) -> Self {
        mod_power_of_2_smith_normal_form_assign_helper(&mut self, pow);
        self
    }
}

impl ModPowerOf2SmithNormalForm for &NaturalMatrix {
    type Output = NaturalMatrix;
    /// Computes the Smith normal form of an [`NaturalMatrix`] modulo $2^k$, taking it by reference.
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
    /// $T(h, w, n) = O(hw\min(h, w) n \log n \log\log n)$
    ///
    /// $M(h, w, n) = O(hwn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if any element of the matrix is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SmithNormalForm;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[2, 4; 6, 0]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_smith_normal_form(3).to_string(),
    ///     "[2, 0; 0, 4]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[4, 2, 6; 2, 0, 4]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_smith_normal_form(3).to_string(),
    ///     "[2, 0, 0; 0, 2, 0]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_smith_normal_form(3).to_string(),
    ///     "[1, 0; 0, 2]"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_smith_normal_form(self, pow: u64) -> NaturalMatrix {
        self.clone().mod_power_of_2_smith_normal_form(pow)
    }
}

impl ModPowerOf2SmithNormalFormAssign for NaturalMatrix {
    /// Replaces an [`NaturalMatrix`] with its Smith normal form modulo $2^k$.
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
    /// $T(h, w, n) = O(hw\min(h, w) n \log n \log\log n)$
    ///
    /// $M(h, w, n) = O(hwn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if any element of the matrix is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SmithNormalFormAssign;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[2, 4; 6, 0]").unwrap();
    /// a.mod_power_of_2_smith_normal_form_assign(3);
    /// assert_eq!(a.to_string(), "[2, 0; 0, 4]");
    ///
    /// let mut a = NaturalMatrix::from_str("[4, 2, 6; 2, 0, 4]").unwrap();
    /// a.mod_power_of_2_smith_normal_form_assign(3);
    /// assert_eq!(a.to_string(), "[2, 0, 0; 0, 2, 0]");
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// a.mod_power_of_2_smith_normal_form_assign(3);
    /// assert_eq!(a.to_string(), "[1, 0; 0, 2]");
    /// ```
    #[inline]
    fn mod_power_of_2_smith_normal_form_assign(&mut self, pow: u64) {
        mod_power_of_2_smith_normal_form_assign_helper(self, pow);
    }
}
