// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::gaussian_rational::GaussianRational;
use malachite_base::num::arithmetic::traits::AbsSquared;
use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};

impl EqAbs<Rational> for GaussianRational {
    /// Determines whether the absolute values of a [`GaussianRational`] and a [`Rational`] are
    /// equal.
    ///
    /// The absolute value of a complex number is its distance from the origin, so two values are
    /// equal in absolute value exactly when their squared absolute values are equal. Purely real
    /// and purely imaginary values are handled by comparing single components. Otherwise, equality
    /// is impossible unless both components are smaller in absolute value than the real operand, so
    /// the squared absolute values are only computed in that case.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and of `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};
    /// use malachite_q::Rational;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// // |3/5+4i/5| = 1
    /// let x = GaussianRational::from_str("3/5+4i/5").unwrap();
    /// assert!(x.eq_abs(&Rational::from(-1)));
    /// assert_eq!(x.eq_abs(&Rational::from_signeds(1, 2)), false);
    /// ```
    fn eq_abs(&self, other: &Rational) -> bool {
        if self.imaginary == 0u32 {
            self.real.eq_abs(other)
        } else if self.real == 0u32 {
            self.imaginary.eq_abs(other)
        } else {
            self.real.lt_abs(other)
                && self.imaginary.lt_abs(other)
                && self.abs_squared() == other.abs_squared()
        }
    }
}

impl EqAbs<GaussianRational> for Rational {
    /// Determines whether the absolute values of a [`Rational`] and a [`GaussianRational`] are
    /// equal.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `other` and of `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};
    /// use malachite_q::Rational;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// // |3/5+4i/5| = 1
    /// let y = GaussianRational::from_str("3/5+4i/5").unwrap();
    /// assert!(Rational::from(-1).eq_abs(&y));
    /// assert_eq!(Rational::from_signeds(1, 2).eq_abs(&y), false);
    /// ```
    #[inline]
    fn eq_abs(&self, other: &GaussianRational) -> bool {
        other.eq_abs(self)
    }
}
