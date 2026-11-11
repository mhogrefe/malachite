// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use core::cmp::Ordering;

macro_rules! impl_primitive_float {
    ($t: ident) => {
        impl PartialOrd<$t> for BigExponentFloat {
            /// Compares a [`BigExponentFloat`] to a primitive float.
            ///
            /// This implementation follows the IEEE 754 standard: NaN is not comparable to
            /// anything. Both primitive float zeros are equal to the [`BigExponentFloat`] zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.complexity()`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_primitive_float#partial_cmp).
            #[inline]
            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                self.partial_cmp(&Self::from(*other))
            }
        }

        impl PartialOrd<BigExponentFloat> for $t {
            /// Compares a primitive float to a [`BigExponentFloat`].
            ///
            /// This implementation follows the IEEE 754 standard: NaN is not comparable to
            /// anything. Both primitive float zeros are equal to the [`BigExponentFloat`] zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `other.complexity()`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_primitive_float#partial_cmp).
            #[inline]
            fn partial_cmp(&self, other: &BigExponentFloat) -> Option<Ordering> {
                other.partial_cmp(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_primitive_floats!(impl_primitive_float);
