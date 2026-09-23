// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::IntegerPolynomial;
use malachite_base::num::arithmetic::traits::IsUnit;

impl IsUnit for IntegerPolynomial {
    /// Determines whether an [`IntegerPolynomial`] is a unit: whether it is the constant polynomial
    /// 1 or $-1$, the only polynomials with integer coefficients that have a multiplicative
    /// inverse.
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
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(IntegerPolynomial::one().is_unit(), true);
    /// assert_eq!(IntegerPolynomial::negative_one().is_unit(), true);
    /// assert_eq!(IntegerPolynomial::ZERO.is_unit(), false);
    /// assert_eq!(IntegerPolynomial::two().is_unit(), false);
    /// assert_eq!(IntegerPolynomial::x().is_unit(), false);
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("-x-1").unwrap().is_unit(),
    ///     false
    /// );
    /// ```
    fn is_unit(&self) -> bool {
        match self.coefficients.as_slice() {
            [c] => c.is_unit(),
            _ => false,
        }
    }
}
