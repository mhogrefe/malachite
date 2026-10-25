// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    CanonicalPrimitivePart, CanonicalPrimitivePartAssign, ContentAndCanonicalPrimitivePart,
    ContentAndPrimitivePart, PrimitivePart, PrimitivePartAssign,
};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;

impl<T: PrimitiveUnsigned> CanonicalPrimitivePart for UnsignedMatrix<T> {
    type Output = Self;

    /// Computes the canonical primitive part of an [`UnsignedMatrix`], taking the matrix by value.
    ///
    /// The elements of an [`UnsignedMatrix`] are never negative, so it has only one associate with
    /// coprime elements, and this is the same as [`primitive_part`](PrimitivePart::primitive_part).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CanonicalPrimitivePart;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[6, 4; 10, 8]").unwrap();
    /// assert_eq!(a.canonical_primitive_part().to_string(), "[3, 2; 5, 4]");
    /// ```
    #[inline]
    fn canonical_primitive_part(self) -> Self {
        self.primitive_part()
    }
}

impl<T: PrimitiveUnsigned> CanonicalPrimitivePart for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Computes the canonical primitive part of an [`UnsignedMatrix`], taking the matrix by
    /// reference.
    ///
    /// The elements of an [`UnsignedMatrix`] are never negative, so it has only one associate with
    /// coprime elements, and this is the same as [`primitive_part`](PrimitivePart::primitive_part).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CanonicalPrimitivePart;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[6, 4; 10, 8]").unwrap();
    /// assert_eq!((&a).canonical_primitive_part().to_string(), "[3, 2; 5, 4]");
    /// ```
    #[inline]
    fn canonical_primitive_part(self) -> UnsignedMatrix<T> {
        self.primitive_part()
    }
}

impl<T: PrimitiveUnsigned> CanonicalPrimitivePartAssign for UnsignedMatrix<T> {
    /// Replaces an [`UnsignedMatrix`] with its canonical primitive part.
    ///
    /// The elements of an [`UnsignedMatrix`] are never negative, so it has only one associate with
    /// coprime elements, and this is the same as [`primitive_part`](PrimitivePart::primitive_part).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CanonicalPrimitivePartAssign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[6, 4; 10, 8]").unwrap();
    /// a.canonical_primitive_part_assign();
    /// assert_eq!(a.to_string(), "[3, 2; 5, 4]");
    /// ```
    #[inline]
    fn canonical_primitive_part_assign(&mut self) {
        self.primitive_part_assign();
    }
}

impl<T: PrimitiveUnsigned> ContentAndCanonicalPrimitivePart for UnsignedMatrix<T> {
    type Content = T;
    type CanonicalPrimitivePart = Self;

    /// Computes the content and the canonical primitive part of an [`UnsignedMatrix`] together,
    /// taking the matrix by value.
    ///
    /// The elements of an [`UnsignedMatrix`] are never negative, so it has only one associate with
    /// coprime elements, and this is the same as [`primitive_part`](PrimitivePart::primitive_part).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentAndCanonicalPrimitivePart;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[6, 4; 10, 8]").unwrap();
    /// let (content, primitive_part) = a.content_and_canonical_primitive_part();
    /// assert_eq!(content, 2);
    /// assert_eq!(primitive_part.to_string(), "[3, 2; 5, 4]");
    /// ```
    #[inline]
    fn content_and_canonical_primitive_part(self) -> (T, Self) {
        self.content_and_primitive_part()
    }
}

impl<T: PrimitiveUnsigned> ContentAndCanonicalPrimitivePart for &UnsignedMatrix<T> {
    type Content = T;
    type CanonicalPrimitivePart = UnsignedMatrix<T>;

    /// Computes the content and the canonical primitive part of an [`UnsignedMatrix`] together,
    /// taking the matrix by reference.
    ///
    /// The elements of an [`UnsignedMatrix`] are never negative, so it has only one associate with
    /// coprime elements, and this is the same as [`primitive_part`](PrimitivePart::primitive_part).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentAndCanonicalPrimitivePart;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[6, 4; 10, 8]").unwrap();
    /// let (content, primitive_part) = (&a).content_and_canonical_primitive_part();
    /// assert_eq!(content, 2);
    /// assert_eq!(primitive_part.to_string(), "[3, 2; 5, 4]");
    /// ```
    #[inline]
    fn content_and_canonical_primitive_part(self) -> (T, UnsignedMatrix<T>) {
        self.content_and_primitive_part()
    }
}
