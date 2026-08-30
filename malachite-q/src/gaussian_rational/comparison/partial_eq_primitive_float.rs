// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_rational::GaussianRational;

macro_rules! impl_float {
    ($t: ident) => {
        impl PartialEq<$t> for GaussianRational {
            /// Determines whether a [`GaussianRational`] is equal to a primitive float.
            ///
            /// No [`GaussianRational`] is equal to an infinity or NaN.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(m) = O(m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is
            /// `max(self.real.significant_bits(), other.sci_exponent().abs())`, and $m$ is
            /// `other.sci_exponent().abs()`.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_float#partial_eq).
            #[inline]
            fn eq(&self, other: &$t) -> bool {
                self.imaginary == 0u32 && self.real == *other
            }
        }

        impl PartialEq<GaussianRational> for $t {
            /// Determines whether a primitive float is equal to a [`GaussianRational`].
            ///
            /// No infinity or NaN is equal to a [`GaussianRational`].
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(m) = O(m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is
            /// `max(other.real.significant_bits(), self.sci_exponent().abs())`, and $m$ is
            /// `self.sci_exponent().abs()`.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_float#partial_eq).
            #[inline]
            fn eq(&self, other: &GaussianRational) -> bool {
                other == self
            }
        }
    };
}
apply_to_primitive_floats!(impl_float);
