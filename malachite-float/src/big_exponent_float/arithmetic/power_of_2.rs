// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::traits::One;

impl PowerOf2<u64> for BigExponentFloat {
    /// Raises 2 to an integer power.
    ///
    /// $f(k) = 2^k$.
    ///
    /// Unlike [`Float::power_of_2`](crate::Float#method.power_of_2), this never overflows.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     BigExponentFloat::power_of_2(0u64),
    ///     BigExponentFloat::from(1)
    /// );
    /// assert_eq!(
    ///     BigExponentFloat::power_of_2(3u64),
    ///     BigExponentFloat::from(8)
    /// );
    ///
    /// // 2^k lies in [2^k, 2^(k + 1)), so its exponent is k + 1.
    /// assert_eq!(
    ///     BigExponentFloat::power_of_2(u64::MAX).get_exponent(),
    ///     Some(&(Integer::from(u64::MAX) + Integer::ONE))
    /// );
    /// ```
    #[inline]
    fn power_of_2(pow: u64) -> Self {
        Self::ONE << pow
    }
}

impl PowerOf2<i64> for BigExponentFloat {
    /// Raises 2 to an integer power.
    ///
    /// $f(k) = 2^k$.
    ///
    /// Unlike [`Float::power_of_2`](crate::Float#method.power_of_2), this never overflows or
    /// underflows.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     BigExponentFloat::power_of_2(0i64),
    ///     BigExponentFloat::from(1)
    /// );
    /// assert_eq!(
    ///     BigExponentFloat::power_of_2(3i64),
    ///     BigExponentFloat::from(8)
    /// );
    /// assert_eq!(
    ///     BigExponentFloat::power_of_2(-3i64),
    ///     BigExponentFloat::from(0.125)
    /// );
    ///
    /// // 2^k lies in [2^k, 2^(k + 1)), so its exponent is k + 1.
    /// assert_eq!(
    ///     BigExponentFloat::power_of_2(i64::MIN).get_exponent(),
    ///     Some(&(Integer::from(i64::MIN) + Integer::ONE))
    /// );
    /// ```
    #[inline]
    fn power_of_2(pow: i64) -> Self {
        Self::ONE << pow
    }
}
