// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::num::arithmetic::traits::{CanonicalizeUnit, CanonicalizeUnitAssign};

impl CanonicalizeUnit for NaturalPolynomial {
    type Output = Self;

    /// Brings a [`NaturalPolynomial`] into canonical unit form, taking it by value.
    ///
    /// The coefficients are non-negative, so the leading coefficient already is, and the polynomial
    /// is its own canonical associate: this is the identity, as it is for the coefficient type.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CanonicalizeUnit;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("3*x^2+2")
    ///         .unwrap()
    ///         .canonicalize_unit()
    ///         .to_string(),
    ///     "3*x^2+2"
    /// );
    /// assert_eq!(
    ///     NaturalPolynomial::ZERO.canonicalize_unit(),
    ///     NaturalPolynomial::ZERO
    /// );
    /// ```
    #[inline]
    fn canonicalize_unit(self) -> Self {
        self
    }
}

impl CanonicalizeUnit for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Brings a [`NaturalPolynomial`] into canonical unit form, taking it by reference.
    ///
    /// The coefficients are non-negative, so the leading coefficient already is, and the polynomial
    /// is its own canonical associate: this is the identity, as it is for the coefficient type.
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
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("3*x^2+2").unwrap())
    ///         .canonicalize_unit()
    ///         .to_string(),
    ///     "3*x^2+2"
    /// );
    /// assert_eq!(
    ///     (&NaturalPolynomial::ZERO).canonicalize_unit(),
    ///     NaturalPolynomial::ZERO
    /// );
    /// ```
    #[inline]
    fn canonicalize_unit(self) -> NaturalPolynomial {
        self.clone()
    }
}

impl CanonicalizeUnitAssign for NaturalPolynomial {
    /// Brings a [`NaturalPolynomial`] into canonical unit form, in place.
    ///
    /// See [`canonicalize_unit`](CanonicalizeUnit::canonicalize_unit).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CanonicalizeUnitAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("3*x^2+2").unwrap();
    /// p.canonicalize_unit_assign();
    /// assert_eq!(p.to_string(), "3*x^2+2");
    /// ```
    #[inline]
    fn canonicalize_unit_assign(&mut self) {}
}
