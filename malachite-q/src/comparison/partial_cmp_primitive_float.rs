// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::FloorLogBase2;

macro_rules! impl_float {
    ($t: ident) => {
        impl PartialOrd<$t> for Rational {
            /// Compares a [`Rational`] to a primitive float.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is
            /// `max(self.significant_bits(), other.sci_exponent().abs())`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_primitive_float#partial_cmp).
            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                if other.is_nan() {
                    None
                } else if self.sign != (*other >= 0.0) {
                    Some(if self.sign { Greater } else { Less })
                } else if !other.is_finite() {
                    Some(if self.sign { Less } else { Greater })
                } else if *other == 0.0 {
                    self.partial_cmp(&0u32)
                } else if *self == 0u32 {
                    0.0.partial_cmp(other)
                } else {
                    let ord_cmp = self
                        .floor_log_base_2_abs()
                        .cmp(&other.abs().floor_log_base_2());
                    Some(if ord_cmp != Equal {
                        if self.sign {
                            ord_cmp
                        } else {
                            ord_cmp.reverse()
                        }
                    } else {
                        self.cmp(&Rational::try_from(*other).unwrap())
                    })
                }
            }
        }

        impl PartialOrd<Rational> for $t {
            /// Compares a primitive float to a [`Rational`].
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is
            /// `max(other.sci_exponent().abs(), self.significant_bits())`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_primitive_float#partial_cmp).
            #[inline]
            fn partial_cmp(&self, other: &Rational) -> Option<Ordering> {
                other.partial_cmp(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_primitive_floats!(impl_float);
