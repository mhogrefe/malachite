// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_rational::GaussianRational;
use malachite_base::num::arithmetic::traits::{
    CanonicalUnitIPow, CanonicalizeGaussianUnit, CanonicalizeGaussianUnitAssign, MulIPowAssign,
};

impl CanonicalizeGaussianUnit for GaussianRational {
    type Output = Self;

    /// Brings a [`GaussianRational`] into canonical Gaussian-unit form, taking it by value.
    ///
    /// The result is the Gaussian associate $x i^k$ whose argument lies in $(-\pi/4, \pi/4]$, where
    /// $k$ is given by
    /// [`canonical_unit_i_pow`](malachite_base::num::arithmetic::traits::CanonicalUnitIPow); zero
    /// is its own canonical form. This differs from
    /// [`canonicalize_unit`](malachite_base::num::arithmetic::traits::CanonicalizeUnit), which
    /// gives 1 for every nonzero [`GaussianRational`], since the Gaussian rationals form a field.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the numerators and denominators of the real and imaginary parts.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalizeGaussianUnit;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     GaussianRational::from_str("-1/2+3i/4")
    ///         .unwrap()
    ///         .canonicalize_gaussian_unit()
    ///         .to_string(),
    ///     "3/4+i/2"
    /// );
    /// assert_eq!(
    ///     GaussianRational::from_str("-3i")
    ///         .unwrap()
    ///         .canonicalize_gaussian_unit()
    ///         .to_string(),
    ///     "3"
    /// );
    /// ```
    #[inline]
    fn canonicalize_gaussian_unit(mut self) -> Self {
        self.canonicalize_gaussian_unit_assign();
        self
    }
}

impl CanonicalizeGaussianUnit for &GaussianRational {
    type Output = GaussianRational;

    /// Brings a [`GaussianRational`] into canonical Gaussian-unit form, taking it by reference.
    ///
    /// The result is the Gaussian associate $x i^k$ whose argument lies in $(-\pi/4, \pi/4]$, where
    /// $k$ is given by
    /// [`canonical_unit_i_pow`](malachite_base::num::arithmetic::traits::CanonicalUnitIPow); zero
    /// is its own canonical form. This differs from
    /// [`canonicalize_unit`](malachite_base::num::arithmetic::traits::CanonicalizeUnit), which
    /// gives 1 for every nonzero [`GaussianRational`], since the Gaussian rationals form a field.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the numerators and denominators of the real and imaginary parts.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalizeGaussianUnit;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("-1/2+3i/4").unwrap();
    /// assert_eq!((&x).canonicalize_gaussian_unit().to_string(), "3/4+i/2");
    /// ```
    #[inline]
    fn canonicalize_gaussian_unit(self) -> GaussianRational {
        self.clone().canonicalize_gaussian_unit()
    }
}

impl CanonicalizeGaussianUnitAssign for GaussianRational {
    /// Replaces a [`GaussianRational`] with its canonical Gaussian-unit form.
    ///
    /// See [`canonicalize_gaussian_unit`](CanonicalizeGaussianUnit::canonicalize_gaussian_unit).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the numerators and denominators of the real and imaginary parts.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalizeGaussianUnitAssign;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let mut x = GaussianRational::from_str("-1/2+3i/4").unwrap();
    /// x.canonicalize_gaussian_unit_assign();
    /// assert_eq!(x.to_string(), "3/4+i/2");
    /// ```
    fn canonicalize_gaussian_unit_assign(&mut self) {
        let k = self.canonical_unit_i_pow();
        self.mul_i_pow_assign(k);
    }
}
