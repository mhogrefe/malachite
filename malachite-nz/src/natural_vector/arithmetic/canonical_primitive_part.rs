// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use malachite_base::num::arithmetic::traits::{
    CanonicalPrimitivePart, CanonicalPrimitivePartAssign, ContentAndCanonicalPrimitivePart,
    ContentAndPrimitivePart, PrimitivePart, PrimitivePartAssign,
};

impl CanonicalPrimitivePart for NaturalVector {
    type Output = Self;

    /// Computes the canonical primitive part of a [`NaturalVector`], taking the vector by value.
    ///
    /// The elements of a [`NaturalVector`] are never negative, so it has only one associate with
    /// coprime elements, and this is the same as [`primitive_part`](PrimitivePart::primitive_part).
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(6, 4, 10)").unwrap();
    /// assert_eq!(v.canonical_primitive_part().to_string(), "(3, 2, 5)");
    /// ```
    #[inline]
    fn canonical_primitive_part(self) -> Self {
        self.primitive_part()
    }
}

impl CanonicalPrimitivePart for &NaturalVector {
    type Output = NaturalVector;

    /// Computes the canonical primitive part of a [`NaturalVector`], taking the vector by
    /// reference.
    ///
    /// The elements of a [`NaturalVector`] are never negative, so it has only one associate with
    /// coprime elements, and this is the same as [`primitive_part`](PrimitivePart::primitive_part).
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(6, 4, 10)").unwrap();
    /// assert_eq!((&v).canonical_primitive_part().to_string(), "(3, 2, 5)");
    /// ```
    #[inline]
    fn canonical_primitive_part(self) -> NaturalVector {
        self.primitive_part()
    }
}

impl CanonicalPrimitivePartAssign for NaturalVector {
    /// Replaces a [`NaturalVector`] with its canonical primitive part.
    ///
    /// The elements of a [`NaturalVector`] are never negative, so it has only one associate with
    /// coprime elements, and this is the same as [`primitive_part`](PrimitivePart::primitive_part).
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(6, 4, 10)").unwrap();
    /// v.canonical_primitive_part_assign();
    /// assert_eq!(v.to_string(), "(3, 2, 5)");
    /// ```
    #[inline]
    fn canonical_primitive_part_assign(&mut self) {
        self.primitive_part_assign();
    }
}

impl ContentAndCanonicalPrimitivePart for NaturalVector {
    type Content = Natural;
    type CanonicalPrimitivePart = Self;

    /// Computes the content and the canonical primitive part of a [`NaturalVector`] together,
    /// taking the vector by value.
    ///
    /// The elements of a [`NaturalVector`] are never negative, so it has only one associate with
    /// coprime elements, and this is the same as [`primitive_part`](PrimitivePart::primitive_part).
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(6, 4, 10)").unwrap();
    /// let (content, primitive_part) = v.content_and_canonical_primitive_part();
    /// assert_eq!(content, 2u32);
    /// assert_eq!(primitive_part.to_string(), "(3, 2, 5)");
    /// ```
    #[inline]
    fn content_and_canonical_primitive_part(self) -> (Natural, Self) {
        self.content_and_primitive_part()
    }
}

impl ContentAndCanonicalPrimitivePart for &NaturalVector {
    type Content = Natural;
    type CanonicalPrimitivePart = NaturalVector;

    /// Computes the content and the canonical primitive part of a [`NaturalVector`] together,
    /// taking the vector by reference.
    ///
    /// The elements of a [`NaturalVector`] are never negative, so it has only one associate with
    /// coprime elements, and this is the same as [`primitive_part`](PrimitivePart::primitive_part).
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(6, 4, 10)").unwrap();
    /// let (content, primitive_part) = (&v).content_and_canonical_primitive_part();
    /// assert_eq!(content, 2u32);
    /// assert_eq!(primitive_part.to_string(), "(3, 2, 5)");
    /// ```
    #[inline]
    fn content_and_canonical_primitive_part(self) -> (Natural, NaturalVector) {
        self.content_and_primitive_part()
    }
}
