// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    Content, ContentAndPrimitivePart, PrimitivePart, PrimitivePartAssign,
};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_polynomial::arithmetic::content::{content, divide_by_content};
use crate::unsigned_vector::UnsignedVector;

impl<T: PrimitiveUnsigned> Content for UnsignedVector<T> {
    type Output = T;

    /// Computes the content of an [`UnsignedVector`], the GCD of its elements, taking the vector by
    /// value.
    ///
    /// The content of the 0-dimensional vector, and of a vector of zeros, is zero. The GCD is taken
    /// element by element, stopping early once it reaches 1.
    ///
    /// $$
    /// f(v) = \gcd(v_0, v_1, \ldots, v_{n-1}),
    /// $$
    ///
    /// where $n$ is the dimension of $v$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Content;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(6, 4, 10)").unwrap();
    /// assert_eq!(v.content(), 2);
    /// assert_eq!(UnsignedVector::<u8>::from_str("()").unwrap().content(), 0);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_content` from `fmpz_vec/content.c`, FLINT 3.6.0.
    #[inline]
    fn content(self) -> T {
        content(&self.elements)
    }
}

impl<T: PrimitiveUnsigned> Content for &UnsignedVector<T> {
    type Output = T;

    /// Computes the content of an [`UnsignedVector`], the GCD of its elements, taking the vector by
    /// reference.
    ///
    /// The content of the 0-dimensional vector, and of a vector of zeros, is zero. The GCD is taken
    /// element by element, stopping early once it reaches 1.
    ///
    /// $$
    /// f(v) = \gcd(v_0, v_1, \ldots, v_{n-1}),
    /// $$
    ///
    /// where $n$ is the dimension of $v$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Content;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(6, 4, 10)").unwrap();
    /// assert_eq!((&v).content(), 2);
    /// assert_eq!(
    ///     (&UnsignedVector::<u8>::from_str("(0, 0)").unwrap()).content(),
    ///     0
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_content` from `fmpz_vec/content.c`, FLINT 3.6.0.
    #[inline]
    fn content(self) -> T {
        content(&self.elements)
    }
}

impl<T: PrimitiveUnsigned> PrimitivePart for UnsignedVector<T> {
    type Output = Self;

    /// Computes the primitive part of an [`UnsignedVector`], taking the vector by value.
    ///
    /// This is the vector divided by its content. The elements are non-negative, so this is also
    /// the [`CanonicalPrimitivePart`](crate::num::arithmetic::traits::CanonicalPrimitivePart).
    ///
    /// $$
    /// v = \operatorname{cont}(v) \operatorname{pp}(v).
    /// $$
    ///
    /// The primitive part of a vector of zeros is itself, keeping its dimension.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::PrimitivePart;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(6, 4, 10)").unwrap();
    /// assert_eq!(v.primitive_part().to_string(), "(3, 2, 5)");
    /// assert_eq!(
    ///     UnsignedVector::<u8>::from_str("(0, 0)")
    ///         .unwrap()
    ///         .primitive_part()
    ///         .to_string(),
    ///     "(0, 0)"
    /// );
    /// ```
    #[inline]
    fn primitive_part(mut self) -> Self {
        self.primitive_part_assign();
        self
    }
}

impl<T: PrimitiveUnsigned> PrimitivePart for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Computes the primitive part of an [`UnsignedVector`], taking the vector by reference.
    ///
    /// This is the vector divided by its content. The elements are non-negative, so this is also
    /// the [`CanonicalPrimitivePart`](crate::num::arithmetic::traits::CanonicalPrimitivePart).
    ///
    /// $$
    /// v = \operatorname{cont}(v) \operatorname{pp}(v).
    /// $$
    ///
    /// The primitive part of a vector of zeros is itself, keeping its dimension.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::PrimitivePart;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(6, 4, 10)").unwrap();
    /// assert_eq!((&v).primitive_part().to_string(), "(3, 2, 5)");
    /// ```
    #[inline]
    fn primitive_part(self) -> UnsignedVector<T> {
        let content = content(&self.elements);
        let mut elements = self.elements.clone();
        divide_by_content(&mut elements, content);
        UnsignedVector { elements }
    }
}

impl<T: PrimitiveUnsigned> PrimitivePartAssign for UnsignedVector<T> {
    /// Replaces an [`UnsignedVector`] with its primitive part, the vector divided by its content.
    ///
    /// See [`primitive_part`](PrimitivePart::primitive_part).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::PrimitivePartAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut v = UnsignedVector::<u8>::from_str("(6, 4, 10)").unwrap();
    /// v.primitive_part_assign();
    /// assert_eq!(v.to_string(), "(3, 2, 5)");
    /// ```
    #[inline]
    fn primitive_part_assign(&mut self) {
        let content = content(&self.elements);
        divide_by_content(&mut self.elements, content);
    }
}

impl<T: PrimitiveUnsigned> ContentAndPrimitivePart for UnsignedVector<T> {
    type Content = T;
    type PrimitivePart = Self;

    /// Computes the content and the primitive part of an [`UnsignedVector`] together, taking the
    /// vector by value.
    ///
    /// See [`content`](Content::content) and [`primitive_part`](PrimitivePart::primitive_part); the
    /// content is found once rather than twice.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentAndPrimitivePart;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(6, 4, 10)").unwrap();
    /// let (content, primitive_part) = v.content_and_primitive_part();
    /// assert_eq!(content, 2);
    /// assert_eq!(primitive_part.to_string(), "(3, 2, 5)");
    /// ```
    #[inline]
    fn content_and_primitive_part(mut self) -> (T, Self) {
        let content = content(&self.elements);
        divide_by_content(&mut self.elements, content);
        (content, self)
    }
}

impl<T: PrimitiveUnsigned> ContentAndPrimitivePart for &UnsignedVector<T> {
    type Content = T;
    type PrimitivePart = UnsignedVector<T>;

    /// Computes the content and the primitive part of an [`UnsignedVector`] together, taking the
    /// vector by reference.
    ///
    /// See [`content`](Content::content) and [`primitive_part`](PrimitivePart::primitive_part); the
    /// content is found once rather than twice.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentAndPrimitivePart;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(6, 4, 10)").unwrap();
    /// let (content, primitive_part) = (&v).content_and_primitive_part();
    /// assert_eq!(content, 2);
    /// assert_eq!(primitive_part.to_string(), "(3, 2, 5)");
    /// ```
    #[inline]
    fn content_and_primitive_part(self) -> (T, UnsignedVector<T>) {
        let content = content(&self.elements);
        let mut elements = self.elements.clone();
        divide_by_content(&mut elements, content);
        (content, UnsignedVector { elements })
    }
}
