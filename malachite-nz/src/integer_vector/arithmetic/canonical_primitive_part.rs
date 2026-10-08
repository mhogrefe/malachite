// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::arithmetic::content::{content, normalize_in_place, normalized};
use crate::integer_vector::IntegerVector;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{
    CanonicalPrimitivePart, CanonicalPrimitivePartAssign, ContentAndCanonicalPrimitivePart,
};

// Whether the canonical primitive part must be negated: when the first nonzero element is negative.
fn negate(elements: &[Integer]) -> bool {
    elements
        .iter()
        .find(|x| **x != 0u32)
        .is_some_and(|x| !x.sign)
}

impl CanonicalPrimitivePart for IntegerVector {
    type Output = Self;

    /// Computes the canonical primitive part of an [`IntegerVector`], taking the vector by value.
    ///
    /// This is the [`primitive_part`](PrimitivePart::primitive_part), negated if its first nonzero
    /// element is negative, so that the first nonzero element is positive. A vector and its
    /// negation have the same canonical primitive part. The canonical primitive part of a vector of
    /// zeros is itself, keeping its dimension.
    ///
    /// $$
    /// v = \pm \operatorname{cont}(v) \operatorname{cpp}(v).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CanonicalPrimitivePart;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(-6, 4, -10)").unwrap();
    /// assert_eq!(v.canonical_primitive_part().to_string(), "(3, -2, 5)");
    ///
    /// // Leading zeros are skipped: the first nonzero element is made positive.
    /// let v = IntegerVector::from_str("(0, -2, 4)").unwrap();
    /// assert_eq!(v.canonical_primitive_part().to_string(), "(0, 1, -2)");
    /// ```
    #[inline]
    fn canonical_primitive_part(mut self) -> Self {
        self.canonical_primitive_part_assign();
        self
    }
}

impl CanonicalPrimitivePart for &IntegerVector {
    type Output = IntegerVector;

    /// Computes the canonical primitive part of an [`IntegerVector`], taking the vector by
    /// reference.
    ///
    /// This is the [`primitive_part`](PrimitivePart::primitive_part), negated if its first nonzero
    /// element is negative, so that the first nonzero element is positive. A vector and its
    /// negation have the same canonical primitive part. The canonical primitive part of a vector of
    /// zeros is itself, keeping its dimension.
    ///
    /// $$
    /// v = \pm \operatorname{cont}(v) \operatorname{cpp}(v).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CanonicalPrimitivePart;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(-6, 4, -10)").unwrap();
    /// assert_eq!((&v).canonical_primitive_part().to_string(), "(3, -2, 5)");
    /// ```
    #[inline]
    fn canonical_primitive_part(self) -> IntegerVector {
        let content = content(&self.elements);
        IntegerVector {
            elements: normalized(&self.elements, &content, negate(&self.elements)),
        }
    }
}

impl CanonicalPrimitivePartAssign for IntegerVector {
    /// Replaces an [`IntegerVector`] with its canonical primitive part.
    ///
    /// See [`canonical_primitive_part`](CanonicalPrimitivePart::canonical_primitive_part).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CanonicalPrimitivePartAssign;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let mut v = IntegerVector::from_str("(-6, 4, -10)").unwrap();
    /// v.canonical_primitive_part_assign();
    /// assert_eq!(v.to_string(), "(3, -2, 5)");
    /// ```
    #[inline]
    fn canonical_primitive_part_assign(&mut self) {
        let content = content(&self.elements);
        let negate = negate(&self.elements);
        normalize_in_place(&mut self.elements, &content, negate);
    }
}

impl ContentAndCanonicalPrimitivePart for IntegerVector {
    type Content = Natural;
    type CanonicalPrimitivePart = Self;

    /// Computes the content and the canonical primitive part of an [`IntegerVector`] together,
    /// taking the vector by value.
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
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentAndCanonicalPrimitivePart;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(-6, 4, -10)").unwrap();
    /// let (content, primitive_part) = v.content_and_canonical_primitive_part();
    /// assert_eq!(content, 2u32);
    /// assert_eq!(primitive_part.to_string(), "(3, -2, 5)");
    /// ```
    #[inline]
    fn content_and_canonical_primitive_part(mut self) -> (Natural, Self) {
        let content = content(&self.elements);
        let negate = negate(&self.elements);
        normalize_in_place(&mut self.elements, &content, negate);
        (content, self)
    }
}

impl ContentAndCanonicalPrimitivePart for &IntegerVector {
    type Content = Natural;
    type CanonicalPrimitivePart = IntegerVector;

    /// Computes the content and the canonical primitive part of an [`IntegerVector`] together,
    /// taking the vector by reference.
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
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentAndCanonicalPrimitivePart;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(-6, 4, -10)").unwrap();
    /// let (content, primitive_part) = (&v).content_and_canonical_primitive_part();
    /// assert_eq!(content, 2u32);
    /// assert_eq!(primitive_part.to_string(), "(3, -2, 5)");
    /// ```
    #[inline]
    fn content_and_canonical_primitive_part(self) -> (Natural, IntegerVector) {
        let content = content(&self.elements);
        let elements = normalized(&self.elements, &content, negate(&self.elements));
        (content, IntegerVector { elements })
    }
}
