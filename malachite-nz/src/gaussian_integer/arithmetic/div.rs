// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use crate::gaussian_integer::arithmetic::div_rem::quotient_or_zero;
use core::ops::{Div, DivAssign};
use malachite_base::num::arithmetic::traits::CheckedDiv;

fn div_ref_ref(x: &GaussianInteger, y: &GaussianInteger) -> GaussianInteger {
    quotient_or_zero(x, y).unwrap_or_else(|| GaussianInteger::from(0u32))
}

impl Div<Self> for GaussianInteger {
    type Output = Self;

    /// Divides a [`GaussianInteger`] by another [`GaussianInteger`], taking both by value.
    ///
    /// The quotient is the Gaussian integer nearest to the exact quotient, with each part rounded
    /// to the nearest integer and ties rounded up. The quotient and remainder (which is not
    /// computed) satisfy $x = qy + r$ and $N(r) \leq N(y) / 2$, where $N$ is the norm. To get both
    /// at once, use [`div_rem`](malachite_base::num::arithmetic::traits::DivRem::div_rem).
    ///
    /// $$
    /// f(x, y) = \left \lfloor \frac{x \bar{y}}{N(y)} + \frac{1 + i}{2} \right \rfloor,
    /// $$
    /// where the floor is taken on each part.
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
    /// assert_eq!((x / y).to_string(), "3");
    /// ```
    #[inline]
    fn div(self, other: Self) -> Self {
        div_ref_ref(&self, &other)
    }
}

impl Div<&Self> for GaussianInteger {
    type Output = Self;

    /// Divides a [`GaussianInteger`] by another [`GaussianInteger`], taking the first by value and
    /// the second by reference.
    ///
    /// The quotient is the Gaussian integer nearest to the exact quotient, with each part rounded
    /// to the nearest integer and ties rounded up. The quotient and remainder (which is not
    /// computed) satisfy $x = qy + r$ and $N(r) \leq N(y) / 2$, where $N$ is the norm. To get both
    /// at once, use [`div_rem`](malachite_base::num::arithmetic::traits::DivRem::div_rem).
    ///
    /// $$
    /// f(x, y) = \left \lfloor \frac{x \bar{y}}{N(y)} + \frac{1 + i}{2} \right \rfloor,
    /// $$
    /// where the floor is taken on each part.
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
    /// assert_eq!((x / &y).to_string(), "3");
    /// ```
    #[inline]
    fn div(self, other: &Self) -> Self {
        div_ref_ref(&self, other)
    }
}

impl Div<GaussianInteger> for &GaussianInteger {
    type Output = GaussianInteger;

    /// Divides a [`GaussianInteger`] by another [`GaussianInteger`], taking the first by reference
    /// and the second by value.
    ///
    /// The quotient is the Gaussian integer nearest to the exact quotient, with each part rounded
    /// to the nearest integer and ties rounded up. The quotient and remainder (which is not
    /// computed) satisfy $x = qy + r$ and $N(r) \leq N(y) / 2$, where $N$ is the norm. To get both
    /// at once, use [`div_rem`](malachite_base::num::arithmetic::traits::DivRem::div_rem).
    ///
    /// $$
    /// f(x, y) = \left \lfloor \frac{x \bar{y}}{N(y)} + \frac{1 + i}{2} \right \rfloor,
    /// $$
    /// where the floor is taken on each part.
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
    /// assert_eq!((&x / y).to_string(), "3");
    /// ```
    #[inline]
    fn div(self, other: GaussianInteger) -> GaussianInteger {
        div_ref_ref(self, &other)
    }
}

impl Div<&GaussianInteger> for &GaussianInteger {
    type Output = GaussianInteger;

    /// Divides a [`GaussianInteger`] by another [`GaussianInteger`], taking both by reference.
    ///
    /// The quotient is the Gaussian integer nearest to the exact quotient, with each part rounded
    /// to the nearest integer and ties rounded up. The quotient and remainder (which is not
    /// computed) satisfy $x = qy + r$ and $N(r) \leq N(y) / 2$, where $N$ is the norm. To get both
    /// at once, use [`div_rem`](malachite_base::num::arithmetic::traits::DivRem::div_rem).
    ///
    /// $$
    /// f(x, y) = \left \lfloor \frac{x \bar{y}}{N(y)} + \frac{1 + i}{2} \right \rfloor,
    /// $$
    /// where the floor is taken on each part.
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
    /// assert_eq!((&x / &y).to_string(), "3");
    /// ```
    #[inline]
    fn div(self, other: &GaussianInteger) -> GaussianInteger {
        div_ref_ref(self, other)
    }
}

