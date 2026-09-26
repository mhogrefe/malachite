// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_polynomial::RationalPolynomial;
use malachite_base::polynomial::{Content, ContentAndPrimitivePart, PrimitivePart};
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;

// The content of A/d is cont(A)/d, which is already in lowest terms: a canonical
// `RationalPolynomial`'s numerator content shares no factor with its denominator.
const fn content_over(numerator_content: Natural, denominator: Natural) -> Rational {
    Rational {
        sign: true,
        numerator: numerator_content,
        denominator,
    }
}

impl Content for RationalPolynomial {
    type Output = Rational;

    /// Computes the content of a [`RationalPolynomial`], taking the polynomial by value.
    ///
    /// Over the rationals every nonzero coefficient is a unit, so the GCD of the coefficients says
    /// nothing. The content is instead the non-negative [`Rational`] $c$ for which $p/c$ is a
    /// primitive polynomial with integer coefficients. For $p = A/d$ it is
    /// $\operatorname{cont}(A)/d$, which is already in lowest terms, so no GCD with the denominator
    /// is needed. The content of the zero polynomial is zero.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::Content;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("-2/3*x-4/3").unwrap();
    /// assert_eq!(p.clone().content().to_string(), "2/3");
    /// assert_eq!(RationalPolynomial::ZERO.content(), 0);
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_content` from `fmpq_poly/content.c`, FLINT 3.6.0.
    #[inline]
    fn content(self) -> Rational {
        content_over(self.numerator.content(), self.denominator)
    }
}

impl Content for &RationalPolynomial {
    type Output = Rational;

    /// Computes the content of a [`RationalPolynomial`], taking the polynomial by reference.
    ///
    /// Over the rationals every nonzero coefficient is a unit, so the GCD of the coefficients says
    /// nothing. The content is instead the non-negative [`Rational`] $c$ for which $p/c$ is a
    /// primitive polynomial with integer coefficients. For $p = A/d$ it is
    /// $\operatorname{cont}(A)/d$, which is already in lowest terms, so no GCD with the denominator
    /// is needed. The content of the zero polynomial is zero.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::Content;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("-2/3*x-4/3").unwrap();
    /// assert_eq!((&p).content().to_string(), "2/3");
    /// assert_eq!((&RationalPolynomial::ZERO).content(), 0);
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_content` from `fmpq_poly/content.c`, FLINT 3.6.0.
    #[inline]
    fn content(self) -> Rational {
        content_over((&self.numerator).content(), self.denominator.clone())
    }
}

impl PrimitivePart for RationalPolynomial {
    type Output = IntegerPolynomial;

    /// Computes the primitive part of a [`RationalPolynomial`], taking the polynomial by value.
    ///
    /// This is the polynomial divided by its content, with the sign chosen so that the leading
    /// coefficient is non-negative. It always has integer coefficients, so it is an
    /// [`IntegerPolynomial`]; for $p = A/d$ it is the primitive part of $A$, and the denominator
    /// plays no part.
    ///
    /// $$
    /// p = \operatorname{sgn}(\operatorname{lc}(p)) \operatorname{cont}(p) \operatorname{pp}(p).
    /// $$
    ///
    /// The primitive part of the zero polynomial is zero.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::PrimitivePart;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("-2/3*x-4/3").unwrap();
    /// assert_eq!(p.clone().primitive_part().to_string(), "x+2");
    /// assert_eq!(RationalPolynomial::ZERO.primitive_part(), 0);
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_primitive_part` from `fmpq_poly/primitive_part.c`, FLINT
    /// 3.6.0, except that the result is an [`IntegerPolynomial`].
    #[inline]
    fn primitive_part(self) -> IntegerPolynomial {
        self.numerator.primitive_part()
    }
}

impl PrimitivePart for &RationalPolynomial {
    type Output = IntegerPolynomial;

    /// Computes the primitive part of a [`RationalPolynomial`], taking the polynomial by reference.
    ///
    /// This is the polynomial divided by its content, with the sign chosen so that the leading
    /// coefficient is non-negative. It always has integer coefficients, so it is an
    /// [`IntegerPolynomial`]; for $p = A/d$ it is the primitive part of $A$, and the denominator
    /// plays no part.
    ///
    /// $$
    /// p = \operatorname{sgn}(\operatorname{lc}(p)) \operatorname{cont}(p) \operatorname{pp}(p).
    /// $$
    ///
    /// The primitive part of the zero polynomial is zero.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::PrimitivePart;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("-2/3*x-4/3").unwrap();
    /// assert_eq!((&p).primitive_part().to_string(), "x+2");
    /// assert_eq!((&RationalPolynomial::ZERO).primitive_part(), 0);
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_primitive_part` from `fmpq_poly/primitive_part.c`, FLINT
    /// 3.6.0, except that the result is an [`IntegerPolynomial`].
    #[inline]
    fn primitive_part(self) -> IntegerPolynomial {
        (&self.numerator).primitive_part()
    }
}

impl ContentAndPrimitivePart for RationalPolynomial {
    type Content = Rational;
    type PrimitivePart = IntegerPolynomial;

    /// Computes the content and the primitive part of a [`RationalPolynomial`] together, taking the
    /// polynomial by value.
    ///
    /// See [`content`](Content::content) and [`primitive_part`](PrimitivePart::primitive_part); the
    /// content is found once rather than twice.
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
    /// use malachite_base::polynomial::ContentAndPrimitivePart;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("-2/3*x-4/3").unwrap();
    /// let (content, primitive_part) = p.clone().content_and_primitive_part();
    /// assert_eq!(content.to_string(), "2/3");
    /// assert_eq!(primitive_part.to_string(), "x+2");
    /// ```
    #[inline]
    fn content_and_primitive_part(self) -> (Rational, IntegerPolynomial) {
        let (content, primitive_part) = self.numerator.content_and_primitive_part();
        (content_over(content, self.denominator), primitive_part)
    }
}

impl ContentAndPrimitivePart for &RationalPolynomial {
    type Content = Rational;
    type PrimitivePart = IntegerPolynomial;

    /// Computes the content and the primitive part of a [`RationalPolynomial`] together, taking the
    /// polynomial by reference.
    ///
    /// See [`content`](Content::content) and [`primitive_part`](PrimitivePart::primitive_part); the
    /// content is found once rather than twice.
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
    /// use malachite_base::polynomial::ContentAndPrimitivePart;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("-2/3*x-4/3").unwrap();
    /// let (content, primitive_part) = (&p).content_and_primitive_part();
    /// assert_eq!(content.to_string(), "2/3");
    /// assert_eq!(primitive_part.to_string(), "x+2");
    /// ```
    #[inline]
    fn content_and_primitive_part(self) -> (Rational, IntegerPolynomial) {
        let (content, primitive_part) = (&self.numerator).content_and_primitive_part();
        (
            content_over(content, self.denominator.clone()),
            primitive_part,
        )
    }
}
