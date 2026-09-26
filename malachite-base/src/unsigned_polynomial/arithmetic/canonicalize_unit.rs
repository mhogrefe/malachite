// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{CanonicalizeUnit, CanonicalizeUnitAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_polynomial::UnsignedPolynomial;

impl<T: PrimitiveUnsigned> CanonicalizeUnit for UnsignedPolynomial<T> {
    type Output = Self;

    /// Brings an [`UnsignedPolynomial`] into canonical unit form, taking it by value.
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
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("3*x^2+2")
    ///         .unwrap()
    ///         .canonicalize_unit()
    ///         .to_string(),
    ///     "3*x^2+2"
    /// );
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::ZERO.canonicalize_unit(),
    ///     UnsignedPolynomial::<u8>::ZERO
    /// );
    /// ```
    #[inline]
    fn canonicalize_unit(self) -> Self {
        self
    }
}

impl<T: PrimitiveUnsigned> CanonicalizeUnit for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Brings an [`UnsignedPolynomial`] into canonical unit form, taking it by reference.
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
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("3*x^2+2").unwrap())
    ///         .canonicalize_unit()
    ///         .to_string(),
    ///     "3*x^2+2"
    /// );
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::ZERO).canonicalize_unit(),
    ///     UnsignedPolynomial::<u8>::ZERO
    /// );
    /// ```
    #[inline]
    fn canonicalize_unit(self) -> UnsignedPolynomial<T> {
        self.clone()
    }
}

impl<T: PrimitiveUnsigned> CanonicalizeUnitAssign for UnsignedPolynomial<T> {
    /// Brings an [`UnsignedPolynomial`] into canonical unit form, in place.
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
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("3*x^2+2").unwrap();
    /// p.canonicalize_unit_assign();
    /// assert_eq!(p.to_string(), "3*x^2+2");
    /// ```
    #[inline]
    fn canonicalize_unit_assign(&mut self) {}
}