impl DivAssign<Self> for GaussianInteger {
    /// Divides a [`GaussianInteger`] by another [`GaussianInteger`] in place, taking the
    /// [`GaussianInteger`] on the right-hand side by value.
    ///
    /// The quotient is the Gaussian integer nearest to the exact quotient, with each part rounded
    /// to the nearest integer and ties rounded up. The quotient and remainder (which is not
    /// computed) satisfy $x = qy + r$ and $N(r) \leq N(y) / 2$, where $N$ is the norm. To get both
    /// at once, use [`div_rem`](malachite_base::num::arithmetic::traits::DivRem::div_rem).
    ///
    /// $$
    /// x \gets \left \lfloor \frac{x \bar{y}}{N(y)} + \frac{1 + i}{2} \right \rfloor,
    /// $$
    /// where the floor is taken on each part.
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
    /// x /= GaussianInteger::from_str("2+i").unwrap();
    /// assert_eq!(x.to_string(), "3");
    /// ```
    #[inline]
    fn div_assign(&mut self, other: Self) {
        *self = div_ref_ref(&*self, &other);
    }
}

impl DivAssign<&Self> for GaussianInteger {
    /// Divides a [`GaussianInteger`] by another [`GaussianInteger`] in place, taking the
    /// [`GaussianInteger`] on the right-hand side by reference.
    ///
    /// The quotient is the Gaussian integer nearest to the exact quotient, with each part rounded
    /// to the nearest integer and ties rounded up. The quotient and remainder (which is not
    /// computed) satisfy $x = qy + r$ and $N(r) \leq N(y) / 2$, where $N$ is the norm. To get both
    /// at once, use [`div_rem`](malachite_base::num::arithmetic::traits::DivRem::div_rem).
    ///
    /// $$
    /// x \gets \left \lfloor \frac{x \bar{y}}{N(y)} + \frac{1 + i}{2} \right \rfloor,
    /// $$
    /// where the floor is taken on each part.
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
    /// x /= &GaussianInteger::from_str("2+i").unwrap();
    /// assert_eq!(x.to_string(), "3");
    /// ```
    #[inline]
    fn div_assign(&mut self, other: &Self) {
        *self = div_ref_ref(&*self, other);
    }
}

impl CheckedDiv<Self> for GaussianInteger {
    type Output = Self;

    /// Divides a [`GaussianInteger`] by another [`GaussianInteger`], taking both by value. Returns
    /// `None` when the second [`GaussianInteger`] is zero.
    ///
    /// The quotient is the Gaussian integer nearest to the exact quotient, with each part rounded
    /// to the nearest integer and ties rounded up. The quotient and remainder (which is not
    /// computed) satisfy $x = qy + r$ and $N(r) \leq N(y) / 2$, where $N$ is the norm. To get both
    /// at once, use [`div_rem`](malachite_base::num::arithmetic::traits::DivRem::div_rem).
    ///
    /// $$
    /// f(x, y) = \left \lfloor \frac{x \bar{y}}{N(y)} + \frac{1 + i}{2} \right \rfloor,
    /// $$
    /// where the floor is taken on each part.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedDiv;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // (2+i)(3) + (-1) = 5+3i
    /// let x = GaussianInteger::from_str("5+3i").unwrap();
    /// let y = GaussianInteger::from_str("2+i").unwrap();
    /// assert_eq!((x.clone().checked_div(y)).unwrap().to_string(), "3");
    /// assert_eq!((x.clone().checked_div(GaussianInteger::ZERO)), None);
    /// ```
    #[inline]
    fn checked_div(self, other: Self) -> Option<Self> {
        if other.real == 0u32 && other.imaginary == 0u32 {
            None
        } else {
            Some(self / other)
        }
    }
}

impl CheckedDiv<&Self> for GaussianInteger {
    type Output = Self;

