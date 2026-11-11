// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use core::cmp::Ordering::*;

macro_rules! impl_primitive_float {
    ($t: ident) => {
        impl PartialEq<$t> for BigExponentFloat {
            /// Determines whether a [`BigExponentFloat`] is equal to a primitive float.
            ///
            /// This implementation follows the IEEE 754 standard: NaN is not equal to anything.
            /// Both primitive float zeros are equal to the [`BigExponentFloat`] zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.complexity()`.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_float#eq).
            #[inline]
            fn eq(&self, other: &$t) -> bool {
                self.partial_cmp(other) == Some(Equal)
            }
        }

        impl PartialEq<BigExponentFloat> for $t {
            /// Determines whether a primitive float is equal to a [`BigExponentFloat`].
            ///
            /// This implementation follows the IEEE 754 standard: NaN is not equal to anything.
            /// Both primitive float zeros are equal to the [`BigExponentFloat`] zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `other.complexity()`.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_float#eq).
            #[inline]
            fn eq(&self, other: &BigExponentFloat) -> bool {
                other == self
            }
        }
    };
}
apply_to_primitive_floats!(impl_primitive_float);
