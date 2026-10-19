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
use malachite_base::num::arithmetic::traits::{Content, ContentAndPrimitivePart, PrimitivePart};
use malachite_nz::integer_matrix::IntegerMatrix;

impl Content for RationalMatrix {
    type Output = Rational;

    /// Computes the content of a [`RationalMatrix`], taking the matrix by value.
    ///
    /// The content is the unique non-negative rational $c$ such that the matrix is $c$ times a
    /// matrix of coprime integers. It is the GCD of the numerators that
    /// [`to_numerators_and_denominator`](RationalMatrix::to_numerators_and_denominator) gives,
    /// divided by the denominator, the least common multiple of the elements' denominators; that
    /// fraction is already in lowest terms. The content of a matrix of zeros, and of a matrix with
    /// no elements, is zero.
    ///
    /// $$
    /// f(A) = \frac{\gcd_{i, j} N_{ij}}{d}, \\quad \text{where} \\quad A_{ij} = \frac{N_{ij}}{d}
    /// \text{ and } d = \operatorname{lcm}_{i, j} \operatorname{den}(A_{ij}).
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::Content;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[-1/2, 2/3; -5, 1]").unwrap();
    /// assert_eq!(a.content().to_string(), "1/6");
    ///
    /// let a = RationalMatrix::from_str("[2/3, 4/3; 2, 0]").unwrap();
    /// assert_eq!(a.content().to_string(), "2/3");
    /// ```
    #[inline]
    fn content(self) -> Rational {
        let (numerators, denominator) = self.to_numerators_and_denominator();
        content_over(numerators.content(), denominator)
    }
}

impl Content for &RationalMatrix {
    type Output = Rational;

    /// Computes the content of a [`RationalMatrix`], taking the matrix by reference.
    ///
    /// The content is the unique non-negative rational $c$ such that the matrix is $c$ times a
    /// matrix of coprime integers. It is the GCD of the numerators that
    /// [`to_numerators_and_denominator`](RationalMatrix::to_numerators_and_denominator) gives,
    /// divided by the denominator, the least common multiple of the elements' denominators; that
    /// fraction is already in lowest terms. The content of a matrix of zeros, and of a matrix with
    /// no elements, is zero.
    ///
    /// $$
    /// f(A) = \frac{\gcd_{i, j} N_{ij}}{d}, \\quad \text{where} \\quad A_{ij} = \frac{N_{ij}}{d}
    /// \text{ and } d = \operatorname{lcm}_{i, j} \operatorname{den}(A_{ij}).
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::Content;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[-1/2, 2/3; -5, 1]").unwrap();
    /// assert_eq!((&a).content().to_string(), "1/6");
    ///
    /// let a = RationalMatrix::from_str("[2/3, 4/3; 2, 0]").unwrap();
    /// assert_eq!((&a).content().to_string(), "2/3");
    /// ```
    #[inline]
    fn content(self) -> Rational {
        let (numerators, denominator) = self.to_numerators_and_denominator();
        content_over(numerators.content(), denominator)
    }
}

impl PrimitivePart for RationalMatrix {
    type Output = IntegerMatrix;

    /// Computes the primitive part of a [`RationalMatrix`], taking the matrix by value.
    ///
    /// This is the matrix divided by its content, with every element keeping its sign. Its elements
    /// are coprime integers, so it is an [`IntegerMatrix`]: the primitive part of the numerators
    /// that [`to_numerators_and_denominator`](RationalMatrix::to_numerators_and_denominator) gives,
    /// the denominator playing no part. For the one whose first nonzero element in row-major order
    /// is positive, see [`canonical_primitive_part`](
    /// malachite_base::num::arithmetic::traits::CanonicalPrimitivePart::canonical_primitive_part).
    ///
    /// $$
    /// A = \operatorname{cont}(A) \operatorname{pp}(A).
    /// $$
    ///
    /// The primitive part of a matrix of zeros is a matrix of zeros of the same shape.
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
    /// use malachite_base::num::arithmetic::traits::PrimitivePart;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[-1/2, 2/3; -5, 1]").unwrap();
    /// assert_eq!(a.primitive_part().to_string(), "[-3, 4; -30, 6]");
    /// ```
    #[inline]
    fn primitive_part(self) -> IntegerMatrix {
        self.to_numerators_and_denominator().0.primitive_part()
    }
}

