// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2022 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::gaussian_rational::GaussianRational;
use core::mem::take;
use malachite_base::num::arithmetic::traits::{Square, SquareAssign};
use malachite_base::num::basic::traits::Zero;

// The general tier of fmpzi_sqr from fmpzi/sqr.c, FLINT 3.6.0, adapted to rational parts: the real
// part is $a^2 - b^2$ and the imaginary part is $2ab$, so two of the three multiplications are
// squarings. Squaring a [`Rational`] is especially cheap because a reduced fraction's square is
// already reduced, so no GCDs are computed; the naive product `x * x` computes four general
// products, each with cross-GCD reductions. fmpzi_sqr's size-based three-squarings tier is not
// used: it trades a multiplication for additions, and for [`Rational`]s addition is not cheaper
// than multiplication.
fn square_val(x: GaussianRational) -> GaussianRational {
    if x.imaginary == 0u32 {
        return GaussianRational {
            real: x.real.square(),
            imaginary: Rational::ZERO,
        };
    }
    if x.real == 0u32 {
        return GaussianRational {
            real: -x.imaginary.square(),
            imaginary: Rational::ZERO,
        };
    }
    // Each part appears in exactly two products: it is borrowed by its first use and consumed by
    // its last.
    let real = (&x.real).square() - (&x.imaginary).square();
    GaussianRational {
        real,
        imaginary: (x.real * x.imaginary) << 1u32,
    }
}

fn square_ref(x: &GaussianRational) -> GaussianRational {
    if x.imaginary == 0u32 {
        return GaussianRational {
            real: (&x.real).square(),
            imaginary: Rational::ZERO,
        };
    }
    if x.real == 0u32 {
        return GaussianRational {
            real: -(&x.imaginary).square(),
            imaginary: Rational::ZERO,
        };
    }
    GaussianRational {
        real: (&x.real).square() - (&x.imaginary).square(),
        imaginary: (&x.real * &x.imaginary) << 1u32,
    }
}

impl Square for GaussianRational {
    type Output = Self;

    /// Squares a [`GaussianRational`], taking it by value.
    ///
    /// $$
    /// f(x) = x^2.
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
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Square;
    /// use malachite_base::num::basic::traits::I;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(GaussianRational::I.square().to_string(), "-1");
    /// let x = GaussianRational::from_str("1/2+i/2").unwrap();
    /// assert_eq!(x.square().to_string(), "i/2");
    /// ```
    #[inline]
    fn square(self) -> Self {
        square_val(self)
    }
}

impl Square for &GaussianRational {
    type Output = GaussianRational;

    /// Squares a [`GaussianRational`], taking it by reference.
    ///
    /// $$
    /// f(x) = x^2.
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
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Square;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("1/2+i/2").unwrap();
    /// assert_eq!((&x).square().to_string(), "i/2");
    /// ```
    #[inline]
    fn square(self) -> GaussianRational {
        square_ref(self)
    }
}

impl SquareAssign for GaussianRational {
    /// Squares a [`GaussianRational`] in place.
    ///
    /// $$
    /// x \gets x^2.
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
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SquareAssign;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let mut x = GaussianRational::from_str("1/2+i/2").unwrap();
    /// x.square_assign();
    /// assert_eq!(x.to_string(), "i/2");
    /// ```
    #[inline]
    fn square_assign(&mut self) {
        *self = square_val(take(self));
    }
}
