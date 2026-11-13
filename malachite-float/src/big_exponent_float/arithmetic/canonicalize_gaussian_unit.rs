// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use malachite_base::num::arithmetic::traits::{
    Abs, AbsAssign, CanonicalizeGaussianUnit, CanonicalizeGaussianUnitAssign,
};

impl CanonicalizeGaussianUnit for BigExponentFloat {
    type Output = Self;

    /// Brings a [`BigExponentFloat`] into canonical Gaussian-unit form, taking it by value.
    ///
    /// The canonical Gaussian-unit form of a real number is its absolute value, so this clears the
    /// sign of every [`BigExponentFloat`], the infinities included, while NaN is left alone. This
    /// differs from
    /// [`canonicalize_unit`](malachite_base::num::arithmetic::traits::CanonicalizeUnit), which
    /// gives 1 for every finite nonzero [`BigExponentFloat`], since those are the units.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalizeGaussianUnit;
    /// use malachite_base::num::basic::traits::{Infinity, NegativeInfinity};
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     BigExponentFloat::from(-1.5).canonicalize_gaussian_unit(),
    ///     BigExponentFloat::from(1.5)
    /// );
    /// assert_eq!(
    ///     BigExponentFloat::NEGATIVE_INFINITY.canonicalize_gaussian_unit(),
    ///     BigExponentFloat::INFINITY
    /// );
    /// ```
    #[inline]
    fn canonicalize_gaussian_unit(self) -> Self {
        self.abs()
    }
}

impl CanonicalizeGaussianUnit for &BigExponentFloat {
    type Output = BigExponentFloat;

    /// Brings a [`BigExponentFloat`] into canonical Gaussian-unit form, taking it by reference.
    ///
    /// The canonical Gaussian-unit form of a real number is its absolute value, so this clears the
    /// sign of every [`BigExponentFloat`], the infinities included, while NaN is left alone. This
    /// differs from
    /// [`canonicalize_unit`](malachite_base::num::arithmetic::traits::CanonicalizeUnit), which
    /// gives 1 for every finite nonzero [`BigExponentFloat`], since those are the units.
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
    /// use malachite_base::num::basic::traits::{Infinity, NegativeInfinity};
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     (&BigExponentFloat::from(-1.5)).canonicalize_gaussian_unit(),
    ///     BigExponentFloat::from(1.5)
    /// );
    /// assert_eq!(
    ///     (&BigExponentFloat::NEGATIVE_INFINITY).canonicalize_gaussian_unit(),
    ///     BigExponentFloat::INFINITY
    /// );
    /// ```
    #[inline]
    fn canonicalize_gaussian_unit(self) -> BigExponentFloat {
        self.abs()
    }
}

impl CanonicalizeGaussianUnitAssign for BigExponentFloat {
    /// Brings a [`BigExponentFloat`] into canonical Gaussian-unit form, in place.
    ///
    /// The canonical Gaussian-unit form of a real number is its absolute value, so this clears the
    /// sign of every [`BigExponentFloat`], the infinities included, while NaN is left alone.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalizeGaussianUnitAssign;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(-1.5);
    /// x.canonicalize_gaussian_unit_assign();
    /// assert_eq!(x, BigExponentFloat::from(1.5));
    /// ```
    #[inline]
    fn canonicalize_gaussian_unit_assign(&mut self) {
        self.abs_assign();
    }
}
