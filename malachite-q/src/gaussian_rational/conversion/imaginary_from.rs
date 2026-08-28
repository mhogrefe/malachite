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
use malachite_base::num::conversion::traits::ImaginaryFrom;

impl<T> ImaginaryFrom<T> for GaussianRational
where
    Rational: From<T>,
{
    /// Converts a value of any type that converts to a [`Rational`] — including [`Rational`]
    /// itself — to a purely imaginary [`GaussianRational`].
    ///
    /// # Worst-case complexity
    /// Same as the complexity of the corresponding [`Rational`] conversion.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::ImaginaryFrom;
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    /// use malachite_q::gaussian_rational::GaussianRational;
    ///
    /// assert_eq!(GaussianRational::imaginary_from(123u32).to_string(), "123i");
    /// assert_eq!(GaussianRational::imaginary_from(-123i64).to_string(), "-123i");
    /// assert_eq!(
    ///     GaussianRational::imaginary_from(Integer::from(-123)).to_string(),
    ///     "-123i"
    /// );
    /// assert_eq!(
    ///     GaussianRational::imaginary_from(Rational::from_signeds(-5, 6)).to_string(),
    ///     "-5i/6"
    /// );
    /// ```
    fn imaginary_from(x: T) -> Self {
        Self {
            real: Rational::ZERO,
            imaginary: Rational::from(x),
        }
    }
}
