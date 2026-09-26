// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::{Content, ContentAndPrimitivePart, PrimitivePart, PrimitivePartAssign};
use crate::unsigned_polynomial::UnsignedPolynomial;

// The GCD of the coefficients. It stops as soon as it reaches 1, since nothing can lower it
// further.
fn content<T: PrimitiveUnsigned>(coefficients: &[T]) -> T {
    let mut gcd = T::ZERO;
    for &c in coefficients {
        gcd.gcd_assign(c);
        if gcd == T::ONE {
            break;
        }
    }
    gcd
}

// Divides every coefficient by the content, which divides each of them exactly.
fn divide_by_content<T: PrimitiveUnsigned>(coefficients: &mut [T], content: T) {
    if content > T::ONE {
        for c in coefficients {
            c.div_exact_assign(content);
        }
    }
}

impl<T: PrimitiveUnsigned> Content for UnsignedPolynomial<T> {
    type Output = T;

    /// Computes the content of an [`UnsignedPolynomial`], the GCD of its coefficients, taking the
    /// polynomial by value.
    ///
    /// The content of the zero polynomial is zero. The GCD is taken coefficient by coefficient,
    /// stopping early once it reaches 1.
    ///
    /// $$
    /// f(p) = \gcd(c_0, c_1, \ldots, c_{n-1}),
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::Content;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("6*x^2+4*x+10").unwrap();
    /// assert_eq!(p.clone().content(), 2);
    /// assert_eq!(UnsignedPolynomial::<u8>::ZERO.content(), 0);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_content` from `fmpz_poly/content.c`, FLINT 3.6.0.
    #[inline]
    fn content(self) -> T {
        content(&self.coefficients)
    }
}

impl<T: PrimitiveUnsigned> Content for &UnsignedPolynomial<T> {
    type Output = T;

    /// Computes the content of an [`UnsignedPolynomial`], the GCD of its coefficients, taking the
    /// polynomial by reference.
    ///
    /// The content of the zero polynomial is zero. The GCD is taken coefficient by coefficient,
    /// stopping early once it reaches 1.
    ///
    /// $$
    /// f(p) = \gcd(c_0, c_1, \ldots, c_{n-1}),
    /// $$
    ///
    /// where $c_i$ is the coefficient of $x^i$ in $p$ and $n$ is its length.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::Content;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("6*x^2+4*x+10").unwrap();
    /// assert_eq!((&p).content(), 2);
    /// assert_eq!((&UnsignedPolynomial::<u8>::ZERO).content(), 0);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_content` from `fmpz_poly/content.c`, FLINT 3.6.0.
    #[inline]
    fn content(self) -> T {
        content(&self.coefficients)
    }
}

impl<T: PrimitiveUnsigned> PrimitivePart for UnsignedPolynomial<T> {
    type Output = Self;

    /// Computes the primitive part of an [`UnsignedPolynomial`], the polynomial divided by its
    /// content, taking the polynomial by value.
    ///
    /// The coefficients are non-negative, so no sign needs normalizing and $p =
    /// \operatorname{cont}(p) \operatorname{pp}(p)$. The primitive part of the zero polynomial is
    /// zero.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::PrimitivePart;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("6*x^2+4*x+10").unwrap();
    /// assert_eq!(p.clone().primitive_part().to_string(), "3*x^2+2*x+5");
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::ZERO.primitive_part(),
    ///     UnsignedPolynomial::<u8>::ZERO
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_primitive_part` from `fmpz_poly/primitive_part.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn primitive_part(mut self) -> Self {
        let content = content(&self.coefficients);
        divide_by_content(&mut self.coefficients, content);
        self
    }
}

impl<T: PrimitiveUnsigned> PrimitivePart for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Computes the primitive part of an [`UnsignedPolynomial`], the polynomial divided by its
    /// content, taking the polynomial by reference.
    ///
    /// The coefficients are non-negative, so no sign needs normalizing and $p =
    /// \operatorname{cont}(p) \operatorname{pp}(p)$. The primitive part of the zero polynomial is
    /// zero.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::PrimitivePart;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("6*x^2+4*x+10").unwrap();
    /// assert_eq!((&p).primitive_part().to_string(), "3*x^2+2*x+5");
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::ZERO).primitive_part(),
    ///     UnsignedPolynomial::<u8>::ZERO
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_primitive_part` from `fmpz_poly/primitive_part.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn primitive_part(self) -> UnsignedPolynomial<T> {
        let content = content(&self.coefficients);
        let mut coefficients = self.coefficients.clone();
        divide_by_content(&mut coefficients, content);
        UnsignedPolynomial { coefficients }
    }
}

impl<T: PrimitiveUnsigned> PrimitivePartAssign for UnsignedPolynomial<T> {
    /// Replaces an [`UnsignedPolynomial`] with its primitive part, the polynomial divided by its
    /// content.
    ///
    /// See [`primitive_part`](PrimitivePart::primitive_part).
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::PrimitivePartAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("6*x^2+4*x+10").unwrap();
    /// p.primitive_part_assign();
    /// assert_eq!(p.to_string(), "3*x^2+2*x+5");
    /// ```
    #[inline]
    fn primitive_part_assign(&mut self) {
        let content = content(&self.coefficients);
        divide_by_content(&mut self.coefficients, content);
    }
}

impl<T: PrimitiveUnsigned> ContentAndPrimitivePart for UnsignedPolynomial<T> {
    type Content = T;
    type PrimitivePart = Self;

    /// Computes the content and the primitive part of an [`UnsignedPolynomial`] together, taking
    /// the polynomial by value.
    ///
    /// See [`content`](Content::content) and [`primitive_part`](PrimitivePart::primitive_part); the
    /// content is found once rather than twice.
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
    /// use malachite_base::polynomial::ContentAndPrimitivePart;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("6*x^2+4*x+10").unwrap();
    /// let (content, primitive_part) = p.clone().content_and_primitive_part();
    /// assert_eq!(content, 2);
    /// assert_eq!(primitive_part.to_string(), "3*x^2+2*x+5");
    /// ```
    #[inline]
    fn content_and_primitive_part(mut self) -> (T, Self) {
        let content = content(&self.coefficients);
        divide_by_content(&mut self.coefficients, content);
        (content, self)
    }
}

impl<T: PrimitiveUnsigned> ContentAndPrimitivePart for &UnsignedPolynomial<T> {
    type Content = T;
    type PrimitivePart = UnsignedPolynomial<T>;

    /// Computes the content and the primitive part of an [`UnsignedPolynomial`] together, taking
    /// the polynomial by reference.
    ///
    /// See [`content`](Content::content) and [`primitive_part`](PrimitivePart::primitive_part); the
    /// content is found once rather than twice.
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
    /// use malachite_base::polynomial::ContentAndPrimitivePart;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("6*x^2+4*x+10").unwrap();
    /// let (content, primitive_part) = (&p).content_and_primitive_part();
    /// assert_eq!(content, 2);
    /// assert_eq!(primitive_part.to_string(), "3*x^2+2*x+5");
    /// ```
    #[inline]
    fn content_and_primitive_part(self) -> (T, UnsignedPolynomial<T>) {
        let content = content(&self.coefficients);
        let mut coefficients = self.coefficients.clone();
        divide_by_content(&mut coefficients, content);
        (content, UnsignedPolynomial { coefficients })
    }
}
