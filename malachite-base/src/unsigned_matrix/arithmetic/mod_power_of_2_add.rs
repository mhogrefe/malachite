// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::matrix::Matrix;
use crate::num::arithmetic::traits::{ModPowerOf2Add, ModPowerOf2AddAssign, ModPowerOf2IsReduced};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;

fn assert_reduced<T: PrimitiveUnsigned>(v: &UnsignedMatrix<T>, w: &UnsignedMatrix<T>, pow: u64) {
    assert!(
        pow <= T::WIDTH,
        "pow must be at most T::WIDTH, but it is {pow}"
    );
    assert!(
        v.height == w.height && v.width == w.width,
        "cannot add matrices of different shapes"
    );
    assert!(
        v.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {v} has an element >= 2^{pow}"
    );
    assert!(
        w.mod_power_of_2_is_reduced(pow),
        "other must be reduced mod 2^pow, but {w} has an element >= 2^{pow}"
    );
}

impl<T: PrimitiveUnsigned> ModPowerOf2Add<Self> for UnsignedMatrix<T> {
    type Output = Self;

    /// Adds two [`UnsignedMatrix`]s modulo $2^k$, taking both by value. The elements of both must
    /// already be reduced modulo $2^k$.
    ///
    /// The sum is taken element by element, each reduced modulo $2^k$, so the result has the same
    /// shape as the summands.
    ///
    /// $$
    /// f(A, B, k) = A + B \bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if `self` and `other` have different shapes, or
    /// if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Add;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[5, 1; 3, 2]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[4, 7; 0, 6]").unwrap();
    /// assert_eq!(a.mod_power_of_2_add(b, 3).to_string(), "[1, 0; 3, 0]");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_add` from `nmod_vec/add.c`, FLINT 3.6.0, with the modulus
    /// $2^k$.
    #[inline]
    fn mod_power_of_2_add(mut self, other: Self, pow: u64) -> Self {
        self.mod_power_of_2_add_assign(other, pow);
        self
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Add<&Self> for UnsignedMatrix<T> {
    type Output = Self;

    /// Adds two [`UnsignedMatrix`]s modulo $2^k$, taking the first by value and the second by
    /// reference. The elements of both must already be reduced modulo $2^k$.
    ///
    /// The sum is taken element by element, each reduced modulo $2^k$, so the result has the same
    /// shape as the summands.
    ///
    /// $$
    /// f(A, B, k) = A + B \bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if `self` and `other` have different shapes, or
    /// if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Add;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[5, 1; 3, 2]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[4, 7; 0, 6]").unwrap();
    /// assert_eq!(a.mod_power_of_2_add(&b, 3).to_string(), "[1, 0; 3, 0]");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_add` from `nmod_vec/add.c`, FLINT 3.6.0, with the modulus
    /// $2^k$.
    #[inline]
    fn mod_power_of_2_add(mut self, other: &Self, pow: u64) -> Self {
        self.mod_power_of_2_add_assign(other, pow);
        self
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Add<UnsignedMatrix<T>> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Adds two [`UnsignedMatrix`]s modulo $2^k$, taking the first by reference and the second by
    /// value. The elements of both must already be reduced modulo $2^k$.
    ///
    /// The sum is taken element by element, each reduced modulo $2^k$, so the result has the same
    /// shape as the summands.
    ///
    /// $$
    /// f(A, B, k) = A + B \bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if `self` and `other` have different shapes, or
    /// if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Add;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[5, 1; 3, 2]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[4, 7; 0, 6]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_add(b, 3).to_string(), "[1, 0; 3, 0]");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_add` from `nmod_vec/add.c`, FLINT 3.6.0, with the modulus
    /// $2^k$.
    #[inline]
    fn mod_power_of_2_add(self, mut other: UnsignedMatrix<T>, pow: u64) -> UnsignedMatrix<T> {
        other.mod_power_of_2_add_assign(self, pow);
        other
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Add<&UnsignedMatrix<T>> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Adds two [`UnsignedMatrix`]s modulo $2^k$, taking both by reference. The elements of both
    /// must already be reduced modulo $2^k$.
    ///
    /// The sum is taken element by element, each reduced modulo $2^k$, so the result has the same
    /// shape as the summands.
    ///
    /// $$
    /// f(A, B, k) = A + B \bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if `self` and `other` have different shapes, or
    /// if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Add;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[5, 1; 3, 2]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[4, 7; 0, 6]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_add(&b, 3).to_string(), "[1, 0; 3, 0]");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_add` from `nmod_vec/add.c`, FLINT 3.6.0, with the modulus
    /// $2^k$.
    fn mod_power_of_2_add(self, other: &UnsignedMatrix<T>, pow: u64) -> UnsignedMatrix<T> {
        assert_reduced(self, other, pow);
        UnsignedMatrix::from_owned_elements(
            self.height(),
            self.width(),
            self.elements
                .iter()
                .zip(&other.elements)
                .map(|(&x, &y)| x.wrapping_add(y).mod_power_of_2(pow))
                .collect(),
        )
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2AddAssign<Self> for UnsignedMatrix<T> {
    /// Adds an [`UnsignedMatrix`] to an [`UnsignedMatrix`] modulo $2^k$, in place, taking the
    /// [`UnsignedMatrix`] on the right-hand side by value. The elements of both must already be
    /// reduced modulo $2^k$.
    ///
    /// The sum is taken element by element, each reduced modulo $2^k$, so the result has the same
    /// shape as the summands.
    ///
    /// $$
    /// f(A, B, k) = A + B \bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if `self` and `other` have different shapes, or
    /// if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddAssign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[5, 1; 3, 2]").unwrap();
    /// a.mod_power_of_2_add_assign(UnsignedMatrix::<u8>::from_str("[4, 7; 0, 6]").unwrap(), 3);
    /// assert_eq!(a.to_string(), "[1, 0; 3, 0]");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_add` from `nmod_vec/add.c`, FLINT 3.6.0, with the modulus
    /// $2^k$.
    fn mod_power_of_2_add_assign(&mut self, other: Self, pow: u64) {
        assert_reduced(self, &other, pow);
        for (x, y) in self.elements.iter_mut().zip(other.elements) {
            *x = x.wrapping_add(y).mod_power_of_2(pow);
        }
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2AddAssign<&Self> for UnsignedMatrix<T> {
    /// Adds an [`UnsignedMatrix`] to an [`UnsignedMatrix`] modulo $2^k$, in place, taking the
    /// [`UnsignedMatrix`] on the right-hand side by reference. The elements of both must already be
    /// reduced modulo $2^k$.
    ///
    /// The sum is taken element by element, each reduced modulo $2^k$, so the result has the same
    /// shape as the summands.
    ///
    /// $$
    /// f(A, B, k) = A + B \bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if `self` and `other` have different shapes, or
    /// if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddAssign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[5, 1; 3, 2]").unwrap();
    /// a.mod_power_of_2_add_assign(&UnsignedMatrix::<u8>::from_str("[4, 7; 0, 6]").unwrap(), 3);
    /// assert_eq!(a.to_string(), "[1, 0; 3, 0]");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_add` from `nmod_vec/add.c`, FLINT 3.6.0, with the modulus
    /// $2^k$.
    fn mod_power_of_2_add_assign(&mut self, other: &Self, pow: u64) {
        assert_reduced(self, other, pow);
        for (x, y) in self.elements.iter_mut().zip(&other.elements) {
            *x = x.wrapping_add(*y).mod_power_of_2(pow);
        }
    }
}
