// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use core::cmp::Ordering::*;
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, CheckedLogBase2, FloorLogBase2, IsPowerOf2,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;

impl Rational {
    /// Returns the floor of the base-2 logarithm of the absolute value of a nonzero [`Rational`].
    ///
    /// $f(x) = \lfloor\log_2 |x|\rfloor$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from(3u32).floor_log_base_2_abs(), 1);
    /// assert_eq!(Rational::from_signeds(1, 3).floor_log_base_2_abs(), -2);
    /// assert_eq!(Rational::from_signeds(1, 4).floor_log_base_2_abs(), -2);
    /// assert_eq!(Rational::from_signeds(1, 5).floor_log_base_2_abs(), -3);
    ///
    /// assert_eq!(Rational::from(-3).floor_log_base_2_abs(), 1);
    /// assert_eq!(Rational::from_signeds(-1, 3).floor_log_base_2_abs(), -2);
    /// assert_eq!(Rational::from_signeds(-1, 4).floor_log_base_2_abs(), -2);
    /// assert_eq!(Rational::from_signeds(-1, 5).floor_log_base_2_abs(), -3);
    /// ```
    pub fn floor_log_base_2_abs(&self) -> i64 {
        let exponent = i64::exact_from(self.numerator.significant_bits())
            - i64::exact_from(self.denominator.significant_bits());
        if self.numerator.cmp_normalized(&self.denominator) == Less {
            exponent - 1
        } else {
            exponent
        }
    }

    /// Returns the ceiling of the base-2 logarithm of the absolute value of a nonzero [`Rational`].
    ///
    /// $f(x) = \lfloor\log_2 |x|\rfloor$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` less than or equal to zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from(3u32).ceiling_log_base_2_abs(), 2);
    /// assert_eq!(Rational::from_signeds(1, 3).ceiling_log_base_2_abs(), -1);
    /// assert_eq!(Rational::from_signeds(1, 4).ceiling_log_base_2_abs(), -2);
    /// assert_eq!(Rational::from_signeds(1, 5).ceiling_log_base_2_abs(), -2);
    ///
    /// assert_eq!(Rational::from(-3).ceiling_log_base_2_abs(), 2);
    /// assert_eq!(Rational::from_signeds(-1, 3).ceiling_log_base_2_abs(), -1);
    /// assert_eq!(Rational::from_signeds(-1, 4).ceiling_log_base_2_abs(), -2);
    /// assert_eq!(Rational::from_signeds(-1, 5).ceiling_log_base_2_abs(), -2);
    /// ```
    pub fn ceiling_log_base_2_abs(&self) -> i64 {
        let exponent = i64::exact_from(self.numerator.significant_bits())
            - i64::exact_from(self.denominator.significant_bits());
        if self.numerator.cmp_normalized(&self.denominator) == Greater {
            exponent + 1
        } else {
            exponent
        }
    }
}

impl<'a> FloorLogBase2 for &'a Rational {
    type Output = i64;

    /// Returns the floor of the base-2 logarithm of a positive [`Rational`].
    ///
    /// $f(x) = \lfloor\log_2 x\rfloor$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` less than or equal to zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::FloorLogBase2;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from(3u32).floor_log_base_2(), 1);
    /// assert_eq!(Rational::from_signeds(1, 3).floor_log_base_2(), -2);
    /// assert_eq!(Rational::from_signeds(1, 4).floor_log_base_2(), -2);
    /// assert_eq!(Rational::from_signeds(1, 5).floor_log_base_2(), -3);
    /// ```
    #[inline]
    fn floor_log_base_2(self) -> i64 {
        assert!(*self > 0u32);
        self.floor_log_base_2_abs()
    }
}

impl<'a> CeilingLogBase2 for &'a Rational {
    type Output = i64;

    /// Returns the ceiling of the base-2 logarithm of a positive [`Rational`].
    ///
    /// $f(x) = \lfloor\log_2 x\rfloor$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` less than or equal to zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CeilingLogBase2;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from(3u32).ceiling_log_base_2(), 2);
    /// assert_eq!(Rational::from_signeds(1, 3).ceiling_log_base_2(), -1);
    /// assert_eq!(Rational::from_signeds(1, 4).ceiling_log_base_2(), -2);
    /// assert_eq!(Rational::from_signeds(1, 5).ceiling_log_base_2(), -2);
    /// ```
    #[inline]
    fn ceiling_log_base_2(self) -> i64 {
        assert!(*self > 0u32);
        self.ceiling_log_base_2_abs()
    }
}

impl<'a> CheckedLogBase2 for &'a Rational {
    type Output = i64;

    /// Returns the base-2 logarithm of a positive [`Rational`]. If the [`Rational`] is not a power
    /// of 2, then `None` is returned.
    ///
    /// $$
    /// f(x) = \\begin{cases}
    ///     \operatorname{Some}(\log_2 x) & \text{if} \\quad \log_2 x \in \Z, \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is less than or equal to zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedLogBase2;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from(3u32).checked_log_base_2(), None);
    /// assert_eq!(Rational::from_signeds(1, 3).checked_log_base_2(), None);
    /// assert_eq!(Rational::from_signeds(1, 4).checked_log_base_2(), Some(-2));
    /// assert_eq!(Rational::from_signeds(1, 5).checked_log_base_2(), None);
    /// ```
    fn checked_log_base_2(self) -> Option<i64> {
        assert!(*self > 0u32);
        if self.denominator == 1u32 && self.numerator.is_power_of_2() {
            Some(i64::exact_from(self.numerator.significant_bits()) - 1)
        } else if self.numerator == 1u32 && self.denominator.is_power_of_2() {
            Some(1 - i64::exact_from(self.denominator.significant_bits()))
        } else {
            None
        }
    }
}
