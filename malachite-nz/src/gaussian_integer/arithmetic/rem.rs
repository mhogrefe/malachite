// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use crate::gaussian_integer::arithmetic::div_rem::{div_rem_ref_ref, div_rem_val_ref};
use core::mem::take;
use core::ops::{Rem, RemAssign};

impl Rem<Self> for GaussianInteger {
    type Output = Self;

    /// Divides a [`GaussianInteger`] by another [`GaussianInteger`], taking both by value, and
    /// returns the remainder.
    ///
    /// The quotient (which is not returned) is the Gaussian integer nearest to the exact quotient,
    /// with each part rounded to the nearest integer and ties rounded up, and the remainder is what
    /// is left over. The quotient and remainder satisfy $x = qy + r$ and $N(r) \leq N(y) / 2$,
    /// where $N$ is the norm. To get both at once, use
    /// [`div_rem`](malachite_base::num::arithmetic::traits::DivRem::div_rem).
    ///
    /// $$
    /// f(x, y) = x - qy, \quad \text{where } q = \left \lfloor \frac{x \bar{y}}{N(y)} +
    /// \frac{1 + i}{2} \right \rfloor
    /// $$
    /// and the floor is taken on each part.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // (2+i)(3) + (-1) = 5+3i
    /// let x = GaussianInteger::from_str("5+3i").unwrap();
    /// let y = GaussianInteger::from_str("2+i").unwrap();
    /// assert_eq!((x % y).to_string(), "-1");
    /// ```
    #[inline]
    fn rem(self, other: Self) -> Self {
        div_rem_val_ref(self, &other).1
    }
}

impl Rem<&Self> for GaussianInteger {
    type Output = Self;

    /// Divides a [`GaussianInteger`] by another [`GaussianInteger`], taking the first by value and
    /// the second by reference, and returns the remainder.
    ///
    /// The quotient (which is not returned) is the Gaussian integer nearest to the exact quotient,
    /// with each part rounded to the nearest integer and ties rounded up, and the remainder is what
    /// is left over. The quotient and remainder satisfy $x = qy + r$ and $N(r) \leq N(y) / 2$,
    /// where $N$ is the norm. To get both at once, use
    /// [`div_rem`](malachite_base::num::arithmetic::traits::DivRem::div_rem).
    ///
    /// $$
    /// f(x, y) = x - qy, \quad \text{where } q = \left \lfloor \frac{x \bar{y}}{N(y)} +
    /// \frac{1 + i}{2} \right \rfloor
    /// $$
    /// and the floor is taken on each part.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // (2+i)(3) + (-1) = 5+3i
    /// let x = GaussianInteger::from_str("5+3i").unwrap();
    /// let y = GaussianInteger::from_str("2+i").unwrap();
    /// assert_eq!((x % &y).to_string(), "-1");
    /// ```
    #[inline]
    fn rem(self, other: &Self) -> Self {
        div_rem_val_ref(self, other).1
    }
}

impl Rem<GaussianInteger> for &GaussianInteger {
    type Output = GaussianInteger;

    /// Divides a [`GaussianInteger`] by another [`GaussianInteger`], taking the first by reference
    /// and the second by value, and returns the remainder.
    ///
    /// The quotient (which is not returned) is the Gaussian integer nearest to the exact quotient,
    /// with each part rounded to the nearest integer and ties rounded up, and the remainder is what
    /// is left over. The quotient and remainder satisfy $x = qy + r$ and $N(r) \leq N(y) / 2$,
    /// where $N$ is the norm. To get both at once, use
    /// [`div_rem`](malachite_base::num::arithmetic::traits::DivRem::div_rem).
    ///
    /// $$
    /// f(x, y) = x - qy, \quad \text{where } q = \left \lfloor \frac{x \bar{y}}{N(y)} +
    /// \frac{1 + i}{2} \right \rfloor
    /// $$
    /// and the floor is taken on each part.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // (2+i)(3) + (-1) = 5+3i
    /// let x = GaussianInteger::from_str("5+3i").unwrap();
    /// let y = GaussianInteger::from_str("2+i").unwrap();
    /// assert_eq!((&x % y).to_string(), "-1");
    /// ```
    #[inline]
    fn rem(self, other: GaussianInteger) -> GaussianInteger {
        div_rem_ref_ref(self, &other).1
    }
}

