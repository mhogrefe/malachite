// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use core::cmp::Ordering::{self, *};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeZero, Zero};
use malachite_base::num::conversion::traits::IntegerMantissaAndExponent;
use malachite_base::rounding_modes::RoundingMode;

impl Float {
    /// Converts a primitive float to a [`Float`]. If the [`Float`] is nonzero and finite, it has
    /// the specified precision. If rounding is needed, the specified rounding mode is used. An
    /// [`Ordering`] is also returned, indicating whether the returned value is less than, equal to,
    /// or greater than the original value. (Although a NaN is not comparable to any [`Float`],
    /// converting a NaN to a NaN will also return `Equal`, indicating an exact conversion.)
    ///
    /// If you're only using `Nearest`, try using [`Float::from_primitive_float_prec`] instead.
    ///
    /// This function does not overflow or underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.sci_exponent().abs())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is exact and the primitive float cannot be exactly
    /// represented with the specified precision.
    ///
    /// # Examples
    /// See [here](super::from_primitive_float#from_primitive_float_prec_round).
    #[inline]
    pub fn from_primitive_float_prec_round<T: PrimitiveFloat>(
        x: T,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if x.is_nan() {
            (Self::NAN, Equal)
        } else if !x.is_finite() {
            if x.is_sign_positive() {
                (Self::INFINITY, Equal)
            } else {
                (Self::NEGATIVE_INFINITY, Equal)
            }
        } else if x == T::ZERO {
            if x.is_sign_positive() {
                (Self::ZERO, Equal)
            } else {
                (Self::NEGATIVE_ZERO, Equal)
            }
        } else {
            let (m, e) = x.integer_mantissa_and_exponent();
            if x.is_sign_positive() {
                let (f, o) = Self::from_unsigned_prec_round(m, prec, rm);
                (f << e, o)
            } else {
                let (abs, o) = Self::from_unsigned_prec_round(m, prec, -rm);
                (-(abs << e), o.reverse())
            }
        }
    }

    /// Converts a primitive float to a [`Float`]. If the [`Float`] is nonzero and finite, it has
    /// the specified precision. An [`Ordering`] is also returned, indicating whether the returned
    /// value is less than, equal to, or greater than the original value. (Although a NaN is not
    /// comparable to any [`Float`], converting a NaN to a NaN will also return `Equal`, indicating
    /// an exact conversion.)
    ///
    /// Rounding may occur, in which case `Nearest` is used by default. To specify a rounding mode
    /// as well as a precision, try [`Float::from_primitive_float_prec_round`].
    ///
    /// This function does not overflow or underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.sci_exponent().abs())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// See [here](super::from_primitive_float#from_primitive_float_prec).
    #[inline]
    pub fn from_primitive_float_prec<T: PrimitiveFloat>(x: T, prec: u64) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if x.is_nan() {
            (Self::NAN, Equal)
        } else if !x.is_finite() {
            if x.is_sign_positive() {
                (Self::INFINITY, Equal)
            } else {
                (Self::NEGATIVE_INFINITY, Equal)
            }
        } else if x == T::ZERO {
            if x.is_sign_positive() {
                (Self::ZERO, Equal)
            } else {
                (Self::NEGATIVE_ZERO, Equal)
            }
        } else {
            let (m, e) = x.integer_mantissa_and_exponent();
            if x.is_sign_positive() {
                let (f, o) = Self::from_unsigned_prec(m, prec);
                (f << e, o)
            } else {
                let (abs, o) = Self::from_unsigned_prec(m, prec);
                (-(abs << e), o.reverse())
            }
        }
    }
}

macro_rules! impl_from_primitive_float {
    ($t: ident) => {
        impl From<$t> for Float {
            /// Converts a primitive float to a [`Float`].
            ///
            /// If the primitive float is finite and nonzero, the precision of the [`Float`] is the
            /// minimum possible precision to represent the primitive float exactly. If you want to
            /// specify a different precision, try [`Float::from_primitive_float_prec`]. This may
            /// require rounding, which uses `Nearest` by default. To specify a rounding mode as
            /// well as a precision, try [`Float::from_primitive_float_prec_round`].
            ///
            /// This function does not overflow or underflow.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `x.sci_exponent().abs()`.
            ///
            /// # Examples
            /// See [here](super::from_primitive_float#from).
            #[inline]
            fn from(x: $t) -> Float {
                if x.is_nan() {
                    Float::NAN
                } else if !x.is_finite() {
                    if x.is_sign_positive() {
                        Float::INFINITY
                    } else {
                        Float::NEGATIVE_INFINITY
                    }
                } else if x == 0.0 {
                    if x.is_sign_positive() {
                        Float::ZERO
                    } else {
                        Float::NEGATIVE_ZERO
                    }
                } else {
                    let (m, e) = x.integer_mantissa_and_exponent();
                    let abs = Float::from(m) << e;
                    if x.is_sign_positive() { abs } else { -abs }
                }
            }
        }
    };
}
apply_to_primitive_floats!(impl_from_primitive_float);
