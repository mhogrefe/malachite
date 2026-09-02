// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::gaussian_rational::GaussianRational;
use core::cmp::Ordering::{self, Greater, Less};
use malachite_base::num::arithmetic::traits::AbsSquared;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;

macro_rules! impl_float {
    ($t: ident) => {
        impl PartialOrdAbs<$t> for GaussianRational {
            /// Compares the absolute values of a [`GaussianRational`] and a primitive float.
            ///
            /// NaN is not comparable to any [`GaussianRational`]. $\infty$ and $-\infty$ are
            /// greater in absolute value than any [`GaussianRational`].
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of
            /// significant bits of the real and imaginary parts of `self` and
            /// `other.sci_exponent().abs()`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_abs_primitive_float#partial_cmp_abs).
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                if other.is_nan() {
                    None
                } else if !other.is_finite() {
                    Some(Less)
                } else if self.imaginary == 0u32 {
                    self.real.partial_cmp_abs(other)
                } else if self.real == 0u32 {
                    self.imaginary.partial_cmp_abs(other)
                } else if !self.real.lt_abs(other) || !self.imaginary.lt_abs(other) {
                    Some(Greater)
                } else {
                    Some(
                        self.abs_squared()
                            .cmp(&Rational::exact_from(*other).abs_squared()),
                    )
                }
            }
        }

        impl PartialOrdAbs<GaussianRational> for $t {
            /// Compares the absolute values of a primitive float and a [`GaussianRational`].
            ///
            /// NaN is not comparable to any [`GaussianRational`]. $\infty$ and $-\infty$ are
            /// greater in absolute value than any [`GaussianRational`].
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of
            /// significant bits of the real and imaginary parts of `other` and
            /// `self.sci_exponent().abs()`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_abs_primitive_float#partial_cmp_abs).
            #[inline]
            fn partial_cmp_abs(&self, other: &GaussianRational) -> Option<Ordering> {
                other.partial_cmp_abs(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_primitive_floats!(impl_float);
