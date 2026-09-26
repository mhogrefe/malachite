// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::arithmetic::traits::{CanonicalizeUnit, CanonicalizeUnitAssign};
use malachite_base::num::basic::traits::{One, Zero};

impl CanonicalizeUnit for Rational {
    type Output = Self;

    /// Brings a [`Rational`] into canonical unit form, taking it by value.
    ///
    /// The rationals form a field, so every nonzero [`Rational`] is a unit and its canonical
    /// associate is 1. Zero is its own canonical associate.
    ///
    /// $$
    /// f(x) = \begin{cases}
    ///     0 & \text{if} \quad x = 0, \\
    ///     1 & \text{otherwise}.
    /// \end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalizeUnit;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from_signeds(-22, 7).canonicalize_unit(), 1);
    /// assert_eq!(Rational::from_signeds(22, 7).canonicalize_unit(), 1);
    /// assert_eq!(Rational::ZERO.canonicalize_unit(), 0);
    /// ```
    #[inline]
    fn canonicalize_unit(mut self) -> Self {
        self.canonicalize_unit_assign();
        self
    }
}

impl CanonicalizeUnit for &Rational {
    type Output = Rational;

    /// Brings a [`Rational`] into canonical unit form, taking it by reference.
    ///
    /// The rationals form a field, so every nonzero [`Rational`] is a unit and its canonical
    /// associate is 1. Zero is its own canonical associate.
    ///
    /// $$
    /// f(x) = \begin{cases}
    ///     0 & \text{if} \quad x = 0, \\
    ///     1 & \text{otherwise}.
    /// \end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalizeUnit;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!((&Rational::from_signeds(-22, 7)).canonicalize_unit(), 1);
    /// assert_eq!((&Rational::from_signeds(22, 7)).canonicalize_unit(), 1);
    /// assert_eq!((&Rational::ZERO).canonicalize_unit(), 0);
    /// ```
    #[inline]
    fn canonicalize_unit(self) -> Rational {
        if *self == 0u32 {
            Rational::ZERO
        } else {
            Rational::ONE
        }
    }
}

impl CanonicalizeUnitAssign for Rational {
    /// Replaces a [`Rational`] with its canonical unit form.
    ///
    /// The rationals form a field, so every nonzero [`Rational`] is a unit and its canonical
    /// associate is 1. Zero is its own canonical associate.
    ///
    /// $$
    /// f(x) = \begin{cases}
    ///     0 & \text{if} \quad x = 0, \\
    ///     1 & \text{otherwise}.
    /// \end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalizeUnitAssign;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::from_signeds(-22, 7);
    /// x.canonicalize_unit_assign();
    /// assert_eq!(x, 1);
    /// ```
    #[inline]
    fn canonicalize_unit_assign(&mut self) {
        if *self != 0u32 {
            *self = Self::ONE;
        }
    }
}
