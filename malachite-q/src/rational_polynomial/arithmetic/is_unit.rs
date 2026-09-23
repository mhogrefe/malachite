// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use malachite_base::num::arithmetic::traits::IsUnit;

impl IsUnit for RationalPolynomial {
    /// Determines whether a [`RationalPolynomial`] is a unit: whether it is a nonzero constant,
    /// which is exactly when it has a multiplicative inverse, since the coefficients form a field.
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
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(RationalPolynomial::one().is_unit(), true);
    /// assert_eq!(RationalPolynomial::two().is_unit(), true);
    /// assert_eq!(RationalPolynomial::one_half().is_unit(), true);
    /// assert_eq!(
    ///     RationalPolynomial::from_str("-22/7").unwrap().is_unit(),
    ///     true
    /// );
    /// assert_eq!(RationalPolynomial::ZERO.is_unit(), false);
    /// assert_eq!(RationalPolynomial::x().is_unit(), false);
    /// assert_eq!(
    ///     RationalPolynomial::from_str("1/2*x+1").unwrap().is_unit(),
    ///     false
    /// );
    /// ```
    #[inline]
    fn is_unit(&self) -> bool {
        self.numerator.coefficients_asc().len() == 1
    }
}
