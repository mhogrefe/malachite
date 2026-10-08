// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_vector::IntegerVector;
use crate::natural::Natural;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    Content, ContentAndPrimitivePart, DivExact, DivExactAssign, GcdAssign, NegAssign,
    PrimitivePart, PrimitivePartAssign,
};
use malachite_base::num::basic::traits::Zero;

// The GCD of the elements' absolute values. It stops as soon as it reaches 1, since nothing can
// lower it further.
pub(crate) fn content(coefficients: &[Integer]) -> Natural {
    let mut gcd = Natural::ZERO;
    for c in coefficients {
        gcd.gcd_assign(&c.abs);
        if gcd == 1u32 {
            break;
        }
    }
    gcd
}

// Divides every element by the content, which divides each of them exactly, and negates them all if
// `negate` is set.
pub(crate) fn normalize_in_place(coefficients: &mut [Integer], content: &Natural, negate: bool) {
    for c in coefficients {
        if *content > 1u32 {
            c.abs.div_exact_assign(content);
        }
        if negate {
            c.neg_assign();
        }
    }
}

// The elements divided by the content, and negated if `negate` is set, as new values.
pub(crate) fn normalized(
    coefficients: &[Integer],
    content: &Natural,
    negate: bool,
) -> Vec<Integer> {
    coefficients
        .iter()
        .map(|c| {
            let abs = if *content > 1u32 {
                (&c.abs).div_exact(content)
            } else {
                c.abs.clone()
            };
            Integer::from_sign_and_abs(c.sign != negate, abs)
        })
        .collect()
}

impl Content for IntegerVector {
    type Output = Natural;

    /// Computes the content of an [`IntegerVector`], the GCD of its elements, taking the vector by
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
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(-6, 4, -10)").unwrap();
    /// assert_eq!(v.content(), 2u32);
    /// assert_eq!(IntegerVector::from_str("()").unwrap().content(), 0u32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_content` from `fmpz_vec/content.c`, FLINT 3.6.0.
    #[inline]
    fn content(self) -> Natural {
        content(&self.elements)
    }
}

impl Content for &IntegerVector {
    type Output = Natural;

    /// Computes the content of an [`IntegerVector`], the GCD of its elements, taking the vector by
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
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(-6, 4, -10)").unwrap();
    /// assert_eq!((&v).content(), 2u32);
    /// assert_eq!(
    ///     (&IntegerVector::from_str("(0, 0)").unwrap()).content(),
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

impl PrimitivePart for IntegerVector {
    type Output = Self;

    /// Computes the primitive part of an [`IntegerVector`], taking the vector by value.
    ///
    /// This is the vector divided by its content, with every element keeping its sign, so the
    /// primitive part of $-v$ is the negation of the primitive part of $v$. For the one whose first
    /// nonzero element is positive, see [`canonical_primitive_part`](
    /// malachite_base::num::arithmetic::traits::CanonicalPrimitivePart::canonical_primitive_part).
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
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(-6, 4, -10)").unwrap();
    /// assert_eq!(v.primitive_part().to_string(), "(-3, 2, -5)");
    /// assert_eq!(
    ///     IntegerVector::from_str("(0, 0)")
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

impl PrimitivePart for &IntegerVector {
    type Output = IntegerVector;

    /// Computes the primitive part of an [`IntegerVector`], taking the vector by reference.
    ///
    /// This is the vector divided by its content, with every element keeping its sign, so the
    /// primitive part of $-v$ is the negation of the primitive part of $v$. For the one whose first
    /// nonzero element is positive, see [`canonical_primitive_part`](
    /// malachite_base::num::arithmetic::traits::CanonicalPrimitivePart::canonical_primitive_part).
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
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(-6, 4, -10)").unwrap();
    /// assert_eq!((&v).primitive_part().to_string(), "(-3, 2, -5)");
    /// ```
    #[inline]
    fn primitive_part(self) -> IntegerVector {
        let content = content(&self.elements);
        IntegerVector {
            elements: normalized(&self.elements, &content, false),
        }
    }
}

impl PrimitivePartAssign for IntegerVector {
    /// Replaces an [`IntegerVector`] with its primitive part, the vector divided by its content.
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
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let mut v = IntegerVector::from_str("(-6, 4, -10)").unwrap();
    /// v.primitive_part_assign();
    /// assert_eq!(v.to_string(), "(-3, 2, -5)");
    /// ```
    #[inline]
    fn primitive_part_assign(&mut self) {
        let content = content(&self.elements);
        normalize_in_place(&mut self.elements, &content, false);
    }
}

impl ContentAndPrimitivePart for IntegerVector {
    type Content = Natural;
    type PrimitivePart = Self;

    /// Computes the content and the primitive part of an [`IntegerVector`] together, taking the
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
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(-6, 4, -10)").unwrap();
    /// let (content, primitive_part) = v.content_and_primitive_part();
    /// assert_eq!(content, 2u32);
    /// assert_eq!(primitive_part.to_string(), "(-3, 2, -5)");
    /// ```
    #[inline]
    fn content_and_primitive_part(mut self) -> (Natural, Self) {
        let content = content(&self.elements);
        normalize_in_place(&mut self.elements, &content, false);
        (content, self)
    }
}

impl ContentAndPrimitivePart for &IntegerVector {
    type Content = Natural;
    type PrimitivePart = IntegerVector;

    /// Computes the content and the primitive part of an [`IntegerVector`] together, taking the
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
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(-6, 4, -10)").unwrap();
    /// let (content, primitive_part) = (&v).content_and_primitive_part();
    /// assert_eq!(content, 2u32);
    /// assert_eq!(primitive_part.to_string(), "(-3, 2, -5)");
    /// ```
    #[inline]
    fn content_and_primitive_part(self) -> (Natural, IntegerVector) {
        let content = content(&self.elements);
        let elements = normalized(&self.elements, &content, false);
        (content, IntegerVector { elements })
    }
}
