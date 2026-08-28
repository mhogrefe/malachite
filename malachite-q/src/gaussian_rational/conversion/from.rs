// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::gaussian_rational::GaussianRational;
use malachite_base::num::basic::traits::Zero;

impl<T> From<T> for GaussianRational
where
    Rational: From<T>,
{
    /// Converts a value of any type that converts to a [`Rational`] — including [`Rational`]
    /// itself, via the standard library's reflexive [`From`] — to a purely real
    /// [`GaussianRational`].
    ///
    /// # Worst-case complexity
    /// Same as the complexity of the corresponding [`Rational`] conversion.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    /// use malachite_q::gaussian_rational::GaussianRational;
    ///
    /// assert_eq!(GaussianRational::from(123u32).to_string(), "123");
    /// assert_eq!(GaussianRational::from(-123i64).to_string(), "-123");
    /// assert_eq!(GaussianRational::from(Integer::from(-123)).to_string(), "-123");
    /// assert_eq!(
    ///     GaussianRational::from(Rational::from_signeds(-5, 6)).to_string(),
    ///     "-5/6"
    /// );
    /// ```
    fn from(x: T) -> Self {
        Self {
            real: Rational::from(x),
            imaginary: Rational::ZERO,
        }
    }
}
