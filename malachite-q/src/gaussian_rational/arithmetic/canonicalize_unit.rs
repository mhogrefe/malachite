// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_rational::GaussianRational;
use malachite_base::num::arithmetic::traits::{CanonicalizeUnit, CanonicalizeUnitAssign};
use malachite_base::num::basic::traits::{One, Zero};

impl CanonicalizeUnit for GaussianRational {
    type Output = Self;

    /// Brings a [`GaussianRational`] into canonical unit form, taking it by value.
    ///
    /// The Gaussian rationals form a field, so every nonzero [`GaussianRational`] is a unit and its
    /// canonical associate is 1. Zero is its own canonical associate.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalizeUnit;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     GaussianRational::from_str("-1+2i")
    ///         .unwrap()
    ///         .canonicalize_unit()
    ///         .to_string(),
    ///     "1"
    /// );
    /// assert_eq!(
    ///     GaussianRational::ZERO.canonicalize_unit(),
    ///     GaussianRational::ZERO
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
    /// The Gaussian rationals form a field, so every nonzero [`GaussianRational`] is a unit and its
    /// canonical associate is 1. Zero is its own canonical associate.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalizeUnit;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     (&GaussianRational::from_str("-1+2i").unwrap())
    ///         .canonicalize_unit()
    ///         .to_string(),
    ///     "1"
    /// );
    /// assert_eq!(
    ///     (&GaussianRational::ZERO).canonicalize_unit(),
    ///     GaussianRational::ZERO
    /// );
    /// ```
    #[inline]
    fn canonicalize_unit(self) -> GaussianRational {
        if *self == 0u32 {
            GaussianRational::ZERO
        } else {
            GaussianRational::ONE
        }
    }
}

impl CanonicalizeUnitAssign for GaussianRational {
    /// Replaces a [`GaussianRational`] with its canonical unit form.
    ///
    /// The Gaussian rationals form a field, so every nonzero [`GaussianRational`] is a unit and its
    /// canonical associate is 1. Zero is its own canonical associate.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalizeUnitAssign;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let mut x = GaussianRational::from_str("-1+2i").unwrap();
    /// x.canonicalize_unit_assign();
    /// assert_eq!(x.to_string(), "1");
    /// ```
    #[inline]
    fn canonicalize_unit_assign(&mut self) {
        if *self != 0u32 {
            *self = Self::ONE;
        }
    }
}
