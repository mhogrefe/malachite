// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_rational::GaussianRational;
use core::iter::Sum;
use core::ops::{Add, AddAssign};
use malachite_base::iterators::balanced_fold;
use malachite_base::num::basic::traits::Zero;

impl Add<Self> for GaussianRational {
    type Output = Self;

    /// Adds two [`GaussianRational`]s, taking both by value.
    ///
    /// $$
    /// f(x, y) = x + y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("1/2+i/2").unwrap();
    /// let y = GaussianRational::from_str("1/3-i/3").unwrap();
    /// assert_eq!((x + y).to_string(), "5/6+i/6");
    /// ```
    #[inline]
    fn add(mut self, other: Self) -> Self {
        self += other;
        self
    }
}

impl Add<&Self> for GaussianRational {
    type Output = Self;

    /// Adds two [`GaussianRational`]s, taking the first by value and the second by reference.
    ///
    /// $$
    /// f(x, y) = x + y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("1/2+i/2").unwrap();
    /// let y = GaussianRational::from_str("1/3-i/3").unwrap();
    /// assert_eq!((x + &y).to_string(), "5/6+i/6");
    /// ```
    #[inline]
    fn add(mut self, other: &Self) -> Self {
        self += other;
        self
    }
}

impl Add<GaussianRational> for &GaussianRational {
    type Output = GaussianRational;

    /// Adds two [`GaussianRational`]s, taking the first by reference and the second by value.
    ///
    /// $$
    /// f(x, y) = x + y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("1/2+i/2").unwrap();
    /// let y = GaussianRational::from_str("1/3-i/3").unwrap();
    /// assert_eq!((&x + y).to_string(), "5/6+i/6");
    /// ```
    #[inline]
    fn add(self, other: GaussianRational) -> GaussianRational {
        GaussianRational {
            real: &self.real + other.real,
            imaginary: &self.imaginary + other.imaginary,
        }
    }
}

impl Add<&GaussianRational> for &GaussianRational {
    type Output = GaussianRational;

    /// Adds two [`GaussianRational`]s, taking both by reference.
    ///
    /// $$
    /// f(x, y) = x + y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("2/3-5i/6").unwrap();
    /// let y = GaussianRational::from_str("i").unwrap();
    /// assert_eq!((&x + &y).to_string(), "2/3+i/6");
    /// ```
    #[inline]
    fn add(self, other: &GaussianRational) -> GaussianRational {
        GaussianRational {
            real: &self.real + &other.real,
            imaginary: &self.imaginary + &other.imaginary,
        }
    }
}

impl AddAssign<Self> for GaussianRational {
    /// Adds a [`GaussianRational`] to a [`GaussianRational`] in place, taking the
    /// [`GaussianRational`] on the right-hand side by value.
    ///
    /// $$
    /// x \gets x + y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("1/2+i/2").unwrap();
    /// let y = GaussianRational::from_str("1/3-i/3").unwrap();
    /// let mut sum = x;
    /// sum += y;
    /// assert_eq!(sum.to_string(), "5/6+i/6");
    /// ```
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.real += other.real;
        self.imaginary += other.imaginary;
    }
}

impl AddAssign<&Self> for GaussianRational {
    /// Adds a [`GaussianRational`] to a [`GaussianRational`] in place, taking the
    /// [`GaussianRational`] on the right-hand side by reference.
    ///
    /// $$
    /// x \gets x + y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("1/2+i/2").unwrap();
    /// let y = GaussianRational::from_str("1/3-i/3").unwrap();
    /// let mut sum = x;
    /// sum += &y;
    /// assert_eq!(sum.to_string(), "5/6+i/6");
    /// ```
    #[inline]
    fn add_assign(&mut self, other: &Self) {
        self.real += &other.real;
        self.imaginary += &other.imaginary;
    }
}

impl Sum for GaussianRational {
    /// Adds up all the [`GaussianRational`]s in an iterator.
    ///
    /// $$
    /// f((x_i)_ {i=0}^{n-1}) = \sum_ {i=0}^{n-1} x_i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of significant bits
    /// of the real and imaginary parts of the [`GaussianRational`]s.
    ///
    /// # Examples
    /// ```
    /// use core::iter::Sum;
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_q::gaussian_rational::GaussianRational;
    ///
    /// assert_eq!(
    ///     GaussianRational::sum(
    ///         vec_from_str::<GaussianRational>("[2, -3i, 5/3+i, 7/2-i/2]")
    ///             .unwrap()
    ///             .into_iter()
    ///     )
    ///     .to_string(),
    ///     "43/6-5i/2"
    /// );
    /// ```
    #[inline]
    fn sum<I>(xs: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        balanced_fold(xs, |_| false, |a, b| *a += b).unwrap_or(Self::ZERO)
    }
}

impl<'a> Sum<&'a Self> for GaussianRational {
    /// Adds up all the [`GaussianRational`]s in an iterator of [`GaussianRational`] references.
    ///
    /// $$
    /// f((x_i)_ {i=0}^{n-1}) = \sum_ {i=0}^{n-1} x_i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of significant bits
    /// of the real and imaginary parts of the [`GaussianRational`]s.
    ///
    /// # Examples
    /// ```
    /// use core::iter::Sum;
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_q::gaussian_rational::GaussianRational;
    ///
    /// assert_eq!(
    ///     GaussianRational::sum(
    ///         vec_from_str::<GaussianRational>("[2, -3i, 5/3+i, 7/2-i/2]")
    ///             .unwrap()
    ///             .iter()
    ///     )
    ///     .to_string(),
    ///     "43/6-5i/2"
    /// );
    /// ```
    #[inline]
    fn sum<I>(xs: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        balanced_fold(xs.cloned(), |_| false, |a, b| *a += b).unwrap_or(Self::ZERO)
    }
}
