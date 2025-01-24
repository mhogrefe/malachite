// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use core::cmp::Ordering::*;
use malachite_base::num::conversion::traits::IntegerMantissaAndExponent;
use malachite_base::num::logic::traits::SignificantBits;

macro_rules! impl_float {
    ($t: ident) => {
        impl PartialEq<$t> for Natural {
            /// Determines whether a [`Natural`] is equal to a primitive float.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_float#partial_eq).
            fn eq(&self, other: &$t) -> bool {
                if !other.is_finite() {
                    false
                } else if *other == 0.0 {
                    *self == 0u32
                } else if *other < 1.0 || *self == 0u32 {
                    false
                } else {
                    let (m, e) = other.integer_mantissa_and_exponent();
                    if let Ok(e) = u64::try_from(e) {
                        self.significant_bits() == m.significant_bits() + e
                            && self.cmp_normalized(&Natural::from(m)) == Equal
                    } else {
                        false
                    }
                }
            }
        }

        impl PartialEq<Natural> for $t {
            /// Determines whether a primitive float is equal to a [`Natural`].
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `other.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_float#partial_eq).
            #[inline]
            fn eq(&self, other: &Natural) -> bool {
                other == self
            }
        }
    };
}
apply_to_primitive_floats!(impl_float);
