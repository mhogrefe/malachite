// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModIsReduced, ModPow, ModPowAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::logic::traits::{BitAccess, SignificantBits};
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::mod_mul_matrix::{ProductShape, matrix_mod_mul};
use alloc::vec;
use alloc::vec::Vec;

// Checks the arguments of a power modulo `m`: `m` is positive, the matrix is square, and its
// elements are reduced.
fn assert_arguments<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, m: T) {
    assert_ne!(m, T::ZERO, "m cannot be zero");
    assert_eq!(
        a.height, a.width,
        "the matrix must be square, but it is {} by {}",
        a.height, a.width
    );
    assert!(
        a.mod_is_reduced(&m),
        "the matrix must be reduced mod m, but {a} has an element >= {m}"
    );
}

// Returns the elements of the `exp`th power of the `n` by `n` matrix with elements `xs`, which are
// reduced modulo `m`, modulo `m`, by left-to-right binary powering. It stops once the power is the
// zero matrix, since every further power is zero too.
//
// `nmod_mat_pow` from `nmod_mat/pow.c`, FLINT 3.6.0, powers right to left instead; both square
// $\lfloor \log_2 e \rfloor$ times, but left to right every other product is by the matrix itself,
// and the first squaring reads `xs` without copying it.
fn matrix_mod_pow<T: PrimitiveUnsigned>(xs: &[T], n: usize, exp: u64, m: T) -> Vec<T> {
    if exp == 0 {
        let mut out = vec![T::ZERO; n * n];
        let one = T::ONE % m;
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
        let u = matrix_mod_mul(previous, previous, shape, m);
        t = if exp.get_bit(i) {
            matrix_mod_mul(&u, xs, shape, m)
        } else {
            u
        };
        if t.iter().all(|&x| x == T::ZERO) {
            break;
        }
    }
    t
}

impl<T: PrimitiveUnsigned> ModPow<u64, T> for UnsignedMatrix<T> {
    type Output = Self;

    /// Raises a square [`UnsignedMatrix`] to a power modulo $m$, taking it by value.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The power $A^e$ of an $n
    /// \times n$ matrix $A$ is the product of $e$ copies of $A$, reduced modulo $m$; $A^0$ is the
    /// $n \times n$ identity matrix, reduced modulo $m$.
    ///
    /// $$
    /// f(A, e, m) = A^e \bmod m.
    /// $$
    ///
    /// The power is computed by left-to-right binary powering, with each product reduced modulo
    /// $m$, so the elements never grow; it stops early if a power is the zero matrix. Each product
    /// is computed as by [`ModMul`](crate::num::arithmetic::traits::ModMul).
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
    /// Panics if `m` is zero, if the matrix is not square, or if any of its elements is greater
    /// than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPow;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!(a.mod_pow(10, 100u8).to_string(), "[89, 55; 55, 34]");
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// assert_eq!(a.mod_pow(4, 7u8).to_string(), "[3, 4, 2; 3, 6, 4; 4, 6, 1]");
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.mod_pow(0, 5u8).to_string(), "[1, 0; 0, 1]");
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[0, 0; 0, 0]").unwrap();
    /// assert_eq!(a.mod_pow(0, 1u8).to_string(), "[0, 0; 0, 0]");
    ///
    /// let a = UnsignedMatrix::<u32>::from_str("[4294967290, 2147483648; 5, 4294967289]").unwrap();
    /// assert_eq!(
    ///     a.mod_pow(5, 4294967291u32).to_string(),
    ///     "[1073740403, 1610613921; 3221227842, 2147481751]"
    /// );
    /// ```
    fn mod_pow(self, exp: u64, m: T) -> Self {
        if exp == 1 {
            assert_arguments(&self, m);
            self
        } else {
            (&self).mod_pow(exp, m)
        }
    }
}

impl<T: PrimitiveUnsigned> ModPow<u64, T> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Raises a square [`UnsignedMatrix`] to a power modulo $m$, taking it by reference.
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
    /// Panics if `m` is zero, if the matrix is not square, or if any of its elements is greater
    /// than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPow;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!((&a).mod_pow(10, 100u8).to_string(), "[89, 55; 55, 34]");
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_pow(4, 7u8).to_string(),
    ///     "[3, 4, 2; 3, 6, 4; 4, 6, 1]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).mod_pow(0, 5u8).to_string(), "[1, 0; 0, 1]");
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[0, 0; 0, 0]").unwrap();
    /// assert_eq!((&a).mod_pow(0, 1u8).to_string(), "[0, 0; 0, 0]");
    ///
    /// let a = UnsignedMatrix::<u32>::from_str("[4294967290, 2147483648; 5, 4294967289]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_pow(5, 4294967291u32).to_string(),
    ///     "[1073740403, 1610613921; 3221227842, 2147481751]"
    /// );
    /// ```
    fn mod_pow(self, exp: u64, m: T) -> UnsignedMatrix<T> {
        assert_arguments(self, m);
        UnsignedMatrix {
            elements: matrix_mod_pow(&self.elements, self.height, exp, m),
            height: self.height,
            width: self.width,
        }
    }
}

impl<T: PrimitiveUnsigned> ModPowAssign<u64, T> for UnsignedMatrix<T> {
    /// Raises a square [`UnsignedMatrix`] to a power modulo $m$ in place.
    ///
    /// See the documentation for the [`ModPow`] implementation that takes the matrix by value for
    /// details.
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
    /// Panics if `m` is zero, if the matrix is not square, or if any of its elements is greater
    /// than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowAssign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[1, 1; 1, 0]").unwrap();
    /// a.mod_pow_assign(10, 100u8);
    /// assert_eq!(a.to_string(), "[89, 55; 55, 34]");
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// a.mod_pow_assign(4, 7u8);
    /// assert_eq!(a.to_string(), "[3, 4, 2; 3, 6, 4; 4, 6, 1]");
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// a.mod_pow_assign(0, 5u8);
    /// assert_eq!(a.to_string(), "[1, 0; 0, 1]");
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[0, 0; 0, 0]").unwrap();
    /// a.mod_pow_assign(0, 1u8);
    /// assert_eq!(a.to_string(), "[0, 0; 0, 0]");
    ///
    /// let mut a =
    ///     UnsignedMatrix::<u32>::from_str("[4294967290, 2147483648; 5, 4294967289]").unwrap();
    /// a.mod_pow_assign(5, 4294967291u32);
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[1073740403, 1610613921; 3221227842, 2147481751]"
    /// );
    /// ```
    fn mod_pow_assign(&mut self, exp: u64, m: T) {
        assert_arguments(self, m);
        if exp != 1 {
            self.elements = matrix_mod_pow(&self.elements, self.height, exp, m);
        }
    }
}
