// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use core::cmp::Ordering::*;
use malachite_base::num::arithmetic::traits::{NextPowerOf2, NextPowerOf2Assign, PowerOf2};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;

impl NextPowerOf2 for Rational {
    type Output = Rational;

    /// Finds the smallest power of 2 greater than or equal to a [`Rational`]. The [`Rational`] is
    /// taken by value.
    ///
    /// $f(x) = 2^{\lceil \log_2 x \rceil}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is less than or equal to zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::NextPowerOf2;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from(123).next_power_of_2(), 128);
    /// assert_eq!(
    ///     Rational::from_signeds(1, 10).next_power_of_2().to_string(),
    ///     "1/8"
    /// );
    /// ```
    #[inline]
    fn next_power_of_2(self) -> Rational {
        assert!(self > 0);
        let mut exponent = i64::exact_from(self.numerator.significant_bits())
            - i64::exact_from(self.denominator.significant_bits());
        match self.numerator.cmp_normalized(&self.denominator) {
            Equal => return self,
            Greater => exponent += 1,
            _ => {}
        }
        Rational::power_of_2(exponent)
    }
}

impl NextPowerOf2 for &Rational {
    type Output = Rational;

    /// Finds the smallest power of 2 greater than or equal to a [`Rational`]. The [`Rational`] is
    /// taken by reference.
    ///
    /// $f(x) = 2^{\lceil \log_2 x \rceil}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is less than or equal to zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::NextPowerOf2;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!((&Rational::from(123)).next_power_of_2(), 128);
    /// assert_eq!(
    ///     (&Rational::from_signeds(1, 10))
    ///         .next_power_of_2()
    ///         .to_string(),
    ///     "1/8"
    /// );
    /// ```
    fn next_power_of_2(self) -> Rational {
        assert!(*self > 0);
        let mut exponent = i64::exact_from(self.numerator.significant_bits())
            - i64::exact_from(self.denominator.significant_bits());
        if self.numerator.cmp_normalized(&self.denominator) == Greater {
            exponent += 1;
        }
        Rational::power_of_2(exponent)
    }
}

impl NextPowerOf2Assign for Rational {
    /// Finds the smallest power of 2 greater than or equal to a [`Rational`]. The [`Rational`] is
    /// taken by reference.
    ///
    /// $f(x) = 2^{\lceil \log_2 x \rceil}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is less than or equal to zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::NextPowerOf2Assign;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::from(123);
    /// x.next_power_of_2_assign();
    /// assert_eq!(x, 128);
    ///
    /// let mut x = Rational::from_signeds(1, 10);
    /// x.next_power_of_2_assign();
    /// assert_eq!(x.to_string(), "1/8");
    /// ```
    #[inline]
    fn next_power_of_2_assign(&mut self) {
        *self = (&*self).next_power_of_2();
    }
}
