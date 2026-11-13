// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::Finite;
use crate::big_exponent_float::arithmetic::add::significand_prec;
use malachite_base::num::arithmetic::traits::IsPowerOf2;

impl IsPowerOf2 for BigExponentFloat {
    /// Determines whether a [`BigExponentFloat`] is an integer power of 2.
    ///
    /// $f(x) = (\exists n \in \Z : 2^n = x)$.
    ///
    /// [`BigExponentFloat`]s that are NaN, infinite, or zero are not powers of 2.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{IsPowerOf2, PowerOf2};
    /// use malachite_base::num::basic::traits::{NaN, One, OneHalf, Two};
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(BigExponentFloat::NAN.is_power_of_2(), false);
    ///
    /// assert_eq!(BigExponentFloat::ONE.is_power_of_2(), true);
    /// assert_eq!(BigExponentFloat::TWO.is_power_of_2(), true);
    /// assert_eq!(BigExponentFloat::ONE_HALF.is_power_of_2(), true);
    /// assert_eq!(BigExponentFloat::from(1024).is_power_of_2(), true);
    /// assert_eq!(BigExponentFloat::power_of_2(i64::MIN).is_power_of_2(), true);
    ///
    /// assert_eq!(BigExponentFloat::from(3).is_power_of_2(), false);
    /// assert_eq!(BigExponentFloat::from(1025).is_power_of_2(), false);
    /// assert_eq!(BigExponentFloat::from(-2).is_power_of_2(), false);
    /// assert_eq!(BigExponentFloat::from(0.1f64).is_power_of_2(), false);
    /// ```
    fn is_power_of_2(&self) -> bool {
        match self {
            Self(Finite {
                sign: true,
                significand,
                ..
            }) => significand_prec(significand) == 1,
            _ => false,
        }
    }
}