    /// Divides a [`GaussianInteger`] by another [`GaussianInteger`], taking the first by value and
    /// the second by reference. Returns `None` when the second [`GaussianInteger`] is zero.
    ///
    /// The quotient is the Gaussian integer nearest to the exact quotient, with each part rounded
    /// to the nearest integer and ties rounded up. The quotient and remainder (which is not
    /// computed) satisfy $x = qy + r$ and $N(r) \leq N(y) / 2$, where $N$ is the norm. To get both
    /// at once, use [`div_rem`](malachite_base::num::arithmetic::traits::DivRem::div_rem).
    ///
    /// $$
    /// f(x, y) = \left \lfloor \frac{x \bar{y}}{N(y)} + \frac{1 + i}{2} \right \rfloor,
    /// $$
    /// where the floor is taken on each part.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedDiv;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // (2+i)(3) + (-1) = 5+3i
    /// let x = GaussianInteger::from_str("5+3i").unwrap();
    /// let y = GaussianInteger::from_str("2+i").unwrap();
    /// assert_eq!((x.clone().checked_div(&y)).unwrap().to_string(), "3");
    /// assert_eq!((x.clone().checked_div(&GaussianInteger::ZERO)), None);
    /// ```
    #[inline]
    fn checked_div(self, other: &Self) -> Option<Self> {
        if other.real == 0u32 && other.imaginary == 0u32 {
            None
        } else {
            Some(self / other)
        }
    }
}

impl CheckedDiv<GaussianInteger> for &GaussianInteger {
    type Output = GaussianInteger;

    /// Divides a [`GaussianInteger`] by another [`GaussianInteger`], taking the first by reference
    /// and the second by value. Returns `None` when the second [`GaussianInteger`] is zero.
    ///
    /// The quotient is the Gaussian integer nearest to the exact quotient, with each part rounded
    /// to the nearest integer and ties rounded up. The quotient and remainder (which is not
    /// computed) satisfy $x = qy + r$ and $N(r) \leq N(y) / 2$, where $N$ is the norm. To get both
    /// at once, use [`div_rem`](malachite_base::num::arithmetic::traits::DivRem::div_rem).
    ///
    /// $$
    /// f(x, y) = \left \lfloor \frac{x \bar{y}}{N(y)} + \frac{1 + i}{2} \right \rfloor,
    /// $$
    /// where the floor is taken on each part.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedDiv;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // (2+i)(3) + (-1) = 5+3i
    /// let x = GaussianInteger::from_str("5+3i").unwrap();
    /// let y = GaussianInteger::from_str("2+i").unwrap();
    /// assert_eq!(((&x).checked_div(y)).unwrap().to_string(), "3");
    /// assert_eq!(((&x).checked_div(GaussianInteger::ZERO)), None);
    /// ```
    #[inline]
    fn checked_div(self, other: GaussianInteger) -> Option<GaussianInteger> {
        if other.real == 0u32 && other.imaginary == 0u32 {
            None
        } else {
            Some(self / other)
        }
    }
}

impl CheckedDiv<&GaussianInteger> for &GaussianInteger {
    type Output = GaussianInteger;

    /// Divides a [`GaussianInteger`] by another [`GaussianInteger`], taking both by reference.
    /// Returns `None` when the second [`GaussianInteger`] is zero.
    ///
    /// The quotient is the Gaussian integer nearest to the exact quotient, with each part rounded
    /// to the nearest integer and ties rounded up. The quotient and remainder (which is not
    /// computed) satisfy $x = qy + r$ and $N(r) \leq N(y) / 2$, where $N$ is the norm. To get both
    /// at once, use [`div_rem`](malachite_base::num::arithmetic::traits::DivRem::div_rem).
    ///
    /// $$
    /// f(x, y) = \left \lfloor \frac{x \bar{y}}{N(y)} + \frac{1 + i}{2} \right \rfloor,
    /// $$
    /// where the floor is taken on each part.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedDiv;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// // (2+i)(3) + (-1) = 5+3i
    /// let x = GaussianInteger::from_str("5+3i").unwrap();
    /// let y = GaussianInteger::from_str("2+i").unwrap();
    /// assert_eq!(((&x).checked_div(&y)).unwrap().to_string(), "3");
    /// assert_eq!(((&x).checked_div(&GaussianInteger::ZERO)), None);
    /// ```
    #[inline]
    fn checked_div(self, other: &GaussianInteger) -> Option<GaussianInteger> {
        if other.real == 0u32 && other.imaginary == 0u32 {
            None
        } else {
            Some(self / other)
        }
    }
}
