// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{
    CanonicalPrimitivePart, CanonicalPrimitivePartAssign, CanonicalizeUnit, CanonicalizeUnitAssign,
    ContentAndCanonicalPrimitivePart, ContentAndPrimitivePart, PrimitivePart, PrimitivePartAssign,
};

impl CanonicalPrimitivePart for GaussianInteger {
    type Output = Self;

    /// Computes the canonical primitive part of a [`GaussianInteger`], taking the
    /// [`GaussianInteger`] by value.
    ///
    /// This is the [`primitive_part`](PrimitivePart::primitive_part), multiplied by the power of
    /// $i$ that brings it into canonical unit form, as
    /// [`canonicalize_unit`](CanonicalizeUnit::canonicalize_unit) chooses it: the associate whose
    /// argument lies in $(-\pi/4, \pi/4]$. So the four associates of a Gaussian integer all have
    /// the same canonical primitive part. Zero's is zero.
    ///
    /// $$
    /// x = i^k \operatorname{cont}(x) \operatorname{cpp}(x)
    /// $$
    ///
    /// for some $k \in \\{0, 1, 2, 3\\}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalPrimitivePart;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianInteger::from_str("-6+9i").unwrap();
    /// assert_eq!(x.canonical_primitive_part().to_string(), "3+2i");
    /// ```
    #[inline]
    fn canonical_primitive_part(self) -> Self {
        self.primitive_part().canonicalize_unit()
    }
}

impl CanonicalPrimitivePart for &GaussianInteger {
    type Output = GaussianInteger;

    /// Computes the canonical primitive part of a [`GaussianInteger`], taking the
    /// [`GaussianInteger`] by reference.
    ///
    /// This is the [`primitive_part`](PrimitivePart::primitive_part), multiplied by the power of
    /// $i$ that brings it into canonical unit form, as
    /// [`canonicalize_unit`](CanonicalizeUnit::canonicalize_unit) chooses it: the associate whose
    /// argument lies in $(-\pi/4, \pi/4]$. So the four associates of a Gaussian integer all have
    /// the same canonical primitive part. Zero's is zero.
    ///
    /// $$
    /// x = i^k \operatorname{cont}(x) \operatorname{cpp}(x)
    /// $$
    ///
    /// for some $k \in \\{0, 1, 2, 3\\}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalPrimitivePart;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianInteger::from_str("-6+9i").unwrap();
    /// assert_eq!((&x).canonical_primitive_part().to_string(), "3+2i");
    /// ```
    #[inline]
    fn canonical_primitive_part(self) -> GaussianInteger {
        self.primitive_part().canonicalize_unit()
    }
}

impl CanonicalPrimitivePartAssign for GaussianInteger {
    /// Replaces a [`GaussianInteger`] with its canonical primitive part.
    ///
    /// See [`canonical_primitive_part`](CanonicalPrimitivePart::canonical_primitive_part).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalPrimitivePartAssign;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let mut x = GaussianInteger::from_str("-6+9i").unwrap();
    /// x.canonical_primitive_part_assign();
    /// assert_eq!(x.to_string(), "3+2i");
    /// ```
    #[inline]
    fn canonical_primitive_part_assign(&mut self) {
        self.primitive_part_assign();
        self.canonicalize_unit_assign();
    }
}

impl ContentAndCanonicalPrimitivePart for GaussianInteger {
    type Content = Natural;
    type CanonicalPrimitivePart = Self;

    /// Splits a [`GaussianInteger`] into its content and its canonical primitive part, taking the
    /// [`GaussianInteger`] by value.
    ///
    /// See [`content_and_primitive_part`](ContentAndPrimitivePart::content_and_primitive_part) and
    /// [`canonical_primitive_part`](CanonicalPrimitivePart::canonical_primitive_part); the content
    /// is found once rather than twice.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ContentAndCanonicalPrimitivePart;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let (content, primitive) = GaussianInteger::from_str("-6+9i")
    ///     .unwrap()
    ///     .content_and_canonical_primitive_part();
    /// assert_eq!(content, 3);
    /// assert_eq!(primitive.to_string(), "3+2i");
    /// ```
    #[inline]
    fn content_and_canonical_primitive_part(self) -> (Natural, Self) {
        let (content, primitive) = self.content_and_primitive_part();
        (content, primitive.canonicalize_unit())
    }
}

impl ContentAndCanonicalPrimitivePart for &GaussianInteger {
    type Content = Natural;
    type CanonicalPrimitivePart = GaussianInteger;

    /// Splits a [`GaussianInteger`] into its content and its canonical primitive part, taking the
    /// [`GaussianInteger`] by reference.
    ///
    /// See [`content_and_primitive_part`](ContentAndPrimitivePart::content_and_primitive_part) and
    /// [`canonical_primitive_part`](CanonicalPrimitivePart::canonical_primitive_part); the content
    /// is found once rather than twice.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ContentAndCanonicalPrimitivePart;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianInteger::from_str("-6+9i").unwrap();
    /// let (content, primitive) = (&x).content_and_canonical_primitive_part();
    /// assert_eq!(content, 3);
    /// assert_eq!(primitive.to_string(), "3+2i");
    /// ```
    #[inline]
    fn content_and_canonical_primitive_part(self) -> (Natural, GaussianInteger) {
        let (content, primitive) = self.content_and_primitive_part();
        (content, primitive.canonicalize_unit())
    }
}
