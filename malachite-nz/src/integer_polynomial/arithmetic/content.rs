// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use crate::natural::Natural;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    Content, ContentAndPrimitivePart, DivExact, DivExactAssign, GcdAssign, NegAssign,
    PrimitivePart, PrimitivePartAssign,
};
use malachite_base::num::basic::traits::Zero;

// The GCD of the coefficients' absolute values. It stops as soon as it reaches 1, since nothing can
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

// Whether the canonical primitive part must be negated: when the leading coefficient is negative.
pub(crate) fn negate(coefficients: &[Integer]) -> bool {
    coefficients.last().is_some_and(|c| !c.sign)
}

// Divides every coefficient by the content, which divides each of them exactly, and negates them
// all if `negate` is set.
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

// The coefficients divided by the content, and negated if `negate` is set, as new values.
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

impl Content for IntegerPolynomial {
    type Output = Natural;

    /// Computes the content of an [`IntegerPolynomial`], the GCD of its coefficients, taking the
    /// polynomial by value.
    ///
    /// The content is non-negative, and the content of the zero polynomial is zero. The GCD is
    /// taken coefficient by coefficient, stopping early once it reaches 1.
    ///
    /// $$
    /// f(p) = \gcd(c_0, c_1, \ldots, c_{n-1}),
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length.
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
    /// use malachite_base::num::arithmetic::traits::Content;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("-6*x^2+4*x-10").unwrap();
    /// assert_eq!(p.clone().content(), 2);
    /// assert_eq!(IntegerPolynomial::ZERO.content(), 0);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_content` from `fmpz_poly/content.c`, FLINT 3.6.0.
    #[inline]
    fn content(self) -> Natural {
        content(&self.coefficients)
    }
}

impl Content for &IntegerPolynomial {
    type Output = Natural;

    /// Computes the content of an [`IntegerPolynomial`], the GCD of its coefficients, taking the
    /// polynomial by reference.
    ///
    /// The content is non-negative, and the content of the zero polynomial is zero. The GCD is
    /// taken coefficient by coefficient, stopping early once it reaches 1.
    ///
    /// $$
    /// f(p) = \gcd(c_0, c_1, \ldots, c_{n-1}),
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length.
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
    /// use malachite_base::num::arithmetic::traits::Content;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("-6*x^2+4*x-10").unwrap();
    /// assert_eq!((&p).content(), 2);
    /// assert_eq!((&IntegerPolynomial::ZERO).content(), 0);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_content` from `fmpz_poly/content.c`, FLINT 3.6.0.
    #[inline]
    fn content(self) -> Natural {
        content(&self.coefficients)
    }
}

impl PrimitivePart for IntegerPolynomial {
    type Output = Self;

    /// Computes the primitive part of an [`IntegerPolynomial`], taking the polynomial by value.
    ///
    /// This is the polynomial divided by its content, with every coefficient keeping its sign, so
    /// the primitive part of $-p$ is the negation of the primitive part of $p$. For the one with a
    /// non-negative leading coefficient, see [`canonical_primitive_part`](
    /// malachite_base::num::arithmetic::traits::CanonicalPrimitivePart::canonical_primitive_part).
    ///
    /// $$
    /// p = \operatorname{cont}(p) \operatorname{pp}(p).
    /// $$
    ///
    /// The primitive part of the zero polynomial is zero.
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
    /// use malachite_base::num::arithmetic::traits::PrimitivePart;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("-6*x^2+4*x-10").unwrap();
    /// assert_eq!(p.clone().primitive_part().to_string(), "-3*x^2+2*x-5");
    /// assert_eq!(
    ///     IntegerPolynomial::ZERO.primitive_part(),
    ///     IntegerPolynomial::ZERO
    /// );
    /// ```
    #[inline]
    fn primitive_part(mut self) -> Self {
        let content = content(&self.coefficients);
        normalize_in_place(&mut self.coefficients, &content, false);
        self
    }
}

impl PrimitivePart for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Computes the primitive part of an [`IntegerPolynomial`], taking the polynomial by reference.
    ///
    /// This is the polynomial divided by its content, with every coefficient keeping its sign, so
    /// the primitive part of $-p$ is the negation of the primitive part of $p$. For the one with a
    /// non-negative leading coefficient, see [`canonical_primitive_part`](
    /// malachite_base::num::arithmetic::traits::CanonicalPrimitivePart::canonical_primitive_part).
    ///
    /// $$
    /// p = \operatorname{cont}(p) \operatorname{pp}(p).
    /// $$
    ///
    /// The primitive part of the zero polynomial is zero.
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
    /// use malachite_base::num::arithmetic::traits::PrimitivePart;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("-6*x^2+4*x-10").unwrap();
    /// assert_eq!((&p).primitive_part().to_string(), "-3*x^2+2*x-5");
    /// assert_eq!(
    ///     (&IntegerPolynomial::ZERO).primitive_part(),
    ///     IntegerPolynomial::ZERO
    /// );
    /// ```
    #[inline]
    fn primitive_part(self) -> IntegerPolynomial {
        let content = content(&self.coefficients);
        IntegerPolynomial {
            coefficients: normalized(&self.coefficients, &content, false),
        }
    }
}

impl PrimitivePartAssign for IntegerPolynomial {
    /// Replaces an [`IntegerPolynomial`] with its primitive part.
    ///
    /// See [`primitive_part`](PrimitivePart::primitive_part).
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
    /// use malachite_base::num::arithmetic::traits::PrimitivePartAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("-6*x^2+4*x-10").unwrap();
    /// p.primitive_part_assign();
    /// assert_eq!(p.to_string(), "-3*x^2+2*x-5");
    /// ```
    #[inline]
    fn primitive_part_assign(&mut self) {
        let content = content(&self.coefficients);
        normalize_in_place(&mut self.coefficients, &content, false);
    }
}

impl ContentAndPrimitivePart for IntegerPolynomial {
    type Content = Natural;
    type PrimitivePart = Self;

    /// Computes the content and the primitive part of an [`IntegerPolynomial`] together, taking the
    /// polynomial by value.
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
    /// coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentAndPrimitivePart;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("-6*x^2+4*x-10").unwrap();
    /// let (content, primitive_part) = p.clone().content_and_primitive_part();
    /// assert_eq!(content, 2);
    /// assert_eq!(primitive_part.to_string(), "-3*x^2+2*x-5");
    /// ```
    #[inline]
    fn content_and_primitive_part(mut self) -> (Natural, Self) {
        let content = content(&self.coefficients);
        normalize_in_place(&mut self.coefficients, &content, false);
        (content, self)
    }
}

impl ContentAndPrimitivePart for &IntegerPolynomial {
    type Content = Natural;
    type PrimitivePart = IntegerPolynomial;

    /// Computes the content and the primitive part of an [`IntegerPolynomial`] together, taking the
    /// polynomial by reference.
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
    /// coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentAndPrimitivePart;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("-6*x^2+4*x-10").unwrap();
    /// let (content, primitive_part) = (&p).content_and_primitive_part();
    /// assert_eq!(content, 2);
    /// assert_eq!(primitive_part.to_string(), "-3*x^2+2*x-5");
    /// ```
    #[inline]
    fn content_and_primitive_part(self) -> (Natural, IntegerPolynomial) {
        let content = content(&self.coefficients);
        let coefficients = normalized(&self.coefficients, &content, false);
        (content, IntegerPolynomial { coefficients })
    }
}
