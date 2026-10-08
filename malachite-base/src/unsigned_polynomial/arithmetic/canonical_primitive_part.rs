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
use crate::unsigned_polynomial::UnsignedPolynomial;

impl<T: PrimitiveUnsigned> CanonicalPrimitivePart for UnsignedPolynomial<T> {
    type Output = Self;

    /// Computes the canonical primitive part of an [`UnsignedPolynomial`], taking the polynomial by
    /// value.
    ///
    /// The coefficients of an [`UnsignedPolynomial`] are never negative, so it has only one
    /// associate with coprime coefficients, and this is the same as
    /// [`primitive_part`](PrimitivePart::primitive_part).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CanonicalPrimitivePart;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("6*x^2+4*x+10").unwrap();
    /// assert_eq!(p.canonical_primitive_part().to_string(), "3*x^2+2*x+5");
    /// ```
    #[inline]
    fn canonical_primitive_part(self) -> Self {
        self.primitive_part()
    }
}

impl<T: PrimitiveUnsigned> CanonicalPrimitivePart for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Computes the canonical primitive part of an [`UnsignedPolynomial`], taking the polynomial by
    /// reference.
    ///
    /// The coefficients of an [`UnsignedPolynomial`] are never negative, so it has only one
    /// associate with coprime coefficients, and this is the same as
    /// [`primitive_part`](PrimitivePart::primitive_part).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CanonicalPrimitivePart;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("6*x^2+4*x+10").unwrap();
    /// assert_eq!((&p).canonical_primitive_part().to_string(), "3*x^2+2*x+5");
    /// ```
    #[inline]
    fn canonical_primitive_part(self) -> UnsignedPolynomial<T> {
        self.primitive_part()
    }
}

impl<T: PrimitiveUnsigned> CanonicalPrimitivePartAssign for UnsignedPolynomial<T> {
    /// Replaces an [`UnsignedPolynomial`] with its canonical primitive part.
    ///
    /// The coefficients of an [`UnsignedPolynomial`] are never negative, so it has only one
    /// associate with coprime coefficients, and this is the same as
    /// [`primitive_part`](PrimitivePart::primitive_part).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CanonicalPrimitivePartAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("6*x^2+4*x+10").unwrap();
    /// p.canonical_primitive_part_assign();
    /// assert_eq!(p.to_string(), "3*x^2+2*x+5");
    /// ```
    #[inline]
    fn canonical_primitive_part_assign(&mut self) {
        self.primitive_part_assign();
    }
}

impl<T: PrimitiveUnsigned> ContentAndCanonicalPrimitivePart for UnsignedPolynomial<T> {
    type Content = T;
    type CanonicalPrimitivePart = Self;

    /// Computes the content and the canonical primitive part of an [`UnsignedPolynomial`] together,
    /// taking the polynomial by value.
    ///
    /// The coefficients of an [`UnsignedPolynomial`] are never negative, so it has only one
    /// associate with coprime coefficients, and this is the same as
    /// [`primitive_part`](PrimitivePart::primitive_part).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentAndCanonicalPrimitivePart;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("6*x^2+4*x+10").unwrap();
    /// let (content, primitive_part) = p.content_and_canonical_primitive_part();
    /// assert_eq!(content, 2);
    /// assert_eq!(primitive_part.to_string(), "3*x^2+2*x+5");
    /// ```
    #[inline]
    fn content_and_canonical_primitive_part(self) -> (T, Self) {
        self.content_and_primitive_part()
    }
}

impl<T: PrimitiveUnsigned> ContentAndCanonicalPrimitivePart for &UnsignedPolynomial<T> {
    type Content = T;
    type CanonicalPrimitivePart = UnsignedPolynomial<T>;

    /// Computes the content and the canonical primitive part of an [`UnsignedPolynomial`] together,
    /// taking the polynomial by reference.
    ///
    /// The coefficients of an [`UnsignedPolynomial`] are never negative, so it has only one
    /// associate with coprime coefficients, and this is the same as
    /// [`primitive_part`](PrimitivePart::primitive_part).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentAndCanonicalPrimitivePart;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("6*x^2+4*x+10").unwrap();
    /// let (content, primitive_part) = (&p).content_and_canonical_primitive_part();
    /// assert_eq!(content, 2);
    /// assert_eq!(primitive_part.to_string(), "3*x^2+2*x+5");
    /// ```
    #[inline]
    fn content_and_canonical_primitive_part(self) -> (T, UnsignedPolynomial<T>) {
        self.content_and_primitive_part()
    }
}
