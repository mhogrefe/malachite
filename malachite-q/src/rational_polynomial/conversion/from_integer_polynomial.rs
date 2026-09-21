// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use malachite_base::num::basic::traits::One;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;

impl From<IntegerPolynomial> for RationalPolynomial {
    /// Converts an [`IntegerPolynomial`] to a [`RationalPolynomial`].
    ///
    /// Every polynomial with [`Integer`](malachite_nz::integer::Integer) coefficients is one with
    /// [`Rational`](crate::Rational) coefficients, so nothing is lost and nothing can fail. A
    /// [`RationalPolynomial`] is a numerator and a denominator, and the polynomial given is already
    /// the numerator it needs, so it is moved rather than copied and the denominator is 1. That
    /// pair is canonical whatever the numerator is, since everything is coprime with 1.
    ///
    /// $f(p) = p$, read on the left over $\Z$ and on the right over $\Q$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^2-3*x+2").unwrap();
    /// assert_eq!(RationalPolynomial::from(p).to_string(), "x^2-3*x+2");
    ///
    /// assert_eq!(
    ///     RationalPolynomial::from(IntegerPolynomial::default()).to_string(),
    ///     "0"
    /// );
    /// ```
    #[inline]
    fn from(p: IntegerPolynomial) -> Self {
        // A denominator of 1 is coprime with every content, and the zero polynomial's denominator
        // must be 1 anyway, so this pair is canonical and needs no reduction.
        Self {
            numerator: p,
            denominator: Natural::ONE,
        }
    }
}