impl PrimitivePart for &RationalMatrix {
    type Output = IntegerMatrix;

    /// Computes the primitive part of a [`RationalMatrix`], taking the matrix by reference.
    ///
    /// This is the matrix divided by its content, with every element keeping its sign. Its elements
    /// are coprime integers, so it is an [`IntegerMatrix`]: the primitive part of the numerators
    /// that [`to_numerators_and_denominator`](RationalMatrix::to_numerators_and_denominator) gives,
    /// the denominator playing no part. For the one whose first nonzero element in row-major order
    /// is positive, see [`canonical_primitive_part`](
    /// malachite_base::num::arithmetic::traits::CanonicalPrimitivePart::canonical_primitive_part).
    ///
    /// $$
    /// A = \operatorname{cont}(A) \operatorname{pp}(A).
    /// $$
    ///
    /// The primitive part of a matrix of zeros is a matrix of zeros of the same shape.
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
    /// use malachite_base::num::arithmetic::traits::PrimitivePart;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[-1/2, 2/3; -5, 1]").unwrap();
    /// assert_eq!((&a).primitive_part().to_string(), "[-3, 4; -30, 6]");
    /// ```
    #[inline]
    fn primitive_part(self) -> IntegerMatrix {
        self.to_numerators_and_denominator().0.primitive_part()
    }
}

impl ContentAndPrimitivePart for RationalMatrix {
    type Content = Rational;
    type PrimitivePart = IntegerMatrix;

    /// Computes the content and the primitive part of a [`RationalMatrix`] together, taking the
    /// matrix by value.
    ///
    /// See [`content`](Content::content) and [`primitive_part`](PrimitivePart::primitive_part); the
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
    /// use malachite_base::num::arithmetic::traits::ContentAndPrimitivePart;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[-1/2, 2/3; -5, 1]").unwrap();
    /// let (content, primitive_part) = a.content_and_primitive_part();
    /// assert_eq!(content.to_string(), "1/6");
    /// assert_eq!(primitive_part.to_string(), "[-3, 4; -30, 6]");
    /// ```
    #[inline]
    fn content_and_primitive_part(self) -> (Rational, IntegerMatrix) {
        let (numerators, denominator) = self.to_numerators_and_denominator();
        let (content, primitive_part) = numerators.content_and_primitive_part();
        (content_over(content, denominator), primitive_part)
    }
}

impl ContentAndPrimitivePart for &RationalMatrix {
    type Content = Rational;
    type PrimitivePart = IntegerMatrix;

    /// Computes the content and the primitive part of a [`RationalMatrix`] together, taking the
    /// matrix by reference.
    ///
    /// See [`content`](Content::content) and [`primitive_part`](PrimitivePart::primitive_part); the
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
    /// use malachite_base::num::arithmetic::traits::ContentAndPrimitivePart;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[-1/2, 2/3; -5, 1]").unwrap();
    /// let (content, primitive_part) = (&a).content_and_primitive_part();
    /// assert_eq!(content.to_string(), "1/6");
    /// assert_eq!(primitive_part.to_string(), "[-3, 4; -30, 6]");
    /// ```
    #[inline]
    fn content_and_primitive_part(self) -> (Rational, IntegerMatrix) {
        let (numerators, denominator) = self.to_numerators_and_denominator();
        let (content, primitive_part) = numerators.content_and_primitive_part();
        (content_over(content, denominator), primitive_part)
    }
}
