// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_rational::GaussianRational;
use malachite_base::num::arithmetic::traits::{
    CanonicalUnitIPow, CanonicalizeUnit, CanonicalizeUnitAssign, MulIPowAssign,
};

impl CanonicalizeUnit for GaussianRational {
    type Output = Self;

    /// Brings a [`GaussianRational`] into canonical unit form, taking it by value.
    ///
    /// The result is the associate $x i^k$ whose argument lies in $(-\pi/4, \pi/4]$, where $k$ is
    /// given by
    /// [`canonical_unit_i_pow`](malachite_base::num::arithmetic::traits::CanonicalUnitIPow); zero
    /// is its own canonical form.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalizeUnit;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     GaussianRational::from_str("-1+2i")
    ///         .unwrap()
    ///         .canonicalize_unit()
    ///         .to_string(),
    ///     "2+i"
    /// );
    /// assert_eq!(
    ///     GaussianRational::from_str("1-i")
    ///         .unwrap()
    ///         .canonicalize_unit()
    ///         .to_string(),
    ///     "1+i"
    /// );
    /// assert_eq!(
    ///     GaussianRational::from_str("-3")
    ///         .unwrap()
    ///         .canonicalize_unit()
    ///         .to_string(),
    ///     "3"
    /// );
    /// ```
    #[inline]
    fn canonicalize_unit(mut self) -> Self {
        self.canonicalize_unit_assign();
        self
    }
}

impl CanonicalizeUnit for &GaussianRational {
    type Output = GaussianRational;

    /// Brings a [`GaussianRational`] into canonical unit form, taking it by reference.
    ///
    /// The result is the associate $x i^k$ whose argument lies in $(-\pi/4, \pi/4]$, where $k$ is
    /// given by
    /// [`canonical_unit_i_pow`](malachite_base::num::arithmetic::traits::CanonicalUnitIPow); zero
    /// is its own canonical form.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalizeUnit;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("-1+2i").unwrap();
    /// assert_eq!((&x).canonicalize_unit().to_string(), "2+i");
    /// ```
    #[inline]
    fn canonicalize_unit(self) -> GaussianRational {
        self.clone().canonicalize_unit()
    }
}

impl CanonicalizeUnitAssign for GaussianRational {
    /// Brings a [`GaussianRational`] into canonical unit form in place.
    ///
    /// The result is the associate $x i^k$ whose argument lies in $(-\pi/4, \pi/4]$, where $k$ is
    /// given by
    /// [`canonical_unit_i_pow`](malachite_base::num::arithmetic::traits::CanonicalUnitIPow); zero
    /// is its own canonical form.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalizeUnitAssign;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let mut x = GaussianRational::from_str("-1+2i").unwrap();
    /// x.canonicalize_unit_assign();
    /// assert_eq!(x.to_string(), "2+i");
    /// ```
    fn canonicalize_unit_assign(&mut self) {
        let k = self.canonical_unit_i_pow();
        self.mul_i_pow_assign(k);
    }
}
