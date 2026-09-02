// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_rational::GaussianRational;
use malachite_base::num::arithmetic::traits::{
    AbsSquared, NegAssign, Reciprocal, ReciprocalAssign,
};

impl Reciprocal for GaussianRational {
    type Output = Self;

    /// Reciprocates a [`GaussianRational`], taking it by value.
    ///
    /// The reciprocal of a complex number is its conjugate divided by its squared absolute value:
    /// $$
    /// f(x) = \frac{1}{x} = \frac{\overline{x}}{|x|^2}.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts.
    ///
    /// # Panics
    /// Panics if `self` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Reciprocal;
    /// use malachite_base::num::basic::traits::I;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(GaussianRational::I.reciprocal().to_string(), "-i");
    /// assert_eq!(
    ///     GaussianRational::from_str("1+i")
    ///         .unwrap()
    ///         .reciprocal()
    ///         .to_string(),
    ///     "1/2-i/2"
    /// );
    /// assert_eq!(
    ///     GaussianRational::from_str("3/5+4i/5")
    ///         .unwrap()
    ///         .reciprocal()
    ///         .to_string(),
    ///     "3/5-4i/5"
    /// );
    /// ```
    #[inline]
    fn reciprocal(mut self) -> Self {
        self.reciprocal_assign();
        self
    }
}

impl Reciprocal for &GaussianRational {
    type Output = GaussianRational;

    /// Reciprocates a [`GaussianRational`], taking it by reference.
    ///
    /// The reciprocal of a complex number is its conjugate divided by its squared absolute value:
    /// $$
    /// f(x) = \frac{1}{x} = \frac{\overline{x}}{|x|^2}.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts.
    ///
    /// # Panics
    /// Panics if `self` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Reciprocal;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("1+i").unwrap();
    /// assert_eq!((&x).reciprocal().to_string(), "1/2-i/2");
    /// ```
    #[inline]
    fn reciprocal(self) -> GaussianRational {
        assert!(
            self.real != 0u32 || self.imaginary != 0u32,
            "Cannot take the reciprocal of zero"
        );
        // Purely real and purely imaginary values reduce to a single Rational reciprocal.
        if self.imaginary == 0u32 {
            GaussianRational {
                real: (&self.real).reciprocal(),
                imaginary: self.imaginary.clone(),
            }
        } else if self.real == 0u32 {
            GaussianRational {
                real: self.real.clone(),
                imaginary: -(&self.imaginary).reciprocal(),
            }
        } else {
            let norm = self.abs_squared();
            GaussianRational {
                real: (&self.real) / &norm,
                imaginary: -(&self.imaginary) / norm,
            }
        }
    }
}

impl ReciprocalAssign for GaussianRational {
    /// Reciprocates a [`GaussianRational`] in place.
    ///
    /// The reciprocal of a complex number is its conjugate divided by its squared absolute value:
    /// $$
    /// x \gets \frac{1}{x} = \frac{\overline{x}}{|x|^2}.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts.
    ///
    /// # Panics
    /// Panics if `self` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ReciprocalAssign;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let mut x = GaussianRational::from_str("1+i").unwrap();
    /// x.reciprocal_assign();
    /// assert_eq!(x.to_string(), "1/2-i/2");
    /// ```
    fn reciprocal_assign(&mut self) {
        assert!(
            self.real != 0u32 || self.imaginary != 0u32,
            "Cannot take the reciprocal of zero"
        );
        // Purely real and purely imaginary values reduce to a single Rational reciprocal.
        if self.imaginary == 0u32 {
            self.real.reciprocal_assign();
        } else if self.real == 0u32 {
            self.imaginary.reciprocal_assign();
            self.imaginary.neg_assign();
        } else {
            let norm = AbsSquared::abs_squared(&*self);
            self.real /= &norm;
            self.imaginary /= norm;
            self.imaginary.neg_assign();
        }
    }
}
