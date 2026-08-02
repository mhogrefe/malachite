// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::arithmetic::traits::{Average, AverageAssign};

impl Average<Self> for Rational {
    type Output = Self;

    /// Computes the average (arithmetic mean) of two [`Rational`]s, taking both by value.
    ///
    /// The result is always exact, so no rounding is involved.
    ///
    /// $$
    /// f(x, y) = \frac{x + y}{2}.
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
    /// use malachite_base::num::arithmetic::traits::Average;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::from_signeds(1, 2)
    ///         .average(Rational::from_signeds(1, 3))
    ///         .to_string(),
    ///     "5/12"
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(22, 7)
    ///         .average(Rational::from_signeds(-22, 7))
    ///         .to_string(),
    ///     "0"
    /// );
    /// assert_eq!(
    ///     Rational::from(3).average(Rational::from(4)).to_string(),
    ///     "7/2"
    /// );
    /// ```
    #[inline]
    fn average(self, other: Self) -> Self {
        (self + other) >> 1u64
    }
}

impl Average<&Self> for Rational {
    type Output = Self;

    /// Computes the average (arithmetic mean) of two [`Rational`]s, taking the first by value and
    /// the second by reference.
    ///
    /// The result is always exact, so no rounding is involved.
    ///
    /// $$
    /// f(x, y) = \frac{x + y}{2}.
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
    /// use malachite_base::num::arithmetic::traits::Average;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::from_signeds(1, 2)
    ///         .average(&Rational::from_signeds(1, 3))
    ///         .to_string(),
    ///     "5/12"
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(22, 7)
    ///         .average(&Rational::from_signeds(-22, 7))
    ///         .to_string(),
    ///     "0"
    /// );
    /// assert_eq!(
    ///     Rational::from(3).average(&Rational::from(4)).to_string(),
    ///     "7/2"
    /// );
    /// ```
    #[inline]
    fn average(self, other: &Self) -> Self {
        (self + other) >> 1u64
    }
}

impl Average<Rational> for &Rational {
    type Output = Rational;

    /// Computes the average (arithmetic mean) of two [`Rational`]s, taking the first by reference
    /// and the second by value.
    ///
    /// The result is always exact, so no rounding is involved.
    ///
    /// $$
    /// f(x, y) = \frac{x + y}{2}.
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
    /// use malachite_base::num::arithmetic::traits::Average;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     (&Rational::from_signeds(1, 2))
    ///         .average(Rational::from_signeds(1, 3))
    ///         .to_string(),
    ///     "5/12"
    /// );
    /// assert_eq!(
    ///     (&Rational::from_signeds(22, 7))
    ///         .average(Rational::from_signeds(-22, 7))
    ///         .to_string(),
    ///     "0"
    /// );
    /// assert_eq!(
    ///     (&Rational::from(3)).average(Rational::from(4)).to_string(),
    ///     "7/2"
    /// );
    /// ```
    #[inline]
    fn average(self, other: Rational) -> Rational {
        (self + other) >> 1u64
    }
}

impl Average<&Rational> for &Rational {
    type Output = Rational;

    /// Computes the average (arithmetic mean) of two [`Rational`]s, taking both by reference.
    ///
    /// The result is always exact, so no rounding is involved.
    ///
    /// $$
    /// f(x, y) = \frac{x + y}{2}.
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
    /// use malachite_base::num::arithmetic::traits::Average;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     (&Rational::from_signeds(1, 2))
    ///         .average(&Rational::from_signeds(1, 3))
    ///         .to_string(),
    ///     "5/12"
    /// );
    /// assert_eq!(
    ///     (&Rational::from_signeds(22, 7))
    ///         .average(&Rational::from_signeds(-22, 7))
    ///         .to_string(),
    ///     "0"
    /// );
    /// assert_eq!(
    ///     (&Rational::from(3)).average(&Rational::from(4)).to_string(),
    ///     "7/2"
    /// );
    /// ```
    #[inline]
    fn average(self, other: &Rational) -> Rational {
        (self + other) >> 1u64
    }
}

impl AverageAssign<Self> for Rational {
    /// Computes the average (arithmetic mean) of two [`Rational`]s, taking the [`Rational`] on the
    /// right-hand side by value and replacing the first [`Rational`] with it.
    ///
    /// The result is always exact, so no rounding is involved.
    ///
    /// $$
    /// x \gets \frac{x + y}{2}.
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
    /// use malachite_base::num::arithmetic::traits::AverageAssign;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::from_signeds(1, 2);
    /// x.average_assign(Rational::from_signeds(1, 3));
    /// assert_eq!(x.to_string(), "5/12");
    /// ```
    #[inline]
    fn average_assign(&mut self, other: Self) {
        *self += other;
        *self >>= 1u64;
    }
}

impl AverageAssign<&Self> for Rational {
    /// Computes the average (arithmetic mean) of two [`Rational`]s, taking the [`Rational`] on the
    /// right-hand side by reference and replacing the first [`Rational`] with it.
    ///
    /// The result is always exact, so no rounding is involved.
    ///
    /// $$
    /// x \gets \frac{x + y}{2}.
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
    /// use malachite_base::num::arithmetic::traits::AverageAssign;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::from_signeds(1, 2);
    /// x.average_assign(&Rational::from_signeds(1, 3));
    /// assert_eq!(x.to_string(), "5/12");
    /// ```
    #[inline]
    fn average_assign(&mut self, other: &Self) {
        *self += other;
        *self >>= 1u64;
    }
}
