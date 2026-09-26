// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use core::ops::Neg;
use malachite_base::num::arithmetic::traits::NegAssign;

impl Neg for RationalPolynomial {
    type Output = Self;

    /// Negates a [`RationalPolynomial`], taking it by value.
    ///
    /// $$
    /// f(p) = -p.
    /// $$
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2-1/3").unwrap();
    /// assert_eq!((-p).to_string(), "-1/2*x^2+1/3");
    /// assert_eq!(-RationalPolynomial::ZERO, RationalPolynomial::ZERO);
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_neg` from `fmpq_poly/neg.c`, FLINT 3.6.0.
    fn neg(mut self) -> Self {
        // The content is unchanged by the sign, so the pair stays canonical
        self.numerator.neg_assign();
        self
    }
}

impl Neg for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Negates a [`RationalPolynomial`], taking it by reference.
    ///
    /// $$
    /// f(p) = -p.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/2*x^2-1/3").unwrap();
    /// assert_eq!((-&p).to_string(), "-1/2*x^2+1/3");
    /// assert_eq!(-&RationalPolynomial::ZERO, RationalPolynomial::ZERO);
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_neg` from `fmpq_poly/neg.c`, FLINT 3.6.0.
    fn neg(self) -> RationalPolynomial {
        RationalPolynomial {
            numerator: -&self.numerator,
            denominator: self.denominator.clone(),
        }
    }
}

impl NegAssign for RationalPolynomial {
    /// Negates a [`RationalPolynomial`] in place.
    ///
    /// $$
    /// p \gets -p.
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::NegAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x^2-1/3").unwrap();
    /// p.neg_assign();
    /// assert_eq!(p.to_string(), "-1/2*x^2+1/3");
    /// ```
    fn neg_assign(&mut self) {
        self.numerator.neg_assign();
    }
}
