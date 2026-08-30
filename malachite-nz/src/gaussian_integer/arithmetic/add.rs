// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use core::iter::Sum;
use core::ops::{Add, AddAssign};
use malachite_base::num::basic::traits::Zero;

impl Add<Self> for GaussianInteger {
    type Output = Self;

    /// Adds two [`GaussianInteger`]s, taking both by value.
    ///
    /// $$
    /// f(x, y) = x + y.
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
    /// assert_eq!((x + y).to_string(), "1+i");
    /// ```
    #[inline]
    fn add(mut self, other: Self) -> Self {
        self += other;
        self
    }
}

impl Add<&Self> for GaussianInteger {
    type Output = Self;

    /// Adds two [`GaussianInteger`]s, taking the first by value and the second by reference.
    ///
    /// $$
    /// f(x, y) = x + y.
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
    /// assert_eq!((x + &y).to_string(), "1+i");
    /// ```
    #[inline]
    fn add(mut self, other: &Self) -> Self {
        self += other;
        self
    }
}

impl Add<GaussianInteger> for &GaussianInteger {
    type Output = GaussianInteger;

    /// Adds two [`GaussianInteger`]s, taking the first by reference and the second by value.
    ///
    /// $$
    /// f(x, y) = x + y.
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
    /// assert_eq!((&x + y).to_string(), "1+i");
    /// ```
    #[inline]
    fn add(self, other: GaussianInteger) -> GaussianInteger {
        GaussianInteger {
            real: &self.real + other.real,
            imaginary: &self.imaginary + other.imaginary,
        }
    }
}

impl Add<&GaussianInteger> for &GaussianInteger {
    type Output = GaussianInteger;

    /// Adds two [`GaussianInteger`]s, taking both by reference.
    ///
    /// $$
    /// f(x, y) = x + y.
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
    /// assert_eq!((&x + &y).to_string(), "1000000000000+2i");
    /// ```
    #[inline]
    fn add(self, other: &GaussianInteger) -> GaussianInteger {
        GaussianInteger {
            real: &self.real + &other.real,
            imaginary: &self.imaginary + &other.imaginary,
        }
    }
}

impl AddAssign<Self> for GaussianInteger {
    /// Adds a [`GaussianInteger`] to a [`GaussianInteger`] in place, taking the [`GaussianInteger`]
    /// on the right-hand side by value.
    ///
    /// $$
    /// x \gets x + y.
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
    /// sum += y;
    /// assert_eq!(sum.to_string(), "1+i");
    /// ```
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.real += other.real;
        self.imaginary += other.imaginary;
    }
}

impl AddAssign<&Self> for GaussianInteger {
    /// Adds a [`GaussianInteger`] to a [`GaussianInteger`] in place, taking the [`GaussianInteger`]
    /// on the right-hand side by reference.
    ///
    /// $$
    /// x \gets x + y.
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
    /// sum += &y;
    /// assert_eq!(sum.to_string(), "1+i");
    /// ```
    #[inline]
    fn add_assign(&mut self, other: &Self) {
        self.real += &other.real;
        self.imaginary += &other.imaginary;
    }
}

impl Sum for GaussianInteger {
    /// Adds up all the [`GaussianInteger`]s in an iterator.
    ///
    /// $$
    /// f((x_i)_ {i=0}^{n-1}) = \sum_ {i=0}^{n-1} x_i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of significant bits
    /// of the real and imaginary parts of the [`GaussianInteger`]s.
    ///
    /// # Examples
    /// ```
    /// use core::iter::Sum;
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    ///
    /// assert_eq!(
    ///     GaussianInteger::sum(
    ///         vec_from_str::<GaussianInteger>("[2, -3i, 5+i, 7-2i]")
    ///             .unwrap()
    ///             .into_iter()
    ///     )
    ///     .to_string(),
    ///     "14-4i"
    /// );
    /// ```
    fn sum<I>(xs: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let mut s = Self::ZERO;
        for x in xs {
            s += x;
        }
        s
    }
}

impl<'a> Sum<&'a Self> for GaussianInteger {
    /// Adds up all the [`GaussianInteger`]s in an iterator of [`GaussianInteger`] references.
    ///
    /// $$
    /// f((x_i)_ {i=0}^{n-1}) = \sum_ {i=0}^{n-1} x_i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of significant bits
    /// of the real and imaginary parts of the [`GaussianInteger`]s.
    ///
    /// # Examples
    /// ```
    /// use core::iter::Sum;
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    ///
    /// assert_eq!(
    ///     GaussianInteger::sum(
    ///         vec_from_str::<GaussianInteger>("[2, -3i, 5+i, 7-2i]")
    ///             .unwrap()
    ///             .iter()
    ///     )
    ///     .to_string(),
    ///     "14-4i"
    /// );
    /// ```
    fn sum<I>(xs: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        let mut s = Self::ZERO;
        for x in xs {
            s += x;
        }
        s
    }
}
