// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    Content, ContentAndPrimitivePart, DivExact, DivExactAssign, GcdAssign, PrimitivePart,
    PrimitivePartAssign,
};
use malachite_base::num::basic::traits::Zero;

// The GCD of the elements. It stops as soon as it reaches 1, since nothing can lower it further.
pub(crate) fn content(coefficients: &[Natural]) -> Natural {
    let mut gcd = Natural::ZERO;
    for c in coefficients {
        gcd.gcd_assign(c);
        if gcd == 1u32 {
            break;
        }
    }
    gcd
}

// Divides every element by the content, which divides each of them exactly.
pub(crate) fn divide_by_content(coefficients: &mut [Natural], content: &Natural) {
    if *content > 1u32 {
        for c in coefficients {
            c.div_exact_assign(content);
        }
    }
}

// The elements divided by the content, as new values.
pub(crate) fn divided_by_content(coefficients: &[Natural], content: &Natural) -> Vec<Natural> {
    if *content > 1u32 {
        coefficients.iter().map(|c| c.div_exact(content)).collect()
    } else {
        coefficients.to_vec()
    }
}

impl Content for NaturalVector {
    type Output = Natural;

    /// Computes the content of a [`NaturalVector`], the GCD of its elements, taking the vector by
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(6, 4, 10)").unwrap();
    /// assert_eq!(v.content(), 2u32);
    /// assert_eq!(NaturalVector::from_str("()").unwrap().content(), 0u32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_content` from `fmpz_vec/content.c`, FLINT 3.6.0.
    #[inline]
    fn content(self) -> Natural {
        content(&self.elements)
    }
}

impl Content for &NaturalVector {
    type Output = Natural;

    /// Computes the content of a [`NaturalVector`], the GCD of its elements, taking the vector by
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(6, 4, 10)").unwrap();
    /// assert_eq!((&v).content(), 2u32);
    /// assert_eq!(
    ///     (&NaturalVector::from_str("(0, 0)").unwrap()).content(),
    ///     0u32
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_content` from `fmpz_vec/content.c`, FLINT 3.6.0.
    #[inline]
    fn content(self) -> Natural {
        content(&self.elements)
    }
}

impl PrimitivePart for NaturalVector {
    type Output = Self;

    /// Computes the primitive part of a [`NaturalVector`], taking the vector by value.
    ///
    /// This is the vector divided by its content. The elements are non-negative, so this is also
    /// the
    /// [`CanonicalPrimitivePart`](malachite_base::num::arithmetic::traits::CanonicalPrimitivePart).
    ///
    /// $$
    /// v = \operatorname{cont}(v) \operatorname{pp}(v).
    /// $$
    ///
    /// The primitive part of a vector of zeros is itself, keeping its dimension.
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(6, 4, 10)").unwrap();
    /// assert_eq!(v.primitive_part().to_string(), "(3, 2, 5)");
    /// assert_eq!(
    ///     NaturalVector::from_str("(0, 0)")
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

impl PrimitivePart for &NaturalVector {
    type Output = NaturalVector;

    /// Computes the primitive part of a [`NaturalVector`], taking the vector by reference.
    ///
    /// This is the vector divided by its content. The elements are non-negative, so this is also
    /// the
    /// [`CanonicalPrimitivePart`](malachite_base::num::arithmetic::traits::CanonicalPrimitivePart).
    ///
    /// $$
    /// v = \operatorname{cont}(v) \operatorname{pp}(v).
    /// $$
    ///
    /// The primitive part of a vector of zeros is itself, keeping its dimension.
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(6, 4, 10)").unwrap();
    /// assert_eq!((&v).primitive_part().to_string(), "(3, 2, 5)");
    /// ```
    #[inline]
    fn primitive_part(self) -> NaturalVector {
        let content = content(&self.elements);
        NaturalVector {
            elements: divided_by_content(&self.elements, &content),
        }
    }
}

impl PrimitivePartAssign for NaturalVector {
    /// Replaces a [`NaturalVector`] with its primitive part, the vector divided by its content.
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(6, 4, 10)").unwrap();
    /// v.primitive_part_assign();
    /// assert_eq!(v.to_string(), "(3, 2, 5)");
    /// ```
    #[inline]
    fn primitive_part_assign(&mut self) {
        let content = content(&self.elements);
        divide_by_content(&mut self.elements, &content);
    }
}

impl ContentAndPrimitivePart for NaturalVector {
    type Content = Natural;
    type PrimitivePart = Self;

    /// Computes the content and the primitive part of a [`NaturalVector`] together, taking the
    /// vector by value.
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(6, 4, 10)").unwrap();
    /// let (content, primitive_part) = v.content_and_primitive_part();
    /// assert_eq!(content, 2u32);
    /// assert_eq!(primitive_part.to_string(), "(3, 2, 5)");
    /// ```
    #[inline]
    fn content_and_primitive_part(mut self) -> (Natural, Self) {
        let content = content(&self.elements);
        divide_by_content(&mut self.elements, &content);
        (content, self)
    }
}

impl ContentAndPrimitivePart for &NaturalVector {
    type Content = Natural;
    type PrimitivePart = NaturalVector;

    /// Computes the content and the primitive part of a [`NaturalVector`] together, taking the
    /// vector by reference.
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(6, 4, 10)").unwrap();
    /// let (content, primitive_part) = (&v).content_and_primitive_part();
    /// assert_eq!(content, 2u32);
    /// assert_eq!(primitive_part.to_string(), "(3, 2, 5)");
    /// ```
    #[inline]
    fn content_and_primitive_part(self) -> (Natural, NaturalVector) {
        let content = content(&self.elements);
        let elements = divided_by_content(&self.elements, &content);
        (content, NaturalVector { elements })
    }
}
