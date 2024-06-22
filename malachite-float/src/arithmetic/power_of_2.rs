// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::Finite;
use malachite_base::num::arithmetic::traits::{PowerOf2, RoundToMultipleOfPowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

impl Float {
    /// Raises 2 to an integer power, returning a [`Float`] with the specified precision.
    ///
    /// If you need a [`Float`] with precision 1, then the
    /// [`PowerOfTwo`](malachite_base::num::arithmetic::traits::PowerOf2) implementation may be used
    /// instead.
    ///
    /// $f(k) = 2^k$,
    ///
    /// and the result has precision `prec`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::power_of_2_prec(0, 1).to_string(), "1.0");
    /// assert_eq!(Float::power_of_2_prec(0, 10).to_string(), "1.0");
    /// assert_eq!(Float::power_of_2_prec(0, 100).to_string(), "1.0");
    ///
    /// assert_eq!(Float::power_of_2_prec(100, 1).to_string(), "1.0e30");
    /// assert_eq!(Float::power_of_2_prec(100, 10).to_string(), "1.268e30");
    /// assert_eq!(
    ///     Float::power_of_2_prec(100, 100).to_string(),
    ///     "1267650600228229401496703205376.0"
    /// );
    ///
    /// assert_eq!(Float::power_of_2_prec(-100, 1).to_string(), "8.0e-31");
    /// assert_eq!(Float::power_of_2_prec(-100, 10).to_string(), "7.89e-31");
    /// assert_eq!(
    ///     Float::power_of_2_prec(-100, 100).to_string(),
    ///     "7.88860905221011805411728565283e-31"
    /// );
    /// ```
    pub fn power_of_2_prec(pow: i64, prec: u64) -> Float {
        assert_ne!(prec, 0);
        Float(Finite {
            sign: true,
            exponent: i32::exact_from(pow.checked_add(1).unwrap()),
            precision: prec,
            significand: Natural::power_of_2(
                prec.round_to_multiple_of_power_of_2(Limb::LOG_WIDTH, Ceiling)
                    .0
                    - 1,
            ),
        })
    }
}

impl PowerOf2<u64> for Float {
    /// Raises 2 to an integer power, returning a [`Float`] with precision 1.
    ///
    /// To get a [`Float`] with a higher precision, try [`Float::power_of_2_prec`].
    ///
    /// $f(k) = 2^k$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::power_of_2(0u64).to_string(), "1.0");
    /// assert_eq!(Float::power_of_2(3u64).to_string(), "8.0");
    /// assert_eq!(Float::power_of_2(100u64).to_string(), "1.0e30");
    /// ```
    #[inline]
    fn power_of_2(pow: u64) -> Float {
        Float(Finite {
            sign: true,
            exponent: i32::exact_from(pow.checked_add(1).unwrap()),
            precision: 1,
            significand: Natural::HIGH_BIT,
        })
    }
}

impl PowerOf2<i64> for Float {
    /// Raises 2 to an integer power, returning a [`Float`] with precision 1.
    ///
    /// To get a [`Float`] with a higher precision, try [`Float::power_of_2_prec`].
    ///
    /// $f(k) = 2^k$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::power_of_2(0i64).to_string(), "1.0");
    /// assert_eq!(Float::power_of_2(3i64).to_string(), "8.0");
    /// assert_eq!(Float::power_of_2(100i64).to_string(), "1.0e30");
    /// assert_eq!(Float::power_of_2(-3i64).to_string(), "0.1");
    /// assert_eq!(Float::power_of_2(-100i64).to_string(), "8.0e-31");
    /// ```
    #[inline]
    fn power_of_2(pow: i64) -> Float {
        Float(Finite {
            sign: true,
            exponent: i32::exact_from(pow.checked_add(1).unwrap()),
            precision: 1,
            significand: Natural::HIGH_BIT,
        })
    }
}
