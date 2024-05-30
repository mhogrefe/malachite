// Copyright © 2024 Mikhail Hogrefe
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
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode;

// This differs from the `precision` function provided by `PrimitiveFloat`. That function returns
// the smallest precision necessary to represent the float, whereas this function returns the
// maximum precision of any float in the same binade. If the float is non-finite or zero, 1 is
// returned.
pub_test! {alt_precision<T: PrimitiveFloat>(x: T) -> u64 {
    if x.is_finite() && x != T::ZERO {
        let (mantissa, exponent) = x.raw_mantissa_and_exponent();
        if exponent == 0 {
            mantissa.significant_bits()
        } else {
            T::MANTISSA_WIDTH + 1
        }
    } else {
        1
    }
}}

impl Float {
    /// Converts a primitive float to a [`Float`]. If the [`Float`] is nonzero and finite, it has
    /// the specified precision. If rounding is needed, the specified rounding mode is used. An
    /// [`Ordering`] is also returned, indicating whether the returned value is less than, equal to,
    /// or greater than the original value. (Although a NaN is not comparable to any [`Float`],
    /// converting a NaN to a NaN will also return `Equal`, indicating an exact conversion.)
    ///
    /// If you're only using `Nearest`, try using [`Float::from_primitive_float_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.sci_exponent().abs())`.
    ///
    /// # Examples
    /// See [here](super::from_primitive_float#from_primitive_float_prec_round).
    #[inline]
    pub fn from_primitive_float_prec_round<T: PrimitiveFloat>(
        x: T,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        assert_ne!(prec, 0);
        if x.is_nan() {
            (Float::NAN, Equal)
        } else if !x.is_finite() {
            if x.is_sign_positive() {
                (Float::INFINITY, Equal)
            } else {
                (Float::NEGATIVE_INFINITY, Equal)
            }
        } else if x == T::ZERO {
            if x.is_sign_positive() {
                (Float::ZERO, Equal)
            } else {
                (Float::NEGATIVE_ZERO, Equal)
            }
        } else {
            let (m, e) = x.integer_mantissa_and_exponent();
            if x.is_sign_positive() {
                let (f, o) = Float::from_unsigned_prec_round(m, prec, rm);
                (f << e, o)
            } else {
                let (abs, o) = Float::from_unsigned_prec_round(m, prec, -rm);
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
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.sci_exponent().abs())`.
    ///
    /// # Examples
    /// See [here](super::from_primitive_float#from_primitive_float_prec).
    #[inline]
    pub fn from_primitive_float_prec<T: PrimitiveFloat>(x: T, prec: u64) -> (Float, Ordering) {
        assert_ne!(prec, 0);
        if x.is_nan() {
            (Float::NAN, Equal)
        } else if !x.is_finite() {
            if x.is_sign_positive() {
                (Float::INFINITY, Equal)
            } else {
                (Float::NEGATIVE_INFINITY, Equal)
            }
        } else if x == T::ZERO {
            if x.is_sign_positive() {
                (Float::ZERO, Equal)
            } else {
                (Float::NEGATIVE_ZERO, Equal)
            }
        } else {
            let (m, e) = x.integer_mantissa_and_exponent();
            if x.is_sign_positive() {
                let (f, o) = Float::from_unsigned_prec(m, prec);
                (f << e, o)
            } else {
                let (abs, o) = Float::from_unsigned_prec(m, prec);
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
            /// If the primitive float is finite and nonzero, the precision of the [`Float`] is
            /// equal to the maximum precision of any primitive float in the same binade (for normal
            /// `f32`s this is 24, and for normal `f64`s it is 53). If you want to specify a
            /// different precision, try [`Float::from_primitive_float_prec`]. This may require
            /// rounding, which uses `Nearest` by default. To specify a rounding mode as well as a
            /// precision, try [`Float::from_primitive_float_prec_round`].
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
                    let abs = Float::from_unsigned_prec(m, alt_precision(x)).0 << e;
                    if x.is_sign_positive() {
                        abs
                    } else {
                        -abs
                    }
                }
            }
        }
    };
}
apply_to_primitive_floats!(impl_from_primitive_float);
