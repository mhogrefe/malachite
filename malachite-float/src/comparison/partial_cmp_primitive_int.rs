// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_nz::natural::Natural;

fn float_partial_cmp_unsigned<T: PrimitiveUnsigned>(x: &Float, y: &T) -> Option<Ordering>
where
    Natural: From<T>,
{
    match (x, y) {
        (float_nan!(), _) => None,
        (float_infinity!(), _) => Some(Greater),
        (float_negative_infinity!(), _) => Some(Less),
        (float_either_zero!(), _) => Some(if *y == T::ZERO { Equal } else { Less }),
        (
            Float(Finite {
                sign: s_x,
                exponent: e_x,
                significand: sig_x,
                ..
            }),
            y,
        ) => Some(if !s_x {
            Less
        } else if *y == T::ZERO {
            Greater
        } else if *e_x <= 0 {
            Less
        } else {
            u64::from(e_x.unsigned_abs())
                .cmp(&y.significant_bits())
                .then_with(|| sig_x.cmp_normalized(&Natural::from(*y)))
        }),
    }
}

macro_rules! impl_from_unsigned {
    ($t: ident) => {
        impl PartialOrd<$t> for Float {
            /// Compares a [`Float`] to an unsigned primitive integer.
            ///
            /// NaN is not comparable to any primitive integer. $\infty$ is greater than any
            /// primitive integer, and $-\infty$ is less. Both the [`Float`] zero and the [`Float`]
            /// negative zero are equal to the integer zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_primitive_int#partial_cmp).
            #[inline]
            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                float_partial_cmp_unsigned(self, other)
            }
        }

        impl PartialOrd<Float> for $t {
            /// Compares an unsigned primitive integer to a [`Float`].
            ///
            /// No integer is comparable to NaN. Every integer is smaller than $\infty$ and greater
            /// than $-\infty$. The integer zero is equal to both the [`Float`] zero and the
            /// [`Float`] negative zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `other.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_primitive_int#partial_cmp).
            #[inline]
            fn partial_cmp(&self, other: &Float) -> Option<Ordering> {
                other.partial_cmp(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_unsigneds!(impl_from_unsigned);

fn float_partial_cmp_signed<T: PrimitiveSigned>(x: &Float, y: &T) -> Option<Ordering>
where
    Natural: From<<T as UnsignedAbs>::Output>,
{
    match (x, y) {
        (float_nan!(), _) => None,
        (float_infinity!(), _) => Some(Greater),
        (float_negative_infinity!(), _) => Some(Less),
        (float_either_zero!(), _) => Some(T::ZERO.cmp(y)),
        (
            Float(Finite {
                sign: s_x,
                exponent: e_x,
                significand: sig_x,
                ..
            }),
            y,
        ) => {
            let s_y = *y > T::ZERO;
            let s_cmp = s_x.cmp(&s_y);
            if s_cmp != Equal {
                return Some(s_cmp);
            }
            let abs_cmp = if *y == T::ZERO {
                Greater
            } else if *e_x <= 0 {
                Less
            } else {
                u64::from(e_x.unsigned_abs())
                    .cmp(&y.significant_bits())
                    .then_with(|| sig_x.cmp_normalized(&Natural::from(y.unsigned_abs())))
            };
            Some(if s_y { abs_cmp } else { abs_cmp.reverse() })
        }
    }
}

macro_rules! impl_from_signed {
    ($t: ident) => {
        impl PartialOrd<$t> for Float {
            /// Compares a [`Float`] to a signed primitive integer.
            ///
            /// NaN is not comparable to any primitive integer. $\infty$ is greater than any
            /// primitive integer, and $-\infty$ is less. Both the [`Float`] zero and the [`Float`]
            /// negative zero are equal to the integer zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_primitive_int#partial_cmp).
            #[inline]
            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                float_partial_cmp_signed(self, other)
            }
        }

        impl PartialOrd<Float> for $t {
            /// Compares a signed primitive integer to a [`Float`].
            ///
            /// No integer is comparable to NaN. Every integer is smaller than $\infty$ and greater
            /// than $-\infty$. The integer zero is equal to both the [`Float`] zero and the
            /// [`Float`] negative zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `other.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_primitive_int#partial_cmp).
            #[inline]
            fn partial_cmp(&self, other: &Float) -> Option<Ordering> {
                other.partial_cmp(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_signeds!(impl_from_signed);
