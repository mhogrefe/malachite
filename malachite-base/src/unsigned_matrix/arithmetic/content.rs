// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::matrix::Matrix;
use crate::num::arithmetic::traits::{
    Content, ContentAndPrimitivePart, PrimitivePart, PrimitivePartAssign,
};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_vector::arithmetic::content::{content, divide_by_content};

impl<T: PrimitiveUnsigned> Content for UnsignedMatrix<T> {
    type Output = T;

    /// Computes the content of an [`UnsignedMatrix`], the GCD of its elements, taking the matrix by
    /// value.
    ///
    /// The content of a matrix with no elements, and of a matrix of zeros, is zero. The GCD is
    /// taken element by element, stopping early once it reaches 1.
    ///
    /// $$
    /// f(A) = \gcd_{i, j} A_{ij}.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.elements.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Content;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[6, 4; 10, 8]").unwrap();
    /// assert_eq!(a.content(), 2);
    /// assert_eq!(
    ///     UnsignedMatrix::<u8>::from_str("[]_{0,0}")
    ///         .unwrap()
    ///         .content(),
    ///     0
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_content` from `fmpz_mat/content.c`, FLINT 3.6.0.
    #[inline]
    fn content(self) -> T {
        content(&self.elements)
    }
}

impl<T: PrimitiveUnsigned> Content for &UnsignedMatrix<T> {
    type Output = T;

    /// Computes the content of an [`UnsignedMatrix`], the GCD of its elements, taking the matrix by
    /// reference.
    ///
    /// The content of a matrix with no elements, and of a matrix of zeros, is zero. The GCD is
    /// taken element by element, stopping early once it reaches 1.
    ///
    /// $$
    /// f(A) = \gcd_{i, j} A_{ij}.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.elements.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Content;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[6, 4; 10, 8]").unwrap();
    /// assert_eq!((&a).content(), 2);
    /// assert_eq!(
    ///     (&UnsignedMatrix::<u8>::from_str("[0, 0; 0, 0]").unwrap()).content(),
    ///     0
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_content` from `fmpz_mat/content.c`, FLINT 3.6.0.
    #[inline]
    fn content(self) -> T {
        content(&self.elements)
    }
}

impl<T: PrimitiveUnsigned> PrimitivePart for UnsignedMatrix<T> {
    type Output = Self;

    /// Computes the primitive part of an [`UnsignedMatrix`], taking the matrix by value.
    ///
    /// This is the matrix divided by its content. The elements are non-negative, so this is also
    /// the [`CanonicalPrimitivePart`](crate::num::arithmetic::traits::CanonicalPrimitivePart).
    ///
    /// $$
    /// A = \operatorname{cont}(A) \operatorname{pp}(A).
    /// $$
    ///
    /// The primitive part of a matrix of zeros is itself, keeping its shape.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.elements.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::PrimitivePart;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[6, 4; 10, 8]").unwrap();
    /// assert_eq!(a.primitive_part().to_string(), "[3, 2; 5, 4]");
    /// assert_eq!(
    ///     UnsignedMatrix::<u8>::from_str("[0, 0; 0, 0]")
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

impl<T: PrimitiveUnsigned> PrimitivePart for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Computes the primitive part of an [`UnsignedMatrix`], taking the matrix by reference.
    ///
    /// This is the matrix divided by its content. The elements are non-negative, so this is also
    /// the [`CanonicalPrimitivePart`](crate::num::arithmetic::traits::CanonicalPrimitivePart).
    ///
    /// $$
    /// A = \operatorname{cont}(A) \operatorname{pp}(A).
    /// $$
    ///
    /// The primitive part of a matrix of zeros is itself, keeping its shape.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.elements.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::PrimitivePart;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[6, 4; 10, 8]").unwrap();
    /// assert_eq!((&a).primitive_part().to_string(), "[3, 2; 5, 4]");
    /// ```
    #[inline]
    fn primitive_part(self) -> UnsignedMatrix<T> {
        let content = content(&self.elements);
        let mut elements = self.elements.clone();
        divide_by_content(&mut elements, content);
        UnsignedMatrix::from_owned_elements(self.height(), self.width(), elements)
    }
}

impl<T: PrimitiveUnsigned> PrimitivePartAssign for UnsignedMatrix<T> {
    /// Replaces an [`UnsignedMatrix`] with its primitive part, the matrix divided by its content.
    ///
    /// See [`primitive_part`](PrimitivePart::primitive_part).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.elements.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::PrimitivePartAssign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[6, 4; 10, 8]").unwrap();
    /// a.primitive_part_assign();
    /// assert_eq!(a.to_string(), "[3, 2; 5, 4]");
    /// ```
    #[inline]
    fn primitive_part_assign(&mut self) {
        let content = content(&self.elements);
        divide_by_content(&mut self.elements, content);
    }
}

impl<T: PrimitiveUnsigned> ContentAndPrimitivePart for UnsignedMatrix<T> {
    type Content = T;
    type PrimitivePart = Self;

    /// Computes the content and the primitive part of an [`UnsignedMatrix`] together, taking the
    /// matrix by value.
    ///
    /// See [`content`](Content::content) and [`primitive_part`](PrimitivePart::primitive_part); the
    /// content is found once rather than twice.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.elements.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentAndPrimitivePart;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[6, 4; 10, 8]").unwrap();
    /// let (content, primitive_part) = a.content_and_primitive_part();
    /// assert_eq!(content, 2);
    /// assert_eq!(primitive_part.to_string(), "[3, 2; 5, 4]");
    /// ```
    #[inline]
    fn content_and_primitive_part(mut self) -> (T, Self) {
        let content = content(&self.elements);
        divide_by_content(&mut self.elements, content);
        (content, self)
    }
}

impl<T: PrimitiveUnsigned> ContentAndPrimitivePart for &UnsignedMatrix<T> {
    type Content = T;
    type PrimitivePart = UnsignedMatrix<T>;

    /// Computes the content and the primitive part of an [`UnsignedMatrix`] together, taking the
    /// matrix by reference.
    ///
    /// See [`content`](Content::content) and [`primitive_part`](PrimitivePart::primitive_part); the
    /// content is found once rather than twice.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.elements.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentAndPrimitivePart;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[6, 4; 10, 8]").unwrap();
    /// let (content, primitive_part) = (&a).content_and_primitive_part();
    /// assert_eq!(content, 2);
    /// assert_eq!(primitive_part.to_string(), "[3, 2; 5, 4]");
    /// ```
    #[inline]
    fn content_and_primitive_part(self) -> (T, UnsignedMatrix<T>) {
        let content = content(&self.elements);
        let mut elements = self.elements.clone();
        divide_by_content(&mut elements, content);
        (
            content,
            UnsignedMatrix::from_owned_elements(self.height(), self.width(), elements),
        )
    }
}
