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
    Content, ContentAndPrimitivePart, DivExact, Lcm, PrimitivePart,
};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

// Scales the parts up by the LCM of their denominators, which makes a Gaussian integer whose
// content is coprime to that LCM: a prime dividing the LCM to its full power in one denominator
// cannot divide that part's scaled numerator.
pub(super) fn scale_up_ref(x: &GaussianRational) -> (GaussianInteger, Natural) {
    let l = x.real.denominator_ref().lcm(x.imaginary.denominator_ref());
    let real = Integer::from_sign_and_abs(
        x.real >= 0u32,
        x.real.numerator_ref() * (&l).div_exact(x.real.denominator_ref()),
    );
    let imaginary = Integer::from_sign_and_abs(
        x.imaginary >= 0u32,
        x.imaginary.numerator_ref() * (&l).div_exact(x.imaginary.denominator_ref()),
    );
    (GaussianInteger { real, imaginary }, l)
}

pub(super) fn scale_up_val(x: GaussianRational) -> (GaussianInteger, Natural) {
    let real_sign = x.real >= 0u32;
    let imaginary_sign = x.imaginary >= 0u32;
    let (real_n, real_d) = x.real.into_numerator_and_denominator();
    let (imaginary_n, imaginary_d) = x.imaginary.into_numerator_and_denominator();
    let l = (&real_d).lcm(&imaginary_d);
    let real = Integer::from_sign_and_abs(real_sign, real_n * (&l).div_exact(real_d));
    let imaginary =
        Integer::from_sign_and_abs(imaginary_sign, imaginary_n * (&l).div_exact(imaginary_d));
    (GaussianInteger { real, imaginary }, l)
}

fn content_and_primitive_part_helper(
    scaled: GaussianInteger,
    l: Natural,
) -> (Rational, GaussianInteger) {
    let (g, primitive) = scaled.content_and_primitive_part();
    // g and l are coprime, so this reduction is a formality.
    (Rational::from_naturals(g, l), primitive)
}

impl ContentAndPrimitivePart for GaussianRational {
    type Content = Rational;
    type PrimitivePart = GaussianInteger;

    /// Splits a [`GaussianRational`] into its content and its primitive part, taking the
    /// [`GaussianRational`] by value.
    ///
    /// The content of a Gaussian rational is the unique non-negative rational $c$ such that the
    /// number is $c$ times a Gaussian integer with coprime parts, and the primitive part is that
    /// Gaussian integer; their product is the original number. Zero has content 0 and primitive
    /// part 0, and the unit of a nonzero number stays in its primitive part.
    ///
    /// $$
    /// f \left ( \frac{p}{q} + \frac{r}{s} i \right ) = \left ( \frac{g}{L}, \frac{pL}{gq} +
    /// \frac{rL}{gs} i \right ), \quad \text{where } L = \operatorname{lcm}(q, s) \text{ and }
    /// g = \gcd \left ( \frac{|p| L}{q}, \frac{|r| L}{s} \right ).
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::ContentAndPrimitivePart;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let (content, primitive) = GaussianRational::from_str("1/2+i/3")
    ///     .unwrap()
    ///     .content_and_primitive_part();
    /// assert_eq!(content.to_string(), "1/6");
    /// assert_eq!(primitive.to_string(), "3+2i");
    ///
    /// let (content, primitive) = GaussianRational::from_str("-6+9i")
    ///     .unwrap()
    ///     .content_and_primitive_part();
    /// assert_eq!(content, 3);
    /// assert_eq!(primitive.to_string(), "-2+3i");
    /// ```
    fn content_and_primitive_part(self) -> (Rational, GaussianInteger) {
        if self == 0u32 {
            return (Rational::ZERO, GaussianInteger::ZERO);
        }
        let (scaled, l) = scale_up_val(self);
        content_and_primitive_part_helper(scaled, l)
    }
}

impl ContentAndPrimitivePart for &GaussianRational {
    type Content = Rational;
    type PrimitivePart = GaussianInteger;

    /// Splits a [`GaussianRational`] into its content and its primitive part, taking the
    /// [`GaussianRational`] by reference.
    ///
    /// The content of a Gaussian rational is the unique non-negative rational $c$ such that the
    /// number is $c$ times a Gaussian integer with coprime parts, and the primitive part is that
    /// Gaussian integer; their product is the original number. Zero has content 0 and primitive
    /// part 0, and the unit of a nonzero number stays in its primitive part.
    ///
    /// $$
    /// f \left ( \frac{p}{q} + \frac{r}{s} i \right ) = \left ( \frac{g}{L}, \frac{pL}{gq} +
    /// \frac{rL}{gs} i \right ), \quad \text{where } L = \operatorname{lcm}(q, s) \text{ and }
    /// g = \gcd \left ( \frac{|p| L}{q}, \frac{|r| L}{s} \right ).
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::ContentAndPrimitivePart;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("1/2+i/3").unwrap();
    /// let (content, primitive) = (&x).content_and_primitive_part();
    /// assert_eq!(content.to_string(), "1/6");
    /// assert_eq!(primitive.to_string(), "3+2i");
    /// ```
    fn content_and_primitive_part(self) -> (Rational, GaussianInteger) {
        if *self == 0u32 {
            return (Rational::ZERO, GaussianInteger::ZERO);
        }
        let (scaled, l) = scale_up_ref(self);
        content_and_primitive_part_helper(scaled, l)
    }
}

