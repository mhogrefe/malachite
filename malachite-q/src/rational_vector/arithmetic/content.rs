// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_vector::RationalVector;
use malachite_base::num::arithmetic::traits::{Content, ContentAndPrimitivePart, PrimitivePart};
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;

// The content of a vector or polynomial written as integers $A$ over a common denominator $d$ is
// $\operatorname{cont}(A)/d$. It is already in lowest terms when the numerators and denominator
// share no factor, as they do for a canonical `RationalPolynomial` and for
// `to_numerators_and_denominator`.
pub(crate) const fn content_over(numerator_content: Natural, denominator: Natural) -> Rational {
    Rational {
        sign: true,
        numerator: numerator_content,
        denominator,
    }
}

impl Content for RationalVector {
    type Output = Rational;

    /// Computes the content of a [`RationalVector`], taking the vector by value.
    ///
    /// The content is the unique non-negative rational $c$ such that the vector is $c$ times a
    /// vector of coprime integers. It is the GCD of the numerators that
    /// [`to_numerators_and_denominator`](RationalVector::to_numerators_and_denominator) gives,
    /// divided by the denominator, the least common multiple of the elements' denominators; that
    /// fraction is already in lowest terms. The content of a vector of zeros, and of the
    /// 0-dimensional vector, is zero.
    ///
    /// $$
    /// f(v) = \frac{\gcd(a_0, a_1, \ldots, a_{m-1})}{d}, \\quad \text{where} \\quad
    /// v_i = \frac{a_i}{d} \text{ and } d = \operatorname{lcm}(\operatorname{den}(v_0), \ldots,
    /// \operatorname{den}(v_{m-1})).
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::Content;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(-1/2, 2/3, -5)").unwrap();
    /// assert_eq!(v.content().to_string(), "1/6");
    ///
    /// let v = RationalVector::from_str("(2/3, 4/3)").unwrap();
    /// assert_eq!(v.content().to_string(), "2/3");
    /// ```
    #[inline]
    fn content(self) -> Rational {
        let (numerators, denominator) = self.to_numerators_and_denominator();
        content_over(numerators.content(), denominator)
    }
}

impl Content for &RationalVector {
    type Output = Rational;

    /// Computes the content of a [`RationalVector`], taking the vector by reference.
    ///
    /// The content is the unique non-negative rational $c$ such that the vector is $c$ times a
    /// vector of coprime integers. It is the GCD of the numerators that
    /// [`to_numerators_and_denominator`](RationalVector::to_numerators_and_denominator) gives,
    /// divided by the denominator, the least common multiple of the elements' denominators; that
    /// fraction is already in lowest terms. The content of a vector of zeros, and of the
    /// 0-dimensional vector, is zero.
    ///
    /// $$
    /// f(v) = \frac{\gcd(a_0, a_1, \ldots, a_{m-1})}{d}, \\quad \text{where} \\quad
    /// v_i = \frac{a_i}{d} \text{ and } d = \operatorname{lcm}(\operatorname{den}(v_0), \ldots,
    /// \operatorname{den}(v_{m-1})).
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::Content;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(-1/2, 2/3, -5)").unwrap();
    /// assert_eq!((&v).content().to_string(), "1/6");
    ///
    /// let v = RationalVector::from_str("(2/3, 4/3)").unwrap();
    /// assert_eq!((&v).content().to_string(), "2/3");
    /// ```
    #[inline]
    fn content(self) -> Rational {
        let (numerators, denominator) = self.to_numerators_and_denominator();
        content_over(numerators.content(), denominator)
    }
}

impl PrimitivePart for RationalVector {
    type Output = IntegerVector;

