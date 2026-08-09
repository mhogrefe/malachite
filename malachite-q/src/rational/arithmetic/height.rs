// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use core::cmp::max;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::Natural;

impl Rational {
    /// Returns the height of a [`Rational`]: the larger of the absolute value of its numerator and
    /// its denominator, taking the [`Rational`] by reference and cloning.
    ///
    /// The height is the measure in which Diophantine approximation bounds are usually stated.
    ///
    /// $$
    /// f(p/q) = H(p/q) = \max(|p|, q).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Rational::from_str("22/7").unwrap().to_height(), 22);
    /// assert_eq!(Rational::from_str("-1/101").unwrap().to_height(), 101);
    /// assert_eq!(Rational::from_str("0").unwrap().to_height(), 1);
    /// ```
    ///
    /// This is fmpq_height from fmpq/height.c, FLINT 3.6.0, where the components are already
    /// magnitudes, so no absolute values need to be taken.
    #[inline]
    pub fn to_height(&self) -> Natural {
        max(&self.numerator, &self.denominator).clone()
    }

    /// Returns the height of a [`Rational`]: the larger of the absolute value of its numerator and
    /// its denominator, taking the [`Rational`] by value.
    ///
    /// The height is the measure in which Diophantine approximation bounds are usually stated.
    ///
    /// $$
    /// f(p/q) = H(p/q) = \max(|p|, q).
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Rational::from_str("22/7").unwrap().into_height(), 22);
    /// assert_eq!(Rational::from_str("-1/101").unwrap().into_height(), 101);
    /// assert_eq!(Rational::from_str("0").unwrap().into_height(), 1);
    /// ```
    ///
    /// This is fmpq_height from fmpq/height.c, FLINT 3.6.0, where the components are already
    /// magnitudes, so no absolute values need to be taken.
    #[inline]
    pub fn into_height(self) -> Natural {
        max(self.numerator, self.denominator)
    }

    /// Returns the number of significant bits of the height of a [`Rational`]: the larger of the
    /// numbers of significant bits of its numerator and its denominator.
    ///
    /// Since bit length is monotone, this is `self.to_height().significant_bits()`, without
    /// materializing the height.
    ///
    /// $$
    /// f(p/q) = \lfloor \log_2 \max(|p|, q) \rfloor + 1.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     Rational::from_str("22/7")
    ///         .unwrap()
    ///         .height_significant_bits(),
    ///     5
    /// );
    /// assert_eq!(
    ///     Rational::from_str("-1/101")
    ///         .unwrap()
    ///         .height_significant_bits(),
    ///     7
    /// );
    /// assert_eq!(
    ///     Rational::from_str("0").unwrap().height_significant_bits(),
    ///     1
    /// );
    /// ```
    ///
    /// This is fmpq_height_bits from fmpq/height_bits.c, FLINT 3.6.0.
    #[inline]
    pub fn height_significant_bits(&self) -> u64 {
        max(
            self.numerator.significant_bits(),
            self.denominator.significant_bits(),
        )
    }
}