impl Content for GaussianRational {
    type Output = Rational;

    /// Computes the content of a [`GaussianRational`], the unique non-negative rational $c$ such
    /// that the number is $c$ times a Gaussian integer with coprime parts, taking the
    /// [`GaussianRational`] by value.
    ///
    /// $$
    /// f \left ( \frac{p}{q} + \frac{r}{s} i \right ) = \frac{g}{L}, \quad
    /// \text{where } L = \operatorname{lcm}(q, s) \text{ and }
    /// g = \gcd \left ( \frac{|p| L}{q}, \frac{|r| L}{s} \right ).
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::Content;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     GaussianRational::from_str("1/2+i/3")
    ///         .unwrap()
    ///         .content()
    ///         .to_string(),
    ///     "1/6"
    /// );
    /// assert_eq!(GaussianRational::from_str("-6+9i").unwrap().content(), 3);
    /// ```
    fn content(self) -> Rational {
        if self == 0u32 {
            return Rational::ZERO;
        }
        let (scaled, l) = scale_up_val(self);
        Rational::from_naturals(scaled.content(), l)
    }
}

impl Content for &GaussianRational {
    type Output = Rational;

    /// Computes the content of a [`GaussianRational`], the unique non-negative rational $c$ such
    /// that the number is $c$ times a Gaussian integer with coprime parts, taking the
    /// [`GaussianRational`] by reference.
    ///
    /// $$
    /// f \left ( \frac{p}{q} + \frac{r}{s} i \right ) = \frac{g}{L}, \quad
    /// \text{where } L = \operatorname{lcm}(q, s) \text{ and }
    /// g = \gcd \left ( \frac{|p| L}{q}, \frac{|r| L}{s} \right ).
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::Content;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     (&GaussianRational::from_str("1/2+i/3").unwrap())
    ///         .content()
    ///         .to_string(),
    ///     "1/6"
    /// );
    /// assert_eq!((&GaussianRational::from_str("-6+9i").unwrap()).content(), 3);
    /// ```
    fn content(self) -> Rational {
        if *self == 0u32 {
            return Rational::ZERO;
        }
        let (scaled, l) = scale_up_ref(self);
        Rational::from_naturals(scaled.content(), l)
    }
}

impl PrimitivePart for GaussianRational {
    type Output = GaussianInteger;

    /// Computes the primitive part of a [`GaussianRational`], the Gaussian integer with coprime
    /// parts that the number is a non-negative rational multiple of, taking the
    /// [`GaussianRational`] by value.
    ///
    /// $$
    /// f \left ( \frac{p}{q} + \frac{r}{s} i \right ) = \frac{pL}{gq} + \frac{rL}{gs} i,
    /// \quad \text{where } L = \operatorname{lcm}(q, s) \text{ and }
    /// g = \gcd \left ( \frac{|p| L}{q}, \frac{|r| L}{s} \right ).
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::PrimitivePart;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     GaussianRational::from_str("1/2+i/3")
    ///         .unwrap()
    ///         .primitive_part()
    ///         .to_string(),
    ///     "3+2i"
    /// );
    /// ```
    #[inline]
    fn primitive_part(self) -> GaussianInteger {
        self.content_and_primitive_part().1
    }
}

impl PrimitivePart for &GaussianRational {
    type Output = GaussianInteger;

    /// Computes the primitive part of a [`GaussianRational`], the Gaussian integer with coprime
    /// parts that the number is a non-negative rational multiple of, taking the
    /// [`GaussianRational`] by reference.
    ///
    /// $$
    /// f \left ( \frac{p}{q} + \frac{r}{s} i \right ) = \frac{pL}{gq} + \frac{rL}{gs} i,
    /// \quad \text{where } L = \operatorname{lcm}(q, s) \text{ and }
    /// g = \gcd \left ( \frac{|p| L}{q}, \frac{|r| L}{s} \right ).
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::PrimitivePart;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     (&GaussianRational::from_str("1/2+i/3").unwrap())
    ///         .primitive_part()
    ///         .to_string(),
    ///     "3+2i"
    /// );
    /// ```
    #[inline]
    fn primitive_part(self) -> GaussianInteger {
        self.content_and_primitive_part().1
    }
}
