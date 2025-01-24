// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::arithmetic::traits::{FloorLogBase2, IsPowerOf2};
use malachite_base::num::comparison::traits::EqAbs;
use malachite_base::num::conversion::traits::ExactFrom;

macro_rules! impl_eq_abs {
    ($t: ident) => {
        impl EqAbs<$t> for Rational {
            /// Determines whether the absolute values of a [`Rational`] and a primitive float are
            /// equal.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(m) = O(m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
            /// other.sci_exponent().abs())`, and $m$ is `other.sci_exponent().abs()`.
            ///
            /// See [here](super::eq_abs_primitive_float#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &$t) -> bool {
                if !other.is_finite() {
                    false
                } else if *other == 0.0 {
                    *self == 0u32
                } else {
                    *self != 0u32
                        && self.denominator.is_power_of_2()
                        && self.floor_log_base_2_abs() == other.abs().floor_log_base_2()
                        && self.eq_abs(&Rational::exact_from(other.abs()))
                }
            }
        }

        impl EqAbs<Rational> for $t {
            /// Determines whether the absolute values of a primitive float and a [`Rational`] are
            /// equal.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(m) = O(m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.sci_exponent().abs(),
            /// other.significant_bits())`, and $m$ is `self.sci_exponent().abs()`.
            ///
            /// See [here](super::eq_abs_primitive_float#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &Rational) -> bool {
                other.eq_abs(self)
            }
        }
    };
}
apply_to_primitive_floats!(impl_eq_abs);
