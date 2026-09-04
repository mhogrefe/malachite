// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use crate::integer::Integer;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{
    Content, ContentAndPrimitivePart, DivExact, Gcd, PrimitivePart, UnsignedAbs,
};
use malachite_base::num::basic::traits::Zero;

fn content_and_primitive_part_val(x: GaussianInteger) -> (Natural, GaussianInteger) {
    let g = x
        .real
        .unsigned_abs_ref()
        .gcd(x.imaginary.unsigned_abs_ref());
    if g == 0u32 {
        (Natural::ZERO, GaussianInteger::ZERO)
    } else if g == 1u32 {
        (g, x)
    } else {
        let g_int = Integer::from(&g);
        let primitive = GaussianInteger {
            real: x.real.div_exact(&g_int),
            imaginary: x.imaginary.div_exact(g_int),
        };
        (g, primitive)
    }
}

fn content_and_primitive_part_ref(x: &GaussianInteger) -> (Natural, GaussianInteger) {
    let g = x
        .real
        .unsigned_abs_ref()
        .gcd(x.imaginary.unsigned_abs_ref());
    if g == 0u32 {
        (Natural::ZERO, GaussianInteger::ZERO)
    } else if g == 1u32 {
        (g, x.clone())
    } else {
        let g_int = Integer::from(&g);
        let primitive = GaussianInteger {
            real: (&x.real).div_exact(&g_int),
            imaginary: (&x.imaginary).div_exact(g_int),
        };
        (g, primitive)
    }
}

impl ContentAndPrimitivePart for GaussianInteger {
    type Content = Natural;
    type PrimitivePart = Self;

    /// Splits a [`GaussianInteger`] into its content and its primitive part, taking the
    /// [`GaussianInteger`] by value.
    ///
    /// The content of a Gaussian integer is the GCD of its real and imaginary parts, a non-negative
    /// integer, and the primitive part is the Gaussian integer with coprime parts that remains
    /// after dividing it out; their product is the original number. Zero has content 0 and
    /// primitive part 0, and the unit of a nonzero number stays in its primitive part.
    ///
    /// $$
    /// f(a + bi) = \left ( g, \frac{a}{g} + \frac{b}{g} i \right ), \quad
    /// \text{where } g = \gcd(|a|, |b|).
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::ContentAndPrimitivePart;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let (content, primitive) = GaussianInteger::from_str("-6+9i")
    ///     .unwrap()
    ///     .content_and_primitive_part();
    /// assert_eq!(content, 3);
    /// assert_eq!(primitive.to_string(), "-2+3i");
    /// ```
    #[inline]
    fn content_and_primitive_part(self) -> (Natural, Self) {
        content_and_primitive_part_val(self)
    }
}

impl ContentAndPrimitivePart for &GaussianInteger {
    type Content = Natural;
    type PrimitivePart = GaussianInteger;

    /// Splits a [`GaussianInteger`] into its content and its primitive part, taking the
    /// [`GaussianInteger`] by reference.
    ///
    /// The content of a Gaussian integer is the GCD of its real and imaginary parts, a non-negative
    /// integer, and the primitive part is the Gaussian integer with coprime parts that remains
    /// after dividing it out; their product is the original number. Zero has content 0 and
    /// primitive part 0, and the unit of a nonzero number stays in its primitive part.
    ///
    /// $$
    /// f(a + bi) = \left ( g, \frac{a}{g} + \frac{b}{g} i \right ), \quad
    /// \text{where } g = \gcd(|a|, |b|).
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::ContentAndPrimitivePart;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianInteger::from_str("-6+9i").unwrap();
    /// let (content, primitive) = (&x).content_and_primitive_part();
    /// assert_eq!(content, 3);
    /// assert_eq!(primitive.to_string(), "-2+3i");
    /// ```
    #[inline]
    fn content_and_primitive_part(self) -> (Natural, GaussianInteger) {
        content_and_primitive_part_ref(self)
    }
}

impl Content for GaussianInteger {
    type Output = Natural;

    /// Computes the content of a [`GaussianInteger`], the GCD of its real and imaginary parts,
    /// taking the [`GaussianInteger`] by value.
    ///
    /// $$
    /// f(a + bi) = \gcd(|a|, |b|).
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::Content;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(GaussianInteger::from_str("-6+9i").unwrap().content(), 3);
    /// assert_eq!(GaussianInteger::from_str("7+11i").unwrap().content(), 1);
    /// ```
    #[inline]
    fn content(self) -> Natural {
        self.real.unsigned_abs().gcd(self.imaginary.unsigned_abs())
    }
}

impl Content for &GaussianInteger {
    type Output = Natural;

    /// Computes the content of a [`GaussianInteger`], the GCD of its real and imaginary parts,
    /// taking the [`GaussianInteger`] by reference.
    ///
    /// $$
    /// f(a + bi) = \gcd(|a|, |b|).
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::Content;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// assert_eq!((&GaussianInteger::from_str("-6+9i").unwrap()).content(), 3);
    /// assert_eq!((&GaussianInteger::from_str("7+11i").unwrap()).content(), 1);
    /// ```
    #[inline]
    fn content(self) -> Natural {
        self.real
            .unsigned_abs_ref()
            .gcd(self.imaginary.unsigned_abs_ref())
    }
}

impl PrimitivePart for GaussianInteger {
    type Output = Self;

    /// Computes the primitive part of a [`GaussianInteger`], the [`GaussianInteger`] with coprime
    /// parts that remains after dividing out the content, taking the [`GaussianInteger`] by value.
    ///
    /// $$
    /// f(a + bi) = \frac{a}{g} + \frac{b}{g} i, \quad \text{where } g = \gcd(|a|, |b|).
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::PrimitivePart;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     GaussianInteger::from_str("-6+9i")
    ///         .unwrap()
    ///         .primitive_part()
    ///         .to_string(),
    ///     "-2+3i"
    /// );
    /// ```
    #[inline]
    fn primitive_part(self) -> Self {
        content_and_primitive_part_val(self).1
    }
}

impl PrimitivePart for &GaussianInteger {
    type Output = GaussianInteger;

    /// Computes the primitive part of a [`GaussianInteger`], the [`GaussianInteger`] with coprime
    /// parts that remains after dividing out the content, taking the [`GaussianInteger`] by
    /// reference.
    ///
    /// $$
    /// f(a + bi) = \frac{a}{g} + \frac{b}{g} i, \quad \text{where } g = \gcd(|a|, |b|).
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::PrimitivePart;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     (&GaussianInteger::from_str("-6+9i").unwrap())
    ///         .primitive_part()
    ///         .to_string(),
    ///     "-2+3i"
    /// );
    /// ```
    #[inline]
    fn primitive_part(self) -> GaussianInteger {
        content_and_primitive_part_ref(self).1
    }
}
