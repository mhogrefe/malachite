// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_polynomial::RationalPolynomial;
use crate::rational_vector::arithmetic::content::content_over;
use malachite_base::num::arithmetic::traits::{
    CanonicalPrimitivePart, ContentAndCanonicalPrimitivePart,
};
use malachite_nz::integer_polynomial::IntegerPolynomial;

impl CanonicalPrimitivePart for RationalPolynomial {
    type Output = IntegerPolynomial;

    /// Computes the canonical primitive part of a [`RationalPolynomial`], taking the polynomial by
    /// value.
    ///
    /// This is the polynomial divided by its content, with the sign chosen so that the leading
    /// coefficient is non-negative. It always has integer coefficients, so it is an
    /// [`IntegerPolynomial`]; for $p = A/d$ it is the canonical primitive part of $A$, and the
    /// denominator plays no part.
    ///
    /// $$
    /// p = \operatorname{sgn}(\operatorname{lc}(p)) \operatorname{cont}(p) \operatorname{cpp}(p).
    /// $$
    ///
    /// The canonical primitive part of the zero polynomial is zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerator's coefficients and the denominator.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CanonicalPrimitivePart;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("-2/3*x-4/3").unwrap();
    /// assert_eq!(p.clone().canonical_primitive_part().to_string(), "x+2");
    /// assert_eq!(RationalPolynomial::ZERO.canonical_primitive_part(), 0);
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_primitive_part` from `fmpq_poly/primitive_part.c`, FLINT
    /// 3.6.0, except that the result is an [`IntegerPolynomial`].
    #[inline]
    fn canonical_primitive_part(self) -> IntegerPolynomial {
        self.numerator.canonical_primitive_part()
    }
}

impl CanonicalPrimitivePart for &RationalPolynomial {
    type Output = IntegerPolynomial;

    /// Computes the canonical primitive part of a [`RationalPolynomial`], taking the polynomial by
    /// reference.
    ///
    /// This is the polynomial divided by its content, with the sign chosen so that the leading
    /// coefficient is non-negative. It always has integer coefficients, so it is an
    /// [`IntegerPolynomial`]; for $p = A/d$ it is the canonical primitive part of $A$, and the
    /// denominator plays no part.
    ///
    /// $$
    /// p = \operatorname{sgn}(\operatorname{lc}(p)) \operatorname{cont}(p) \operatorname{cpp}(p).
    /// $$
    ///
    /// The canonical primitive part of the zero polynomial is zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerator's coefficients and the denominator.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CanonicalPrimitivePart;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("-2/3*x-4/3").unwrap();
    /// assert_eq!((&p).canonical_primitive_part().to_string(), "x+2");
    /// assert_eq!((&RationalPolynomial::ZERO).canonical_primitive_part(), 0);
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_primitive_part` from `fmpq_poly/primitive_part.c`, FLINT
    /// 3.6.0, except that the result is an [`IntegerPolynomial`].
    #[inline]
    fn canonical_primitive_part(self) -> IntegerPolynomial {
        (&self.numerator).canonical_primitive_part()
    }
}

impl ContentAndCanonicalPrimitivePart for RationalPolynomial {
    type Content = Rational;
    type CanonicalPrimitivePart = IntegerPolynomial;

    /// Computes the content and the canonical primitive part of a [`RationalPolynomial`] together,
    /// taking the polynomial by value.
    ///
    /// See [`content`](Content::content) and
    /// [`canonical_primitive_part`](CanonicalPrimitivePart::canonical_primitive_part); the content
    /// is found once rather than twice.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerator's coefficients and the denominator.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentAndCanonicalPrimitivePart;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("-2/3*x-4/3").unwrap();
    /// let (content, primitive_part) = p.clone().content_and_canonical_primitive_part();
    /// assert_eq!(content.to_string(), "2/3");
    /// assert_eq!(primitive_part.to_string(), "x+2");
    /// ```
    #[inline]
    fn content_and_canonical_primitive_part(self) -> (Rational, IntegerPolynomial) {
        let (content, primitive_part) = self.numerator.content_and_canonical_primitive_part();
        (content_over(content, self.denominator), primitive_part)
    }
}

impl ContentAndCanonicalPrimitivePart for &RationalPolynomial {
    type Content = Rational;
    type CanonicalPrimitivePart = IntegerPolynomial;

    /// Computes the content and the canonical primitive part of a [`RationalPolynomial`] together,
    /// taking the polynomial by reference.
    ///
    /// See [`content`](Content::content) and
    /// [`canonical_primitive_part`](CanonicalPrimitivePart::canonical_primitive_part); the content
    /// is found once rather than twice.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerator's coefficients and the denominator.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentAndCanonicalPrimitivePart;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("-2/3*x-4/3").unwrap();
    /// let (content, primitive_part) = (&p).content_and_canonical_primitive_part();
    /// assert_eq!(content.to_string(), "2/3");
    /// assert_eq!(primitive_part.to_string(), "x+2");
    /// ```
    #[inline]
    fn content_and_canonical_primitive_part(self) -> (Rational, IntegerPolynomial) {
        let (content, primitive_part) = (&self.numerator).content_and_canonical_primitive_part();
        (
            content_over(content, self.denominator.clone()),
            primitive_part,
        )
    }
}
