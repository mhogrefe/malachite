// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::matrix::Matrix;
use crate::num::arithmetic::traits::{ModPowerOf2Inverse, ModPowerOf2IsReduced};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::mod_strong_echelon_form::{swap_rows, two_rows};
use alloc::vec;
use alloc::vec::Vec;

// The pivot row `pivot` and another row `other`, in either order, of the `width`-wide matrix with
// elements `xs`.
fn pivot_and_other_row<T>(
    xs: &mut [T],
    width: usize,
    pivot: usize,
    other: usize,
) -> (&mut [T], &mut [T]) {
    if pivot < other {
        two_rows(xs, width, pivot, other)
    } else {
        let (other_row, pivot_row) = two_rows(xs, width, other, pivot);
        (pivot_row, other_row)
    }
}

// Replaces `ys` with `ys` - s·`xs` modulo 2^pow.
fn sub_mul_row<T: PrimitiveUnsigned>(ys: &mut [T], s: T, xs: &[T], pow: u64) {
    for (y, &x) in ys.iter_mut().zip(xs) {
        *y = y.mod_power_of_2_sub(s.mod_power_of_2_mul(x, pow), pow);
    }
}

// The inverse modulo 2^pow of the n by n matrix with elements `xs`, which are reduced modulo 2^pow,
// or `None` if there is none, by Gauss-Jordan elimination with odd pivots: a matrix is invertible
// modulo 2^pow exactly when its determinant is odd, that is, when it is invertible modulo 2, and
// then each column has an odd element, a unit, in or below the diagonal when it is reached.
pub(crate) fn inverse_mod_power_of_2<T: PrimitiveUnsigned>(
    mut xs: Vec<T>,
    n: usize,
    pow: u64,
) -> Option<Vec<T>> {
    let mut inv = vec![T::ZERO; n * n];
    if pow == 0 {
        // Modulo 1, every matrix is the zero matrix, which is the identity.
        return Some(inv);
    }
    for i in 0..n {
        inv[i * (n + 1)] = T::ONE;
    }
    for col in 0..n {
        let best = (col..n).find(|&i| xs[i * n + col].odd())?;
        swap_rows(&mut xs, n, col, best);
        swap_rows(&mut inv, n, col, best);
        let u_inverse = xs[col * n + col].mod_power_of_2_inverse(pow).unwrap();
        for x in &mut xs[col * n..(col + 1) * n] {
            *x = x.mod_power_of_2_mul(u_inverse, pow);
        }
        for x in &mut inv[col * n..(col + 1) * n] {
            *x = x.mod_power_of_2_mul(u_inverse, pow);
        }
        for i in 0..n {
            if i == col {
                continue;
            }
            let s = xs[i * n + col];
            if s != T::ZERO {
                let (pivot_row, other_row) = pivot_and_other_row(&mut xs, n, col, i);
                sub_mul_row(other_row, s, pivot_row, pow);
                let (pivot_row, other_row) = pivot_and_other_row(&mut inv, n, col, i);
                sub_mul_row(other_row, s, pivot_row, pow);
            }
        }
    }
    Some(inv)
}

fn assert_arguments<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, pow: u64) {
    assert!(
        pow <= T::WIDTH,
        "pow must be at most the width of the element type, {}, but it is {pow}",
        T::WIDTH
    );
    assert_eq!(a.height, a.width, "the matrix must be square");
    assert!(
        a.mod_power_of_2_is_reduced(pow),
        "the matrix must be reduced mod 2^{pow}, but {a} has an element >= 2^{pow}"
    );
}

impl<T: PrimitiveUnsigned> ModPowerOf2Inverse for UnsignedMatrix<T> {
    type Output = Self;
    /// Computes the inverse of a square [`UnsignedMatrix`] modulo $2^k$, taking it by value.
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
    /// $T(n) = O(n^3)$
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the dimension of the matrix.
    ///
    /// # Panics
    /// Panics if `pow` is greater than the width of `T`, or if any element of the matrix is greater
    /// than or equal to $2^k$, or if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Inverse;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 5]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_inverse(3).to_debug_string(),
    ///     "Some([3, 2; 3, 7])"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[3, 1; 2, 1]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_inverse(8).to_debug_string(),
    ///     "Some([1, 255; 254, 3])"
    /// );
    ///
    /// // The determinant is even
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 4; 6, 0]").unwrap();
    /// assert_eq!(a.mod_power_of_2_inverse(3).to_debug_string(), "None");
    /// ```
    fn mod_power_of_2_inverse(self, pow: u64) -> Option<Self> {
        assert_arguments(&self, pow);
        let n = self.height;
        inverse_mod_power_of_2(self.elements, n, pow)
            .map(|xs| Self::from_owned_elements(u64::exact_from(n), u64::exact_from(n), xs))
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Inverse for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;
    /// Computes the inverse of a square [`UnsignedMatrix`] modulo $2^k$, taking it by reference.
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
    /// $T(n) = O(n^3)$
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the dimension of the matrix.
    ///
    /// # Panics
    /// Panics if `pow` is greater than the width of `T`, or if any element of the matrix is greater
    /// than or equal to $2^k$, or if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Inverse;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 5]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_inverse(3).to_debug_string(),
    ///     "Some([3, 2; 3, 7])"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[3, 1; 2, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_inverse(8).to_debug_string(),
    ///     "Some([1, 255; 254, 3])"
    /// );
    ///
    /// // The determinant is even
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 4; 6, 0]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_inverse(3).to_debug_string(), "None");
    /// ```
    #[inline]
    fn mod_power_of_2_inverse(self, pow: u64) -> Option<UnsignedMatrix<T>> {
        self.clone().mod_power_of_2_inverse(pow)
    }
}
