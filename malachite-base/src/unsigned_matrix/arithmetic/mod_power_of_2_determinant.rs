// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModPowerOf2Determinant, ModPowerOf2IsReduced};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::logic::traits::TrailingZeros;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::mod_strong_echelon_form::{swap_rows, two_rows};

// The determinant modulo 2^pow of the n by n matrix with elements `xs`, which are reduced modulo
// 2^pow, by elimination over the chain ring Z/2^pow: the pivot of each column is an element of
// smallest 2-adic valuation, which divides the others below it, so subtracting multiples of the
// pivot row clears the column without changing the determinant. The determinant is the product of
// the pivots, negated for each row swap; it is 0 once the valuations of the pivots add up to pow.
pub(crate) fn determinant_mod_power_of_2<T: PrimitiveUnsigned>(
    xs: &mut [T],
    n: usize,
    pow: u64,
) -> T {
    let mut det = T::ONE.mod_power_of_2(pow);
    let mut valuation = 0;
    for col in 0..n {
        let Some(best) = (col..n)
            .filter(|&i| xs[i * n + col] != T::ZERO)
            .min_by_key(|&i| TrailingZeros::trailing_zeros(xs[i * n + col]))
        else {
            return T::ZERO;
        };
        if best != col {
            swap_rows(xs, n, col, best);
            det.mod_power_of_2_neg_assign(pow);
        }
        let p = xs[col * n + col];
        let v = TrailingZeros::trailing_zeros(p);
        valuation += v;
        if valuation >= pow {
            return T::ZERO;
        }
        det.mod_power_of_2_mul_assign(p, pow);
        let u_inverse = (p >> v).mod_power_of_2_inverse(pow).unwrap();
        for i in col + 1..n {
            let (pivot_row, other_row) = two_rows(xs, n, col, i);
            let b = other_row[col];
            if b != T::ZERO {
                let s = (b >> v).mod_power_of_2_mul(u_inverse, pow);
                for (y, &x) in other_row[col..].iter_mut().zip(&pivot_row[col..]) {
                    *y = y.mod_power_of_2_sub(s.mod_power_of_2_mul(x, pow), pow);
                }
            }
        }
    }
    det
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

impl<T: PrimitiveUnsigned> ModPowerOf2Determinant for UnsignedMatrix<T> {
    type Output = T;
    /// Computes the determinant of a square [`UnsignedMatrix`] modulo $2^k$, taking it by value.
    ///
    /// The elements of the matrix must already be reduced modulo $2^k$. The determinant is found by
    /// elimination over the chain ring $\mathbb{Z}/2^k\mathbb{Z}$: the pivot of each column is an
    /// element of smallest 2-adic valuation, which divides the others below it, so subtracting
    /// multiples of the pivot row clears the column without changing the determinant. The
    /// determinant is the product of the pivots, negated for each row swap; it is 0 once the
    /// valuations of the pivots add up to $k$.
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
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Determinant;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.mod_power_of_2_determinant(3).to_string(), "6");
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 5]").unwrap();
    /// assert_eq!(a.mod_power_of_2_determinant(3).to_string(), "7");
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 4; 6, 0]").unwrap();
    /// assert_eq!(a.mod_power_of_2_determinant(3).to_string(), "0");
    /// ```
    #[inline]
    fn mod_power_of_2_determinant(mut self, pow: u64) -> T {
        assert_arguments(&self, pow);
        determinant_mod_power_of_2(&mut self.elements, self.height, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Determinant for &UnsignedMatrix<T> {
    type Output = T;
    /// Computes the determinant of a square [`UnsignedMatrix`] modulo $2^k$, taking it by
    /// reference.
    ///
    /// The elements of the matrix must already be reduced modulo $2^k$. The determinant is found by
    /// elimination over the chain ring $\mathbb{Z}/2^k\mathbb{Z}$: the pivot of each column is an
    /// element of smallest 2-adic valuation, which divides the others below it, so subtracting
    /// multiples of the pivot row clears the column without changing the determinant. The
    /// determinant is the product of the pivots, negated for each row swap; it is 0 once the
    /// valuations of the pivots add up to $k$.
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
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Determinant;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_determinant(3).to_string(), "6");
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 5]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_determinant(3).to_string(), "7");
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 4; 6, 0]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_determinant(3).to_string(), "0");
    /// ```
    #[inline]
    fn mod_power_of_2_determinant(self, pow: u64) -> T {
        self.clone().mod_power_of_2_determinant(pow)
    }
}
