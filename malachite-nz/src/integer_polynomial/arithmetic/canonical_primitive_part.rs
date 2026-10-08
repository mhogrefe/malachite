// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::IntegerPolynomial;
use crate::integer_polynomial::arithmetic::content::{
    content, negate, normalize_in_place, normalized,
};
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{
    CanonicalPrimitivePart, CanonicalPrimitivePartAssign, ContentAndCanonicalPrimitivePart,
};

impl CanonicalPrimitivePart for IntegerPolynomial {
    type Output = Self;

    /// Computes the canonical primitive part of an [`IntegerPolynomial`], taking the polynomial by
    /// value.
    ///
    /// This is the polynomial divided by its content, with the sign chosen so that the leading
    /// coefficient is non-negative: the [`primitive_part`](
    /// malachite_base::num::arithmetic::traits::PrimitivePart::primitive_part), negated if the
    /// leading coefficient is negative. The sign matters: when the leading coefficient is negative,
    /// the content times the canonical primitive part is the negation of the polynomial, and the
    /// identity needs the sign of the leading coefficient $\operatorname{lc}(p)$.
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
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CanonicalPrimitivePart;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("-6*x^2+4*x-10").unwrap();
    /// assert_eq!(
    ///     p.clone().canonical_primitive_part().to_string(),
    ///     "3*x^2-2*x+5"
    /// );
    /// assert_eq!(
    ///     IntegerPolynomial::ZERO.canonical_primitive_part(),
    ///     IntegerPolynomial::ZERO
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_primitive_part` from `fmpz_poly/primitive_part.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn canonical_primitive_part(mut self) -> Self {
        let content = content(&self.coefficients);
        let negate = negate(&self.coefficients);
        normalize_in_place(&mut self.coefficients, &content, negate);
        self
    }
}

impl CanonicalPrimitivePart for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Computes the canonical primitive part of an [`IntegerPolynomial`], taking the polynomial by
    /// reference.
    ///
    /// This is the polynomial divided by its content, with the sign chosen so that the leading
    /// coefficient is non-negative: the [`primitive_part`](
    /// malachite_base::num::arithmetic::traits::PrimitivePart::primitive_part), negated if the
    /// leading coefficient is negative. The sign matters: when the leading coefficient is negative,
    /// the content times the canonical primitive part is the negation of the polynomial, and the
    /// identity needs the sign of the leading coefficient $\operatorname{lc}(p)$.
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
    /// coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CanonicalPrimitivePart;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("-6*x^2+4*x-10").unwrap();
    /// assert_eq!((&p).canonical_primitive_part().to_string(), "3*x^2-2*x+5");
    /// assert_eq!(
    ///     (&IntegerPolynomial::ZERO).canonical_primitive_part(),
    ///     IntegerPolynomial::ZERO
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_primitive_part` from `fmpz_poly/primitive_part.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn canonical_primitive_part(self) -> IntegerPolynomial {
        let content = content(&self.coefficients);
        IntegerPolynomial {
            coefficients: normalized(&self.coefficients, &content, negate(&self.coefficients)),
        }
    }
}

impl CanonicalPrimitivePartAssign for IntegerPolynomial {
    /// Replaces an [`IntegerPolynomial`] with its canonical primitive part.
    ///
    /// See [`canonical_primitive_part`](CanonicalPrimitivePart::canonical_primitive_part).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CanonicalPrimitivePartAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("-6*x^2+4*x-10").unwrap();
    /// p.canonical_primitive_part_assign();
    /// assert_eq!(p.to_string(), "3*x^2-2*x+5");
    /// ```
    #[inline]
    fn canonical_primitive_part_assign(&mut self) {
        let content = content(&self.coefficients);
        let negate = negate(&self.coefficients);
        normalize_in_place(&mut self.coefficients, &content, negate);
    }
}

impl ContentAndCanonicalPrimitivePart for IntegerPolynomial {
    type Content = Natural;
    type CanonicalPrimitivePart = Self;

    /// Computes the content and the canonical primitive part of an [`IntegerPolynomial`] together,
    /// taking the polynomial by value.
    ///
    /// See [`content`](malachite_base::num::arithmetic::traits::Content::content) and
    /// [`canonical_primitive_part`](CanonicalPrimitivePart::canonical_primitive_part); the content
    /// is found once rather than twice.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentAndCanonicalPrimitivePart;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("-6*x^2+4*x-10").unwrap();
    /// let (content, primitive_part) = p.clone().content_and_canonical_primitive_part();
    /// assert_eq!(content, 2);
    /// assert_eq!(primitive_part.to_string(), "3*x^2-2*x+5");
    /// ```
    #[inline]
    fn content_and_canonical_primitive_part(mut self) -> (Natural, Self) {
        let content = content(&self.coefficients);
        let negate = negate(&self.coefficients);
        normalize_in_place(&mut self.coefficients, &content, negate);
        (content, self)
    }
}

impl ContentAndCanonicalPrimitivePart for &IntegerPolynomial {
    type Content = Natural;
    type CanonicalPrimitivePart = IntegerPolynomial;

    /// Computes the content and the canonical primitive part of an [`IntegerPolynomial`] together,
    /// taking the polynomial by reference.
    ///
    /// See [`content`](malachite_base::num::arithmetic::traits::Content::content) and
    /// [`canonical_primitive_part`](CanonicalPrimitivePart::canonical_primitive_part); the content
    /// is found once rather than twice.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentAndCanonicalPrimitivePart;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("-6*x^2+4*x-10").unwrap();
    /// let (content, primitive_part) = (&p).content_and_canonical_primitive_part();
    /// assert_eq!(content, 2);
    /// assert_eq!(primitive_part.to_string(), "3*x^2-2*x+5");
    /// ```
    #[inline]
    fn content_and_canonical_primitive_part(self) -> (Natural, IntegerPolynomial) {
        let content = content(&self.coefficients);
        let coefficients = normalized(&self.coefficients, &content, negate(&self.coefficients));
        (content, IntegerPolynomial { coefficients })
    }
}