impl Rem<&GaussianInteger> for &GaussianInteger {
    type Output = GaussianInteger;

    /// Divides a [`GaussianInteger`] by another [`GaussianInteger`], taking both by reference, and
    /// returns the remainder.
    ///
    /// The quotient (which is not returned) is the Gaussian integer nearest to the exact quotient,
    /// with each part rounded to the nearest integer and ties rounded up, and the remainder is what
    /// is left over. The quotient and remainder satisfy $x = qy + r$ and $N(r) \leq N(y) / 2$,
    /// where $N$ is the norm. To get both at once, use
    /// [`div_rem`](malachite_base::num::arithmetic::traits::DivRem::div_rem).
    ///
    /// $$
    /// f(x, y) = x - qy, \quad \text{where } q = \left \lfloor \frac{x \bar{y}}{N(y)} +
    /// \frac{1 + i}{2} \right \rfloor
    /// $$
    /// and the floor is taken on each part.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // (2+i)(3) + (-1) = 5+3i
    /// let x = GaussianInteger::from_str("5+3i").unwrap();
    /// let y = GaussianInteger::from_str("2+i").unwrap();
    /// assert_eq!((&x % &y).to_string(), "-1");
    /// ```
    #[inline]
    fn rem(self, other: &GaussianInteger) -> GaussianInteger {
        div_rem_ref_ref(self, other).1
    }
}

impl RemAssign<Self> for GaussianInteger {
    /// Divides a [`GaussianInteger`] by another [`GaussianInteger`] in place, taking the
    /// [`GaussianInteger`] on the right-hand side by value and replacing the first
    /// [`GaussianInteger`] with the remainder.
    ///
    /// The quotient (which is not returned) is the Gaussian integer nearest to the exact quotient,
    /// with each part rounded to the nearest integer and ties rounded up, and the remainder is what
    /// is left over. The quotient and remainder satisfy $x = qy + r$ and $N(r) \leq N(y) / 2$,
    /// where $N$ is the norm. To get both at once, use
    /// [`div_rem`](malachite_base::num::arithmetic::traits::DivRem::div_rem).
    ///
    /// $$
    /// x \gets x - qy, \quad \text{where } q = \left \lfloor \frac{x \bar{y}}{N(y)} +
    /// \frac{1 + i}{2} \right \rfloor
    /// $$
    /// and the floor is taken on each part.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // (2+i)(3) + (-1) = 5+3i
    /// let mut x = GaussianInteger::from_str("5+3i").unwrap();
    /// x %= GaussianInteger::from_str("2+i").unwrap();
    /// assert_eq!(x.to_string(), "-1");
    /// ```
    #[inline]
    fn rem_assign(&mut self, other: Self) {
        *self = div_rem_val_ref(take(self), &other).1;
    }
}

impl RemAssign<&Self> for GaussianInteger {
    /// Divides a [`GaussianInteger`] by another [`GaussianInteger`] in place, taking the
    /// [`GaussianInteger`] on the right-hand side by reference and replacing the first
    /// [`GaussianInteger`] with the remainder.
    ///
    /// The quotient (which is not returned) is the Gaussian integer nearest to the exact quotient,
    /// with each part rounded to the nearest integer and ties rounded up, and the remainder is what
    /// is left over. The quotient and remainder satisfy $x = qy + r$ and $N(r) \leq N(y) / 2$,
    /// where $N$ is the norm. To get both at once, use
    /// [`div_rem`](malachite_base::num::arithmetic::traits::DivRem::div_rem).
    ///
    /// $$
    /// x \gets x - qy, \quad \text{where } q = \left \lfloor \frac{x \bar{y}}{N(y)} +
    /// \frac{1 + i}{2} \right \rfloor
    /// $$
    /// and the floor is taken on each part.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // (2+i)(3) + (-1) = 5+3i
    /// let mut x = GaussianInteger::from_str("5+3i").unwrap();
    /// x %= &GaussianInteger::from_str("2+i").unwrap();
    /// assert_eq!(x.to_string(), "-1");
    /// ```
    #[inline]
    fn rem_assign(&mut self, other: &Self) {
        *self = div_rem_val_ref(take(self), other).1;
    }
}
