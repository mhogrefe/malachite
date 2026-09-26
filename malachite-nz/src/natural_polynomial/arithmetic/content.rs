// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign, GcdAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{
    Content, ContentAndPrimitivePart, PrimitivePart, PrimitivePartAssign,
};

// The GCD of the coefficients. It stops as soon as it reaches 1, since nothing can lower it
// further.
fn content(coefficients: &[Natural]) -> Natural {
    let mut gcd = Natural::ZERO;
    for c in coefficients {
        gcd.gcd_assign(c);
        if gcd == 1u32 {
            break;
        }
    }
    gcd
}

// Divides every coefficient by the content, which divides each of them exactly.
fn divide_by_content(coefficients: &mut [Natural], content: &Natural) {
    if *content > 1u32 {
        for c in coefficients {
            c.div_exact_assign(content);
        }
    }
}

// The coefficients divided by the content, as new values.
fn divided_by_content(coefficients: &[Natural], content: &Natural) -> Vec<Natural> {
    if *content > 1u32 {
        coefficients.iter().map(|c| c.div_exact(content)).collect()
    } else {
        coefficients.to_vec()
    }
}

impl Content for NaturalPolynomial {
    type Output = Natural;

    /// Computes the content of a [`NaturalPolynomial`], the GCD of its coefficients, taking the
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::Content;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("6*x^2+4*x+10").unwrap();
    /// assert_eq!(p.clone().content(), 2);
    /// assert_eq!(NaturalPolynomial::ZERO.content(), 0);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_content` from `fmpz_poly/content.c`, FLINT 3.6.0.
    #[inline]
    fn content(self) -> Natural {
        content(&self.coefficients)
    }
}

impl Content for &NaturalPolynomial {
    type Output = Natural;

    /// Computes the content of a [`NaturalPolynomial`], the GCD of its coefficients, taking the
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::Content;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("6*x^2+4*x+10").unwrap();
    /// assert_eq!((&p).content(), 2);
    /// assert_eq!((&NaturalPolynomial::ZERO).content(), 0);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_content` from `fmpz_poly/content.c`, FLINT 3.6.0.
    #[inline]
    fn content(self) -> Natural {
        content(&self.coefficients)
    }
}

impl PrimitivePart for NaturalPolynomial {
    type Output = Self;

    /// Computes the primitive part of a [`NaturalPolynomial`], taking the polynomial by value.
    ///
    /// This is the polynomial divided by its content. The coefficients are non-negative, so no sign
    /// needs normalizing.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::PrimitivePart;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("6*x^2+4*x+10").unwrap();
    /// assert_eq!(p.clone().primitive_part().to_string(), "3*x^2+2*x+5");
    /// assert_eq!(
    ///     NaturalPolynomial::ZERO.primitive_part(),
    ///     NaturalPolynomial::ZERO
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_primitive_part` from `fmpz_poly/primitive_part.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn primitive_part(mut self) -> Self {
        let content = content(&self.coefficients);
        divide_by_content(&mut self.coefficients, &content);
        self
    }
}

impl PrimitivePart for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Computes the primitive part of a [`NaturalPolynomial`], taking the polynomial by reference.
    ///
    /// This is the polynomial divided by its content. The coefficients are non-negative, so no sign
    /// needs normalizing.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::PrimitivePart;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("6*x^2+4*x+10").unwrap();
    /// assert_eq!((&p).primitive_part().to_string(), "3*x^2+2*x+5");
    /// assert_eq!(
    ///     (&NaturalPolynomial::ZERO).primitive_part(),
    ///     NaturalPolynomial::ZERO
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_primitive_part` from `fmpz_poly/primitive_part.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn primitive_part(self) -> NaturalPolynomial {
        let content = content(&self.coefficients);
        NaturalPolynomial {
            coefficients: divided_by_content(&self.coefficients, &content),
        }
    }
}

impl PrimitivePartAssign for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its primitive part.
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
    /// use malachite_base::polynomial::PrimitivePartAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("6*x^2+4*x+10").unwrap();
    /// p.primitive_part_assign();
    /// assert_eq!(p.to_string(), "3*x^2+2*x+5");
    /// ```
    #[inline]
    fn primitive_part_assign(&mut self) {
        let content = content(&self.coefficients);
        divide_by_content(&mut self.coefficients, &content);
    }
}

impl ContentAndPrimitivePart for NaturalPolynomial {
    type Content = Natural;
    type PrimitivePart = Self;

    /// Computes the content and the primitive part of a [`NaturalPolynomial`] together, taking the
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
    /// use malachite_base::polynomial::ContentAndPrimitivePart;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("6*x^2+4*x+10").unwrap();
    /// let (content, primitive_part) = p.clone().content_and_primitive_part();
    /// assert_eq!(content, 2);
    /// assert_eq!(primitive_part.to_string(), "3*x^2+2*x+5");
    /// ```
    #[inline]
    fn content_and_primitive_part(mut self) -> (Natural, Self) {
        let content = content(&self.coefficients);
        divide_by_content(&mut self.coefficients, &content);
        (content, self)
    }
}

impl ContentAndPrimitivePart for &NaturalPolynomial {
    type Content = Natural;
    type PrimitivePart = NaturalPolynomial;

    /// Computes the content and the primitive part of a [`NaturalPolynomial`] together, taking the
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
    /// use malachite_base::polynomial::ContentAndPrimitivePart;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("6*x^2+4*x+10").unwrap();
    /// let (content, primitive_part) = (&p).content_and_primitive_part();
    /// assert_eq!(content, 2);
    /// assert_eq!(primitive_part.to_string(), "3*x^2+2*x+5");
    /// ```
    #[inline]
    fn content_and_primitive_part(self) -> (Natural, NaturalPolynomial) {
        let content = content(&self.coefficients);
        let coefficients = divided_by_content(&self.coefficients, &content);
        (content, NaturalPolynomial { coefficients })
    }
}
