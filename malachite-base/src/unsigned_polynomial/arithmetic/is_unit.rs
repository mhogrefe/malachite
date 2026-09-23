// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::IsUnit;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_polynomial::UnsignedPolynomial;

impl<T: PrimitiveUnsigned> IsUnit for UnsignedPolynomial<T> {
    /// Determines whether an [`UnsignedPolynomial`] is a unit: whether it is the constant
    /// polynomial 1, the only polynomial with non-negative integer coefficients that has a
    /// multiplicative inverse.
    ///
    /// No modulus is involved, as with [`IsUnit`] for the unsigned primitive types. Modulo $n$,
    /// every constant coprime to $n$ is also a unit, and so, when $n$ is composite, are some
    /// polynomials of positive degree.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::IsUnit;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::Polynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(UnsignedPolynomial::<u64>::one().is_unit(), true);
    /// assert_eq!(UnsignedPolynomial::<u64>::ZERO.is_unit(), false);
    /// assert_eq!(UnsignedPolynomial::<u64>::two().is_unit(), false);
    /// assert_eq!(UnsignedPolynomial::<u8>::x().is_unit(), false);
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x+1").unwrap().is_unit(),
    ///     false
    /// );
    /// ```
    #[inline]
    fn is_unit(&self) -> bool {
        *self == T::ONE
    }
}
