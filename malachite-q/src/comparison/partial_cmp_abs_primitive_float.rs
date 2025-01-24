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
use malachite_base::num::comparison::traits::{OrdAbs, PartialOrdAbs};
use malachite_base::num::conversion::traits::ExactFrom;

macro_rules! impl_float {
    ($t: ident) => {
        impl PartialOrdAbs<$t> for Rational {
            /// Compares the absolute values of a [`Rational`] and a primitive float.
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
            /// See [here](super::partial_cmp_abs_primitive_float#partial_cmp_abs).
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                if other.is_nan() {
                    None
                } else if *other == 0.0 {
                    self.partial_cmp_abs(&0u32)
                } else if !other.is_finite() || *self == 0u32 {
                    Some(Less)
                } else {
                    let ord_cmp = self
                        .floor_log_base_2_abs()
                        .cmp(&other.abs().floor_log_base_2());
                    Some(if ord_cmp != Equal {
                        ord_cmp
                    } else {
                        self.cmp_abs(&Rational::exact_from(*other))
                    })
                }
            }
        }

        impl PartialOrdAbs<Rational> for $t {
            /// Compares the absolute values of a primitive float and a [`Rational`].
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n \log n \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is
            /// `max(self.sci_exponent().abs(), other.significant_bits())`.
            ///
            /// See [here](super::partial_cmp_abs_primitive_float#partial_cmp_abs).
            #[inline]
            fn partial_cmp_abs(&self, other: &Rational) -> Option<Ordering> {
                other.partial_cmp_abs(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_primitive_floats!(impl_float);
