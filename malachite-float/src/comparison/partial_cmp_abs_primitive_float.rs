// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use core::cmp::Ordering::{self, *};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_nz::natural::Natural;

fn float_partial_cmp_abs_primitive_float<T: PrimitiveFloat>(x: &Float, y: &T) -> Option<Ordering> {
    match (x, y) {
        (float_nan!(), _) => None,
        (_, y) if y.is_nan() => None,
        (Float(Infinity { .. }), y) if !y.is_finite() => Some(Equal),
        (Float(Infinity { .. }), _) => Some(Greater),
        (_, y) if !y.is_finite() => Some(Less),
        (float_either_zero!(), y) => Some(if *y == T::ZERO { Equal } else { Less }),
        (_, y) if *y == T::ZERO => Some(Greater),
        (
            Float(Finite {
                exponent: e_x,
                significand: m_x,
                ..
            }),
            y,
        ) => Some(
            (i64::from(*e_x) - 1)
                .cmp(&y.sci_exponent())
                .then_with(|| m_x.cmp_normalized(&Natural::from(y.integer_mantissa()))),
        ),
    }
}

macro_rules! impl_partial_cmp_abs_primitive_float {
    ($t: ident) => {
        impl PartialOrdAbs<$t> for Float {
            /// Compares the absolute values of a [`Float`] and a primitive float.
            ///
            /// The [`Float`] NaN is not comparable to any primitive float, not even the primitive
            /// float NaN. Every [`Float`] zero is equal to every primitive float zero, regardless
            /// of sign.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is
            /// `max(self.significant_bits(), other.sci_exponent().abs())`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_abs_primitive_float#partial_cmp_abs).
            #[inline]
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                float_partial_cmp_abs_primitive_float(self, other)
            }
        }

        impl PartialOrdAbs<Float> for $t {
            /// Compares the absolute values of a primitive float and a [`Float`].
            ///
            /// The primitive float NaN is not comparable to any primitive float, not even the
            /// [`Float`] NaN. Every primitive float zero is equal to every [`Float`] zero,
            /// regardless of sign.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is
            /// `max(self.significant_bits(), other.sci_exponent().abs())`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_primitive_float#partial_cmp_abs).
            #[inline]
            fn partial_cmp_abs(&self, other: &Float) -> Option<Ordering> {
                other.partial_cmp_abs(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_primitive_floats!(impl_partial_cmp_abs_primitive_float);
