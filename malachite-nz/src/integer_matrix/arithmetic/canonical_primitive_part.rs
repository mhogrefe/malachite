// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;
use crate::integer_vector::arithmetic::content::{content, normalize_in_place, normalized};
use crate::natural::Natural;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    CanonicalPrimitivePart, CanonicalPrimitivePartAssign, ContentAndCanonicalPrimitivePart,
};

// Whether the canonical primitive part must be negated: when the first nonzero element, in
// row-major order, is negative.
fn negate(a: &IntegerMatrix) -> bool {
    a.elements
        .iter()
        .find(|&x| *x != 0u32)
        .is_some_and(|x| !x.sign)
}

impl CanonicalPrimitivePart for IntegerMatrix {
    type Output = Self;

    /// Computes the canonical primitive part of an [`IntegerMatrix`], taking the matrix by value.
    ///
    /// This is the
    /// [`primitive_part`](malachite_base::num::arithmetic::traits::PrimitivePart::primitive_part),
    /// negated if its first nonzero element in row-major order is negative, so that that element is
    /// positive. A matrix and its negation have the same canonical primitive part. The canonical
    /// primitive part of a matrix of zeros is itself, keeping its shape.
    ///
    /// $$
    /// A = \pm \operatorname{cont}(A) \operatorname{cpp}(A).
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
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[-6, 4; -10, 8]").unwrap();
    /// assert_eq!(a.canonical_primitive_part().to_string(), "[3, -2; 5, -4]");
    ///
    /// // Leading zeros are skipped: the first nonzero element in row-major order is made positive.
    /// let a = IntegerMatrix::from_str("[0, -2; 4, 6]").unwrap();
    /// assert_eq!(a.canonical_primitive_part().to_string(), "[0, 1; -2, -3]");
    /// ```
    #[inline]
    fn canonical_primitive_part(mut self) -> Self {
        self.canonical_primitive_part_assign();
        self
    }
}

impl CanonicalPrimitivePart for &IntegerMatrix {
    type Output = IntegerMatrix;

    /// Computes the canonical primitive part of an [`IntegerMatrix`], taking the matrix by
    /// reference.
    ///
    /// This is the
    /// [`primitive_part`](malachite_base::num::arithmetic::traits::PrimitivePart::primitive_part),
    /// negated if its first nonzero element in row-major order is negative, so that that element is
    /// positive. A matrix and its negation have the same canonical primitive part. The canonical
    /// primitive part of a matrix of zeros is itself, keeping its shape.
    ///
    /// $$
    /// A = \pm \operatorname{cont}(A) \operatorname{cpp}(A).
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
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[-6, 4; -10, 8]").unwrap();
    /// assert_eq!(
    ///     (&a).canonical_primitive_part().to_string(),
    ///     "[3, -2; 5, -4]"
    /// );
    /// ```
    #[inline]
    fn canonical_primitive_part(self) -> IntegerMatrix {
        let content = content(&self.elements);
        IntegerMatrix::from_owned_elements(
            self.height(),
            self.width(),
            normalized(&self.elements, &content, negate(self)),
        )
    }
}

impl CanonicalPrimitivePartAssign for IntegerMatrix {
    /// Replaces an [`IntegerMatrix`] with its canonical primitive part.
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
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let mut a = IntegerMatrix::from_str("[-6, 4; -10, 8]").unwrap();
    /// a.canonical_primitive_part_assign();
    /// assert_eq!(a.to_string(), "[3, -2; 5, -4]");
    /// ```
    #[inline]
    fn canonical_primitive_part_assign(&mut self) {
        let content = content(&self.elements);
        let negate = negate(self);
        normalize_in_place(&mut self.elements, &content, negate);
    }
}

impl ContentAndCanonicalPrimitivePart for IntegerMatrix {
    type Content = Natural;
    type CanonicalPrimitivePart = Self;

    /// Computes the content and the canonical primitive part of an [`IntegerMatrix`] together,
    /// taking the matrix by value.
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
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[-6, 4; -10, 8]").unwrap();
    /// let (content, primitive_part) = a.content_and_canonical_primitive_part();
    /// assert_eq!(content, 2u32);
    /// assert_eq!(primitive_part.to_string(), "[3, -2; 5, -4]");
    /// ```
    #[inline]
    fn content_and_canonical_primitive_part(mut self) -> (Natural, Self) {
        let content = content(&self.elements);
        let negate = negate(&self);
        normalize_in_place(&mut self.elements, &content, negate);
        (content, self)
    }
}

impl ContentAndCanonicalPrimitivePart for &IntegerMatrix {
    type Content = Natural;
    type CanonicalPrimitivePart = IntegerMatrix;

    /// Computes the content and the canonical primitive part of an [`IntegerMatrix`] together,
    /// taking the matrix by reference.
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
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[-6, 4; -10, 8]").unwrap();
    /// let (content, primitive_part) = (&a).content_and_canonical_primitive_part();
    /// assert_eq!(content, 2u32);
    /// assert_eq!(primitive_part.to_string(), "[3, -2; 5, -4]");
    /// ```
    #[inline]
    fn content_and_canonical_primitive_part(self) -> (Natural, IntegerMatrix) {
        let content = content(&self.elements);
        let elements = normalized(&self.elements, &content, negate(self));
        (
            content,
            IntegerMatrix::from_owned_elements(self.height(), self.width(), elements),
        )
    }
}
