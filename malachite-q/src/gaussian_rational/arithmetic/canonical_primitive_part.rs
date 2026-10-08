// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::gaussian_rational::GaussianRational;
use malachite_base::num::arithmetic::traits::{
    CanonicalPrimitivePart, CanonicalizeUnit, ContentAndCanonicalPrimitivePart,
    ContentAndPrimitivePart, PrimitivePart,
};
use malachite_nz::gaussian_integer::GaussianInteger;

impl CanonicalPrimitivePart for GaussianRational {
    type Output = GaussianInteger;

    /// Computes the canonical primitive part of a [`GaussianRational`], taking the
    /// [`GaussianRational`] by value.
    ///
    /// This is the [`primitive_part`](PrimitivePart::primitive_part), a [`GaussianInteger`] with
    /// coprime parts, multiplied by the power of $i$ that brings it into canonical unit form, as
    /// [`canonicalize_unit`](CanonicalizeUnit::canonicalize_unit) chooses it for a
    /// [`GaussianInteger`]: the associate whose argument lies in $(-\pi/4, \pi/4]$. So the four
    /// associates of a Gaussian rational all have the same canonical primitive part. Zero's is
    /// zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the numerators and denominators of the real and imaginary parts of `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalPrimitivePart;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("-1/2+3i/4").unwrap();
    /// assert_eq!(x.canonical_primitive_part().to_string(), "3+2i");
    /// ```
    #[inline]
    fn canonical_primitive_part(self) -> GaussianInteger {
        self.primitive_part().canonicalize_unit()
    }
}

impl CanonicalPrimitivePart for &GaussianRational {
    type Output = GaussianInteger;

    /// Computes the canonical primitive part of a [`GaussianRational`], taking the
    /// [`GaussianRational`] by reference.
    ///
    /// This is the [`primitive_part`](PrimitivePart::primitive_part), a [`GaussianInteger`] with
    /// coprime parts, multiplied by the power of $i$ that brings it into canonical unit form, as
    /// [`canonicalize_unit`](CanonicalizeUnit::canonicalize_unit) chooses it for a
    /// [`GaussianInteger`]: the associate whose argument lies in $(-\pi/4, \pi/4]$. So the four
    /// associates of a Gaussian rational all have the same canonical primitive part. Zero's is
    /// zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the numerators and denominators of the real and imaginary parts of `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalPrimitivePart;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("-1/2+3i/4").unwrap();
    /// assert_eq!((&x).canonical_primitive_part().to_string(), "3+2i");
    /// ```
    #[inline]
    fn canonical_primitive_part(self) -> GaussianInteger {
        self.primitive_part().canonicalize_unit()
    }
}

impl ContentAndCanonicalPrimitivePart for GaussianRational {
    type Content = Rational;
    type CanonicalPrimitivePart = GaussianInteger;

    /// Splits a [`GaussianRational`] into its content and its canonical primitive part, taking the
    /// [`GaussianRational`] by value.
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
    /// bits of the numerators and denominators of the real and imaginary parts of `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ContentAndCanonicalPrimitivePart;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let (content, primitive) = GaussianRational::from_str("-1/2+3i/4")
    ///     .unwrap()
    ///     .content_and_canonical_primitive_part();
    /// assert_eq!(content.to_string(), "1/4");
    /// assert_eq!(primitive.to_string(), "3+2i");
    /// ```
    #[inline]
    fn content_and_canonical_primitive_part(self) -> (Rational, GaussianInteger) {
        let (content, primitive) = self.content_and_primitive_part();
        (content, primitive.canonicalize_unit())
    }
}

impl ContentAndCanonicalPrimitivePart for &GaussianRational {
    type Content = Rational;
    type CanonicalPrimitivePart = GaussianInteger;

    /// Splits a [`GaussianRational`] into its content and its canonical primitive part, taking the
    /// [`GaussianRational`] by reference.
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
    /// bits of the numerators and denominators of the real and imaginary parts of `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ContentAndCanonicalPrimitivePart;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("-1/2+3i/4").unwrap();
    /// let (content, primitive) = (&x).content_and_canonical_primitive_part();
    /// assert_eq!(content.to_string(), "1/4");
    /// assert_eq!(primitive.to_string(), "3+2i");
    /// ```
    #[inline]
    fn content_and_canonical_primitive_part(self) -> (Rational, GaussianInteger) {
        let (content, primitive) = self.content_and_primitive_part();
        (content, primitive.canonicalize_unit())
    }
}
