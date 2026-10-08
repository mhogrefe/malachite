// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_polynomial::arithmetic::content::content_over;
use crate::rational_vector::RationalVector;
use malachite_base::num::arithmetic::traits::{
    CanonicalPrimitivePart, ContentAndCanonicalPrimitivePart,
};
use malachite_nz::integer_vector::IntegerVector;

impl CanonicalPrimitivePart for RationalVector {
    type Output = IntegerVector;

    /// Computes the canonical primitive part of a [`RationalVector`], taking the vector by value.
    ///
    /// This is the
    /// [`primitive_part`](malachite_base::num::arithmetic::traits::PrimitivePart::primitive_part),
    /// negated if its first nonzero element is negative, so that the first nonzero element is
    /// positive: the canonical primitive part of the numerators that
    /// [`to_numerators_and_denominator`](RationalVector::to_numerators_and_denominator) gives. A
    /// vector and its negation have the same canonical primitive part.
    ///
    /// $$
    /// v = \pm \operatorname{cont}(v) \operatorname{cpp}(v).
    /// $$
    ///
    /// The canonical primitive part of a vector of zeros is a vector of zeros of the same
    /// dimension.
    ///
    /// Taking the vector by value saves nothing, since the numerators are built afresh either way.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn (\log n)^2 \log\log n)$
    ///
    /// $M(n, m) = O(mn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
    /// elements' numerators and denominators, and $m$ is the dimension.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CanonicalPrimitivePart;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(-1/2, 2/3, -5)").unwrap();
    /// assert_eq!(v.canonical_primitive_part().to_string(), "(3, -4, 30)");
    /// ```
    #[inline]
    fn canonical_primitive_part(self) -> IntegerVector {
        self.to_numerators_and_denominator()
            .0
            .canonical_primitive_part()
    }
}

impl CanonicalPrimitivePart for &RationalVector {
    type Output = IntegerVector;

    /// Computes the canonical primitive part of a [`RationalVector`], taking the vector by
    /// reference.
    ///
    /// This is the
    /// [`primitive_part`](malachite_base::num::arithmetic::traits::PrimitivePart::primitive_part),
    /// negated if its first nonzero element is negative, so that the first nonzero element is
    /// positive: the canonical primitive part of the numerators that
    /// [`to_numerators_and_denominator`](RationalVector::to_numerators_and_denominator) gives. A
    /// vector and its negation have the same canonical primitive part.
    ///
    /// $$
    /// v = \pm \operatorname{cont}(v) \operatorname{cpp}(v).
    /// $$
    ///
    /// The canonical primitive part of a vector of zeros is a vector of zeros of the same
    /// dimension.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn (\log n)^2 \log\log n)$
    ///
    /// $M(n, m) = O(mn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
    /// elements' numerators and denominators, and $m$ is the dimension.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CanonicalPrimitivePart;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(-1/2, 2/3, -5)").unwrap();
    /// assert_eq!((&v).canonical_primitive_part().to_string(), "(3, -4, 30)");
    /// ```
    #[inline]
    fn canonical_primitive_part(self) -> IntegerVector {
        self.to_numerators_and_denominator()
            .0
            .canonical_primitive_part()
    }
}

impl ContentAndCanonicalPrimitivePart for RationalVector {
    type Content = Rational;
    type CanonicalPrimitivePart = IntegerVector;

    /// Computes the content and the canonical primitive part of a [`RationalVector`] together,
    /// taking the vector by value.
    ///
    /// See [`content`](malachite_base::num::arithmetic::traits::Content::content) and
    /// [`canonical_primitive_part`](CanonicalPrimitivePart::canonical_primitive_part); the
    /// numerators are cleared and the content found once rather than twice.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn (\log n)^2 \log\log n)$
    ///
    /// $M(n, m) = O(mn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
    /// elements' numerators and denominators, and $m$ is the dimension.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentAndCanonicalPrimitivePart;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(-1/2, 2/3, -5)").unwrap();
    /// let (content, primitive_part) = v.content_and_canonical_primitive_part();
    /// assert_eq!(content.to_string(), "1/6");
    /// assert_eq!(primitive_part.to_string(), "(3, -4, 30)");
    /// ```
    #[inline]
    fn content_and_canonical_primitive_part(self) -> (Rational, IntegerVector) {
        let (numerators, denominator) = self.to_numerators_and_denominator();
        let (content, primitive_part) = numerators.content_and_canonical_primitive_part();
        (content_over(content, denominator), primitive_part)
    }
}

impl ContentAndCanonicalPrimitivePart for &RationalVector {
    type Content = Rational;
    type CanonicalPrimitivePart = IntegerVector;

    /// Computes the content and the canonical primitive part of a [`RationalVector`] together,
    /// taking the vector by reference.
    ///
    /// See [`content`](malachite_base::num::arithmetic::traits::Content::content) and
    /// [`canonical_primitive_part`](CanonicalPrimitivePart::canonical_primitive_part); the
    /// numerators are cleared and the content found once rather than twice.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn (\log n)^2 \log\log n)$
    ///
    /// $M(n, m) = O(mn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
    /// elements' numerators and denominators, and $m$ is the dimension.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentAndCanonicalPrimitivePart;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(-1/2, 2/3, -5)").unwrap();
    /// let (content, primitive_part) = (&v).content_and_canonical_primitive_part();
    /// assert_eq!(content.to_string(), "1/6");
    /// assert_eq!(primitive_part.to_string(), "(3, -4, 30)");
    /// ```
    #[inline]
    fn content_and_canonical_primitive_part(self) -> (Rational, IntegerVector) {
        let (numerators, denominator) = self.to_numerators_and_denominator();
        let (content, primitive_part) = numerators.content_and_canonical_primitive_part();
        (content_over(content, denominator), primitive_part)
    }
}
