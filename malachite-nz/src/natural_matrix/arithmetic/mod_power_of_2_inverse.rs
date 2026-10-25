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
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Inverse, ModPowerOf2IsReduced, ModPowerOf2Mul, ModPowerOf2MulAssign,
    ModPowerOf2SubAssign, Parity,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};

// The pivot row `pivot` and another row `other`, in either order, of the `width`-wide matrix with
// elements `xs`.
fn pivot_and_other_row(
    xs: &mut [Natural],
    width: usize,
    pivot: usize,
    other: usize,
) -> (&mut [Natural], &mut [Natural]) {
    if pivot < other {
        two_rows(xs, width, pivot, other)
    } else {
        let (other_row, pivot_row) = two_rows(xs, width, other, pivot);
        (pivot_row, other_row)
    }
}

// Replaces `ys` with `ys` - s·`xs` modulo 2^pow.
fn sub_mul_row(ys: &mut [Natural], s: &Natural, xs: &[Natural], pow: u64) {
    for (y, x) in ys.iter_mut().zip(xs) {
        y.mod_power_of_2_sub_assign(s.mod_power_of_2_mul(x, pow), pow);
    }
}

// The inverse modulo 2^pow of the n by n matrix with elements `xs`, which are reduced modulo 2^pow,
// or `None` if there is none, by Gauss-Jordan elimination with odd pivots, as for
// `UnsignedMatrix`s.
fn inverse_mod_power_of_2(mut xs: Vec<Natural>, n: usize, pow: u64) -> Option<Vec<Natural>> {
    let mut inv = vec![Natural::ZERO; n * n];
    for i in 0..n {
        inv[i * (n + 1)] = Natural::ONE;
    }
    for col in 0..n {
        let best = (col..n).find(|&i| xs[i * n + col].odd())?;
        swap_rows(&mut xs, n, col, best);
        swap_rows(&mut inv, n, col, best);
        let u_inverse = (&xs[col * n + col]).mod_power_of_2_inverse(pow).unwrap();
        for x in &mut xs[col * n..(col + 1) * n] {
            x.mod_power_of_2_mul_assign(&u_inverse, pow);
        }
        for x in &mut inv[col * n..(col + 1) * n] {
            x.mod_power_of_2_mul_assign(&u_inverse, pow);
        }
        for i in 0..n {
            if i == col || xs[i * n + col] == 0u32 {
                continue;
            }
            let s = xs[i * n + col].clone();
            let (pivot_row, other_row) = pivot_and_other_row(&mut xs, n, col, i);
            sub_mul_row(other_row, &s, pivot_row, pow);
            let (pivot_row, other_row) = pivot_and_other_row(&mut inv, n, col, i);
            sub_mul_row(other_row, &s, pivot_row, pow);
        }
    }
    Some(inv)
}

fn inverse_helper(a: NaturalMatrix, pow: u64) -> Option<NaturalMatrix> {
    assert_eq!(a.height, a.width, "the matrix must be square");
    assert!(
        a.mod_power_of_2_is_reduced(pow),
        "the matrix must be reduced mod 2^{pow}, but {a} has an element >= 2^{pow}"
    );
    if pow <= Limb::WIDTH {
        to_limb_matrix(&a)
            .mod_power_of_2_inverse(pow)
            .map(NaturalMatrix::from)
    } else {
        let n = a.height;
        inverse_mod_power_of_2(a.elements, n, pow).map(|elements| NaturalMatrix {
            elements,
            height: n,
            width: n,
        })
    }
}

impl ModPowerOf2Inverse for NaturalMatrix {
    type Output = Self;
    /// Computes the inverse of a square [`NaturalMatrix`] modulo $2^k$, taking it by value.
    ///
    /// The elements of the matrix must already be reduced modulo $2^k$. The inverse of an $n \times
    /// n$ matrix $A$ is the matrix $B$ with $AB = BA = I$ modulo $2^k$; it exists if and only if
    /// the determinant of $A$ is odd, and `None` is returned otherwise. Modulo 1, every matrix is
    /// the zero matrix, which is both the identity and its own inverse.
    ///
    /// The inverse is found by Gauss-Jordan elimination with odd pivots: an invertible matrix is
    /// invertible modulo 2, so each column has an odd element, a unit, in or below the diagonal
    /// when it is reached.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^3 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $m$ is
    /// `pow`.
    ///
    /// # Panics
    /// Panics if any element of the matrix is greater than or equal to $2^k$, or if the matrix is
    /// not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Inverse;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 5]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_inverse(3).to_debug_string(),
    ///     "Some([3, 2; 3, 7])"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[3, 1; 2, 1]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_inverse(8).to_debug_string(),
    ///     "Some([1, 255; 254, 3])"
    /// );
    ///
    /// // The determinant is even
    /// let a = NaturalMatrix::from_str("[2, 4; 6, 0]").unwrap();
    /// assert_eq!(a.mod_power_of_2_inverse(3).to_debug_string(), "None");
    /// ```
    #[inline]
    fn mod_power_of_2_inverse(self, pow: u64) -> Option<Self> {
        inverse_helper(self, pow)
    }
}

impl ModPowerOf2Inverse for &NaturalMatrix {
    type Output = NaturalMatrix;
    /// Computes the inverse of a square [`NaturalMatrix`] modulo $2^k$, taking it by reference.
    ///
    /// The elements of the matrix must already be reduced modulo $2^k$. The inverse of an $n \times
    /// n$ matrix $A$ is the matrix $B$ with $AB = BA = I$ modulo $2^k$; it exists if and only if
    /// the determinant of $A$ is odd, and `None` is returned otherwise. Modulo 1, every matrix is
    /// the zero matrix, which is both the identity and its own inverse.
    ///
    /// The inverse is found by Gauss-Jordan elimination with odd pivots: an invertible matrix is
    /// invertible modulo 2, so each column has an odd element, a unit, in or below the diagonal
    /// when it is reached.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^3 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $m$ is
    /// `pow`.
    ///
    /// # Panics
    /// Panics if any element of the matrix is greater than or equal to $2^k$, or if the matrix is
    /// not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Inverse;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 5]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_inverse(3).to_debug_string(),
    ///     "Some([3, 2; 3, 7])"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[3, 1; 2, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_inverse(8).to_debug_string(),
    ///     "Some([1, 255; 254, 3])"
    /// );
    ///
    /// // The determinant is even
    /// let a = NaturalMatrix::from_str("[2, 4; 6, 0]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_inverse(3).to_debug_string(), "None");
    /// ```
    #[inline]
    fn mod_power_of_2_inverse(self, pow: u64) -> Option<NaturalMatrix> {
        inverse_helper(self.clone(), pow)
    }
}
