// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;

macro_rules! impl_float {
    ($t: ident) => {
        impl PartialEq<$t> for GaussianInteger {
            /// Determines whether a [`GaussianInteger`] is equal to a primitive float.
            ///
            /// No [`GaussianInteger`] is equal to an infinity or NaN.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is
            /// `self.real.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_float#partial_eq).
            #[inline]
            fn eq(&self, other: &$t) -> bool {
                self.imaginary == 0u32 && self.real == *other
            }
        }

        impl PartialEq<GaussianInteger> for $t {
            /// Determines whether a primitive float is equal to a [`GaussianInteger`].
            ///
            /// No infinity or NaN is equal to a [`GaussianInteger`].
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is
            /// `other.real.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_float#partial_eq).
            #[inline]
            fn eq(&self, other: &GaussianInteger) -> bool {
                other == self
            }
        }
    };
}
apply_to_primitive_floats!(impl_float);
