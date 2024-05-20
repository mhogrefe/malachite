// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;

impl Integer {
    /// Converts a sign and a [`Natural`] to an [`Integer`], taking the [`Natural`] by value. The
    /// [`Natural`] becomes the [`Integer`]'s absolute value, and the sign indicates whether the
    /// [`Integer`] should be non-negative. If the [`Natural`] is zero, then the [`Integer`] will be
    /// non-negative regardless of the sign.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Integer::from_sign_and_abs(true, Natural::from(123u32)), 123);
    /// assert_eq!(
    ///     Integer::from_sign_and_abs(false, Natural::from(123u32)),
    ///     -123
    /// );
    /// ```
    pub fn from_sign_and_abs(sign: bool, abs: Natural) -> Integer {
        Integer {
            sign: sign || abs == 0,
            abs,
        }
    }

    /// Converts a sign and an [`Natural`] to an [`Integer`], taking the [`Natural`] by reference.
    /// The [`Natural`] becomes the [`Integer`]'s absolute value, and the sign indicates whether the
    /// [`Integer`] should be non-negative. If the [`Natural`] is zero, then the [`Integer`] will be
    /// non-negative regardless of the sign.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `abs.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Integer::from_sign_and_abs_ref(true, &Natural::from(123u32)),
    ///     123
    /// );
    /// assert_eq!(
    ///     Integer::from_sign_and_abs_ref(false, &Natural::from(123u32)),
    ///     -123
    /// );
    /// ```
    pub fn from_sign_and_abs_ref(sign: bool, abs: &Natural) -> Integer {
        Integer {
            sign: sign || *abs == 0,
            abs: abs.clone(),
        }
    }
}

impl From<Natural> for Integer {
    /// Converts a [`Natural`] to an [`Integer`], taking the [`Natural`] by value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Integer::from(Natural::from(123u32)), 123);
    /// assert_eq!(
    ///     Integer::from(Natural::from(10u32).pow(12)),
    ///     1000000000000u64
    /// );
    /// ```
    fn from(value: Natural) -> Integer {
        Integer {
            sign: true,
            abs: value,
        }
    }
}

impl<'a> From<&'a Natural> for Integer {
    /// Converts a [`Natural`] to an [`Integer`], taking the [`Natural`] by reference.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `value.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Integer::from(&Natural::from(123u32)), 123);
    /// assert_eq!(
    ///     Integer::from(&Natural::from(10u32).pow(12)),
    ///     1000000000000u64
    /// );
    /// ```
    fn from(value: &'a Natural) -> Integer {
        Integer {
            sign: true,
            abs: value.clone(),
        }
    }
}
