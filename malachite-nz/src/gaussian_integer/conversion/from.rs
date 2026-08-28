// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use crate::integer::Integer;
use malachite_base::num::basic::traits::Zero;

impl<T> From<T> for GaussianInteger
where
    Integer: From<T>,
{
    /// Converts a value of any type that converts to an [`Integer`] — including [`Integer`]
    /// itself, via the standard library's reflexive [`From`] — to a purely real
    /// [`GaussianInteger`].
    ///
    /// # Worst-case complexity
    /// Same as the complexity of the corresponding [`Integer`] conversion.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(GaussianInteger::from(123u32).to_string(), "123");
    /// assert_eq!(GaussianInteger::from(-123i64).to_string(), "-123");
    /// assert_eq!(GaussianInteger::from(Natural::from(123u32)).to_string(), "123");
    /// assert_eq!(GaussianInteger::from(Integer::from(-123)).to_string(), "-123");
    /// ```
    fn from(x: T) -> Self {
        Self {
            real: Integer::from(x),
            imaginary: Integer::ZERO,
        }
    }
}
