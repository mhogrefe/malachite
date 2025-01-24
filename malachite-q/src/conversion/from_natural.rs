// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::basic::traits::One;
use malachite_nz::natural::Natural;

impl From<Natural> for Rational {
    /// Converts a [`Natural`] to a [`Rational`], taking the [`Natural`] by value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from(Natural::from(123u32)), 123);
    /// ```
    fn from(value: Natural) -> Rational {
        Rational {
            sign: true,
            numerator: value,
            denominator: Natural::ONE,
        }
    }
}

impl From<&Natural> for Rational {
    /// Converts a [`Natural`] to a [`Rational`], taking the [`Natural`] by reference.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from(&Natural::from(123u32)), 123);
    /// ```
    fn from(value: &Natural) -> Rational {
        Rational {
            sign: true,
            numerator: value.clone(),
            denominator: Natural::ONE,
        }
    }
}
