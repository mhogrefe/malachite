// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use core::cmp::Ordering::{self, *};
use malachite_base::num::conversion::traits::{ExactFrom, IntegerMantissaAndExponent};
use malachite_base::num::logic::traits::SignificantBits;

macro_rules! impl_float {
    ($t: ident) => {
        impl PartialOrd<$t> for Natural {
            /// Compares a [`Natural`] to a primitive float.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// See [here](super::partial_cmp_primitive_float#partial_cmp).
            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                if other.is_nan() {
                    None
                } else if *other < 0.0 {
                    Some(Greater)
                } else if !other.is_finite() {
                    Some(Less)
                } else if *other == 0.0 {
                    self.partial_cmp(&0u32)
                } else if *self == 0u32 {
                    Some(Less)
                } else {
                    let (m, e) = other.integer_mantissa_and_exponent();
                    let log_cmp = i64::exact_from(self.significant_bits())
                        .cmp(&(i64::exact_from(m.significant_bits()) + e));
                    Some(if log_cmp != Equal {
                        log_cmp
                    } else {
                        self.cmp_normalized(&Natural::from(m))
                    })
                }
            }
        }

        impl PartialOrd<Natural> for $t {
            /// Compares a primitive float to a [`Natural`].
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `other.significant_bits()`.
            ///
            /// See [here](super::partial_cmp_primitive_float#partial_cmp).
            #[inline]
            fn partial_cmp(&self, other: &Natural) -> Option<Ordering> {
                other.partial_cmp(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_primitive_floats!(impl_float);
