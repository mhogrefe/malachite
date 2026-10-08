// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::arithmetic::traits::{
    Abs, AbsAssign, CanonicalizeGaussianUnit, CanonicalizeGaussianUnitAssign,
};

impl CanonicalizeGaussianUnit for Rational {
    type Output = Self;

    /// Brings a [`Rational`] into canonical Gaussian-unit form, taking it by value.
    ///
    /// The canonical Gaussian-unit form of a real number is its absolute value. This differs from
    /// [`canonicalize_unit`](malachite_base::num::arithmetic::traits::CanonicalizeUnit), which
    /// gives 1 for every nonzero [`Rational`], since the rationals form a field.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalizeGaussianUnit;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::from_signeds(-22, 7).canonicalize_gaussian_unit(),
    ///     Rational::from_signeds(22, 7)
    /// );
    /// ```
    #[inline]
    fn canonicalize_gaussian_unit(self) -> Self {
        self.abs()
    }
}

impl CanonicalizeGaussianUnit for &Rational {
    type Output = Rational;

    /// Brings a [`Rational`] into canonical Gaussian-unit form, taking it by reference.
    ///
    /// The canonical Gaussian-unit form of a real number is its absolute value. This differs from
    /// [`canonicalize_unit`](malachite_base::num::arithmetic::traits::CanonicalizeUnit), which
    /// gives 1 for every nonzero [`Rational`], since the rationals form a field.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalizeGaussianUnit;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     (&Rational::from_signeds(-22, 7)).canonicalize_gaussian_unit(),
    ///     Rational::from_signeds(22, 7)
    /// );
    /// ```
    #[inline]
    fn canonicalize_gaussian_unit(self) -> Rational {
        self.abs()
    }
}

impl CanonicalizeGaussianUnitAssign for Rational {
    /// Replaces a [`Rational`] with its canonical Gaussian-unit form: its absolute value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalizeGaussianUnitAssign;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::from_signeds(-22, 7);
    /// x.canonicalize_gaussian_unit_assign();
    /// assert_eq!(x, Rational::from_signeds(22, 7));
    /// ```
    #[inline]
    fn canonicalize_gaussian_unit_assign(&mut self) {
        self.abs_assign();
    }
}