    /// Computes the primitive part of a [`RationalVector`], taking the vector by value.
    ///
    /// This is the vector divided by its content, with every element keeping its sign. Its elements
    /// are coprime integers, so it is an [`IntegerVector`]: the primitive part of the numerators
    /// that [`to_numerators_and_denominator`](RationalVector::to_numerators_and_denominator) gives,
    /// the denominator playing no part. For the one whose first nonzero element is positive, see
    /// [`canonical_primitive_part`](
    /// malachite_base::num::arithmetic::traits::CanonicalPrimitivePart::canonical_primitive_part).
    ///
    /// $$
    /// v = \operatorname{cont}(v) \operatorname{pp}(v).
    /// $$
    ///
    /// The primitive part of a vector of zeros is a vector of zeros of the same dimension.
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
    /// use malachite_base::num::arithmetic::traits::PrimitivePart;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(-1/2, 2/3, -5)").unwrap();
    /// assert_eq!(v.primitive_part().to_string(), "(-3, 4, -30)");
    /// ```
    #[inline]
    fn primitive_part(self) -> IntegerVector {
        self.to_numerators_and_denominator().0.primitive_part()
    }
}

impl PrimitivePart for &RationalVector {
    type Output = IntegerVector;

    /// Computes the primitive part of a [`RationalVector`], taking the vector by reference.
    ///
    /// This is the vector divided by its content, with every element keeping its sign. Its elements
    /// are coprime integers, so it is an [`IntegerVector`]: the primitive part of the numerators
    /// that [`to_numerators_and_denominator`](RationalVector::to_numerators_and_denominator) gives,
    /// the denominator playing no part. For the one whose first nonzero element is positive, see
    /// [`canonical_primitive_part`](
    /// malachite_base::num::arithmetic::traits::CanonicalPrimitivePart::canonical_primitive_part).
    ///
    /// $$
    /// v = \operatorname{cont}(v) \operatorname{pp}(v).
    /// $$
    ///
    /// The primitive part of a vector of zeros is a vector of zeros of the same dimension.
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
    /// use malachite_base::num::arithmetic::traits::PrimitivePart;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(-1/2, 2/3, -5)").unwrap();
    /// assert_eq!((&v).primitive_part().to_string(), "(-3, 4, -30)");
    /// ```
    #[inline]
    fn primitive_part(self) -> IntegerVector {
        self.to_numerators_and_denominator().0.primitive_part()
    }
}

impl ContentAndPrimitivePart for RationalVector {
    type Content = Rational;
    type PrimitivePart = IntegerVector;

    /// Computes the content and the primitive part of a [`RationalVector`] together, taking the
    /// vector by value.
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
    /// elements' numerators and denominators, and $m$ is the dimension.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentAndPrimitivePart;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(-1/2, 2/3, -5)").unwrap();
    /// let (content, primitive_part) = v.content_and_primitive_part();
    /// assert_eq!(content.to_string(), "1/6");
    /// assert_eq!(primitive_part.to_string(), "(-3, 4, -30)");
    /// ```
    #[inline]
    fn content_and_primitive_part(self) -> (Rational, IntegerVector) {
        let (numerators, denominator) = self.to_numerators_and_denominator();
        let (content, primitive_part) = numerators.content_and_primitive_part();
        (content_over(content, denominator), primitive_part)
    }
}

impl ContentAndPrimitivePart for &RationalVector {
    type Content = Rational;
    type PrimitivePart = IntegerVector;

    /// Computes the content and the primitive part of a [`RationalVector`] together, taking the
    /// vector by reference.
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
    /// elements' numerators and denominators, and $m$ is the dimension.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentAndPrimitivePart;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(-1/2, 2/3, -5)").unwrap();
    /// let (content, primitive_part) = (&v).content_and_primitive_part();
    /// assert_eq!(content.to_string(), "1/6");
    /// assert_eq!(primitive_part.to_string(), "(-3, 4, -30)");
    /// ```
    #[inline]
    fn content_and_primitive_part(self) -> (Rational, IntegerVector) {
        let (numerators, denominator) = self.to_numerators_and_denominator();
        let (content, primitive_part) = numerators.content_and_primitive_part();
        (content_over(content, denominator), primitive_part)
    }
}
