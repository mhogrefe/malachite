// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1991-2018, 2020 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::arithmetic::traits::{Abs, AbsAssign, AbsDiff, AbsDiffAssign};

impl AbsDiff<Rational> for Rational {
    type Output = Rational;

    /// Computes the absolute value of the difference between two [`Rational`]s, taking both by
    /// value.
    ///
    /// $$
    /// f(x, y) = |x - y|.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AbsDiff;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::ONE_HALF.abs_diff(Rational::ONE_HALF), 0);
    /// assert_eq!(
    ///     Rational::from_signeds(22, 7)
    ///         .abs_diff(Rational::from_signeds(99, 100))
    ///         .to_string(),
    ///     "1507/700"
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(22, 7)
    ///         .abs_diff(Rational::from_signeds(99, 100))
    ///         .to_string(),
    ///     "1507/700"
    /// );
    /// ```
    #[inline]
    fn abs_diff(self, other: Rational) -> Rational {
        (self - other).abs()
    }
}

impl AbsDiff<&Rational> for Rational {
    type Output = Rational;

    /// Computes the absolute value of the difference between two [`Rational`]s, taking the first by
    /// value and the second by reference.
    ///
    /// $$
    /// f(x, y) = |x - y|.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AbsDiff;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::ONE_HALF.abs_diff(&Rational::ONE_HALF), 0);
    /// assert_eq!(
    ///     Rational::from_signeds(22, 7)
    ///         .abs_diff(&Rational::from_signeds(99, 100))
    ///         .to_string(),
    ///     "1507/700"
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(22, 7)
    ///         .abs_diff(&Rational::from_signeds(99, 100))
    ///         .to_string(),
    ///     "1507/700"
    /// );
    /// ```
    #[inline]
    fn abs_diff(self, other: &Rational) -> Rational {
        (self - other).abs()
    }
}

impl AbsDiff<Rational> for &Rational {
    type Output = Rational;

    /// Computes the absolute value of the difference between two [`Rational`]s, taking the first by
    /// reference and the second by value.
    ///
    /// $$
    /// f(x, y) = |x - y|.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AbsDiff;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::ONE_HALF.abs_diff(Rational::ONE_HALF), 0);
    /// assert_eq!(
    ///     (&Rational::from_signeds(22, 7))
    ///         .abs_diff(Rational::from_signeds(99, 100))
    ///         .to_string(),
    ///     "1507/700"
    /// );
    /// assert_eq!(
    ///     (&Rational::from_signeds(22, 7))
    ///         .abs_diff(Rational::from_signeds(99, 100))
    ///         .to_string(),
    ///     "1507/700"
    /// );
    /// ```
    #[inline]
    fn abs_diff(self, other: Rational) -> Rational {
        (self - other).abs()
    }
}

impl AbsDiff<&Rational> for &Rational {
    type Output = Rational;

    /// Computes the absolute value of the difference between two [`Rational`]s, taking both by
    /// reference.
    ///
    /// $$
    /// f(x, y) = |x - y|.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AbsDiff;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::ONE_HALF.abs_diff(Rational::ONE_HALF), 0);
    /// assert_eq!(
    ///     (&Rational::from_signeds(22, 7))
    ///         .abs_diff(&Rational::from_signeds(99, 100))
    ///         .to_string(),
    ///     "1507/700"
    /// );
    /// assert_eq!(
    ///     (&Rational::from_signeds(22, 7))
    ///         .abs_diff(&Rational::from_signeds(99, 100))
    ///         .to_string(),
    ///     "1507/700"
    /// );
    /// ```
    #[inline]
    fn abs_diff(self, other: &Rational) -> Rational {
        (self - other).abs()
    }
}

impl AbsDiffAssign<Rational> for Rational {
    /// Subtracts a [`Rational`] by another [`Rational`] in place and takes the absolute value,
    /// taking the [`Rational`] on the right-hand side by value.
    ///
    /// $$
    /// x \gets |x - y|.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `other` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AbsDiffAssign;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x.abs_diff_assign(Rational::ONE_HALF);
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Rational::from_signeds(22, 7);
    /// x.abs_diff_assign(Rational::from_signeds(99, 100));
    /// assert_eq!(x.to_string(), "1507/700");
    ///
    /// let mut x = Rational::from_signeds(99, 100);
    /// x.abs_diff_assign(Rational::from_signeds(22, 7));
    /// assert_eq!(x.to_string(), "1507/700");
    /// ```
    #[inline]
    fn abs_diff_assign(&mut self, other: Rational) {
        *self -= other;
        self.abs_assign();
    }
}

impl<'a> AbsDiffAssign<&'a Rational> for Rational {
    /// Subtracts a [`Rational`] by another [`Rational`] in place and takes the absolute value,
    /// taking the [`Rational`] on the right-hand side by reference.
    ///
    /// $$
    /// x \gets |x - y|.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `other` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AbsDiffAssign;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x.abs_diff_assign(&Rational::ONE_HALF);
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Rational::from_signeds(22, 7);
    /// x.abs_diff_assign(&Rational::from_signeds(99, 100));
    /// assert_eq!(x.to_string(), "1507/700");
    ///
    /// let mut x = Rational::from_signeds(99, 100);
    /// x.abs_diff_assign(&Rational::from_signeds(22, 7));
    /// assert_eq!(x.to_string(), "1507/700");
    /// ```
    #[inline]
    fn abs_diff_assign(&mut self, other: &'a Rational) {
        *self -= other;
        self.abs_assign();
    }
}
