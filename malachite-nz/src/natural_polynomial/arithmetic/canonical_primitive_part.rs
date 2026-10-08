// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::num::arithmetic::traits::{
    CanonicalPrimitivePart, CanonicalPrimitivePartAssign, ContentAndCanonicalPrimitivePart,
    ContentAndPrimitivePart, PrimitivePart, PrimitivePartAssign,
};

impl CanonicalPrimitivePart for NaturalPolynomial {
    type Output = Self;

    /// Computes the canonical primitive part of a [`NaturalPolynomial`], taking the polynomial by
    /// value.
    ///
    /// The coefficients of a [`NaturalPolynomial`] are never negative, so it has only one associate
    /// with coprime coefficients, and this is the same as
    /// [`primitive_part`](PrimitivePart::primitive_part).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CanonicalPrimitivePart;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("6*x^2+4*x+10").unwrap();
    /// assert_eq!(p.canonical_primitive_part().to_string(), "3*x^2+2*x+5");
    /// ```
    #[inline]
    fn canonical_primitive_part(self) -> Self {
        self.primitive_part()
    }
}

impl CanonicalPrimitivePart for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Computes the canonical primitive part of a [`NaturalPolynomial`], taking the polynomial by
    /// reference.
    ///
    /// The coefficients of a [`NaturalPolynomial`] are never negative, so it has only one associate
    /// with coprime coefficients, and this is the same as
    /// [`primitive_part`](PrimitivePart::primitive_part).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CanonicalPrimitivePart;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("6*x^2+4*x+10").unwrap();
    /// assert_eq!((&p).canonical_primitive_part().to_string(), "3*x^2+2*x+5");
    /// ```
    #[inline]
    fn canonical_primitive_part(self) -> NaturalPolynomial {
        self.primitive_part()
    }
}

impl CanonicalPrimitivePartAssign for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its canonical primitive part.
    ///
    /// The coefficients of a [`NaturalPolynomial`] are never negative, so it has only one associate
    /// with coprime coefficients, and this is the same as
    /// [`primitive_part`](PrimitivePart::primitive_part).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CanonicalPrimitivePartAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("6*x^2+4*x+10").unwrap();
    /// p.canonical_primitive_part_assign();
    /// assert_eq!(p.to_string(), "3*x^2+2*x+5");
    /// ```
    #[inline]
    fn canonical_primitive_part_assign(&mut self) {
        self.primitive_part_assign();
    }
}

impl ContentAndCanonicalPrimitivePart for NaturalPolynomial {
    type Content = Natural;
    type CanonicalPrimitivePart = Self;

    /// Computes the content and the canonical primitive part of a [`NaturalPolynomial`] together,
    /// taking the polynomial by value.
    ///
    /// The coefficients of a [`NaturalPolynomial`] are never negative, so it has only one associate
    /// with coprime coefficients, and this is the same as
    /// [`primitive_part`](PrimitivePart::primitive_part).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentAndCanonicalPrimitivePart;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("6*x^2+4*x+10").unwrap();
    /// let (content, primitive_part) = p.content_and_canonical_primitive_part();
    /// assert_eq!(content, 2u32);
    /// assert_eq!(primitive_part.to_string(), "3*x^2+2*x+5");
    /// ```
    #[inline]
    fn content_and_canonical_primitive_part(self) -> (Natural, Self) {
        self.content_and_primitive_part()
    }
}

impl ContentAndCanonicalPrimitivePart for &NaturalPolynomial {
    type Content = Natural;
    type CanonicalPrimitivePart = NaturalPolynomial;

    /// Computes the content and the canonical primitive part of a [`NaturalPolynomial`] together,
    /// taking the polynomial by reference.
    ///
    /// The coefficients of a [`NaturalPolynomial`] are never negative, so it has only one associate
    /// with coprime coefficients, and this is the same as
    /// [`primitive_part`](PrimitivePart::primitive_part).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentAndCanonicalPrimitivePart;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("6*x^2+4*x+10").unwrap();
    /// let (content, primitive_part) = (&p).content_and_canonical_primitive_part();
    /// assert_eq!(content, 2u32);
    /// assert_eq!(primitive_part.to_string(), "3*x^2+2*x+5");
    /// ```
    #[inline]
    fn content_and_canonical_primitive_part(self) -> (Natural, NaturalPolynomial) {
        self.content_and_primitive_part()
    }
}
