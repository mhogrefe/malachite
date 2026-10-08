// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use malachite_base::num::arithmetic::traits::{
    Abs, AbsAssign, CanonicalizeGaussianUnit, CanonicalizeGaussianUnitAssign,
};

impl CanonicalizeGaussianUnit for Float {
    type Output = Self;

    /// Brings a [`Float`] into canonical Gaussian-unit form, taking it by value.
    ///
    /// The canonical Gaussian-unit form of a real number is its absolute value, so this clears the
    /// sign bit of every [`Float`], the special values included: $-0.0$ becomes $0.0$ and $-\infty$
    /// becomes $\infty$, while NaN is left alone. This differs from
    /// [`canonicalize_unit`](malachite_base::num::arithmetic::traits::CanonicalizeUnit), which
    /// gives 1 for every finite nonzero [`Float`], since those are the units.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalizeGaussianUnit;
    /// use malachite_base::num::basic::traits::NegativeInfinity;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::from(-1.5).canonicalize_gaussian_unit(), 1.5);
    /// assert_eq!(Float::from(1.5).canonicalize_gaussian_unit(), 1.5);
    /// assert_eq!(
    ///     Float::NEGATIVE_INFINITY
    ///         .canonicalize_gaussian_unit()
    ///         .to_string(),
    ///     "Infinity"
    /// );
    /// ```
    #[inline]
    fn canonicalize_gaussian_unit(self) -> Self {
        self.abs()
    }
}

impl CanonicalizeGaussianUnit for &Float {
    type Output = Float;

    /// Brings a [`Float`] into canonical Gaussian-unit form, taking it by reference.
    ///
    /// The canonical Gaussian-unit form of a real number is its absolute value, so this clears the
    /// sign bit of every [`Float`], the special values included: $-0.0$ becomes $0.0$ and $-\infty$
    /// becomes $\infty$, while NaN is left alone. This differs from
    /// [`canonicalize_unit`](malachite_base::num::arithmetic::traits::CanonicalizeUnit), which
    /// gives 1 for every finite nonzero [`Float`], since those are the units.
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
    /// use malachite_base::num::basic::traits::NegativeInfinity;
    /// use malachite_float::Float;
    ///
    /// assert_eq!((&Float::from(-1.5)).canonicalize_gaussian_unit(), 1.5);
    /// assert_eq!((&Float::from(1.5)).canonicalize_gaussian_unit(), 1.5);
    /// assert_eq!(
    ///     (&Float::NEGATIVE_INFINITY)
    ///         .canonicalize_gaussian_unit()
    ///         .to_string(),
    ///     "Infinity"
    /// );
    /// ```
    #[inline]
    fn canonicalize_gaussian_unit(self) -> Float {
        self.abs()
    }
}

impl CanonicalizeGaussianUnitAssign for Float {
    /// Replaces a [`Float`] with its canonical Gaussian-unit form: its absolute value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalizeGaussianUnitAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(-1.5);
    /// x.canonicalize_gaussian_unit_assign();
    /// assert_eq!(x, 1.5);
    /// ```
    #[inline]
    fn canonicalize_gaussian_unit_assign(&mut self) {
        self.abs_assign();
    }
}
