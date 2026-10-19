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
    Content, ContentAndPrimitivePart, PrimitivePart, PrimitivePartAssign,
};

impl Content for IntegerMatrix {
    type Output = Natural;

    /// Computes the content of an [`IntegerMatrix`], the GCD of its elements, taking the matrix by
    /// value.
    ///
    /// The content of a matrix with no elements, and of a matrix of zeros, is zero. The GCD is
    /// taken element by element, stopping early once it reaches 1.
    ///
    /// $$
    /// f(A) = \gcd_{i, j} A_{ij}.
    /// $$
    ///
    /// The content is the GCD of the absolute values, so it is non-negative.
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
    /// use malachite_base::num::arithmetic::traits::Content;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[-6, 4; -10, 8]").unwrap();
    /// assert_eq!(a.content(), 2u32);
    /// assert_eq!(IntegerMatrix::from_str("[]_{0,0}").unwrap().content(), 0u32);
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_content` from `fmpz_mat/content.c`, FLINT 3.6.0.
    #[inline]
    fn content(self) -> Natural {
        content(&self.elements)
    }
}

impl Content for &IntegerMatrix {
    type Output = Natural;

    /// Computes the content of an [`IntegerMatrix`], the GCD of its elements, taking the matrix by
    /// reference.
    ///
    /// The content of a matrix with no elements, and of a matrix of zeros, is zero. The GCD is
    /// taken element by element, stopping early once it reaches 1.
    ///
    /// $$
    /// f(A) = \gcd_{i, j} A_{ij}.
    /// $$
    ///
    /// The content is the GCD of the absolute values, so it is non-negative.
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
    /// use malachite_base::num::arithmetic::traits::Content;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[-6, 4; -10, 8]").unwrap();
    /// assert_eq!((&a).content(), 2u32);
    /// assert_eq!(
    ///     (&IntegerMatrix::from_str("[0, 0; 0, 0]").unwrap()).content(),
    ///     0u32
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_content` from `fmpz_mat/content.c`, FLINT 3.6.0.
    #[inline]
    fn content(self) -> Natural {
        content(&self.elements)
    }
}

impl PrimitivePart for IntegerMatrix {
    type Output = Self;

    /// Computes the primitive part of an [`IntegerMatrix`], taking the matrix by value.
    ///
    /// This is the matrix divided by its content, with every element keeping its sign, so the
    /// primitive part of $-A$ is the negation of the primitive part of $A$. For the one whose first
    /// nonzero element in row-major order is positive, see [`canonical_primitive_part`](
    /// malachite_base::num::arithmetic::traits::CanonicalPrimitivePart::canonical_primitive_part).
    ///
    /// $$
    /// A = \operatorname{cont}(A) \operatorname{pp}(A).
    /// $$
    ///
    /// The primitive part of a matrix of zeros is itself, keeping its shape.
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
    /// use malachite_base::num::arithmetic::traits::PrimitivePart;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[-6, 4; -10, 8]").unwrap();
    /// assert_eq!(a.primitive_part().to_string(), "[-3, 2; -5, 4]");
    /// assert_eq!(
    ///     IntegerMatrix::from_str("[0, 0; 0, 0]")
    ///         .unwrap()
    ///         .primitive_part()
    ///         .to_string(),
    ///     "[0, 0; 0, 0]"
    /// );
    /// ```
    #[inline]
    fn primitive_part(mut self) -> Self {
        self.primitive_part_assign();
        self
    }
}

impl PrimitivePart for &IntegerMatrix {
    type Output = IntegerMatrix;

    /// Computes the primitive part of an [`IntegerMatrix`], taking the matrix by reference.
    ///
    /// This is the matrix divided by its content, with every element keeping its sign, so the
    /// primitive part of $-A$ is the negation of the primitive part of $A$. For the one whose first
    /// nonzero element in row-major order is positive, see [`canonical_primitive_part`](
    /// malachite_base::num::arithmetic::traits::CanonicalPrimitivePart::canonical_primitive_part).
    ///
    /// $$
    /// A = \operatorname{cont}(A) \operatorname{pp}(A).
    /// $$
    ///
    /// The primitive part of a matrix of zeros is itself, keeping its shape.
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
    /// use malachite_base::num::arithmetic::traits::PrimitivePart;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[-6, 4; -10, 8]").unwrap();
    /// assert_eq!((&a).primitive_part().to_string(), "[-3, 2; -5, 4]");
    /// ```
    #[inline]
    fn primitive_part(self) -> IntegerMatrix {
        let content = content(&self.elements);
        IntegerMatrix::from_owned_elements(
            self.height(),
            self.width(),
            normalized(&self.elements, &content, false),
        )
    }
}

impl PrimitivePartAssign for IntegerMatrix {
    /// Replaces an [`IntegerMatrix`] with its primitive part, the matrix divided by its content.
    ///
    /// See [`primitive_part`](PrimitivePart::primitive_part).
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
    /// use malachite_base::num::arithmetic::traits::PrimitivePartAssign;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let mut a = IntegerMatrix::from_str("[-6, 4; -10, 8]").unwrap();
    /// a.primitive_part_assign();
    /// assert_eq!(a.to_string(), "[-3, 2; -5, 4]");
    /// ```
    #[inline]
    fn primitive_part_assign(&mut self) {
        let content = content(&self.elements);
        normalize_in_place(&mut self.elements, &content, false);
    }
}

impl ContentAndPrimitivePart for IntegerMatrix {
    type Content = Natural;
    type PrimitivePart = Self;

    /// Computes the content and the primitive part of an [`IntegerMatrix`] together, taking the
    /// matrix by value.
    ///
    /// See [`content`](Content::content) and [`primitive_part`](PrimitivePart::primitive_part); the
    /// content is found once rather than twice.
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
    /// use malachite_base::num::arithmetic::traits::ContentAndPrimitivePart;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[-6, 4; -10, 8]").unwrap();
    /// let (content, primitive_part) = a.content_and_primitive_part();
    /// assert_eq!(content, 2u32);
    /// assert_eq!(primitive_part.to_string(), "[-3, 2; -5, 4]");
    /// ```
    #[inline]
    fn content_and_primitive_part(mut self) -> (Natural, Self) {
        let content = content(&self.elements);
        normalize_in_place(&mut self.elements, &content, false);
        (content, self)
    }
}

impl ContentAndPrimitivePart for &IntegerMatrix {
    type Content = Natural;
    type PrimitivePart = IntegerMatrix;

    /// Computes the content and the primitive part of an [`IntegerMatrix`] together, taking the
    /// matrix by reference.
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
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentAndPrimitivePart;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[-6, 4; -10, 8]").unwrap();
    /// let (content, primitive_part) = (&a).content_and_primitive_part();
    /// assert_eq!(content, 2u32);
    /// assert_eq!(primitive_part.to_string(), "[-3, 2; -5, 4]");
    /// ```
    #[inline]
    fn content_and_primitive_part(self) -> (Natural, IntegerMatrix) {
        let content = content(&self.elements);
        let elements = normalized(&self.elements, &content, false);
        (
            content,
            IntegerMatrix::from_owned_elements(self.height(), self.width(), elements),
        )
    }
}
