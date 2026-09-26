// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use malachite_base::num::arithmetic::traits::{CanonicalizeUnit, CanonicalizeUnitAssign};
use malachite_base::polynomial::{MakeMonic, MakeMonicAssign};

impl CanonicalizeUnit for RationalPolynomial {
    type Output = Self;

    /// Brings a [`RationalPolynomial`] into canonical unit form, taking it by value.
    ///
    /// The units of the polynomials over the rationals are the nonzero constants, so the canonical
    /// associate of a nonzero polynomial is its monic multiple, the one whose leading coefficient
    /// is 1. This is [`make_monic`](MakeMonic::make_monic). The zero polynomial is its own
    /// canonical associate.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerator's coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CanonicalizeUnit;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     RationalPolynomial::from_str("-1/2*x^2+3")
    ///         .unwrap()
    ///         .canonicalize_unit()
    ///         .to_string(),
    ///     "x^2-6"
    /// );
    /// assert_eq!(
    ///     RationalPolynomial::from_str("6*x+4")
    ///         .unwrap()
    ///         .canonicalize_unit()
    ///         .to_string(),
    ///     "x+2/3"
    /// );
    /// assert_eq!(
    ///     RationalPolynomial::ZERO.canonicalize_unit(),
    ///     RationalPolynomial::ZERO
    /// );
    /// ```
    #[inline]
    fn canonicalize_unit(self) -> Self {
        self.make_monic()
    }
}

impl CanonicalizeUnit for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Brings a [`RationalPolynomial`] into canonical unit form, taking it by reference.
    ///
    /// The units of the polynomials over the rationals are the nonzero constants, so the canonical
    /// associate of a nonzero polynomial is its monic multiple, the one whose leading coefficient
    /// is 1. This is [`make_monic`](MakeMonic::make_monic). The zero polynomial is its own
    /// canonical associate.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerator's coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CanonicalizeUnit;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("-1/2*x^2+3").unwrap())
    ///         .canonicalize_unit()
    ///         .to_string(),
    ///     "x^2-6"
    /// );
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("6*x+4").unwrap())
    ///         .canonicalize_unit()
    ///         .to_string(),
    ///     "x+2/3"
    /// );
    /// assert_eq!(
    ///     (&RationalPolynomial::ZERO).canonicalize_unit(),
    ///     RationalPolynomial::ZERO
    /// );
    /// ```
    #[inline]
    fn canonicalize_unit(self) -> RationalPolynomial {
        self.make_monic()
    }
}

impl CanonicalizeUnitAssign for RationalPolynomial {
    /// Brings a [`RationalPolynomial`] into canonical unit form, in place.
    ///
    /// The units of the polynomials over the rationals are the nonzero constants, so the canonical
    /// associate of a nonzero polynomial is its monic multiple, the one whose leading coefficient
    /// is 1. This is [`make_monic`](MakeMonic::make_monic). The zero polynomial is its own
    /// canonical associate.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerator's coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CanonicalizeUnitAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("-1/2*x^2+3").unwrap();
    /// p.canonicalize_unit_assign();
    /// assert_eq!(p.to_string(), "x^2-6");
    /// ```
    #[inline]
    fn canonicalize_unit_assign(&mut self) {
        self.make_monic_assign();
    }
}
