// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModPowerOf2IsReduced, ModPowerOf2Pow, ModPowerOf2PowAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::logic::traits::{BitAccess, SignificantBits};
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::mod_mul_matrix::ProductShape;
use crate::unsigned_matrix::arithmetic::mod_power_of_2_mul_matrix::matrix_mod_power_of_2_mul;
use alloc::vec;
use alloc::vec::Vec;

// Checks the arguments of a power modulo $2^k$: `pow` is at most the width of `T`, the matrix is
// square, and its elements are reduced.
fn assert_arguments<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, pow: u64) {
    assert!(
        pow <= T::WIDTH,
        "pow must be at most {}, but is {pow}",
        T::WIDTH
    );
    assert_eq!(
        a.height, a.width,
        "the matrix must be square, but it is {} by {}",
        a.height, a.width
    );
    assert!(
        a.mod_power_of_2_is_reduced(pow),
        "the matrix must be reduced mod 2^{pow}, but {a} has an element >= 2^{pow}"
    );
}

// Returns the elements of the `exp`th power of the `n` by `n` matrix with elements `xs`, which are
// reduced modulo $2^k$, modulo $2^k$, by left-to-right binary powering. It stops once the power is
// the zero matrix, since every further power is zero too.
crate_test_fn! {matrix_mod_power_of_2_pow<T: PrimitiveUnsigned>(
    xs: &[T],
    n: usize,
    exp: u64,
    pow: u64,
) -> Vec<T> {
    if exp == 0 {
        let mut out = vec![T::ZERO; n * n];
        let one = T::ONE.mod_power_of_2(pow);
        for x in out.iter_mut().step_by(n + 1) {
            *x = one;
        }
        return out;
    }
    if exp == 1 {
        return xs.to_vec();
    }
    let shape = ProductShape {
        ar: n,
        br: n,
        bc: n,
    };
    let top = exp.significant_bits() - 2;
    let mut t = Vec::new();
    for i in (0..=top).rev() {
        // The first squaring reads `xs` itself.
        let previous = if i == top { xs } else { &t };
        let u = matrix_mod_power_of_2_mul(previous, previous, shape, pow);
        t = if exp.get_bit(i) {
            matrix_mod_power_of_2_mul(&u, xs, shape, pow)
        } else {
            u
        };
        if t.iter().all(|&x| x == T::ZERO) {
            break;
        }
    }
    t
}}

impl<T: PrimitiveUnsigned> ModPowerOf2Pow<u64> for UnsignedMatrix<T> {
    type Output = Self;

    /// Raises a square [`UnsignedMatrix`] to a power modulo $2^k$, taking it by value.
    ///
    /// The elements of the matrix must already be reduced modulo $2^k$. The power $A^e$ of an $n
    /// \times n$ matrix $A$ is the product of $e$ copies of $A$, reduced modulo $2^k$; $A^0$ is the
    /// $n \times n$ identity matrix, reduced modulo $2^k$.
    ///
    /// $$
    /// f(A, e, k) = A^e \bmod 2^k.
    /// $$
    ///
    /// The power is computed by left-to-right binary powering, with each product reduced modulo
    /// $2^k$, so the elements never grow; it stops early if a power is the zero matrix. Each
    /// product is computed as by
    /// [`ModPowerOf2Mul`](crate::num::arithmetic::traits::ModPowerOf2Mul).
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n^3 \log e)$
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $e$ is
    /// `exp`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if the matrix is not square, or if any of its
    /// elements is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Pow;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!(a.mod_power_of_2_pow(10, 6).to_string(), "[25, 55; 55, 34]");
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_pow(4, 5).to_string(),
    ///     "[24, 25, 16; 2, 26, 25; 25, 9, 8]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.mod_power_of_2_pow(0, 3).to_string(), "[1, 0; 0, 1]");
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[0, 0; 0, 0]").unwrap();
    /// assert_eq!(a.mod_power_of_2_pow(0, 0).to_string(), "[0, 0; 0, 0]");
    /// ```
    fn mod_power_of_2_pow(self, exp: u64, pow: u64) -> Self {
        if exp == 1 {
            assert_arguments(&self, pow);
            self
        } else {
            (&self).mod_power_of_2_pow(exp, pow)
        }
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Pow<u64> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Raises a square [`UnsignedMatrix`] to a power modulo $2^k$, taking it by reference.
    ///
    /// See the documentation for the implementation that takes the matrix by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n^3 \log e)$
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $e$ is
    /// `exp`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if the matrix is not square, or if any of its
    /// elements is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Pow;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_pow(10, 6).to_string(),
    ///     "[25, 55; 55, 34]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_pow(4, 5).to_string(),
    ///     "[24, 25, 16; 2, 26, 25; 25, 9, 8]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_pow(0, 3).to_string(), "[1, 0; 0, 1]");
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[0, 0; 0, 0]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_pow(0, 0).to_string(), "[0, 0; 0, 0]");
    /// ```
    fn mod_power_of_2_pow(self, exp: u64, pow: u64) -> UnsignedMatrix<T> {
        assert_arguments(self, pow);
        UnsignedMatrix {
            elements: matrix_mod_power_of_2_pow(&self.elements, self.height, exp, pow),
            height: self.height,
            width: self.width,
        }
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2PowAssign<u64> for UnsignedMatrix<T> {
    /// Raises a square [`UnsignedMatrix`] to a power modulo $2^k$ in place.
    ///
    /// See the documentation for the implementation that takes the matrix by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n^3 \log e)$
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $e$ is
    /// `exp`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if the matrix is not square, or if any of its
    /// elements is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2PowAssign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[1, 1; 1, 0]").unwrap();
    /// a.mod_power_of_2_pow_assign(10, 6);
    /// assert_eq!(a.to_string(), "[25, 55; 55, 34]");
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// a.mod_power_of_2_pow_assign(4, 5);
    /// assert_eq!(a.to_string(), "[24, 25, 16; 2, 26, 25; 25, 9, 8]");
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// a.mod_power_of_2_pow_assign(0, 3);
    /// assert_eq!(a.to_string(), "[1, 0; 0, 1]");
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[0, 0; 0, 0]").unwrap();
    /// a.mod_power_of_2_pow_assign(0, 0);
    /// assert_eq!(a.to_string(), "[0, 0; 0, 0]");
    /// ```
    fn mod_power_of_2_pow_assign(&mut self, exp: u64, pow: u64) {
        assert_arguments(self, pow);
        if exp != 1 {
            self.elements = matrix_mod_power_of_2_pow(&self.elements, self.height, exp, pow);
        }
    }
}
