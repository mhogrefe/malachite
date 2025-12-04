// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::{Finite, Zero};
use core::cmp::Ordering::*;
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::EqAbs;
use malachite_nz::natural::Natural;

fn float_eq_abs_unsigned<T: PrimitiveUnsigned>(x: &Float, y: &T) -> bool
where
    Natural: From<T>,
{
    match x {
        float_either_zero!() => *y == T::ZERO,
        Float(Finite {
            exponent,
            significand,
            ..
        }) => {
            *y != T::ZERO
                && *exponent >= 0
                && y.significant_bits() == u64::from(exponent.unsigned_abs())
                && significand.cmp_normalized(&Natural::from(*y)) == Equal
        }
        _ => false,
    }
}

macro_rules! impl_eq_abs_unsigned {
    ($t: ident) => {
        impl EqAbs<$t> for Float {
            /// Determines whether the absolute value of a [`Float`] is equal to an unsigned
            /// primitive integer.
            ///
            /// $\infty$, $-\infty$, and NaN are not equal to any primitive integer. Both the
            /// [`Float`] zero and the [`Float`] negative zero are equal to the integer zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_int#partial_eq).
            #[inline]
            fn eq_abs(&self, other: &$t) -> bool {
                float_eq_abs_unsigned(self, other)
            }
        }

        impl EqAbs<Float> for $t {
            /// Determines whether an unsigned primitive integer is equal to the absolute value of a
            /// [`Float`].
            ///
            /// No primitive integer is equal to $\infty$, $-\infty$, or NaN. The integer zero is
            /// equal to both the [`Float`] zero and the [`Float`] negative zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `other.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_eq_primitive_int#partial_eq).
            #[inline]
            fn eq_abs(&self, other: &Float) -> bool {
                other.eq_abs(self)
            }
        }
    };
}
apply_to_unsigneds!(impl_eq_abs_unsigned);

fn float_eq_abs_signed<T: PrimitiveSigned>(x: &Float, y: &T) -> bool
where
    Natural: From<<T as UnsignedAbs>::Output>,
{
    match x {
        float_either_zero!() => *y == T::ZERO,
        Float(Finite {
            exponent,
            significand,
            ..
        }) => {
            *y != T::ZERO
                && *exponent >= 0
                && y.significant_bits() == u64::from(exponent.unsigned_abs())
                && significand.cmp_normalized(&Natural::from(y.unsigned_abs())) == Equal
        }
        _ => false,
    }
}

macro_rules! impl_eq_abs_signed {
    ($t: ident) => {
        impl EqAbs<$t> for Float {
            /// Determines whether the absolute value of a [`Float`] is equal to the absolute value
            /// of a signed primitive integer.
            ///
            /// $\infty$, $-\infty$, and NaN are not equal to any primitive integer. Both the
            /// [`Float`] zero and the [`Float`] negative zero are equal to the integer zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::eq_abs#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &$t) -> bool {
                float_eq_abs_signed(self, other)
            }
        }

        impl EqAbs<Float> for $t {
            /// Determines whether the absolute value of a signed primitive integer is equal to the
            /// absolute value of a [`Float`].
            ///
            /// No primitive integer is equal to $\infty$, $-\infty$, or NaN. The integer zero is
            /// equal to both the [`Float`] zero and the [`Float`] negative zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `other.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::eq_abs#eq_abs).
            #[inline]
            fn eq_abs(&self, other: &Float) -> bool {
                other.eq_abs(self)
            }
        }
    };
}
apply_to_signeds!(impl_eq_abs_signed);
