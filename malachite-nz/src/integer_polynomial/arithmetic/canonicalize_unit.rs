// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::IntegerPolynomial;
use malachite_base::num::arithmetic::traits::{
    CanonicalizeUnit, CanonicalizeUnitAssign, NegAssign,
};
use malachite_base::polynomial::Polynomial;

impl CanonicalizeUnit for IntegerPolynomial {
    type Output = Self;

    /// Brings an [`IntegerPolynomial`] into canonical unit form, taking it by value.
    ///
    /// The canonical associate is the one whose leading coefficient is non-negative, so a
    /// polynomial with a negative leading coefficient is negated and any other is left alone. The
    /// zero polynomial is its own canonical associate.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CanonicalizeUnit;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("-3*x^2+2")
    ///         .unwrap()
    ///         .canonicalize_unit()
    ///         .to_string(),
    ///     "3*x^2-2"
    /// );
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("3*x^2-2")
    ///         .unwrap()
    ///         .canonicalize_unit()
    ///         .to_string(),
    ///     "3*x^2-2"
    /// );
    /// assert_eq!(
    ///     IntegerPolynomial::ZERO.canonicalize_unit(),
    ///     IntegerPolynomial::ZERO
    /// );
    /// ```
    #[inline]
    fn canonicalize_unit(mut self) -> Self {
        self.canonicalize_unit_assign();
        self
    }
}

impl CanonicalizeUnit for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Brings an [`IntegerPolynomial`] into canonical unit form, taking it by reference.
    ///
    /// The canonical associate is the one whose leading coefficient is non-negative, so a
    /// polynomial with a negative leading coefficient is negated and any other is left alone. The
    /// zero polynomial is its own canonical associate.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total size of the coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CanonicalizeUnit;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("-3*x^2+2").unwrap())
    ///         .canonicalize_unit()
    ///         .to_string(),
    ///     "3*x^2-2"
    /// );
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("3*x^2-2").unwrap())
    ///         .canonicalize_unit()
    ///         .to_string(),
    ///     "3*x^2-2"
    /// );
    /// assert_eq!(
    ///     (&IntegerPolynomial::ZERO).canonicalize_unit(),
    ///     IntegerPolynomial::ZERO
    /// );
    /// ```
    #[inline]
    fn canonicalize_unit(self) -> IntegerPolynomial {
        if *self.leading_coefficient() < 0u32 {
            -self
        } else {
            self.clone()
        }
    }
}

impl CanonicalizeUnitAssign for IntegerPolynomial {
    /// Brings an [`IntegerPolynomial`] into canonical unit form, in place.
    ///
    /// See [`canonicalize_unit`](CanonicalizeUnit::canonicalize_unit).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CanonicalizeUnitAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("-3*x^2+2").unwrap();
    /// p.canonicalize_unit_assign();
    /// assert_eq!(p.to_string(), "3*x^2-2");
    /// ```
    #[inline]
    fn canonicalize_unit_assign(&mut self) {
        if *self.leading_coefficient() < 0u32 {
            self.neg_assign();
        }
    }
}
