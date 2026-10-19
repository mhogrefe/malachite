// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_matrix::RationalMatrix;
use crate::rational_vector::arithmetic::content::content_over;
use malachite_base::num::arithmetic::traits::{
    CanonicalPrimitivePart, ContentAndCanonicalPrimitivePart,
};
use malachite_nz::integer_matrix::IntegerMatrix;

impl CanonicalPrimitivePart for RationalMatrix {
    type Output = IntegerMatrix;

    /// Computes the canonical primitive part of a [`RationalMatrix`], taking the matrix by value.
    ///
    /// This is the
    /// [`primitive_part`](malachite_base::num::arithmetic::traits::PrimitivePart::primitive_part),
    /// negated if its first nonzero element in row-major order is negative, so that that element is
    /// positive: the canonical primitive part of the numerators that
    /// [`to_numerators_and_denominator`](RationalMatrix::to_numerators_and_denominator) gives. A
    /// matrix and its negation have the same canonical primitive part.
    ///
    /// $$
    /// A = \pm \operatorname{cont}(A) \operatorname{cpp}(A).
    /// $$
    ///
    /// The canonical primitive part of a matrix of zeros is a matrix of zeros of the same shape.
    ///
    /// Taking the matrix by value saves nothing, since the numerators are built afresh either way.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn (\log n)^2 \log\log n)$
    ///
    /// $M(n, m) = O(mn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
    /// elements' numerators and denominators, and $m$ is the number of elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CanonicalPrimitivePart;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[-1/2, 2/3; -5, 1]").unwrap();
    /// assert_eq!(a.canonical_primitive_part().to_string(), "[3, -4; 30, -6]");
    /// ```
    #[inline]
    fn canonical_primitive_part(self) -> IntegerMatrix {
        self.to_numerators_and_denominator()
            .0
            .canonical_primitive_part()
    }
}

impl CanonicalPrimitivePart for &RationalMatrix {
    type Output = IntegerMatrix;

    /// Computes the canonical primitive part of a [`RationalMatrix`], taking the matrix by
    /// reference.
    ///
    /// This is the
    /// [`primitive_part`](malachite_base::num::arithmetic::traits::PrimitivePart::primitive_part),
    /// negated if its first nonzero element in row-major order is negative, so that that element is
    /// positive: the canonical primitive part of the numerators that
    /// [`to_numerators_and_denominator`](RationalMatrix::to_numerators_and_denominator) gives. A
    /// matrix and its negation have the same canonical primitive part.
    ///
    /// $$
    /// A = \pm \operatorname{cont}(A) \operatorname{cpp}(A).
    /// $$
    ///
    /// The canonical primitive part of a matrix of zeros is a matrix of zeros of the same shape.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn (\log n)^2 \log\log n)$
    ///
    /// $M(n, m) = O(mn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
    /// elements' numerators and denominators, and $m$ is the number of elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CanonicalPrimitivePart;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[-1/2, 2/3; -5, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).canonical_primitive_part().to_string(),
    ///     "[3, -4; 30, -6]"
    /// );
    /// ```
    #[inline]
    fn canonical_primitive_part(self) -> IntegerMatrix {
        self.to_numerators_and_denominator()
            .0
            .canonical_primitive_part()
    }
}

impl ContentAndCanonicalPrimitivePart for RationalMatrix {
    type Content = Rational;
    type CanonicalPrimitivePart = IntegerMatrix;

    /// Computes the content and the canonical primitive part of a [`RationalMatrix`] together,
    /// taking the matrix by value.
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
    /// elements' numerators and denominators, and $m$ is the number of elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentAndCanonicalPrimitivePart;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[-1/2, 2/3; -5, 1]").unwrap();
    /// let (content, primitive_part) = a.content_and_canonical_primitive_part();
    /// assert_eq!(content.to_string(), "1/6");
    /// assert_eq!(primitive_part.to_string(), "[3, -4; 30, -6]");
    /// ```
    #[inline]
    fn content_and_canonical_primitive_part(self) -> (Rational, IntegerMatrix) {
        let (numerators, denominator) = self.to_numerators_and_denominator();
        let (content, primitive_part) = numerators.content_and_canonical_primitive_part();
        (content_over(content, denominator), primitive_part)
    }
}

impl ContentAndCanonicalPrimitivePart for &RationalMatrix {
    type Content = Rational;
    type CanonicalPrimitivePart = IntegerMatrix;

    /// Computes the content and the canonical primitive part of a [`RationalMatrix`] together,
    /// taking the matrix by reference.
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
    /// elements' numerators and denominators, and $m$ is the number of elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentAndCanonicalPrimitivePart;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[-1/2, 2/3; -5, 1]").unwrap();
    /// let (content, primitive_part) = (&a).content_and_canonical_primitive_part();
    /// assert_eq!(content.to_string(), "1/6");
    /// assert_eq!(primitive_part.to_string(), "[3, -4; 30, -6]");
    /// ```
    #[inline]
    fn content_and_canonical_primitive_part(self) -> (Rational, IntegerMatrix) {
        let (numerators, denominator) = self.to_numerators_and_denominator();
        let (content, primitive_part) = numerators.content_and_canonical_primitive_part();
        (content_over(content, denominator), primitive_part)
    }
}
