// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::num::arithmetic::traits::IsUnit;

impl IsUnit for NaturalPolynomial {
    /// Determines whether a [`NaturalPolynomial`] is a unit: whether it is the constant polynomial
    /// 1, the only polynomial with natural coefficients that has a multiplicative inverse.
    ///
    /// No modulus is involved, as with [`IsUnit`] for [`Natural`](crate::natural::Natural).
    /// Modulo $n$, every constant coprime to $n$ is also a unit, and so, when $n$ is composite, are
    /// some polynomials of positive degree.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::IsUnit;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(NaturalPolynomial::one().is_unit(), true);
    /// assert_eq!(NaturalPolynomial::ZERO.is_unit(), false);
    /// assert_eq!(NaturalPolynomial::two().is_unit(), false);
    /// assert_eq!(NaturalPolynomial::x().is_unit(), false);
    /// assert_eq!(NaturalPolynomial::from_str("x+1").unwrap().is_unit(), false);
    /// ```
    #[inline]
    fn is_unit(&self) -> bool {
        *self == 1u32
    }
}
