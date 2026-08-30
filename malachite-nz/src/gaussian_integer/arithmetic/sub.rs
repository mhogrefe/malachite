// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use core::ops::{Sub, SubAssign};

impl Sub<Self> for GaussianInteger {
    type Output = Self;

    /// Subtracts two [`GaussianInteger`]s, taking both by value.
    ///
    /// $$
    /// f(x, y) = x - y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianInteger::from_str("2-3i").unwrap();
    /// let y = GaussianInteger::from_str("-1+4i").unwrap();
    /// assert_eq!((x - y).to_string(), "3-7i");
    /// ```
    #[inline]
    fn sub(mut self, other: Self) -> Self {
        self -= other;
        self
    }
}

impl Sub<&Self> for GaussianInteger {
    type Output = Self;

    /// Subtracts two [`GaussianInteger`]s, taking the first by value and the second by reference.
    ///
    /// $$
    /// f(x, y) = x - y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianInteger::from_str("2-3i").unwrap();
    /// let y = GaussianInteger::from_str("-1+4i").unwrap();
    /// assert_eq!((x - &y).to_string(), "3-7i");
    /// ```
    #[inline]
    fn sub(mut self, other: &Self) -> Self {
        self -= other;
        self
    }
}

impl Sub<GaussianInteger> for &GaussianInteger {
    type Output = GaussianInteger;

    /// Subtracts two [`GaussianInteger`]s, taking the first by reference and the second by value.
    ///
    /// $$
    /// f(x, y) = x - y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianInteger::from_str("2-3i").unwrap();
    /// let y = GaussianInteger::from_str("-1+4i").unwrap();
    /// assert_eq!((&x - y).to_string(), "3-7i");
    /// ```
    #[inline]
    fn sub(self, other: GaussianInteger) -> GaussianInteger {
        GaussianInteger {
            real: &self.real - other.real,
            imaginary: &self.imaginary - other.imaginary,
        }
    }
}

impl Sub<&GaussianInteger> for &GaussianInteger {
    type Output = GaussianInteger;

    /// Subtracts two [`GaussianInteger`]s, taking both by reference.
    ///
    /// $$
    /// f(x, y) = x - y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianInteger::from_str("1000000000000+i").unwrap();
    /// let y = GaussianInteger::from_str("i").unwrap();
    /// assert_eq!((&x - &y).to_string(), "1000000000000");
    /// ```
    #[inline]
    fn sub(self, other: &GaussianInteger) -> GaussianInteger {
        GaussianInteger {
            real: &self.real - &other.real,
            imaginary: &self.imaginary - &other.imaginary,
        }
    }
}

impl SubAssign<Self> for GaussianInteger {
    /// Subtracts a [`GaussianInteger`] from a [`GaussianInteger`] in place, taking the
    /// [`GaussianInteger`] on the right-hand side by value.
    ///
    /// $$
    /// x \gets x - y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianInteger::from_str("2-3i").unwrap();
    /// let y = GaussianInteger::from_str("-1+4i").unwrap();
    /// let mut sum = x;
    /// sum -= y;
    /// assert_eq!(sum.to_string(), "3-7i");
    /// ```
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.real -= other.real;
        self.imaginary -= other.imaginary;
    }
}

impl SubAssign<&Self> for GaussianInteger {
    /// Subtracts a [`GaussianInteger`] from a [`GaussianInteger`] in place, taking the
    /// [`GaussianInteger`] on the right-hand side by reference.
    ///
    /// $$
    /// x \gets x - y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianInteger::from_str("2-3i").unwrap();
    /// let y = GaussianInteger::from_str("-1+4i").unwrap();
    /// let mut sum = x;
    /// sum -= &y;
    /// assert_eq!(sum.to_string(), "3-7i");
    /// ```
    #[inline]
    fn sub_assign(&mut self, other: &Self) {
        self.real -= &other.real;
        self.imaginary -= &other.imaginary;
    }
}
