// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2011 Sebastian Pancratz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use crate::rational_polynomial::arithmetic::div_exact::checked_div_exact_helper;
use alloc::borrow::Cow;
use malachite_base::num::arithmetic::traits::DivisibleBy;
use malachite_base::num::basic::traits::Zero;

// Whether `q` divides `p`. Zero divides only zero.
//
// This is equivalent to `fmpq_poly_divides` from `fmpq_poly/divides.c`, FLINT 3.6.0, returning only
// the flag.
fn divisible_by_helper(p: &RationalPolynomial, q: &RationalPolynomial) -> bool {
    *p == RationalPolynomial::ZERO || checked_div_exact_helper(Cow::Borrowed(p), q).is_some()
}

impl DivisibleBy<Self> for RationalPolynomial {
    /// Determines whether a [`RationalPolynomial`] is divisible by another, taking both by value.
    /// Zero is divisible by every polynomial, and only zero is divisible by zero.
    ///
    /// $$
    /// f(a, b) = (\exists c \in \mathbb{Q}\[x\] : a = bc).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerators and denominators of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivisibleBy;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert!(
    ///     (RationalPolynomial::from_str("x^2-1/4").unwrap())
    ///         .divisible_by(RationalPolynomial::from_str("x-1/2").unwrap())
    /// );
    /// assert!(
    ///     !(RationalPolynomial::from_str("x^2+1").unwrap())
    ///         .divisible_by(RationalPolynomial::from_str("x+1").unwrap())
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_divides` from `fmpq_poly/divides.c`, FLINT 3.6.0, returning
    /// only the flag.
    #[inline]
    fn divisible_by(self, other: Self) -> bool {
        divisible_by_helper(&self, &other)
    }
}

impl DivisibleBy<&Self> for RationalPolynomial {
    /// Determines whether a [`RationalPolynomial`] is divisible by another, taking the first by
    /// value and the second by reference. Zero is divisible by every polynomial, and only zero is
    /// divisible by zero.
    ///
    /// $$
    /// f(a, b) = (\exists c \in \mathbb{Q}\[x\] : a = bc).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerators and denominators of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivisibleBy;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert!(
    ///     (RationalPolynomial::from_str("x^2-1/4").unwrap())
    ///         .divisible_by(&RationalPolynomial::from_str("x-1/2").unwrap())
    /// );
    /// assert!(
    ///     !(RationalPolynomial::from_str("x^2+1").unwrap())
    ///         .divisible_by(&RationalPolynomial::from_str("x+1").unwrap())
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_divides` from `fmpq_poly/divides.c`, FLINT 3.6.0, returning
    /// only the flag.
    #[inline]
    fn divisible_by(self, other: &Self) -> bool {
        divisible_by_helper(&self, other)
    }
}

impl DivisibleBy<RationalPolynomial> for &RationalPolynomial {
    /// Determines whether a [`RationalPolynomial`] is divisible by another, taking the first by
    /// reference and the second by value. Zero is divisible by every polynomial, and only zero is
    /// divisible by zero.
    ///
    /// $$
    /// f(a, b) = (\exists c \in \mathbb{Q}\[x\] : a = bc).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerators and denominators of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivisibleBy;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert!(
    ///     (&RationalPolynomial::from_str("x^2-1/4").unwrap())
    ///         .divisible_by(RationalPolynomial::from_str("x-1/2").unwrap())
    /// );
    /// assert!(
    ///     !(&RationalPolynomial::from_str("x^2+1").unwrap())
    ///         .divisible_by(RationalPolynomial::from_str("x+1").unwrap())
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_divides` from `fmpq_poly/divides.c`, FLINT 3.6.0, returning
    /// only the flag.
    #[inline]
    fn divisible_by(self, other: RationalPolynomial) -> bool {
        divisible_by_helper(self, &other)
    }
}

impl DivisibleBy<&RationalPolynomial> for &RationalPolynomial {
    /// Determines whether a [`RationalPolynomial`] is divisible by another, taking both by
    /// reference. Zero is divisible by every polynomial, and only zero is divisible by zero.
    ///
    /// $$
    /// f(a, b) = (\exists c \in \mathbb{Q}\[x\] : a = bc).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerators and denominators of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivisibleBy;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert!(
    ///     (&RationalPolynomial::from_str("x^2-1/4").unwrap())
    ///         .divisible_by(&RationalPolynomial::from_str("x-1/2").unwrap())
    /// );
    /// assert!(
    ///     !(&RationalPolynomial::from_str("x^2+1").unwrap())
    ///         .divisible_by(&RationalPolynomial::from_str("x+1").unwrap())
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_divides` from `fmpq_poly/divides.c`, FLINT 3.6.0, returning
    /// only the flag.
    #[inline]
    fn divisible_by(self, other: &RationalPolynomial) -> bool {
        divisible_by_helper(self, other)
    }
}
